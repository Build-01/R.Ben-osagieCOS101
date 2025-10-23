//Rust program to determine the annual incentive of an employee

use std::io;


fn main() {
   

    loop{
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    println!(" Do you have an experience in this job? Y/N:");
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
     let input_1 = input_1.trim().to_uppercase();  //To enable my code not to break because of some case sensitivity issue
   
    if  input_1 != "Y" && input_1 != "N"{
           println!("Invalid input. You can only enter 'Y' or 'N'");
           continue;
           } 

     println!("Please input your age :");
    io::stdin().read_line(&mut input_2).expect("Not not a valid string");
    let age:u32 = input_2.trim().parse().expect("Not a valid number");
    
        if input_1 == "Y"{
            println!("You're an experienced worker and your age is : {}", age);
        
        }
        else if input_1 == "N"{
            println!("You are inexperienced and your age is : {}", age);
                   }
    
        if input_1 == "Y" && age >=40{
            println!("Your incentive is 1,560,000");
        }
        else if input_1 == "Y" && age >= 30 && age < 40{
            println!("Your incentive is 1,480,000");
        }
        else{
            println!("Your incentive is 100,000");
        }
        break;

        }



 

        
          
    
    }





