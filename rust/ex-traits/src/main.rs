trait Pet {
    fn speak(&self);
}

struct Cat;
struct Dog;

impl Pet for Cat {
    fn speak(&self) {
        println!("Meoww");
    }
}

impl Pet for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

// inheritance vs rust traits
trait Animal {
    fn sound(&self);
}

impl Animal for Cat {
    fn sound(&self) {
        println!("Meow");
    }
}

fn make_sound<T: Animal>(animal: &T) {
    animal.sound();
}

fn test_trait_2() {
    println!("Example - traits 2");
    let c = Cat {};
    make_sound(&c);
}

fn test_trait_1() {
    println!("Example - traits 1");
    let c = Cat {};
    let d = Dog {};
    c.speak(); // there is no is-a relationship between cat and dog
    d.speak(); // there is no is-a relationship between cat and dog
}

// Trait bounds and generic constraints
// C++ template equivalent (less constrained)
// template<typename T>
// T add_and_print(T a, T b) {
//     // No guarantee T supports + or printing
//     return a + b;  // Might fail at compile time
// }
use std::fmt::Display;
use std::ops::Add;

//rust explicit trait bounds
fn add_and_print<T>(a: T, b: T) -> T
where
    T: Display + Add<Output = T> + Copy,
{
    println!("Adding {} + {}", a, b);
    a + b // Add trait
}

fn test_trait_3() {
    println!("Example -trait bounds");
    println!("sum: {}",add_and_print(40, 2));
}

// C++ operator overloading --> Rust std::ops/ std::fmt Traits
// In C++, you overload operators by writing free functions or member functions
// with special names (operator>, operator+ etc.). In rust, every operator maps to
// a trait in std::ops (or std::fmt for output). you implement that trait instead of 
// writing a magic-named function.
// use std::ops::Add;
#[derive(Debug, Clone, Copy)]
struct Vec2 {x: f64, y: f64}

impl Add for Vec2 {
    type Output = Vec2; // Associated type - the result of +
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

fn test_trait_4() {
    println!("Example - trait operator overloading");
    let a = Vec2{x:1.0, y: 2.0};
    let b = Vec2{x: 2.0, y: 4.0};
    let c = a + b;
    println!("{c:?}");
}

fn main() {
    println!("Exercise- Traits and Generics");
    println!("{}", "-".repeat(20));
    // Traits are Rust's answer to interfaces, abstract base classes, and
    // operator overloading. In C++ terms they cover virtual functions (dynamic
    // dispatch via `dyn Trait`), CRTP (static dispatch via generics), and
    // concepts (trait bounds). Unlike abstract base classes, traits hold no
    // data — only behavior. For C developers, they're a structured way to do
    // polymorphism (vs. hand-rolled function-pointer vtables).

    test_trait_1();
    println!("{}", "-".repeat(20));
    test_trait_2();
    println!("{}", "-".repeat(20));
    test_trait_3();
    println!("{}", "-".repeat(20));
    test_trait_4();
    println!("{}", "-".repeat(20));
}
