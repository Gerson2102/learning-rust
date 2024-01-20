pub mod closures;
pub mod modules;
pub mod optiontest;
fn main() {
    println!("Hello, world!");
    // modules::helper::greet();
    // closures::closures();
    let result = optiontest::test_option();
    println!("{}", result.unwrap());
}
