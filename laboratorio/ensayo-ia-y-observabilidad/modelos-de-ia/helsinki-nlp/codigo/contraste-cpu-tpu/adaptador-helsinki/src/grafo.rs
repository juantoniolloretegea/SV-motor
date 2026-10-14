//! Expresión acotada de Marian: operaciones de Candle 0.11.0 y salida StableHLO.
//! No modifica Candle ni el núcleo del SV. Pesos y parámetros se verifican aparte.
use anyhow::{Result, Context, ensure};
use candle::{Tensor, Device};
use std::collections::HashMap;
use std::fmt::Write;

pub type Id = usize;
#[derive(Clone, Debug)]
enum Op {
    Param(String), Scalar(f32), Add(Id,Id), Sub(Id,Id), Mul(Id,Id), Div(Id,Id),
    Exp(Id), Rsqrt(Id), Neg(Id), Reshape(Id), Transpose(Id,Vec<usize>),
    Broadcast(Id,Vec<usize>), Sum(Id,usize), Max(Id,usize), Dot(Id,Id),
    Gather(Id,Id), Row(Id,Id),
}
#[derive(Clone,Debug)]
struct Node { op: Op, shape: Vec<usize>, integer: bool }
pub struct Graph {
    nodes: Vec<Node>, pub params: Vec<(String,Id)>, pub weights: HashMap<String,Tensor>,
    pub output: Id, pub name: String,
}
impl Graph {
    fn new(name: &str, weights: &HashMap<String,Tensor>) -> Self {
        Self{nodes:vec![],params:vec![],weights:weights.clone(),output:0,name:name.into()}
    }
    fn add(&mut self,op:Op,shape:Vec<usize>,integer:bool)->Id {
        let i=self.nodes.len(); self.nodes.push(Node{op,shape,integer}); i
    }
    fn param(&mut self,key:&str,shape:Vec<usize>,integer:bool)->Id {
        if let Some((_,i))=self.params.iter().find(|(k,_)|k==key){return *i}
        let i=self.add(Op::Param(key.into()),shape,integer); self.params.push((key.into(),i)); i
    }
    fn weight(&mut self,key:&str)->Id {
        let t=self.weights.get(key).unwrap_or_else(||panic!("Peso ausente: {key}"));
        self.param(key,t.dims().to_vec(),false)
    }
    fn scalar(&mut self,v:f32)->Id {self.add(Op::Scalar(v),vec![],false)}
    fn shape(&self,i:Id)->Vec<usize>{self.nodes[i].shape.clone()}
    fn reshape(&mut self,i:Id,shape:Vec<usize>)->Id {
        assert_eq!(self.nodes[i].shape.iter().product::<usize>(),shape.iter().product::<usize>());
        self.add(Op::Reshape(i),shape,self.nodes[i].integer)
    }
    fn transpose(&mut self,i:Id,p:Vec<usize>)->Id {
        let shape=p.iter().map(|&d|self.nodes[i].shape[d]).collect();
        self.add(Op::Transpose(i,p),shape,false)
    }
    fn broadcast(&mut self,i:Id,shape:Vec<usize>,axes:Vec<usize>)->Id {
        self.add(Op::Broadcast(i,axes),shape,false)
    }
    fn bin(&mut self,a:Id,b:Id,kind:&str)->Id {
        let sa=self.shape(a); let sb=self.shape(b);
        let (a,b,shape)=if sa==sb {(a,b,sa)} else if sa.len()>=sb.len() {
            let axes=(sa.len()-sb.len()..sa.len()).collect();
            let b=self.broadcast(b,sa.clone(),axes); (a,b,sa)
        }else{let axes=(sb.len()-sa.len()..sb.len()).collect();let a=self.broadcast(a,sb.clone(),axes);(a,b,sb)};
        let op=match kind{"add"=>Op::Add(a,b),"sub"=>Op::Sub(a,b),"mul"=>Op::Mul(a,b),"div"=>Op::Div(a,b),_=>unreachable!()};
        self.add(op,shape,false)
    }
    fn sum(&mut self,x:Id,axis:usize,max:bool)->Id {
        let mut s=self.shape(x);s.remove(axis);
        self.add(if max{Op::Max(x,axis)}else{Op::Sum(x,axis)},s,false)
    }
    fn dot(&mut self,a:Id,b:Id)->Id {
        let sa=self.shape(a);let sb=self.shape(b);assert_eq!(sa.len(),sb.len());
        assert_eq!(sa[sa.len()-1],sb[sb.len()-2]);
        let mut s=sa;s.pop();s.push(*sb.last().unwrap());self.add(Op::Dot(a,b),s,false)
    }
    fn linear(&mut self,x:Id,key:&str)->Id {
        let w=self.weight(&format!("{key}.weight"));let w=self.transpose(w,vec![1,0]);
        let y=self.dot(x,w);let b=self.weight(&format!("{key}.bias"));self.bin(y,b,"add")
    }
    fn norm(&mut self,x:Id,key:&str)->Id {
        let shape=self.shape(x);let axis=shape.len()-1;
        let n=self.scalar(shape[axis] as f32);let sum=self.sum(x,axis,false);let mean=self.bin(sum,n,"div");
        let mean=self.broadcast(mean,shape.clone(),(0..axis).collect());
        let c=self.bin(x,mean,"sub");let sq=self.bin(c,c,"mul");let sum=self.sum(sq,axis,false);let var=self.bin(sum,n,"div");
        let eps=self.scalar(1e-5);let var=self.bin(var,eps,"add");let inv=self.add(Op::Rsqrt(var),self.shape(var),false);
        let inv=self.broadcast(inv,shape,(0..axis).collect());let normalized=self.bin(c,inv,"mul");
        let w=self.weight(&format!("{key}.weight"));let b=self.weight(&format!("{key}.bias"));
        let y=self.bin(normalized,w,"mul");self.bin(y,b,"add")
    }
    fn softmax(&mut self,x:Id)->Id {
        let shape=self.shape(x);let ax=shape.len()-1;
        let max=self.sum(x,ax,true);let max=self.broadcast(max,shape.clone(),(0..ax).collect());
        let sub=self.bin(x,max,"sub");let exp=self.add(Op::Exp(sub),shape.clone(),false);
        let sum=self.sum(exp,ax,false);let sum=self.broadcast(sum,shape,(0..ax).collect());self.bin(exp,sum,"div")
    }
    fn heads(&mut self,x:Id)->Id {let n=self.nodes[x].shape[0];let x=self.reshape(x,vec![n,8,64]);self.transpose(x,vec![1,0,2])}
    fn attention(&mut self,x:Id,kv:Id,key:&str,mask:Option<Id>)->Id {
        let q=self.linear(x,&format!("{key}.q_proj"));let scale=self.scalar(0.125);let q=self.bin(q,scale,"mul");let q=self.heads(q);
        let k=self.linear(kv,&format!("{key}.k_proj"));let k=self.heads(k);let k=self.transpose(k,vec![0,2,1]);
        let v=self.linear(kv,&format!("{key}.v_proj"));let v=self.heads(v);
        let mut score=self.dot(q,k);if let Some(m)=mask{score=self.bin(score,m,"add")}
        let prob=self.softmax(score);let y=self.dot(prob,v);let y=self.transpose(y,vec![1,0,2]);
        let y=self.reshape(y,vec![self.nodes[x].shape[0],512]);self.linear(y,&format!("{key}.out_proj"))
    }
    fn ffn(&mut self,x:Id,key:&str)->Id {
        let y=self.linear(x,&format!("{key}.fc1"));let neg=self.add(Op::Neg(y),self.shape(y),false);
        let exp=self.add(Op::Exp(neg),self.shape(y),false);let one=self.scalar(1.);let den=self.bin(exp,one,"add");
        let y=self.bin(y,den,"div");let y=self.linear(y,&format!("{key}.fc2"));let y=self.bin(x,y,"add");
        self.norm(y,&format!("{key}.final_layer_norm"))
    }
    fn embed(&mut self,len:usize)->Id {
        let ids=self.param("ids",vec![len],true);let w=self.weight("model.shared.weight");
        let e=self.add(Op::Gather(w,ids),vec![len,512],false);let scale=self.scalar(512f64.sqrt() as f32);let e=self.bin(e,scale,"mul");
        let pos=self.weight("sv.positions");self.bin(e,pos,"add")
    }
    pub fn encoder(w:&HashMap<String,Tensor>,len:usize)->Result<Self> {
        let mut g=Self::new(&format!("sv_helsinki_encoder_{len}"),w);g.positions(len)?;
        let mut x=g.embed(len);
        for i in 0..6 {let k=format!("model.encoder.layers.{i}");let y=g.attention(x,x,&format!("{k}.self_attn"),None);
            let y=g.bin(x,y,"add");let y=g.norm(y,&format!("{k}.self_attn_layer_norm"));x=g.ffn(y,&k);}
        g.output=x;Ok(g)
    }
    pub fn decoder(w:&HashMap<String,Tensor>,source_len:usize,len:usize)->Result<Self> {
        let mut g=Self::new(&format!("sv_helsinki_decoder_{source_len}_{len}"),w);g.positions(len)?;
        let mask=(0..len).flat_map(|r|(0..len).map(move|c|if c>r{f32::NEG_INFINITY}else{0.})).collect::<Vec<_>>();
        g.weights.insert("sv.causal".into(),Tensor::from_vec(mask,(len,len),&Device::Cpu)?);
        let mask=g.weight("sv.causal");let enc=g.param("encoder",vec![source_len,512],false);let mut x=g.embed(len);
        for i in 0..6 {let k=format!("model.decoder.layers.{i}");let y=g.attention(x,x,&format!("{k}.self_attn"),Some(mask));
            let y=g.bin(x,y,"add");let x1=g.norm(y,&format!("{k}.self_attn_layer_norm"));
            let y=g.attention(x1,enc,&format!("{k}.encoder_attn"),None);let y=g.bin(x1,y,"add");
            let y=g.norm(y,&format!("{k}.encoder_attn_layer_norm"));x=g.ffn(y,&k);}
        let pos=g.param("position",vec![],true);let x=g.add(Op::Row(x,pos),vec![1,512],false);
        let w=g.weight("model.shared.weight");let w=g.transpose(w,vec![1,0]);let logits=g.dot(x,w);
        let bias=g.weight("final_logits_bias");g.output=g.bin(logits,bias,"add");Ok(g)
    }
    fn positions(&mut self,len:usize)->Result<()> {
        let inv=(0..256).map(|i|1f32/10000f32.powf((2*i)as f32/512.)).collect::<Vec<_>>();
        let t=Tensor::arange(0u32,len as u32,&Device::Cpu)?.to_dtype(candle::DType::F32)?.reshape((len,1))?;
        let inv=Tensor::from_vec(inv,(1,256),&Device::Cpu)?;let f=t.matmul(&inv)?;
        self.weights.insert("sv.positions".into(),Tensor::cat(&[f.sin()?,f.cos()?],1)?.contiguous()?);Ok(())
    }
    pub fn evaluate(&self,inputs:&HashMap<String,Tensor>)->Result<Tensor> {
        let mut values:Vec<Tensor>=Vec::with_capacity(self.nodes.len());
        for n in &self.nodes {
            let v=&values;
            let t=match &n.op {
                Op::Param(k)=>inputs.get(k).or_else(||self.weights.get(k)).context("Parametro ausente")?.clone(),
                Op::Scalar(s)=>Tensor::new(*s,&Device::Cpu)?,
                Op::Add(a,b)=>(&v[*a]+&v[*b])?, Op::Sub(a,b)=>(&v[*a]-&v[*b])?,
                Op::Mul(a,b)=>(&v[*a]*&v[*b])?, Op::Div(a,b)=>(&v[*a]/&v[*b])?,
                Op::Exp(a)=>v[*a].exp()?,Op::Neg(a)=>v[*a].neg()?,Op::Rsqrt(a)=>v[*a].sqrt()?.recip()?,
                Op::Reshape(a)=>v[*a].reshape(n.shape.as_slice())?,
                Op::Transpose(a,p)=>v[*a].permute(p.clone())?.contiguous()?,
                Op::Broadcast(a,axes)=>{let mut s=vec![1;n.shape.len()];for (j,&ax) in axes.iter().enumerate(){s[ax]=v[*a].dims()[j]};v[*a].reshape(s)?.broadcast_as(n.shape.as_slice())?.contiguous()?},
                Op::Sum(a,ax)=>v[*a].sum(*ax)?,Op::Max(a,ax)=>v[*a].max(*ax)?,
                Op::Dot(a,b)=>v[*a].matmul(&v[*b])?,
                Op::Gather(a,b)=>v[*a].index_select(&v[*b],0)?,
                Op::Row(a,b)=>v[*a].narrow(0,v[*b].to_scalar::<u32>()? as usize,1)?,
            };ensure!(t.dims()==n.shape,"Forma no conforme");values.push(t);
        }Ok(values[self.output].clone())
    }
    pub fn mlir(&self)->String {
        let mut s=String::new();let ty=|i:Id|self.ty(i);let val=|i:Id|format!("%v{i}");
        writeln!(s,"module @{} {{\n func.func @main({}) -> {} {{",self.name,self.params.iter().map(|(_,i)|format!("%v{i}: {}",ty(*i))).collect::<Vec<_>>().join(", "),ty(self.output)).unwrap();
        for (i,n) in self.nodes.iter().enumerate(){
            let expr=match &n.op {
                Op::Param(_)=>continue,
                Op::Scalar(f)=>format!("stablehlo.constant dense<0x{:08X}> : tensor<f32>",f.to_bits()),
                Op::Add(a,b)|Op::Sub(a,b)|Op::Mul(a,b)|Op::Div(a,b)=>{
                    let op=match n.op{Op::Add(..)=>"add",Op::Sub(..)=>"subtract",Op::Mul(..)=>"multiply",_=>"divide"};
                    format!("stablehlo.{op} {}, {} : {}",val(*a),val(*b),ty(i))},
                Op::Exp(a)|Op::Neg(a)|Op::Rsqrt(a)=>{let op=match n.op{Op::Exp(_)=>"exponential",Op::Neg(_)=>"negate",_=>"rsqrt"};format!("stablehlo.{op} {} : {}",val(*a),ty(i))},
                Op::Reshape(a)=>format!("stablehlo.reshape {} : ({}) -> {}",val(*a),ty(*a),ty(i)),
                Op::Transpose(a,p)=>format!("stablehlo.transpose {}, dims = {:?} : ({}) -> {}",val(*a),p,ty(*a),ty(i)),
                Op::Broadcast(a,p)=>format!("stablehlo.broadcast_in_dim {}, dims = {:?} : ({}) -> {}",val(*a),p,ty(*a),ty(i)),
                Op::Sum(a,ax)|Op::Max(a,ax)=>{
                    let (op,init)=if matches!(n.op,Op::Sum(..)){("add","0.000000e+00")}else{("maximum","0xFF800000")};
                    writeln!(s," %init{i} = stablehlo.constant dense<{init}> : tensor<f32>").unwrap();
                    format!("stablehlo.reduce({} init: %init{i}) applies stablehlo.{op} across dimensions = [{ax}] : ({}, tensor<f32>) -> {}",val(*a),ty(*a),ty(i))},
                Op::Dot(a,b)=>{let rank=self.nodes[*a].shape.len();let dims=if rank==2{"lhs_contracting_dimensions = [1], rhs_contracting_dimensions = [0]"}else{"lhs_batching_dimensions = [0], rhs_batching_dimensions = [0], lhs_contracting_dimensions = [2], rhs_contracting_dimensions = [1]"};
                    format!("\"stablehlo.dot_general\"({}, {}) {{dot_dimension_numbers = #stablehlo.dot<{dims}>, precision_config = [#stablehlo<precision HIGHEST>, #stablehlo<precision HIGHEST>]}} : ({}, {}) -> {}",val(*a),val(*b),ty(*a),ty(*b),ty(i))},
                Op::Gather(a,b)=>format!("\"stablehlo.gather\"({}, {}) {{dimension_numbers = #stablehlo.gather<offset_dims = [1], collapsed_slice_dims = [0], start_index_map = [0], index_vector_dim = 1>, indices_are_sorted = false, slice_sizes = array<i64: 1, 512>}} : ({}, {}) -> {}",val(*a),val(*b),ty(*a),ty(*b),ty(i)),
                Op::Row(a,b)=>{writeln!(s," %zero{i} = stablehlo.constant dense<0> : tensor<i32>").unwrap();format!("\"stablehlo.dynamic_slice\"({}, {}, %zero{i}) {{slice_sizes = array<i64: 1, 512>}} : ({}, tensor<i32>, tensor<i32>) -> {}",val(*a),val(*b),ty(*a),ty(i))},
            };writeln!(s," %v{i} = {expr}").unwrap();
        }writeln!(s," return {} : {}\n }}\n}}",val(self.output),ty(self.output)).unwrap();s
    }
    fn ty(&self,i:Id)->String {let n=&self.nodes[i];let dims=n.shape.iter().map(|d|format!("{d}x")).collect::<String>();format!("tensor<{dims}{}>",if n.integer{"i32"}else{"f32"})}
}
