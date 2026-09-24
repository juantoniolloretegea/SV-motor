    fn gather_forward_dequantize(&self, x: &Tensor, indices: &Tensor) -> Result<Tensor> {
        match std::env::var("EIO_MXFP4_MODO").as_deref() {
            Ok("referencia") => self.gather_forward_referencia(x, indices),
            Ok("paralelo") | Err(_) => self.gather_forward_paralelo(x, indices),
            Ok(_) => candle_core::bail!("Modo experimental MXFP4 desconocido"),
        }
    }

    fn gather_forward_paralelo(&self, x: &Tensor, indices: &Tensor) -> Result<Tensor> {
        use rayon::prelude::*;
        let inicio = std::time::Instant::now();
        let xd = x.dims();
        let id = indices.dims();
        if id.len() != 2 || !matches!(xd.len(), 2 | 3) || xd[0] != id[0]
            || (xd.len() == 3 && xd[1] != 1 && xd[1] != id[1]) {
            candle_core::bail!("MXFP4 CPU: formas de entrada y seleccion incompatibles");
        }
        let tokens = xd[0];
        let topk = id[1];
        let k = *xd.last().unwrap();
        let expandida = xd.len() == 3 && xd[1] != 1;
        let (expertos, n, k_half) = self.blocks.dims3()?;
        if k % MXFP4_BLOCK_SIZE != 0 || k_half != k / 2 || self.scales.dims() != [expertos, n, k / MXFP4_BLOCK_SIZE] {
            candle_core::bail!("MXFP4 CPU: dimensiones de pesos incompatibles");
        }
        let blocks_cpu = self.blocks.to_device(&Device::Cpu)?.contiguous()?;
        let scales_cpu = self.scales.to_device(&Device::Cpu)?.contiguous()?;
        let (blocks_storage, blocks_layout) = blocks_cpu.storage_and_layout();
        let (scales_storage, scales_layout) = scales_cpu.storage_and_layout();
        let (bs, be) = blocks_layout.contiguous_offsets().ok_or_else(|| candle_core::Error::Msg("Bloques no contiguos".into()))?;
        let (ss, se) = scales_layout.contiguous_offsets().ok_or_else(|| candle_core::Error::Msg("Escalas no contiguas".into()))?;
        let blocks_data = match &*blocks_storage {
            candle_core::Storage::Cpu(s) => &s.as_slice::<u8>()?[bs..be],
            _ => candle_core::bail!("Almacenamiento CPU ausente"),
        };
        let scales_data = match &*scales_storage {
            candle_core::Storage::Cpu(s) => &s.as_slice::<u8>()?[ss..se],
            _ => candle_core::bail!("Almacenamiento CPU ausente"),
        };
        let x_data = x.to_dtype(DType::F32)?.to_device(&Device::Cpu)?.flatten_all()?.to_vec1::<f32>()?;
        let ids = indices.to_device(&Device::Cpu)?.to_dtype(DType::U32)?.flatten_all()?.to_vec1::<u32>()?;
        if ids.iter().any(|&i| i as usize >= expertos) {
            candle_core::bail!("Indice de experto fuera del intervalo");
        }
        let bias = self.bias.as_ref().map(|b| b.to_dtype(DType::F32)?.to_device(&Device::Cpu)?.flatten_all()?.to_vec1::<f32>()).transpose()?;
        let preparado = inicio.elapsed();
        let bloques_fila = k / MXFP4_BLOCK_SIZE;
        let mut output = vec![0f32; tokens * topk * n];
        output.par_iter_mut().enumerate().for_each(|(pos, out)| {
            let pareja = pos / n;
            let fila = pos % n;
            let token = pareja / topk;
            let experto = ids[pareja] as usize;
            let x_offset = if expandida { pareja * k } else { token * k };
            let peso_offset = (experto * n + fila) * k_half;
            let escala_offset = (experto * n + fila) * bloques_fila;
            let mut suma = 0f32;
            for bloque in 0..bloques_fila {
                let lut = &Self::DEQUANT_LUT[scales_data[escala_offset + bloque] as usize];
                let mut producto = 0f32;
                for byte in 0..MXFP4_BLOCK_SIZE / 2 {
                    let packed = blocks_data[peso_offset + bloque * (MXFP4_BLOCK_SIZE / 2) + byte];
                    let offset = x_offset + bloque * MXFP4_BLOCK_SIZE + byte * 2;
                    producto += x_data[offset] * lut[(packed & 15) as usize]
                        + x_data[offset + 1] * lut[(packed >> 4) as usize];
                }
                suma += producto;
            }
            if let Some(b) = &bias { suma += b[experto * n + fila]; }
            *out = suma;
        });
        let calculado = inicio.elapsed();
        if std::env::var_os("EIO_MXFP4_PERFIL").is_some() {
            eprintln!("{{\"evento\":\"perfil_mxfp4\",\"modo\":\"paralelo\",\"tokens\":{tokens},\"expertos_seleccionados\":{topk},\"filas\":{n},\"dimension\":{k},\"preparacion_ns\":{},\"calculo_ns\":{}}}", preparado.as_nanos(), (calculado-preparado).as_nanos());
        }
        Tensor::from_vec(output, (tokens, topk, n), &Device::Cpu)?.to_device(x.device())?.to_dtype(x.dtype())
    }
