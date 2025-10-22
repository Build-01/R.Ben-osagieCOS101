//Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
    println!("Let's find the roots of your quadratic equation!");

    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();


    println!("Enter the value for a :");
    io::stdin().read_line(&mut input_1).expect("Not not a valid string");
    let a:f32 = input_1.trim().parse().expect("Not a valid number");


    println!("Enter the value for b :");
    io::stdin().read_line(&mut input_2).expect("Not not a valid string");
    let b:f32 = input_2.trim().parse().expect("Not a valid number");


    println!("Enter the value for c :");
    io::stdin().read_line(&mut input_3).expect("Not not a valid string");
    let c:f32 = input_3.trim().parse().expect("Not a valid number");
    



    let  d = (b.powf(2.0) - (4.0 * a * c)).powf(0.5);
    let  x_1a = -b + d;
    let x_1 = x_1a/2.0 * a; 
    let  x_2a = -b - d;
    let x_2 = x_2a/2.0 * a; 

    if d > 0.0 {
        println!("Your equation has 2 distinct roots which are: {}", x_1);
        println!("{}", x_2);
    }
    else if d == 0.0 {
        println!("Your equation has one real root which are: {}",  x_1);
        println!("{}", x_2);

    }
    else if d < 0.0 {
        println!("There are no real roots for your equation {}",  x_1);
        println!("{:?}", x_2);

    }
}
