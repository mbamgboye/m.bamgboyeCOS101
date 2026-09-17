fn main(){
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let t:i32 = 5;

	//amount 
	let a = p * (1.0 + (r / 100.0)).powi(t);

	//Compound interest
	let c = a - p;

	println!("The Compound interest is {}", c);
}