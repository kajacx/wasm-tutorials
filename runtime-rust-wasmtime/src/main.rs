use wasmtime::*;

fn main() {
    for (name, bytes) in load_module_files() {
        run_module(&name, &bytes);
    }
}

fn run_module(name: &str, bytes: &[u8]) {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), bytes).expect("should create module");

    let instance = Instance::new(&mut store, &module, &[]).expect("should create instance");

    let rate_number = instance
        .get_typed_func::<i32, i32>(&mut store, "rate_number")
        .expect("should get rate_number exported function");

    for number in [15, 8, -20, 162, 1023] {
        let rating = rate_number
            .call(&mut store, number)
            .expect("should call rate_number");
        println!("Module {} rates number {} as {}/10", name, number, rating);
    }
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
