use mylib::module1::*;

fn say_hello_from_main() {
    println!("Hello world from func in main.rs!");
}
fn main() {
  say_hello_from_main();
  say_hello_from_module1();
}
