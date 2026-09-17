fn main(){
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let t:i32 = 3;

	let value = p * (1.0 - (r / 100.0)).powi(t);

	println!("The value after 3 years is {}", value);
}