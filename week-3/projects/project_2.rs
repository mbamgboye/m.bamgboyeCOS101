fn main() {
    let items = ["toshiba", "mac", "hp", "dell", "acer"];

    let quantities = [2, 1, 3, 3, 1];
    let amounts = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    let total_quantity: i32 = quantities.iter().sum();
    let total_amount: f64 = amounts.iter().sum();

    let average_quantity = total_quantity as f64 / quantities.len() as f64;
    let average_amount = total_amount / amounts.len() as f64;

    println!("Sales Record");

    for i in 0..items.len() {
        println!(
            "{}: Quantity = {}, Amount = {}",
            items[i], quantities[i], amounts[i]
        );
    }

    println!("\nTotal quantity sold: {}", total_quantity);
    println!("Total sales amount: {}", total_amount);
    println!("Average quantity: {}", average_quantity);
    println!("Average sales amount: {}", average_amount);
}