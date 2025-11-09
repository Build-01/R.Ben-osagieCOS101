//Rust program that displays the menu for food items available for order.

use std::io;

fn main() {

    println!("Welcome to Tarka, place your order now!");
    println!("Menu                                            Price 
             \n P - Poundo yam/ Edinkiako soup             - N3,200         
             \n F - fried Rice and Chicken                  - N3,000
             \n A - Amala and Ewedu soup                    - N2,500
             \n E - Eba and Egusi soup                      - N2,000
             \n W - White rice and stew                     - N2,500"); //Printing out the menu



    

    
    println!("Dear Customer, kindly enter the code of the food item listed above");
    let mut food_item = String::new();
    let mut quantity = String::new();

    // Type of food item
    io::stdin().read_line(&mut food_item).expect("Failed to read input");
    let food_item = food_item.trim().to_uppercase();

    // Inputing quantity
    println!("Enter quantity:");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: i32 = quantity.trim().parse().expect("Please input a valid number!");


    let price = match food_item.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,

               _ => {
            println!("Kindly select the valid food item!");
            return;
        }
    };
    let total = price * quantity;
    let discount = (total as f32) * 0.05;
    let final_amount = (total as f32) - discount;
    


    if total > 10_000 {

        println!("Congratulations customer, you won a 5% Discount");
        println!("Proceed to pay    N-{}", final_amount);
    } 
    else {
        println!("Kindly Proceed to pay N- {}",total);
    }

}
