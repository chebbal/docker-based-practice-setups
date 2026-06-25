
fn test_if(x: i32) {
    println!("test_if: start \n");
    if x < 42 {
        println!("Smaller than the secret of life!");
    }
    else if x == 42 {
        println!("Is equals than secret of life!!!");
    }
    else {
        println!("Greater than secret of life!")
    }
    println!("test_if: end \n");
}

fn test_while() {
    println!("test_while: start \n");
    let mut x = 40;
    while x < 42 {
        println!("x = {}\n", x);
        x += 1;
    }
    println!("test_while: end \n");
}

fn test_for() {
    println!("test_for: start \n");
    for x in 40..42 {
        println!("x = {}\n", x);
    }
    println!("test_for: end \n");
}

fn test_loop() -> i32 {
    println!("test_loop: start \n");
    let mut x : i32 = 40;

    loop {
        if x == 42 {
            println!("test_loop: end \n");
            break x;
        }

        x += 1; 
    }
}

fn test_expression_blocks()
{
    println!("test_expression_blocks: start \n");
    let x = {
        let y = 40;
        y + 2 // note: ; must be omitted
    };

    println!("x : {}", x);
    println!("test_expressions_blocks: end \n");
}

fn is_secret_of_life(x : u32) -> bool{
    // rust style is to omit the return keyword
    println!("test_return \n");
    x == 42
}

fn main() {
    println!("Control flow practice\n");
    println!("----------------------------------------------------");
    test_if(42);
    println!("----------------------------------------------------");
    test_while();
    println!("----------------------------------------------------");
    test_for();
    println!("----------------------------------------------------");
    println!("test_loop ret: {}", test_loop());
    println!("----------------------------------------------------");
    test_expression_blocks();
    println!("----------------------------------------------------");
    println!("{}",is_secret_of_life(42));
    println!("----------------------------------------------------");

}
