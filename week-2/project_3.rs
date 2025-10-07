fn main(){
	//Listing out all given parameters
	let p:f64 = 510_000.00;
	let r:f64 = 5.0;
	let _n:f64 = 3.0;
    
    //breaking down the calculation
	let x = 1.0 - r/100.0;
	println!(" still processing ...{}", x);

	//Finding the third sqare of x which was represented by let x = 1.0 - r/100.0;
	let y = x * x * x;
	
	//Finding the value of the TV using the compound interest formular
	let a = p * y;
	println!("The Value of the TV is {}", a);


}