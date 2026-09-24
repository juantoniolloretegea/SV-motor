
#[cfg(test)]
mod eio_comparacion_mxfp4 {
    use super::*;
    #[test]
    fn equivalencia_bit_a_bit_con_entradas_compartidas_y_expandidas() -> Result<()> {
        let dev = Device::Cpu;
        let (e,n,k,t,top) = (4usize,7usize,64usize,3usize,4usize);
        let packed: Vec<u8> = (0..e*n*k/2).map(|i| ((i*37+19)%256) as u8).collect();
        let scales: Vec<u8> = (0..e*n*k/32).map(|i| (123+i%8) as u8).collect();
        let values: Vec<f32> = (0..t*k).map(|i| ((i*17%31) as f32-15.)/8.).collect();
        let shared = Tensor::from_vec(values,(t,1,k),&dev)?;
        let ids = Tensor::from_vec(vec![0u32,3,1,2,2,1,3,0,3,0,2,1],(t,top),&dev)?;
        for with_bias in [false,true] {
            let bias = if with_bias {Some(Tensor::from_vec((0..e*n).map(|i|i as f32/16.).collect::<Vec<_>>(),(e,n),&dev)?)} else {None};
            let layer = MXFP4Layer::from_parts(Tensor::from_vec(packed.clone(),(e,n,k/2),&dev)?,Tensor::from_vec(scales.clone(),(e,n,k/32),&dev)?,bias);
            for input in [shared.clone(),shared.squeeze(1)?,shared.broadcast_as((t,top,k))?.contiguous()?] {
                let a=layer.gather_forward_referencia(&input,&ids)?;
                let b=layer.gather_forward_paralelo(&input,&ids)?;
                assert_eq!(a.dims(),b.dims());
                assert_eq!(a.flatten_all()?.to_vec1::<f32>()?.iter().map(|x|x.to_bits()).collect::<Vec<_>>(),b.flatten_all()?.to_vec1::<f32>()?.iter().map(|x|x.to_bits()).collect::<Vec<_>>());
            }
        }
        Ok(())
    }
    #[test]
    fn rechaza_indice_de_experto_fuera_de_intervalo() -> Result<()> {
        let d=Device::Cpu;
        let layer=MXFP4Layer::from_parts(Tensor::from_vec(vec![0x22u8;32],(2,1,16),&d)?,Tensor::from_vec(vec![127u8;2],(2,1,1),&d)?,None);
        let x=Tensor::from_vec(vec![1f32;32],(1,1,32),&d)?;
        let ids=Tensor::from_vec(vec![2u32],(1,1),&d)?;
        assert!(layer.gather_forward_paralelo(&x,&ids).is_err());
        Ok(())
    }
}
