use super::*;
fn actual() -> Value { json!({"object":"list","data":[{"id":"default","object":"model","created":1790196093,"owned_by":"local","root":"default"},{"id":"gguf","object":"model","created":1790196093,"owned_by":"local","root":"gguf","status":"loaded"}]}) }
#[test] fn accepts_observed_model_and_default_alias() { assert_eq!(select_loaded_gguf(&actual()).unwrap(), "gguf"); }
#[test] fn rejects_only_default_alias() { let mut v=actual(); v["data"].as_array_mut().unwrap().pop(); assert!(select_loaded_gguf(&v).is_err()); }
#[test] fn rejects_two_real_models() { let mut v=actual(); let duplicate=v["data"][1].clone(); v["data"].as_array_mut().unwrap().push(duplicate); assert!(select_loaded_gguf(&v).is_err()); }
#[test] fn rejects_unloaded_model() { let mut v=actual(); v["data"][1]["status"]=json!("unloaded"); assert!(select_loaded_gguf(&v).is_err()); }
#[test] fn rejects_different_model() { let mut v=actual(); v["data"][1]["id"]=json!("other"); assert!(select_loaded_gguf(&v).is_err()); }
