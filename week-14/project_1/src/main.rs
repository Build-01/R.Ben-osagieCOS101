use std::io::{self, Read};
use std::fs::File;

fn admin() {
    let mut file = File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn project_manager() {
    let mut file = File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee() {
    let mut file = File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer() {
    let mut file = File::open("CustomerTable_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn vendor() {
    let mut file = File::open("DataPlan.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main() {
    println!("Dear User, Kindly enter you role:");
    println!("Admin , Project_manager , employee , Customer , Vendor");

    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();
    let role = role.trim();

    match role {
        "admin" => admin(),
        "project_manager" => project_manager(),
        "employee" => employee(),
        "customer" => customer(),
        "vendor" => vendor(),
        _ => println!("Invalid role"),
    }
}
