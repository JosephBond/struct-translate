use d_parse::MyMacro;
use libc::c_void;

#[derive(MyMacro)]
struct Tester {
    fieldone : i32,
    fieldtwo: *mut libc::c_void,
    fieldthree: Box<Option<i32>>,
}


fn main() {
    println!("Hello, world!");
}
