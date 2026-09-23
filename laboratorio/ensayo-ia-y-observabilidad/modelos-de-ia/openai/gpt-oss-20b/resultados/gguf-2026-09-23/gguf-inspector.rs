use std::{env,fs::File,io::{self,Read,BufReader}};
fn bytes<const N:usize>(r:&mut impl Read)->io::Result<[u8;N]>{let mut b=[0;N];r.read_exact(&mut b)?;Ok(b)}
fn u32v(r:&mut impl Read)->io::Result<u32>{Ok(u32::from_le_bytes(bytes(r)?))}
fn u64v(r:&mut impl Read)->io::Result<u64>{Ok(u64::from_le_bytes(bytes(r)?))}
fn string(r:&mut impl Read)->io::Result<String>{let n=u64v(r)?;if n>100_000_000{return Err(io::Error::other("cadena_excesiva"));}let mut b=vec![0;n as usize];r.read_exact(&mut b)?;String::from_utf8(b).map_err(io::Error::other)}
fn value(r:&mut impl Read,t:u32,key:&str)->io::Result<()>{
 match t {
 8=>{let s=string(r)?;if key.contains("chat_template")||key.ends_with("model")||key.ends_with("pre"){println!("{key}={s:?}");}},
 9=>{let item=u32v(r)?;let n=u64v(r)?;if key=="tokenizer.ggml.tokens"{println!("vocab_size={n}");for i in 0..n{let s=string(r)?;if (199990..200030).contains(&i){println!("token[{i}]={s:?}");}}}else{for _ in 0..n{value(r,item,"")?;}}},
 0|1|7=>{let _=bytes::<1>(r)?;},2|3=>{let _=bytes::<2>(r)?;},4|5|6=>{let b=bytes::<4>(r)?;if key.contains("token_id"){println!("{key}={}",u32::from_le_bytes(b));}},10|11|12=>{let _=bytes::<8>(r)?;},
 _=>return Err(io::Error::other("tipo_GGUF_desconocido"))
 }Ok(())
}
fn main()->io::Result<()>{let path=env::args().nth(1).ok_or(io::Error::other("falta_ruta"))?;let mut r=BufReader::new(File::open(path)?);if bytes::<4>(&mut r)?!=*b"GGUF"{return Err(io::Error::other("cabecera_invalida"));}println!("version={}",u32v(&mut r)?);let _=u64v(&mut r)?;let n=u64v(&mut r)?;for _ in 0..n{let key=string(&mut r)?;let t=u32v(&mut r)?;value(&mut r,t,&key)?;}Ok(())}
