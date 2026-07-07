fn test_box_ptr() {
    println!("Example- Box pointer");
    // Box<T> - unique ptr, analogous to std::unique_ptr<T>
    let f = Box::new(42);
    println!("f:{} *f:{}", f, *f);

    // cloning a box creates a new heap allocation.
    let mut g = f.clone();
    // explicit `*` is mainly needed when you write through the pointer. 
    // For reads and method/field access, Rust auto-derefs.
    *g = 43;
    println!("{f} {g}");
    // g and f go out of scope and are automatically deallocated.
}

fn test_rust_ownership() {
    println!("Example- rust ownership system");
    // In C, manual memory management is error-prone: use-after-free, double-free,
    //   leaks — many are Undefined Behavior.
    // C++ smart pointers (unique_ptr/shared_ptr) express ownership and cut down
    //   these bugs, but ownership is a library convention the compiler doesn't enforce —
    //   you can still use a moved-from value or dangle a pointer.
    // Rust encodes ownership in the type system; the borrow checker enforces it at
    //   compile time, so safe Rust is free of those classes of bugs by construction.

    let data = Box::new(42); // data owns heap allocation
    let moved_data = data; // ownership transferred to moved_data
    // data is no longer accessible, compile error if used

    let borrowed = &moved_data; // immutable borrow
    println!("{}", borrowed); // safe to use
    // moved_data automatically freed when it goes out of scope.
}

fn main() {
    println!("Example - Ownership Smart Pointers");
    println!("{}", "-".repeat(20));
    test_box_ptr();
    println!("{}", "-".repeat(20));
    test_rust_ownership();
    println!("{}", "-".repeat(20));
}
