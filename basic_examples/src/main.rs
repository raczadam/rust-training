use std::i32;

use colored::Colorize;
fn main() {
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();

    problem_plus_one(100);
}

// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/
fn problem_1() {
    println!("{}", "Problem1 - Type an i32 number!".green());
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;

    let mut i = 0;
    while n > i {
        i += 1;
        square_of_sum = square_of_sum + i;
        sum_of_squares = sum_of_squares + i.pow(2);
    }
    square_of_sum = square_of_sum.pow(2);
    println!("square_of_sum: {}", square_of_sum);
    println!("sum_of_squares: {}", sum_of_squares);
}

// Problem 2:
/*
Write a program to find the sum of natural numbers below a given number N, where N is provided by the user.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/
fn problem_2() {
    println!("{}", "Problem1 - Type an i32 number!".blue());
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    /* Add your code below this line */

    let mut sum: i32 = 0;
    let mut counter: i32 = 0;
    while counter < n {
        if counter % 3 == 0 || counter % 5 == 0 {
            sum = sum + counter;
        }
        counter += 1;
    }
    println!("The amount of sum is: {}", sum);
}

// Problem 3:

/*
This question involves writing code to analyze the production of an assembly line in a car factory.
The assembly line has different speeds, ranging from 0 (off) to 10 (maximum).
At the lowest speed of 1, the assembly line produces a total of 221 cars per hour.
The production rate increases linearly with the speed,
meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

However, higher speeds increase the likelihood of producing faulty cars that need to be discarded.
The success rate depends on the speed, as shown in the table below:
· Speeds 1 to 4: 100% success rate.
· Speeds 5 to 8: 90% success rate.
· Speeds 9 and 10: 77% success rate.

You need to write two functions:
1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.
2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.
Write the code for both functions based on the provided specifications.
*/

fn problem_3() {
    println!("{}", total_production(6, 5) as i32); // to round the values we use i32. just ignore for mow
    println!("{}", cars_produced_per_minutes(6, 5) as i32); // to round the values we use i32. just ignore for mow
}

fn total_production(hours: u8, speed: u8) -> f32 {
    let success_rate: f32 = success_rate(speed);
    return 221.0 * (hours as f32) * (speed as f32) * success_rate;
}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    return total_production(hours, speed) / (60.0 * hours as f32);
}

fn success_rate(speed: u8) -> f32 {
    if speed <= 4 {
        return 1.0;
    } else if speed >= 5 && speed <= 8 {
        return 0.9;
    } else {
        return 0.77;
    }
}

// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/
fn problem_4() {
    let input = String::from("1211");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );

    let input = String::from("1221");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );
}

fn palindrome(input: String) -> bool {
    if input.len() == 0 {
        return true;
    }
    let mut last = input.len() - 1;
    let mut first = 0;

    let my_vec = input.as_bytes();

    while first < last {
        if my_vec[first] != my_vec[last] {
            return false;
        }
        first += 1;
        last -= 1;
    }
    return true;
}

// Problem 5:
/*
A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2.
These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).

Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000.
*/
fn problem_5() {
    for a in 1..=1000 {
        for b in a..=1000 - a {
            let c = 1000 - a - b;
            if is_pythagorean_triplet(a, b, c) {
                println!("Got a triplet {:?}", (a, b, c));
            }
        }
    }
}

fn is_pythagorean_triplet(a: i32, b: i32, c: i32) -> bool {
    a * a + b * b == c * c
}

// Problem 6:

/*
Write a function that implements the logic,
'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'
*/
fn problem_6() {
    println!("{}", can_see_movie(17, false));
    println!("{}", can_see_movie(13, true));
    println!("{}", can_see_movie(13, false));
}

fn can_see_movie(age: i32, permission: bool) -> bool {
    // Write your code here to implement the logic
    (age >= 17) || (age >= 13 && permission)
}

/*
   Print all pythagorean triplet, where a, b, or c side is less, than param number.
*/
fn problem_plus_one(number: i16) {
    let mut counter: i64 = 0;
    for a in 1..=number {
        for b in (a + 1)..number {
            for c in (b + 1)..number {
                counter += 1;
                if is_pythagorean_triplet(a as i32, b as i32, c as i32) {
                    println!("Got a triplet {:?}", (a, b, c));
                }
            }
        }
    }
    println!("Available steps: {}", counter);
}
