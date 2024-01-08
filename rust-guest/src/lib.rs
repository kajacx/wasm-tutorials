#[no_mangle]
pub fn add_three(number: i32) -> i32 {
    let number = unsafe { add_one(number) };
    let number = unsafe { add_one(number) };
    let number = unsafe { add_one(number) };
    number
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one(number: i32) -> i32;
}
