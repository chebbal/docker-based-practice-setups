mod math {
    pub fn add(a:u32, b:u32) -> u32 {
        a + b
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}!, the secret number is {}", name, math::add(40,2))
}

fn test_greet() {
    println!("Exercise- module intro");
    // a. functions are scoped by their modules. there is no name collision for two functions
    // having same name but located in different modules.  ex: a::foo() and  b::foo()
    // b. module scoping also extends to all types
    println!("{}", greet("Rustacean"));
}

fn main() {
    println!("Exercise - crates and modules");
    println!("{}", "-".repeat(20));
    test_greet();
    println!("{}", "-".repeat(20));

}
