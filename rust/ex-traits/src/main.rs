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
    let c = Cat{};
    let d = Dog{};
    c.speak(); // there is no is-a relationship between cat and dog
    d.speak(); // there is no is-a relationship between cat and dog
  
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

}
