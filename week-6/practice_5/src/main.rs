fn main() {
    let fullname = " Ben-Osagie Ruth ";

    println!();
    println!("Name: {}", fullname);
    println!();
    println!("Before trim");
    println!("Lenght is {}", fullname.len());
    println!();
    println!("After trim");
    println!("Lenght is {}", fullname.trim().len());
}
