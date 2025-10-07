fn main(){
	//Listing all parameters 
	let a:f64 = 450_000.00;
	let b:f64 = 1_500_000.00;
	let c:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let e:f64 = 250_000.00;

	//Outputing the represented variables
	println!("The amount of Toshiba is {}", a );
	println!("The amount of Mac is {}", b );
	println!("The amount of Hp is {}", c );
	println!("The amount of Dell is {}", d );
	println!("The amount of Acer is {}", e );


	//Calculationg the sum 

	let _f = a  + b  + c + d +  e;
	println!("The sum is {}", _f);

	//Calculating the average

    let m = (a  + b  + c + d +  e)/5.0;
    println!("The average is {}", m);

}
