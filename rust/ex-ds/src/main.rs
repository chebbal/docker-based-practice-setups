// core data structures: arrays, tuples, slices, strings, structs, Vec, and HashMap
// notes: all rust data types are immutable by default. you can create mutable data types
// by using `mut` keyword. example: `let mut x = 42;`

// arrays
fn get_index(y: usize) -> usize {
    y + 1
}

fn test_array_1() {
    println!("test_array_1: start");
    // initializes the array of type u8 with 3 elements and all set to 42
    let a: [u8; 3] = [42; 3];
    for x in a {
        println!("{x}");
    }
    let y = get_index(a.len());
    println!("{}", y);
    // println!("{}", a[y]); // out of bounds test
    println!("test_array_1: end");
}

fn test_array_2() {
    println!("test_array_2: start");
    // multi-dimensional array
    let a = [[40, 0], [41, 0], [42, 1]];

    for x in a {
        println!("{x:?}"); // using the built-in print formatters. `:? (debug), :#? (pretty-print)`
    }
    // index access:
    println!("a[2][1]: {}", a[2][1]);
    println!("test_array_2: end");
}

// Tuples
fn get_tuple() -> (u32, bool) {
    (42, true)
}

fn test_tuple() {
    println!("test_tuple: start");
    let t: (u32, bool) = (42, true);
    let u: (u8, bool) = (43, false);
    println!("{} {}", t.0, t.1);
    println!("{} {}", u.0, u.1);
    let (num, flag) = get_tuple(); // tuple destructuring
    println!("{num} {flag}");
    println!("test_tuple: end");
}

// rust references
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

// slices - arrays and strings
fn test_slices() {
    // internally rust implements slices as "fat pointer (length of slice + poiter to starting element)"
    println!("test_slices: start");
    let a = [40, 41, 42, 43, 44];
    let b = &a[1..a.len()]; // slice from idx 1 to end
    let c = &a[1..]; // slice from idx 1 to end
    let d = &a[..]; // Same as original array, `&a[0..] or &a[0..a.len()]`
    println!("{b:?} {c:?} {d:?}");
    println!("test_slices: end");
}

// const and static in rust
const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE: u32 = 2;

fn test_const_and_static() {
    println!("test_const_and_static: start");
    println!("The secret of life is {}", SECRET_OF_LIFE);
    println!("The global variable is {}", GLOBAL_VARIABLE);
    println!("test_const_and_static: end");
}

// Strings
fn test_strings() {
    // Rust has two string types that serve two different purposes:
    // 1. String - owned, heap-allocated, growable, akin to std::string and malloc'd string
    // 2. &str - borrowed, lightweight reference with length, akin to const char * and std::string_view.
    //           with clear semantics of immutability and it is lifetime checked, thus never allowed to dangle.
    // Note: Unlike C's null terminated strings, rust strings track their length and are guaranteed valid UTF-8

    let greeting: &str = "Hello"; // points to read-only memory

    //String - heap allocated and growable
    let mut owned: String = String::from(greeting); //copies data to heap
    owned.push_str(", World"); // Grow the string
    owned.push('!'); // append a single character

    println!("{owned}");

    //converting between String and &str
    let slice: &str = &owned; // String -> &str, free just a borrow
    let owned2: String = slice.to_string(); // &str -> String, allocates
    let owned3: String = String::from(slice); // same as above

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

// novice way -- of writing this
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

// rust way
fn count_words_1(text: &str) -> usize {
    text.split_whitespace().count()
}

fn test_count_words() {
    let input_word = "Hello World !";
    assert_eq!(3, count_words_1(input_word));
}

// novice way
fn longest_word(text: &str) -> &str {
    let words = text.split_whitespace();
    let mut long_word = "";
    for word in words {
        if word.len() > long_word.len() {
            long_word = word;
        }
    }
    long_word
}

//rust way
fn longest_word_1(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("")
}

fn test_longest_word() {
    let word = "Hello World!";
    assert_eq!("World!", longest_word_1(word));
}

// Structs
fn test_structs() {
    struct MyStruct {
        num: u32,
        is_secret_of_life: bool,
    }

    let x = MyStruct {
        num: 42,
        is_secret_of_life: true,
    };
    let y = MyStruct {
        num: x.num,
        is_secret_of_life: x.is_secret_of_life,
    };
    let z = MyStruct { num: x.num, ..x }; // ..x mean copies rest of the entries from x
    println! {"MyStruct : {} {} {}", x.num, y.is_secret_of_life, z.num};
}

// tuple structs are analogous to anonymous structs
struct WeightInGrams(u32);
struct WeightInMilligrams(u32);

fn to_weight_in_grams(kilograms: u32) -> WeightInGrams {
    WeightInGrams(kilograms * 1000)
}

fn to_weight_in_milligrams(w: WeightInGrams) -> WeightInMilligrams {
    WeightInMilligrams(w.0 * 1000)
}

fn test_tuple_structs() {
    let x = to_weight_in_grams(42);
    let y = to_weight_in_milligrams(x);
    // let z: WeightInGrams = x; // compiler error: value used here after move. reason: move occurs because `x` has type `WeightInGrams`, which does not implement the `Copy` trait
    // let a: WeightInGrams = y; // compiler error: type mismatch
}

//#[derive(...)] attribute automatically generates common trait implementations for structs and enums
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn test_automatic_traits_for_structs() {
    let p1: Point = Point { x: 10, y: 10 };
    println!("{:?}", p1); // :? debug formatter, :#? pretty print formatter
    let p2: Point = p1.clone();
    println!("{:#?}", p2);
    assert_eq!(p1, p2);
}

// Vectors
// dynamic heap allocated array, similar to std::vector<T>
// Common operations: push(), pop(), insert(), remove(), len(), capacity()

fn test_vectors() {
    let mut v = Vec::new();
    v.push(42);
    v.push(43);

    // println!("{:#?}", v);
    // safe iteration (preferred way), borrow elements, don't consume vector
    for x in &v {
        println!("{x}");
    }

    // initialization shortcuts
    let mut v2 = vec![1,2,3,4,5]; // vec! macro for initialization
    let v3 = vec![0; 10]; // initialize with 10 zeros

    // safe access methods, preferred over indexing
    match v2.get(0) {
        Some(first) => println!("First: {first}"),
        None => println!("empty vector"),
    }

    //useful methods: len(), capacity()
    println!("Length:{}, capacity:{}", v2.len(), v2.capacity());
    if let Some(last) = v2.pop() {
        println!("Popped: {last}");
    }

    // Dangerous -direct indexing (can panic!)
    // println!("{}", v2[100]);

}

// Hashmap - dictionary
// key-value maps/ lookups. same as std::unordered_map<T, T> in c++
fn test_hash_maps() {
    use std::collections::HashMap; // need an explicit import unlike Vec
    let mut map = HashMap::new(); // allocate memory

    map.insert(40, false); // Type is inferred as int -> bool
    map.insert(41, false);
    map.insert(42, true);

    for (key, value) in map {
        println!("{key} {value}");
    }

    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    if let Some(x) = map.get(&43) {
        println!("43 was mapped to {x:?}");
    } else {
        println!("No mapping was found for 43");
    }

    let x = map.get(&43).or(Some(&false)); // default value if key not found
    println!("{x:?}");

}

// simple exercise with vectors and hashmaps
fn test_ex1() {
    use std::collections::HashMap;
    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    let mut keys = Vec::new();
    let mut values = Vec::new();

    for (key, value) in &map {
        keys.push(*key);
        values.push(*value);
    }

    println!("{keys:?}");
    println!("{values:?}");

    // approach 2: using unzip - fp way
    let (keys2, values2): (Vec<u32>, Vec<bool>) = map.into_iter().unzip();
    println!("{keys2:?} (unzip)");
    println!("{values2:?} (unzip)");

}

// some notes on references in rust vs c++ - https://microsoft.github.io/RustTraining/c-cpp-book/ch05-data-structures.html
// 1. No rvalue references or universal references. `&&` is a boolean AND operator in rust
// 2. In rust moves are bitwise, no move constructors
// 3. Auto-deref - compiler sees through indirection.
// 4. No null references, No optional references
// 5. references cannot be resated.
// Mental model: In C++, a reference is a permanent alias for one object. In Rust, a reference is a value (a pointer with lifetime guarantees) 
//               that follows normal variable binding rules — immutable by default, rebindable only if declared mut.


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
    test_longest_word();
    println!("{}", "-".repeat(20));
    test_structs();
    println!("{}", "-".repeat(20));
    test_tuple_structs();
    println!("{}", "-".repeat(20));
    test_automatic_traits_for_structs();
    println!("{}", "-".repeat(20));
    test_vectors();
    println!("{}", "-".repeat(20));
    test_hash_maps();
    println!("{}", "-".repeat(20));
    test_ex1();
    println!("{}", "-".repeat(20));
}
