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
    } else {
        println!("Not found in string");
    }

    // this will panic
    // "1234".find("5").unwrap();
    // this will not panic
    "1234".find("5").unwrap_or(42);
}

fn test_result_type() {
    println!("Example - test Result<T> type");
    // Result<T> is extensively used in Rust API's that can fail.
    // On success, functions will return Ok<T> or they will return
    // a specific Err<T>

    use std::num::ParseIntError;
    let a: Result<i32, ParseIntError> = "1234z".parse();
    match a {
        Ok(n) => println!("Parsed {n}"),
        Err(e) => println!("Parsing failed {e:?}"),
    }

    let a: Result<i32, ParseIntError> = "1234z".parse().or(Ok(-1));
    println!("{a:?}");
    if let Ok(a) = "1234".parse::<i32>() {
        println!("Let Ok {a}");
    }

    // this will panic
    // "1234z".parse().unwrap();
}

fn test_option_and_result() {
    println!("Example - relation between Option and Result");
    // Option<T> <==> Result<T, ()> i.e. a result where error carries no
    // information
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("Value was None"); // option -> result
    match res {
        Ok(a) => println!("Value: {a:?}"),
        Err(e) => println!("Error: {e:?}"),
    }

    let res: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = res.ok(); // result -> option (discards error - Err(e))
    // They share many of the same methods:
    // .map(), .and_then(), .unwrap_or(), .unwrap_or_else(), .is_some()/is_ok()
    match opt {
        Some(a) => println!("{a:?}"),
        None => println!("No value"),
    }

    // Rule of thumb, Use Option when absence is normal,
    // Use Result, when failure needs explanation (like file I/O parsing)
}

use std::fs::File;
use std::io::Read;

fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?; // ? automatically propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn test_error_handling_1() {
    println!("Example - rust error handling 1");
    // In Rust errors are either recoverable (Result<T, E>) or
    // unrecoverable (panic!). Prefer Result; reserve panics for
    // broken invariants. panic!/assert! are ways to *raise* a panic.
    match read_file_content("example.txt") {
        Ok(content) => println!("File content {}", content),
        Err(e) => println!("Failed to read file: {}", e),
        // compiler forces us to handle both the cases
    }
}

fn test_error_handling_2() {
    println!("Example - rust error handling 2");

    let x = "1234x".parse::<u32>();
    match x {
        Ok(x) => println!("Parsed number {x}"),
        Err(e) => println!("Parsing error {e:?}"),
    }

    // same as above, but with valid number
    let x = "1234".parse::<u32>();
    if let Ok(x) = &x {
        println!("Parsed number {x}");
    } else if let Err(e) = &x {
        println!("Error {e:?}");
    }
}

fn double_string_number(num: &str) -> Result<u32, std::num::ParseIntError> {
    let x  = num.parse::<u32>()?; // returns immediately in case of an error
    Ok(x*2)
}

fn double_string_number_2(num: &str) -> Result<u32, ()> {
    // changes the error type to () in case of error
    let x = num.parse::<u32>().map_err(|_|())?;
    Ok(x * 2)
}

fn double_string_number_3(num: &str) -> Result<u32, ()> {
    // returns default value in case of error
    let x = num.parse::<u32>().unwrap_or_default();
    Ok(x * 2)
}

fn double_optional_number(num: Option<u32>) -> Result<u32, ()> {
    // ok_or converts Option<None> to Result<u32,()>
    num.ok_or(()).map(|x|x*2) // note: .map is applied only on Ok(u32)
}

fn test_error_handling_3() {
    println!("Example - rust error handling 3");
    // The try-operator ? is a convenient short hand for the match Ok / Err pattern
    // Note the method must return Result<T, E> to enable use of ?
    let result = double_string_number("1234");
    println!("{result:?}");
    let result = double_string_number("1234x");
    println!("{result:?}");
    // Errors can be mapped to other types, or to default values 
    // (https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default)
    let result = double_string_number_2("1234x");
    println!("{result:?}");
    let result = double_string_number_3("1234x");
    println!("{result:?}");
    let result = double_optional_number(None);
    println!("{result:?}");
    let result = double_optional_number(Some(1234));
    println!("{result:?}");

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
    test_result_type();
    println!("{}", "-".repeat(20));
    test_option_and_result();
    println!("{}", "-".repeat(20));
    test_error_handling_1();
    println!("{}", "-".repeat(20));
    test_error_handling_2();
    println!("{}", "-".repeat(20));
    test_error_handling_3();
    println!("{}", "-".repeat(20));
}
