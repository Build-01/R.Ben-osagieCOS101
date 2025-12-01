//A Rust program that saves student info file format

use std::io::Write;

fn main() {
    let headers = vec!["Student name", "Matric.Number", "Department", "Level"];
    let students = vec![
        vec!["Oluchi Mordi", "ACC10211111", "Accounting", "300"],
        vec!["Adams Aliyu", " ECO10110101", "Economics", " 100"],
        vec!["Shania Bolade", "CSC10328828", "Computer", "200"],
        vec!["Adekunle Gold", "EEE11020202", "Electrical", "200"],
        vec!["Bianca Edemoh", "MEE10202001", "Mechanical", "100"],
    ];

    let mut file = std::fs::File::create("PAU_SMIS.txt").expect("Create failed");
    
    // Write headers row
    let header_line = headers.join("\t");
    writeln!(file, "{}", header_line).expect("Write failed");
    
    // Write each student as a row
    for student in students {
        let student_line = student.join("\t");
        writeln!(file, "{}", student_line).expect("Write failed");
    }
    
    println!("\nStudent data saved successfully!");
}