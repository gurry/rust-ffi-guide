#[no_mangle]
pub extern "C" fn foo() {
    panic!("Oops...");
}