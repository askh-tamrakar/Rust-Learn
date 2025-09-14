pub fn data_types_1_2(){
    println!(r#"
        //Learning RUST
        //1. Common Concepts of Programming

        //1.2 Data Types
        /*
            Every value bleongs to a different data type.
            In rust there two data types subsets: Scalar and Compount.
            Rust is a statically typed language, it means it must 
            know the types of all variable at compile time.
        */

        // SCALAR Type => it represents a single value, rust has
                          four primary scalar types:
        1. Integers 
        2. Floating-point numbers 
        3. Boolean
        4. Characters

        ==> Interger Type
        => An integer is a number without a fractional component.
        the u32 type declaration indicates that the value its 
        associated with should be an unsigned integer (signed 
        integer types start with i instead of u) that takes up 
        32 bits of space.
        
                 Length	         Signed	    Unsigned
                 8-bit             i8	       u8
                 16-bit	           i16         u16
                 32-bit	           i32	       u32
                 64-bit	           i64	       u64
                 128-bit	       i128	       u128
        architecture dependent	  isize	      usize

        => Signed and unsigned refer to whether its possible 
        for the number to be negative -
        
        signed => both ( - / + ), Each signed variant can store 
                  numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive, 
                  where n is the number of bits that variant uses.
            Ex.=> i8 -(2^7) to (2^7) - 1, which equals -128 to 127.


        unsigned => only ( + ), Unsigned variants can store numbers 
                    from 0 to 2^n - 1, where n is the number of bits
                    that variant uses.
              Ex.=> u8 0 to 2^8 - 1, which equals 0 to 255.
        
        Number literals	      Example
            Decimal	            98_222
            Hex	                0xff
            Octal	            0o77
            Binary	            0b1111_0000
            Byte (u8 only)	    b'A'

        
        ==> Floating-Point Type
        => Rust also has two primitive types for floating-point numbers,
           which are numbers with decimal points. Rust's floating-point 
           types are f32 and f64, which are 32 bits and 64 bits in size,
           respectively. 
        /*
            fn main() {{
                let x = 2.0; // f64

                let y: f32 = 3.0; // f32
            }}
        */

        ==> Numeric Operators
        => Rust supports the basic mathematical operations you'd expect 
           for all the number types: addition, subtraction, multiplication, 
           division, and remainder. Integer division truncates toward zero 
           to the nearest integer.
        /*
            // addition
            let sum = 5 + 10;

            // subtraction
            let difference = 95.5 - 4.3;

            // multiplication
            let product = 4 * 30;

            // division
            let quotient = 56.7 / 32.2;
            let truncated = -5 / 3; // Results in -1

            // remainder
            let remainder: i32 = 43 % 5;
        */

    "#);
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder: i32 = 43 % 5;

    println!(r#"sum of 5 + 10 is{},
                    difference of 95.5 - 4.3 is {},
                    product of 4 * 30 is {},
                    quotient of 56.7 / 32.2 is {} and
                    trncated of -5 / 3 is {},
                    remainder of 43 % 5 is {}"#,
             sum, difference, product, quotient, truncated, remainder);

    println!(r#"
            ==> Boolean Type
            => Boolean type in Rust has two possible values: 
               true and false. Booleans are one byte in size. 
               The Boolean type in Rust is specified using bool.
            /*
                fn main() {{
                    let t = true;

                    let f: bool = false; // with explicit type annotation
                }}
            */
            
            ==> Character Type
            => Rust's char type is the language's most primitive alphabetic 
               type. Rust's char type is four bytes in size and represents 
               a Unicode scalar value, which means it can represent a lot 
               more than just ASCII. Accented letters; Chinese, Japanese, 
               and Korean characters; emoji; and zero-width spaces are all 
               valid char values in Rust. Unicode scalar values range from 
               U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
            /*
                fn main() {{
                    let c = 'z';
                    let z: char = 'ℤ'; // with explicit type annotation
                    let heart_eyed_cat = '😻';
                }}
            */
        "#);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c {}, z {}, Loving Cat {}",c, z, heart_eyed_cat );

    println!(r#"

            // COMPOUND Type => Compound types can group multiple 
                                values into one type. Rust has two 
                                primitive compound types: 
                                tuples and arrays.
            
        "#);

}
