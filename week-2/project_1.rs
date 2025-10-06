fn main(){

	//listing out the available parameters
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let _n:f64 = 5.0;
	//representing (r)/100.0 as x
	let x:f64 = (r)/100.0;

	//Getting the sum of x
    println!("The rate divided by 100 is {}", x );

	//Finding the value of p * (1.0 + x)
	let _y = p * (1.0 + x);
	let q = 1.0 + x;
	//Getting the fifth root of the function
	let z = q * q * q * q *  q;
	//getting the value for the amount
	let a = p * z;
	println!("The amount is {}", a);


	//compound interest
	let ci = a - p;
	println!("The compound interest is {}", ci);
}

