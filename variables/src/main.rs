fn main() {
    basics();
    scope_example();
    type_aliases();
    type_conversion();
}

fn scope_example() {
    let x: i8 = 10;
    println!("The value of x is: {x}");
    {
        let x = x + 15;
        println!("The value of x is: {x} (in the scope)");
    }
    println!("The value of x is: {x} (after the scope)");
}

fn type_aliases() {
    type Age = u8;

    let adam_age: Age = 36;
    println!("Age: {}", adam_age)
}

fn type_conversion() {
    let age: u8 = 36;
    println!("Age: {}", age);
    let age_float = age as f32;
    println!("Age: {}", age_float);
}

fn basics() {
    let x = 6;
    println!("The value of x is {}", x);
    // x = 5; => it will not work, we cannot assign twice to immutable variable `x`

    // but we can do this:
    let x = "IAmAStringNow"; // shadowing
    println!("The value of x is {}", x);

    let mut y = 5;
    println!("The value of y is {}", y);
    y = 6;
    println!("The new value of y is {}", y);

    const MEANING_OF_LIFE: u32 = 42;
    println!("The secret meaning of life is {}", MEANING_OF_LIFE);

    let default_integer = 23; // i32 is the default integer type
    println!("default_integer value is: {}", default_integer);

    let default_integer = 0xABCDEF; // it can be hexadecimal
    println!("default_integer (hexa) value is: {}", default_integer);

    let default_integer = 0o777; // or octal
    println!("default_integer (octal) value is: {}", default_integer);

    let default_integer = 0b010101010001010101011010100101; // or binary
    println!("default_integer (binary) value is: {}", default_integer);

    let one_byte: u8 = 255; // unsigned: can't be negative
    println!("byte value is: {}", one_byte);

    let large_signed_integer: i128 = -1_000_000_000_000_000_000_000_000_000_000_000; // signed: can be negative
    println!("byte value is: {}", large_signed_integer);

    let x = 2.0; // f64 is the default floating point type
    println!("The value of x is {}", x);

    let y: f32 = 3.0; // f32
    println!("The value of y is {}", y);
}
