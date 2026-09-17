use std::io;

fn main() {
    println!("Enter a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut a:f64 = input1.trim().parse().expect("Failed to input");

    println!("Enter b: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut b:f64 = input1.trim().parse().expect("Failed to input");

    println!("Enter c: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut c:f64 = input1.trim().parse().expect("Failed to input");


    let d:f64 = b*b - 4.0*a*c;
    if d > 0.0{
        let root1:f64 = (-b + d.sqrt()) / (2.0 * a);
        let root2:f64 = (-b - d.sqrt()) / (2.0 * a);  

        println!("Root 1 is: {}", root1);
        println!("Root 2 is: {}", root2);
    }
    else {
        println!("No real roots");
    }
}
