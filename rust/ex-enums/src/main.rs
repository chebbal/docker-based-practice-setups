use ex_enums::Numbers;

fn test_enum_1() {
    // rust enum types are discriminated unions/tagged unions
    //i.e. they are sum type of several possible different types with a tag
    // that identifies the specific variant. analogous to std::variant from c++
    // but with exhaustive matching, no std::get exceptions and no std::visit boilerplate
    // - size of enum is the size of its largest type
    // - individual variants are not related to one another and can be of completely different types

    let a = Numbers::Zero;
    let b = Numbers::SmallNumber(42);
    let c: Numbers = a.clone();
    let d: Numbers = b.clone();

    println!("{:?} {:?} {:?} {:?}", a, b, c, d);
}

fn test_enum2_match() {
    // rust `match` stmt is C's `switch` stmt on steroids
    // - can be used to pattern matching on simple data types, structs, enums
    // - match statemnet must be exhaustive and must cover all possible cases. `_` can be used
    //   for 'all-else' case
    // - match can yield a value. but all arms (=>) must return a value of same type.

    let mut x = 42;

    let is_secret_of_life = match x {
        42 => true,
        _ => false,
    };

    println!("{is_secret_of_life}");

    // match supports ranges, boolean filters and if guard statements
    x = 43;
    match x {
        // ..=41 ensures the inclusive range
        0..=41 => println!("less than the secret of life"),
        42 => println!("The secret of life"),
        _ => println!("greater than the secret of life"),
    }

    let y = 100;
    match y {
        // match arm with a guard, c++: y == 100 && x == 43
        100 if x == 43 => println!("y is 100% not secret of life"),
        100 if x == 42 => println!("y is 100% secret of life"),
        _ => (), // do-nothing
    }

    // match and enums can be combined together
    // - `match` stmt can bind the contained value to a variable. Use `_` if value is dont-care.
    // - `matches!` macro can be used to match to a specific variant

    let b = Numbers::SmallNumber(42);
    match b {
        Numbers::Zero => println!("Zero"),
        Numbers::SmallNumber(value) => println!("Small number {value}"),
        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("Some Bigger number"),
    }

    // boolean test for specific variants
    if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
        println!("Matched zero or small number");
    }

    // `match` can also perform matches using destructuring and slices
    struct Foo {
        x: (u32, bool),
        y: u32,
    }
    let f = Foo {
        x: (42, true),
        y: 100,
    };
    match f {
        // capture the value of x into a variable called tuple
        Foo { y: 100, x: tuple } => println!("Matched x: {tuple:?}"),
        _ => (),
    }

    // matching using slices
    let a = [40, 41, 42];
    match a {
        // last element of slice must be 42. @ is used to bind the match
        [rest @ .., 42] => println!("{rest:?}"),
        // first elements of slice must be 42. @ is used to bind the match
        [42, rest @ ..] => println!("{rest:?}"),
        _ => (),
    }
}

enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}

#[derive(Debug, Clone)]
enum CalcResult {
    Ok(u64),
    Invalid(String),
}

fn test_rust_ex1() {
    match calculate(Operation::Add(3,5)) {
        CalcResult::Ok(value) => println!("sum: {value}"),
        CalcResult::Invalid(msg) => println!("Error : {msg}"),
    }
    match calculate(Operation::Subtract(3,5)) {
        CalcResult::Ok(value) => println!("difference: {value}"),
        CalcResult::Invalid(msg) => println!("Error : {msg}"),
    }
}

fn calculate(op: Operation) -> CalcResult {
    match op {
        Operation::Add(x, y) => {
            CalcResult::Ok(x + y)
        }
        Operation::Subtract(x, y) => {
            if x >= y {
                CalcResult::Ok(x - y)
            } else {
                CalcResult::Invalid("Underflow".to_string())
            }
        
        },
    }
}

#[derive(Debug)]
struct Point {x: u32, y: u32}
impl Point{
    fn new(x: u32, y: u32) -> Self {
        Point{x, y}
    }

    fn increment_x(&mut self) {
        self.x += 1;
    }

    fn add(&mut self, p: &Point) {
        self.x += p.x;
        self.y += p.y; 
    }

    fn transform(self) -> Point {
        Point { x: self.x * self.x, y: self.y * self.y}
    }
}

fn test_associated_methods() {
    // `impl` can define methods associated for types like struct,enum etc
    // the methods may optionally take `self` as a parameter.
    // reference to self can be immutable(default:&self), mutable(&mut self) or self(transferring ownership)
    // `Self` keyword can be used as a shortcut to imply the type
    let mut p = Point::new(10, 20);
    p.increment_x();

    println!("p: {p:?}");

    let other = Point::new(20, 30);
    p.add(&other);
    println!("p: {p:?}");
    let p1 = p.transform();
    // println!("p: {p:?}"); // compiler error: since p is transferred to p1
    println!("p1: {p1:?}");

}

fn main() {
    println!("{}", "-".repeat(20));
    println!("Rust Enums");
    println!("{}", "-".repeat(20));
    test_enum_1();
    println!("{}", "-".repeat(20));
    test_enum2_match();
    println!("{}", "-".repeat(20));
    test_rust_ex1();
    println!("{}", "-".repeat(20));
    test_associated_methods();
    println!("{}", "-".repeat(20));
}
