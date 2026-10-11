// Instrumento de diagnóstico; no modifica ni integra componentes del SV.
use std::{fs, path::Path, time::Instant};
use logicaffeine_language::{analysis, mwe, semantics, pragmatics, Arena, AstContext,
    Interner, Lexer, Parser, WorldState, SymbolRegistry, OutputFormat, parser::NounParsing};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
type Error = Box<dyn std::error::Error>;
fn guardar(p: &Path, value: &Value) -> Result<(), Error> {
    fs::write(p, serde_json::to_vec_pretty(value)?)?; Ok(())
}
fn identidad(p: &Path, nombre: &str) -> Result<Value, Error> {
    let b = fs::read(p)?;
    Ok(json!({"archivo":nombre,"bytes":b.len(),"sha256":format!("{:x}",Sha256::digest(&b))}))
}
fn representar(ast: &logicaffeine_language::ast::logic::LogicExpr<'_>, i: &Interner) -> Value {
    let mut r = SymbolRegistry::new();
    json!({"estructura":format!("{ast:#?}"),"formula":ast.transpile_discourse(&mut r,i,OutputFormat::Unicode)})
}
fn trazar(texto: &str, nominal: bool) -> Value {
    let mut interner = Interner::new();
    let tokens = Lexer::new(texto,&mut interner).tokenize();
    let lexico: Vec<Value> = tokens.iter().map(|t| json!({"texto":interner.resolve(t.lexeme),"tipo":format!("{:?}",t.kind),"posicion":format!("{:?}",t.span)})).collect();
    let tokens = mwe::apply_mwe_pipeline(tokens,&mwe::build_mwe_trie(),&mut interner);
    let compuestos: Vec<Value> = tokens.iter().map(|t| json!({"texto":interner.resolve(t.lexeme),"tipo":format!("{:?}",t.kind),"posicion":format!("{:?}",t.span)})).collect();
    let tipos = analysis::DiscoveryPass::new(&tokens,&mut interner).run();
    let a=Arena::new(); let b=Arena::new(); let c=Arena::new();
    let d=Arena::new(); let e=Arena::new(); let f=Arena::new();
    let ctx=AstContext::new(&a,&b,&c,&d,&e,&f);
    let mut estado=WorldState::new();
    let mut parser=Parser::new(tokens,&mut estado,&mut interner,ctx,tipos);
    if nominal {
        return match parser.parse_noun_phrase(true) {
            Ok(np)=> {
                let modificadores: Vec<Value>=np.pps.iter().map(|p| representar(p,&interner)).collect();
                let adjetivos: Vec<&str>=np.adjectives.iter().map(|s| interner.resolve(*s)).collect();
                json!({"sustantivo":interner.resolve(np.noun),"determinacion":format!("{:?}",np.definiteness),"adjetivos":adjetivos,"complementos":modificadores})
            },
            Err(e)=>json!({"error":format!("{e:?}")})
        };
    }
    let ast=match parser.parse() {
        Ok(v)=>v,
        Err(e)=>return json!({"lexico":lexico,"expresiones_compuestas":compuestos,"error_analisis":format!("{e:?}")})
    };
    let inicial=representar(ast,&interner);
    let ast=semantics::apply_axioms(ast,ctx.exprs,ctx.terms,&mut interner);
    let axiomas=representar(ast,&interner);
    let ast=pragmatics::apply_pragmatics(ast,ctx.exprs,&interner);
    let final_=representar(ast,&interner);
    let restricciones:Vec<String>=estado.time_constraints().iter().map(|c| match c.relation {
        logicaffeine_language::drs::TimeRelation::Precedes=>format!("Precedes({}, {})",c.left,c.right),
        logicaffeine_language::drs::TimeRelation::Equals=>format!("{}={}",c.left,c.right),
    }).collect();
    let formula=final_["formula"].as_str().unwrap();
    let completa=if restricciones.is_empty(){formula.to_string()}else{format!("{} ∧ {}",formula,restricciones.join(" ∧ "))};
    json!({"lexico":lexico,"expresiones_compuestas":compuestos,"analisis":inicial,"axiomas":axiomas,"pragmatica":final_,"restricciones_temporales":restricciones,"formula_completa":completa})
}
fn main()->Result<(),Error>{
    let a:Vec<String>=std::env::args().collect();
    if a.len()!=5{return Err("Uso: diagnostico EVIDENCIA_ANTERIOR FUENTE_CANDIDATO DIRECTORIO_PROPIO SALIDA".into())}
    let anterior=Path::new(&a[1]); let fuente=Path::new(&a[2]); let propio=Path::new(&a[3]); let salida=Path::new(&a[4]);
    fs::create_dir_all(salida)?;
    let mut fijacion=vec![];
    for n in ["ENTRADAS-ADMISION.json","EJECUCION-01-SALIDA.json"] {fijacion.push(identidad(&anterior.join(n),n)?)}
    for n in ["Cargo.toml","Cargo.lock","src/main.rs"] {fijacion.push(identidad(&propio.join(n),&format!("instrumento/{n}"))?)}
    fijacion.push(identidad(&salida.join("PROTOCOLO-DIAGNOSTICO.md"),"PROTOCOLO-DIAGNOSTICO.md")?);
    for n in [".cargo_vcs_info.json","src/compile.rs","src/parser/mod.rs","src/parser/noun.rs","src/parser/quantifier.rs","src/parser/modal.rs","src/transpile.rs"] {fijacion.push(identidad(&fuente.join(n),&format!("candidato/{n}"))?)}
    guardar(&salida.join("FIJACION.json"),&json!({"previa":true,"archivos":fijacion}))?;
    let entradas:Vec<Value>=serde_json::from_slice(&fs::read(anterior.join("ENTRADAS-ADMISION.json"))?)?;
    let previo:Value=serde_json::from_slice(&fs::read(anterior.join("EJECUCION-01-SALIDA.json"))?)?;
    let mut resultados=vec![];
    let mut identicos=0; let mut trazas_conformes=0;
    for entrada in entradas {
        let id=entrada["id"].as_str().ok_or("Identificador ausente")?;
        let texto=entrada["texto"].as_str().ok_or("Texto ausente")?;
        let inicio=Instant::now();
        let publico=match logicaffeine_language::compile(texto){Ok(v)=>json!({"estado":"admitido","salida":v}),Err(e)=>json!({"estado":"rechazado","error":format!("{e:?}")})};
        let traza=trazar(texto,false);
        let duracion=inicio.elapsed().as_nanos();
        let original=previo["resultados"].as_array().ok_or("Resultados ausentes")?.iter().find(|v|v["entrada"]["id"]==id).ok_or("Caso previo ausente")?;
        let igual=original["analisis"]["resultado"]==publico;
        let traza_igual=if publico["estado"]=="admitido"{publico["salida"]==traza["formula_completa"]}else{publico["error"]==traza["error_analisis"]};
        if igual{identicos+=1} if traza_igual{trazas_conformes+=1}
        let nominal=if id=="M01"||id=="M02"{trazar(texto,true)}else{Value::Null};
        resultados.push(json!({"entrada":entrada,"publico":publico,"identico_al_antecedente":igual,"reproduccion_por_etapas":traza_igual,"traza":traza,"analisis_nominal_separado":nominal,"duracion_ns":duracion}));
    }
    guardar(&salida.join("DIAGNOSTICO.json"),&json!({"resultados":resultados,"limite":"Diagnóstico sobre casos ya conocidos; no constituye una nueva validación independiente."}))?;
    let conforme=identicos==14&&trazas_conformes==14;
    guardar(&salida.join("COTEJO-DIAGNOSTICO-RUST.json"),&json!({"casos":14,"identicos_al_antecedente":identicos,"reproducciones_por_etapas":trazas_conformes,"conforme_instrumentalmente":conforme,"limite":"Comprueba identidad de resultados y reproducción de etapas; no decide conservación semántica."}))?;
    if !conforme{return Err("Discrepancia con el resultado público o el antecedente".into())}
    println!("14 antecedentes idénticos; 14 reproducciones por etapas concordantes."); Ok(())
}
