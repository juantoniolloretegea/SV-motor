use std::{collections::BTreeSet, env, fs};

#[derive(Debug, PartialEq)]
struct Score {
    total: usize, aciertos: usize, errores_no_criticos: usize,
    errores_criticos: Vec<String>, indeterminaciones: usize,
    blancos: usize, impedimentos: usize, neto: i64,
}
impl Score {
    fn valor_100(&self) -> f64 { 100.0 * self.neto as f64 / self.total as f64 }
    fn dictamen(&self) -> &'static str {
        if self.errores_criticos.is_empty() { "Sin eliminación por error crítico" }
        else { "No apto" }
    }
}
fn calcular(input: &str) -> Result<Score, String> {
    let mut s=Score{total:0,aciertos:0,errores_no_criticos:0,errores_criticos:vec![],indeterminaciones:0,blancos:0,impedimentos:0,neto:0};
    let mut ids=BTreeSet::new();
    for l in input.lines().skip(1) {
        let p:Vec<_>=l.split('\t').collect();
        if p.len()!=4 || !ids.insert(p[0].to_string()) { return Err("Fila inválida o duplicada".into()); }
        let critica=match p[1] {"true"=>true,"false"=>false,_=>return Err("Criticidad no definida".into())};
        match (p[2],p[3]) {
            ("0","terminada")=>s.aciertos+=1,
            ("1","terminada")=>if critica {s.errores_criticos.push(p[0].to_string())} else {s.errores_no_criticos+=1},
            ("U","terminada")=>s.indeterminaciones+=1,
            ("BLANCO","en_blanco")=>s.blancos+=1,
            ("-","pendiente_por_impedimento_tecnico")=>s.impedimentos+=1,
            _=>return Err(format!("Estado no reconocido: {}",p[0])),
        }
        s.total+=1;
    }
    if s.total==0 {return Err("Banco vacío".into())}
    s.neto=s.aciertos as i64-s.errores_no_criticos as i64;
    Ok(s)
}
fn main(){
    let a:Vec<String>=env::args().collect();
    assert_eq!(a.len(),3,"entrada.tsv salida.json");
    let s=calcular(&fs::read_to_string(&a[1]).unwrap()).unwrap();
    let ids=s.errores_criticos.iter().map(|i|format!("\"{}\"",i)).collect::<Vec<_>>().join(",");
    let out=format!("{{\n  \"preguntas_banco\": {},\n  \"aciertos_0\": {},\n  \"errores_no_criticos_1\": {},\n  \"errores_criticos_1\": {},\n  \"preguntas_criticas_falladas\": [{}],\n  \"indeterminaciones_U\": {},\n  \"blancos\": {},\n  \"impedimentos_fuera_de_terna\": {},\n  \"respuestas_finales\": {},\n  \"puntos_netos\": {},\n  \"score_sobre_100\": {:.6},\n  \"dictamen_por_criticidad\": \"{}\"\n}}\n",
    s.total,s.aciertos,s.errores_no_criticos,s.errores_criticos.len(),ids,s.indeterminaciones,s.blancos,s.impedimentos,s.total-s.blancos-s.impedimentos,s.neto,s.valor_100(),s.dictamen());
    fs::write(&a[2],&out).unwrap();
    print!("{}",out);
}
#[cfg(test)]
mod tests {
    use super::*;
    const H:&str="id\tcritica\tvalor\testado\n";
    #[test] fn resta_error_no_critico(){
        let s=calcular(&format!("{H}P1\tfalse\t0\tterminada\nP2\tfalse\t1\tterminada\n")).unwrap();
        assert_eq!(s.neto,0);assert_eq!(s.valor_100(),0.0);
    }
    #[test] fn un_critico_elimina_aunque_haya_aciertos(){
        let s=calcular(&format!("{H}P1\ttrue\t0\tterminada\nP2\ttrue\t1\tterminada\n")).unwrap();
        assert_eq!(s.neto,1);assert_eq!(s.dictamen(),"No apto");
    }
    #[test] fn u_blanco_impedimento_no_se_confunden(){
        let s=calcular(&format!("{H}P1\tfalse\tU\tterminada\nP2\ttrue\tBLANCO\ten_blanco\nP3\ttrue\t-\tpendiente_por_impedimento_tecnico\nP4\tfalse\t0\tterminada\n")).unwrap();
        assert_eq!((s.indeterminaciones,s.blancos,s.impedimentos,s.neto),(1,1,1,1));
        assert_eq!(s.valor_100(),25.0);assert_ne!(s.dictamen(),"No apto");
    }
    #[test] fn no_oculta_saldo_negativo_ni_admite_duplicados(){
        let s=calcular(&format!("{H}P1\tfalse\t1\tterminada\n")).unwrap();
        assert_eq!(s.neto,-1);assert_eq!(s.valor_100(),-100.0);
        assert!(calcular(&format!("{H}P1\tfalse\t0\tterminada\nP1\tfalse\t0\tterminada\n")).is_err());
    }
}
