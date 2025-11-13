// A rust program to calculate the area and volume of differnt shapes, based on user choice


use std::io;

fn main()
{
    loop {
    println!("Trapezium - T \nRhombus   - R\n Parallelogram - P\n Cube - U \n Cylinder - C  ");
    println!("Hello, please select the shape you want to calculate area or volume for  T/R/P/U/C");

    let mut input_1 = String::new();

    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let determiner:char= input_1.trim().to_uppercase().parse().expect("Invalid input");

    if determiner == 'T'{
        trapezium()
    }
     else if determiner == 'R'{
        rhombus()
    }
     else if determiner == 'P'{
        parallelogram()
    }
     else if determiner == 'U'{
        cube()
    }
      else if determiner == 'C'{
        cylinder()
    }
    else{
        println!("Invalid input, kindly select from the options below.");
    }
    continue;
    }


}


fn trapezium()
{
         println!("Let's calculate the area of your trapezium!");

         // Inputing for the area of a trapezium
        
         // For the height
          let mut input_3 = String::new();
          println!("Please input the height");
          io::stdin().read_line(&mut input_3).expect("Failed to read input");
          let h:f32= input_3.trim().parse().expect("Invalid input");

         // For the base1
          let mut input_4 = String::new();
          println!("Please input the base 1");
          io::stdin().read_line(&mut input_4).expect("Failed to read input");
          let b1:f32= input_4.trim().parse().expect("Invalid input");

          // For base2
          let mut input_5 = String::new();
          println!("Please input the base 2");
          io::stdin().read_line(&mut input_5).expect("Failed to read input");
          let b2:f32= input_5.trim().parse().expect("Invalid input");

          let area_t = h * 0.5 * (b1 + b2);
          println!("The area of the trapezium is {}", area_t);

}


fn rhombus()
{
        println!("Let's calculate the area of your Rhombus!");

         // Inputing for the area of a rhombus
        
         // For the diagonal1
          let mut input_3 = String::new();
          println!("Please input the diagonal 1");
          io::stdin().read_line(&mut input_3).expect("Failed to read input");
          let d1:f32= input_3.trim().parse().expect("Invalid input");

         // For the diagonal2
          let mut input_4 = String::new();
          println!("Please input the diagonal 2");
          io::stdin().read_line(&mut input_4).expect("Failed to read input");
          let d2:f32= input_4.trim().parse().expect("Invalid input");



          let area_r = 0.5 * d1 * d2;

          println!("The area of the rhombus is {}", area_r);
       }
fn parallelogram()
{
         println!("Let's calculate the area of your Parallelogram!");

         // Inputing for the area of a parallelogram
        
         // For the base
          let mut input_3 = String::new();
          println!("Please input the base");
          io::stdin().read_line(&mut input_3).expect("Failed to read input");
          let b:f32= input_3.trim().parse().expect("Invalid input");

         // For the altitude
          let mut input_4 = String::new();
          println!("Please input the altitude");
          io::stdin().read_line(&mut input_4).expect("Failed to read input");
          let a:f32= input_4.trim().parse().expect("Invalid input");



          let area_p = b * a;
          println!("The area of the parallelogram is {}", area_p);
}


fn cube()
{
        println!("Let's calculate the area of your Cube!");

          // Inputing for the area of a cube
        
          // For lenght of the side
          let mut input_3 = String::new();
          println!("Please input the lenght");
          io::stdin().read_line(&mut input_3).expect("Failed to read input");
          let l:f32= input_3.trim().parse().expect("Invalid input");

          let area_u = 6.0 * (l).powf(2.0);
          println!("The area of the cube is {}", area_u);
}



fn cylinder(){
          println!("Let's calculate the Volume of your Cylinder!");

          // Inputing for the volume of a cylinder
        
          // For radius
          let mut input_3 = String::new();
          println!("Please input the radius");
          io::stdin().read_line(&mut input_3).expect("Failed to read input");
          let r:f32= input_3.trim().parse().expect("Invalid input");
        
          // For height
          let mut input_4 = String::new();
          println!("Please input the height");
          io::stdin().read_line(&mut input_4).expect("Failed to read input");
          let h:f32= input_4.trim().parse().expect("Invalid input");

          let pi:f32 = 3.142;
          let volume = pi * r.powf(2.0) * h;
          println!("The volume of the cylinder is {}", volume);
}

