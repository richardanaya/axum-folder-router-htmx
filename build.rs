use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let api_root = Path::new("src/api");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("routes.rs");
    let mut code = String::new();

    fn visit_dirs(dir: &Path, base: &Path, code: &mut String) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, base, code);
            } else if path.file_name().unwrap() == "route.rs" {
                let rel_dir = path.parent().unwrap().strip_prefix(base).unwrap();

                let (axum_path, mod_name) = if rel_dir.components().count() == 0 {
                    ("/".to_string(), "root".to_string())
                } else {
                    let mut axum_path = String::new();
                    let mut mod_name = String::new();

                    for segment in rel_dir.iter() {
                        let s = segment.to_str().unwrap();
                        if s.starts_with('[') && s.ends_with(']') {
                            let inner = &s[1..s.len() - 1];
                            if inner.starts_with("...") {
                                let param = &inner[3..];
                                axum_path.push_str(&format!("/*{}", param));
                                mod_name.push_str(&format!("__{}", param));
                            } else {
                                axum_path.push_str(&format!("/:{}", inner));
                                mod_name.push_str(&format!("__{}", inner));
                            }
                        } else {
                            axum_path.push('/');
                            axum_path.push_str(s);
                            mod_name.push_str("__");
                            mod_name.push_str(s);
                        }
                    }

                    (axum_path, mod_name.trim_start_matches('_').to_string())
                };

                code.push_str(&format!("mod {} {{ include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/api/{}/route.rs\")); }}\n", mod_name, rel_dir.display()));

                let methods = ["get", "post", "put", "delete", "patch"];
                for method in &methods {
                    code.push_str(&format!(
                        "if let Ok(f) = {}::{} {{ router = router.route(\"{}\", axum::routing::{}(f)); }}\n",
                        mod_name, method, axum_path, method
                    ));
                }
            }
        }
    }

    code.push_str(
        r#"
use axum::Router;

fn maybe_fn<F>(f: F) -> Option<F> {
    Some(f)
}

pub fn build_router() -> Router {
    let mut router = Router::new();
"#,
    );

    visit_dirs(api_root, api_root, &mut code);
    code.push_str("    router\n}\n");

    fs::write(dest_path, code).unwrap();
    println!("cargo:rerun-if-changed=src/api");
}
