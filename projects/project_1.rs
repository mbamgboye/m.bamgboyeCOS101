fn main(){
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	//amount 
	let a = p * (1.0 + (r / 100)) * t;

	//Compound interest
	let C = a - p;

	println!("The Compound interest is {}", C);
}