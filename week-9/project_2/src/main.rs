use std::io::Write;

fn main() {
    let student_name = vec!["- Oluchi Mordi", "- Adams Aliyu", "- Shina Bolade", "- Adekunle Gold", "- Bianca Edemoh"];
    let matric_number = vec!["- ACC10211111", "- ECO10110101", "- CSC10328828", "- EEE11020202", "- MEE10202001"];
    let department = vec!["- Accounting", "- Economics", "- Computer", "- Electrical", "- Mechanical"];
    let level = vec!["- 300", "- 100", "- 200", "- 200", "- 100"];

    let mut file = std::fs::File::create("data.txt").expect("Create failed");
    
    // Write headers
    file.write_all("Student Name\n".as_bytes()).expect("Write failed");
    for name in &student_name {
        file.write_all(name.as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }
    
    file.write_all("\nMatric Number\n".as_bytes()).expect("Write failed");
    for matric in &matric_number {
        file.write_all(matric.as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }
    
    file.write_all("\nDepartment\n".as_bytes()).expect("Write failed");
    for dept in &department {
        file.write_all(dept.as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }
    
    file.write_all("\nLevel\n".as_bytes()).expect("Write failed");
    for lvl in &level {
        file.write_all(lvl.as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
    }
    
    println!("\nStudent data saved successfully!");
}