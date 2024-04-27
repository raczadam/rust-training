fn main() {
    str_types();
    array_types();
}

fn str_types() {
    let fixed_str = "I am immutable";
    println!("{}", fixed_str);
    let fixed_str_2: &str = "I am immutable too!"; // the result will be the same type
    println!("{}", fixed_str_2);

    let mut flexible_string = String::from("I am a flexible String");
    println!("{}", flexible_string);
    flexible_string.push_str(" - so you can concat a new text to me.");
    println!("{}", flexible_string);
}

fn array_types() {
    let mut array_1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array_1);

    let num = array_1[0];
    println!("{}", num);

    array_1[0] = 11;
    let num = array_1[0];
    println!("{}", num);
}
