fn test_option_type() {
    println!("Example - option type");
    // returns Option<usize>
    let a = "1234".find("1");
    match a {
        Some(a) => println!("found index at {a}"),
        None => println!("Couldn't find 1"),
    }
    // Option<T> can be processed with unwrap(), unwrap_or() and `if let` 
    // lets us test for Some<T>
    println!("{a:?} {}", a.unwrap());
    let a = "1234".find("5").or(Some(42));
    println!("{a:?}");
    if let Some(a) = "1234".find("1") {
        println!("{a}");
    }
    else {
        println!("Not found in string");
    }

    // this will panic
    // "1234".find("5").unwrap();
    // this will not panic 
    "1234".find("5").unwrap_or(42);
}

fn test_result_type() {

}

fn main() {
    println!("Exercise - Error handling");
    // In rust there are no null pointers. Option<T> should be used.
    // Compiler forces us to handle for None case.
    // // This is literally how Option is defined in std:
    // enum Option<T> {
    //     Some(T),  // Contains a value
    //     None,     // No value
    // }

    // // And Result:
    // enum Result<T, E> {
    //     Ok(T),    // Success with value
    //     Err(E),   // Error with details
    // }

    println!("{}", "-".repeat(20));
    test_option_type();
    println!("{}", "-".repeat(20));
}
