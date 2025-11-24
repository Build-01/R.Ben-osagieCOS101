use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Nigerian_breweries_Limited.txt")
        .expect("Create failed");
    
    // Write LAGER category with its drinks
    file.write_all("LAGER\n".as_bytes()).expect("Write failed");
    file.write_all("- 33 Export\n".as_bytes()).expect("Write failed");
    file.write_all("- Desperados\n".as_bytes()).expect("Write failed");
    file.write_all("- Goldberg\n".as_bytes()).expect("Write failed");
    file.write_all("- Gulder\n".as_bytes()).expect("Write failed");
    file.write_all("- Heineken\n".as_bytes()).expect("Write failed");
    file.write_all("- Star\n".as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed"); 
    
    // Write STOUT category with its drinks  
    file.write_all("STOUT\n".as_bytes()).expect("Write failed");
    file.write_all("- Legend\n".as_bytes()).expect("Write failed");
    file.write_all("- Turbo King\n".as_bytes()).expect("Write failed");
    file.write_all("- Williams\n".as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
    
    // Write NON-ALCOHOLICS category with its drinks
    file.write_all("NON-ALCOHOLICS\n".as_bytes()).expect("Write failed");
    file.write_all("- Maltina\n".as_bytes()).expect("Write failed");
    file.write_all("- Amstel Malta\n".as_bytes()).expect("Write failed");
    file.write_all("- Malta Gold\n".as_bytes()).expect("Write failed");
    file.write_all("- Fayrouz\n".as_bytes()).expect("Write failed");
    
    println!("The high quality of drinks have been saved successfully!");
}
