#[no_mangle]
pub fn rate_number(number: i32) -> i32 {
    if number % 2 == 0 {
        8 // even is good
    } else {
        3 // od is bad
    }
}
