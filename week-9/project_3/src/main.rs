// A rust program that merges three files into one format using vectors


use std::io::Write;

fn main() {
    let headers = vec!["S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONES"];
    let info = vec![
        vec!["1","Ajagbogun Alamba Dauda", "Internal Affairs", "South West",],
        vec!["2","Murtala Afeez Bendu", " Justice", "North east",],
        vec!["3","Okorocha Calistus Ogbona", "Defense", "South South",],
        vec!["4","Adewale Jimoh Akanbi", "Power & Steel", "South West",],
        vec!["5","Osazuwa Faith Etieye", "Petroleum", "South East",],
    ];

    let mut file = std::fs::File::create("COMMISSIONER'S INFO.txt").expect("Create failed");
    
    // Write headers row
    let header_line = headers.join("            ");
    writeln!(file, "{}", header_line).expect("Write failed");
    
    // Write each student as a row
    for commissioner in info {
        let commissioner_line = commissioner.join("          ");
        writeln!(file, "{}", commissioner_line).expect("Write failed");
    }
    
    println!("\nData merged successfully!");
}