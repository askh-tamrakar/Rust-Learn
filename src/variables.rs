pub fn variables_mutability_1_1() {
    println!(r#"
     // LEARNING RUST
    //  1. COMMON CONCEPTS of PROGRAMMING

    // 1.1 Variables and Mutability
    /*
        all the variables are Immutable by default in rust,
        we can add `mut` before the variable name to make
        it mutable.
    */

    // Immutable
    /*
        let x = 5;
        println!("The value of x is: {{}}", x);
        x = 6;
        println!("The value of x is: {{}}", x);
    */
    // will return error that `cannot assign twice to immutable variable`

    // Mutable
    /*
        let mut x = 5;
        println!("The value of x is: {{}}", x);
        x = 6;
        println!("The value of x is: {{}}", x);
    */
    // this will run successfully
       "# );

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!(r#"
    // Shadowing

    /*
        shadowing allows to use the same of a variable more than
        ones, we can also change it's data type and can assign
        new value to it without making it mutable.
    */

    /*
        let x = 5;
        println!("The value of x is: {{x}}");

        let x = x + 1;

        {{
            let x = x * 2;
            println!("The value of x in the inner scope is: {{x}}");
        }}

        println!("The value of x is: {{x}}");

    ====================================================================
    "#);
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;

    {{
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }}

    println!("The value of x is: {x}");

    println!(r#"
    /*
        let spaces = "Let's Get rusty.";
        let spaces = spaces.len();
    *// this possible.

    /*
        let mut spaces = "   ";
        spaces = spaces.len();
    */ this is not possible.

    */
    "#);

    let spaces = "Let's Get rusty.";
    println!("{}", &spaces);

    let spaces = spaces.len();
    println!("{}", &spaces);
}