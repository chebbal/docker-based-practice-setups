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

fn test_borrowing_rules() {
    println!("Example - smart pointer borrowing rules");

    let mut data = vec![1, 2, 3, 4, 5];

    // multiple immutable borrows - ok
    let ref1 = &data;
    let ref2 = &data;
    println!("{:?} {:?}", ref1, ref2);

    // mutable borrow - exclusive access
    let ref_mut = &mut data;
    ref_mut.push(6);
    // ref1 and ref2 cannot be used, since ref_mut is active.

    // After ref_mut is done, immutable borrow works again
    let ref3 = &data;
    println!("{:?}", ref3);
}

use std::cell::{Cell, RefCell};
#[derive(Debug)]
struct Employee {
    emp_id: u64,             // immutable, can't be changed behind &
    on_vacation: Cell<bool>, // write-access permitted even though a shared &
}

fn test_field_level_mutability() {
    println!("Example - field level interior immutability");

    let emp = Employee {
        emp_id: 1001,
        on_vacation: Cell::new(false),
    };

    // Note: emp is immutable
    let e = &emp;

    e.on_vacation.set(true);
    println!("on vacation? {}", e.on_vacation.get()); // true

    // e.emp_id = 1002; // compiler error, emp not mutable and emp_id is not Cell
    // println!("id: {}", e.emp_id); // true
}

fn test_cell_refcell() {
        // --- RefCell<T>: for non-Copy types. Hands out real borrows via
    //    --- Cell<T>: for Copy types. No references to the inner value are handed
    //     out — you swap whole values in/out, so it can't be misused. Zero cost.

    // --- RefCell<T>: for non-Copy types. Hands out real borrows via
    //     .borrow() / .borrow_mut(), tracked at runtime.

    println!("Example - internal mutability via cell/Refcell");
    let counter = Cell::new(0);
    let inc = || counter.set(counter.get() + 1); // closure holds &counter, still mutates
    inc();
    inc();
    assert_eq!(2, counter.get());
    println!("cell = {}", counter.get()); //2

    let log = RefCell::new(Vec::new());
    log.borrow_mut().push("first"); // &mut to the Vec, through a shared &
    log.borrow_mut().push("second");
    println!("refcell = {:?}", log.borrow()); // immutable reference

    // the catch: violate borrow rules -> panic at runtime, not compile error
    let _read = log.borrow();
    // log.borrow_mut(); // <- would panic: already borrowed
}

fn without_rc() {
    println!("Without using Rc");
    let mut us_employees = vec![];
    let mut global_employees = Vec::<Employee>::new();
    let employee = Employee {emp_id: 1001, on_vacation:Cell::<bool>::new(false)};
    us_employees.push(employee);
    println!("us_employees: {us_employees:?}");
    // won't compile - as employee is already moved
    // global_employees.push(employee);

}

fn with_rc() {
    println!("Using Rc");
    use std::rc::Rc;
    let mut us_employees = vec![];
    let mut global_employees = vec![];
    // employee leaves on stack
    let employee = Employee {emp_id: 1001, on_vacation: Cell::<bool>::new(false)};
    let employee_rc = Rc::new(employee); // employee moved to Rc on heap
    us_employees.push(employee_rc.clone());
    println!("us_employees: {us_employees:?}");
    global_employees.push(employee_rc.clone());
    println!("global_employees: {global_employees:?}");
}

fn rc_simple_example() {
    println!("Rc simple example");
    use std::rc::Rc;
    let a = Rc::new(String::from("hi")); // strong ref ->1
    println!("strong ref: {}", Rc::strong_count(&a)); // 1
    let b = Rc::clone(&a); // strong ref -> 2
    println!("strong ref: {}", Rc::strong_count(&a)); // 2
    drop(b);
    println!("strong ref: {}", Rc::strong_count(&a)); // 1
    // end of scope: strong 1 -> 0, String dropped, allocation freed.
}

fn test_rc() {
    println!("Example - Rc(shared_ptr) test");
    // Rc<T> allows reference counted shared ownership of immutable data.
    without_rc();
    with_rc();
}

use std::rc::{Rc, Weak};
struct Node {
    value: i32,
    parent: Option<Weak<Node>>, // weak reference, doesn't prevent drop
}

fn test_weak_ptr() {
    println!("Example- weak ptr");
    // use `weak` for back-references in tree/graph to avoid memory leaks
    let parent = Rc::new(Node {value: 1, parent: None} );
    let child = Rc:: new(Node {
        value: 2,
        parent: Some(Rc::downgrade(&parent)), // weak ref to a parent
    });

    // To use a weak, try to upgrade it - returns Option<Rc<T>>
    if let Some(parent_rc) = child.parent.as_ref().unwrap().upgrade() {
        println!("Parent value: {}", parent_rc.value);
    }

    println!("Parent strong count: {}", Rc::strong_count(&parent));
}

fn main() {
    println!("Example - Ownership Smart Pointers");
    println!("{}", "-".repeat(20));
    test_box_ptr();
    println!("{}", "-".repeat(20));
    test_rust_ownership();
    println!("{}", "-".repeat(20));
    test_borrowing_rules();
    println!("{}", "-".repeat(20));
    test_field_level_mutability();
    println!("{}", "-".repeat(20));
    test_cell_refcell();
    println!("{}", "-".repeat(20));
    test_rc();
    println!("{}", "-".repeat(20));
    rc_simple_example();
    println!("{}", "-".repeat(20));
    test_weak_ptr();
    println!("{}", "-".repeat(20));
}
