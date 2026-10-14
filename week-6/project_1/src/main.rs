use std::io;

fn main() {
    // Display the menu
    println!("Welcome to The Restaurant!");
    println!("P - Poundo Yam / Edinkaiko Soup : N3,200");
    println!("F - Fried Rice & Chicken        : N3,000");
    println!("A - Amala & Ewedu Soup          : N2,500");
    println!("E - Eba & Egusi Soup            : N2,000");
    println!("W - White Rice & Stew           : N2,500");

    // Input reading for food selection
    println!("\nEnter food type letter (P, F, A, E, W):");
    let mut food_input = String::new();
    io::stdin().read_line(&mut food_input).expect("Failed to read input");
    let food_type = food_input.trim().to_uppercase();

    // Input reading for quantity
    println!("Enter quantity:");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    
    // Parse quantity to a floating number for math operations
    let quantity: f32 = qty_input.trim().parse().expect("Please enter a valid number");

    let price: f32 ;

    // A decision on the letter
    if food_type == "P" {
        price = 3_200.0;
    } else if food_type == "F" {
        price = 3_000.0;
    } else if food_type == "A" {
        price = 2_500.0;
    } else if food_type == "E" {
        price = 2_000.0;
    } else if food_type == "W" {
        price = 2_500.0;
    } else {
        println!("Invalid choice selection.");
        return;
    }

    // Arithmetic for the total
    let mut total: f32 = price * quantity;
    println!("\nSubtotal: N{}", total);

    //the discount
    if total > 10_000.0 {
        let discount: f32 = total * 0.05;
        total = total - discount;
        println!("5% Discount applied: -N{}", discount);
    }

    println!("Total Charge: N{}", total);
    println!("Enjoy your meal, Thanks for shopping!");
}

