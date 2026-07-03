// Notes on rust ownership:
// 1. Move is destructive - the compiler refuses to let you touch the moved-from variable.
// 2. No rule of five needed (no copy ctor, no move ctor, copy assign, move assign, dtor)
// 3. Rust gives complete control for memory allocation, but enforces safety at compile-time
//    This is done by a combination of mechanisms including ownership, borrowing, mutability and lifetimes
// 4. Rust runtime allocations can happen both on stack and heap.
// rust pointers terminology:
// Box<T> - analogous to std::unique_ptr<T>. Comes from FP terminology. Put the value in a box on a heap
//          and hold the pointer to it. Single owner of one heap allocation. When it drops, the allocation is freed.
// Rc<T>  - Analogous to std::shared_ptr<T>. Stands for "Reference-counted". Enables shared ownership. Value is freed, when count hits zero.
//          Note: This is single-threaded only(the counter is a plain, non-atomic integer- fast but not thread-safe)
// Arc<T>  - Analogous to std::shared_ptr<T> (Thread-safe). Stands for Atomic reference counter. Same idea as Rc, but it uses atomic operations.
// Weak<T> - Analogous to std::weak_ptr<T>. A weak reference i.e. it points to a value without owning it. Put simply, it 
//           does not add to strong count and doesn't keep the value alive. You can call .upgrade() to get a real Rc/Arc, but it returns an Option(None if the value was dropped)
// *const T / *mut T - Raw pointers. allowed only in unsafe blocks.   

fn test_ownership_1() {
    // Note: rust permits only permits a single mutable reference to a variable and 
    // multiple read-only references - never both simultaneously. A borrow lasts until its last use
    // (non-lexical lifetimes), not until the end of the scope
    // borrow semantics follow the `aliasing-XOR-Mutability`rule.
    println!("Example 1 ownership");
    let mut a: u8 = 42; // ownership
    let b = &a;  // first borrow
    println!("ref to a - {b}"); // last use of b
    let c = &mut a; // fine b is no longer live. Not fine if below line is uncommented. `b is still active`
    //println!("ref to a {b}"); 
    *c = 43;
    println!("mutable ref to a - {c}"); 

    // shared borrows example
    let b1 = &a;
    let b2 = &a;  // many shared borrows can co-exist
    println!("two refs- {b1} {b2}"); // both live here, fine
    let c = &mut a; // shared borrow dead - mutable ok
    *c = 44;
    println!("mutable ref to a - {c}");

}

fn test_borrow_lifetime_1() {
    // lifetime of a borrow cannot exceed the owning lifetime
    println!("Example 2  borrow-ownership lifetime");
    let a = 42; // owner
    let b = &a; // first borrow
    {
        let aa = 43;
        let c = &a; //second borrow; a still in scope
        // ok: c goes out of scope here
        // aa goes out of scope here
    }
    // let d = &aa; // error: aa is not in scope here. cannot borrow
    // b implictly goes out of scope here
    // a goes out of scope last
}

fn foo(x: &u32) {
    println!("foo: {x} - pass by reference");
}
fn bar(x: u32) {
    println!("bar: {x} - pass by value");
}

fn test_parameter_passing() {
    // rust can pass parameters to methods using several different mechanisms
    // 1. pass by value (copy): Typically types that can be trivially copied(ex:u8, u32, i8, i32)
    // 2. pass by reference: Analogous to pass by pointer in c. This is commonly known as borrowing.
    //                       reference can be immutable (&) or mutable(&mut)
    // 3. by moving: This transfers ownership of the value to the function. The caller can no longer reference 
    //               the original value.
    println!("Example 3  Parameter passing");
    let a = 42;
    foo(&a); // by reference
    bar(a); // By value (copy)
    println!("original value a: {a}");

}

// fn no_dangling() -> &u32 {
//     // lifetime of `a` begins here.
//     let a = 42;
//     // Won't compile. lifetime of `a` ends here.
//     &a
// }

fn ok_reference(a: &u32) -> &u32 {
    a // okay because, lifetime of `a` always exceeds ok_reference()
}


fn test_return_values() {
    println!("Example 4  return values from methods");
    // rust prohibits dangling reference from methods.
    // 1. references returned by methods must be still in scope.
    // 2. rust will automatically drop a reference when it goes out of scope.
    let a =42; // lifetime `a` begins here
    let b = ok_reference(&a);
    // lifetime `b` ends here
    // lifetime of `a` ends here.
}

fn main() {
    println!("Ownership in Rust!!");
    println!("{}", "-".repeat(20));
    test_ownership_1();
    println!("{}", "-".repeat(20));
    test_borrow_lifetime_1();
    println!("{}", "-".repeat(20));
    test_parameter_passing();
    println!("{}", "-".repeat(20));
    test_return_values();
    println!("{}", "-".repeat(20));
}
