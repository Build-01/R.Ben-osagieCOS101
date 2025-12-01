struct Device {
    name: String,
    quantity: u32,
    price_per_unit: u32  // Renamed to avoid conflict with method name
}

impl Device {
    fn total_price_for_three(&self) -> u32 {
        self.price_per_unit * 3
    }
}

fn main() {
    let device1 = Device {
        name: String::from("HP laptop"),
        quantity: 10,
        price_per_unit: 650_000
    };

    let device2 = Device {
        name: String::from("IBM laptop"),
        quantity: 6,
        price_per_unit: 755_000
    };

    let device3 = Device {
        name: String::from("Toshiba laptop"),
        quantity: 10,
        price_per_unit: 550_000
    };

    let device4 = Device {
        name: String::from("Dell laptop"),
        quantity: 4,
        price_per_unit: 850_000
    };

    // Calculate price for 3 quantities of each device
    let price1 = device1.total_price_for_three();
    let price2 = device2.total_price_for_three();
    let price3 = device3.total_price_for_three();
    let price4 = device4.total_price_for_three();

    // Calculate total
    let total = price1 + price2 + price3 + price4;

    println!("Price for 3 {} is : {}", device1.name, price1);
    println!("Price for 3 {} is : {}", device2.name,price2);
    println!("Price for 3 {} is : {}", device3.name, price3);
    println!("Price for 3 {} is : {}", device4.name, price4);
    println!("The total price for 3 of each device is: {}", total);
 }
