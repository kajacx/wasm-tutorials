use std::fmt::Write;

use wasm_bridge::*;

static GUEST_BYTES: &[u8] =
    include_bytes!("../../rust-guest/target/wasm32-unknown-unknown/release/rust_guest.wasm");

pub async fn get_text() -> String {
    let mut store = Store::<()>::default();

    let module = new_module_async(&store.engine(), GUEST_BYTES)
        .await
        .expect("should create module");

    let mut linker = Linker::new(store.engine());

    linker
        .func_wrap("imported_fns", "add_one", |_: Caller<()>, num: i32| num + 1)
        .expect("should add import function");

    let instance = instantiate_async(&mut store, &linker, &module)
        .await
        .expect("should create instance");

    let rate_number = instance
        .get_typed_func::<i32, i32>(&mut store, "add_three")
        .expect("should get add_three exported function");

    let mut text = String::new();

    for number in [15, 8, -20, 162, 1023] {
        let result = rate_number
            .call(&mut store, number)
            .expect("should call add_three");

        writeln!(text, "{number} + 3 = {result}").unwrap();
    }

    text
}
