fn main() {
    println!("{}", load_module_files().len());
}

fn load_module_files() -> Vec<(String, Vec<u8>)> {
    let entries = std::fs::read_dir("../plugins").expect("should read plugins directory");

    let mut results = vec![];
    for entry in entries {
        let path = entry.expect("should find directory entry").path();
        let name = path
            .file_name()
            .expect("should get filename")
            .to_str()
            .expect("should convert filename to str")
            .to_string();
        let content = std::fs::read(path).expect("should read plugin bytes from file");
        results.push((name, content));
    }

    results
}
