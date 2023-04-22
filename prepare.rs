use std::fs::{read_dir, write};

fn main() {
    let path = read_dir("./twemoji/assets/72x72").unwrap();
    let mut v = Vec::new();
    let mut w = Vec::new();
    w.push("match x.as_str() {".to_string());
    for entry in path {
        let entry = entry.unwrap();
        let path = entry.path().into_os_string().into_string().unwrap();
        let len = entry.metadata().unwrap().len();
        let name = &entry.file_name().into_string().unwrap().replace(".png", "");
        let char = name.clone();
        let name = format!("S_{}", name.replace("-", "_").to_uppercase());
        v.push(format!(
            "const {}:&[u8;{}] = include_bytes!(\".{}\");",
            name, len, path
        ));
        w.push(format!("\"{}\" => Some({}.to_vec()),", char, name));
    }
    w.push("_=>None,}".to_string());
    let v = v.join("");
    let w = w.join("");
    let v = format!("{}pub fn get(x: String) -> Option<Vec<u8>>{{{}}}", v, w);
    write("./src/dist.rs", v.as_bytes()).unwrap();
}
