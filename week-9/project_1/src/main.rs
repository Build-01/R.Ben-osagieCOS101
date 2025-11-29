use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Nigerian_breweries_Limited.txt")
        .expect("Create failed");
    
    //  LAGER category with its drinks
    file.write_all("LAGER          STOUT          NON-ALCOHOLICS\n".as_bytes()).expect("Write failed");
    file.write_all("33 Export      Legend         Maltina\n".as_bytes()).expect("Write failed");
    file.write_all("Desperados     Turbo King     Amstel Maltina\n".as_bytes()).expect("Write failed");
    file.write_all("Goldberg       Williams       Malta Gold\n".as_bytes()).expect("Write failed");
    file.write_all("Gulder                        Fayrouz\n".as_bytes()).expect("Write failed");
    file.write_all("Heineken\n".as_bytes()).expect("Write failed");
    file.write_all("Star\n".as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed"); 
    
    
    
    println!("The high quality of drinks have been saved successfully!");
}
