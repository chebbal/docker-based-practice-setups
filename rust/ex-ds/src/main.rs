// core data structures: arrays, tuples, slices, strings, structs, Vec, and HashMap
// notes: all rust data types are immutable by default. you can create mutable data types
// by using `mut` keyword. example: `let mut x = 42;`


fn get_index(y : usize) -> usize {
    y + 1
}

fn test_array_1()
{
    println!("test_array_1: start");
    // initializes the array of type u8 with 3 elements and all set to 42
    let a : [u8; 3] = [42; 3];
    for x in a {
        println!("{x}");
    }
    let y = get_index(a.len());
    println!("{}", y);
    // println!("{}", a[y]); // out of bounds test
    println!("test_array_1: end");
}

fn test_array_2()
{
    println!("test_array_2: start");
    // multi-dimensional array
    let a = [
        [40, 0],
        [41, 0],
        [42, 1]
    ];

    for x in a {
        println!("{x:?}"); // using the built-in print formatters. `:? (debug), :#? (pretty-print)`
    }
    // index access:
    println!("a[2][1]: {}", a[2][1]);
    println!("test_array_2: end");
}

fn get_tuple() -> (u32, bool) {
    (42, true)
}

fn test_tuple(){
    println!("test_tuple: start");
    let t: (u32, bool) = (42, true);
    let u: (u8, bool) = (43, false);
    println!("{} {}", t.0, t.1);
    println!("{} {}", u.0, u.1);
    let (num, flag) = get_tuple(); // tuple destructuring
    println!("{num} {flag}");
    println!("test_tuple: end");
}

fn test_references() {
    // rust references are akin to pinters in C with two properties:
    // 1. you can have any number of read-only references to a variable at any point of time
    //    but a reference cannot outlive variable scope. (key concept: lifetime)
    // 2. only a single mutable write reference to a mutable variable is permitted and it must not overlap
    //    with any other reference.
    println!("test_references: start");
    let mut a = 42;
    {
        let b = &a;
        let c = b;
        println!("{} {}", *b, *c);

        let d = &mut a;
        //error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable
        //println!("{}", *b);
    }
    let d = &mut a; // ok: b and c are not in scope
    println!("{}", *d);
    println!("test_references: end");

}

fn test_slices()
{
    // internally rust implements slices as "fat pointer (length of slice + poiter to starting element)"
    println!("test_slices: start");
    let a = [40, 41, 42, 43, 44];
    let b  = &a[1..a.len()]; // slice from idx 1 to end
    let c = &a[1..]; // slice from idx 1 to end
    let d = &a[..];; // Same as original array, `&a[0..] or &a[0..a.len()]`
    println!("{b:?} {c:?} {d:?}");
    println!("test_slices: end");
}

const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE: u32 = 2;

fn test_const_and_static()
{
    println!("test_const_and_static: start");
    println!("The secret of life is {}", SECRET_OF_LIFE);
    println!("The global variable is {}", GLOBAL_VARIABLE);
    println!("test_const_and_static: end");
}

fn test_strings() {
    // Rust has two string types that serve two different purposes:
    // 1. String - owned, heap-allocated, growable, akin to std::string and malloc'd string
    // 2. &str - borrowed, lightweight reference with length, akin to const char * and std::string_view.
    //           with clear semantics of immutability and it is lifetime checked, thus never allowed to dangle.
    // Note: Unlike C's null terminated strings, rust strings track their length and are guaranteed valid UTF-8

    let greeting : &str = "Hello"; // points to read-only memory

    //String - heap allocated and growable
    let mut owned : String  = String::from(greeting); //copies data to heap
    owned.push_str(", World"); // Grow the string
    owned.push('!'); // append a single character

    println!("{owned}");

    //converting between String and &str
    let slice : &str = &owned; // String -> &str, free just a borrow
    let owned2 : String = slice.to_string(); // &str -> String, allocates
    let owned3 : String = String::from(slice); // same as above

    println!("{owned3}");
    
    // String concatenation (note: + consumes the left operand)
    let hello = String::from("Hello");
    let world = String::from(", World!");
    let combined = hello + &world; // hello is moved (consumed), world is borrowed
    // println!("{hello}"); // wont compile, hello was moved

    // use format! to avoid move issues
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{a}, {b}!"); // Neither a and b is consumed

    println!("{combined}");

}

fn test_idx_strings() {
    let s = String::from("hello");
    // let c = s[0]; // does not work. Rust strings are UTF-8, not byte arrays

    //safe alternatives:
    let first_char = s.chars().next(); // Option<char>: Some('h')
    let as_bytes = s.as_bytes(); //&[u8]: raw UTF-8 bytes
    let substring = &s[0..1];

    println!("First char: {:?}", first_char);
    println!("Bytes: {:?}", &as_bytes[..5]);
}

// novice way -- of wrining this
fn count_words(text: &str) -> usize {

    // let white_space = String::as_bytes(' ');
    let mut cnt: usize = 0;
    for c in text.chars() {
        if c == ' ' {
            cnt += 1;
        } 
    }

    cnt + 1
    
}

fn test_count_words() {
    let input_word = "Hello World !";
    let count : usize = count_words(input_word);
    assert_eq!(3, count_words(input_word));
}



fn main() {
    println!("Data structure exercises");
    println!("{}", "-".repeat(20));
    test_array_1();
    println!("{}", "-".repeat(20));
    test_array_2();
    println!("{}", "-".repeat(20));
    test_tuple();
    println!("{}", "-".repeat(20));
    test_references();
    println!("{}", "-".repeat(20));
    test_slices();
    println!("{}", "-".repeat(20));
    test_const_and_static();
    println!("{}", "-".repeat(20));
    test_strings();
    println!("{}", "-".repeat(20));
    test_idx_strings();
    println!("{}", "-".repeat(20));
    test_count_words();
    println!("{}", "-".repeat(20));
}
