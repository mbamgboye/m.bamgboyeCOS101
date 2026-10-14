fn main() {
    //Using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of the vector 
    println!("\nThe size of Vec::new() is: {}", v.len());

    //using macro
    let v = vec!["Grace", "Effiong", "Chudi", "Samuel", "Haleemah"];

    //printing the size of the vector 
    println!("\nThe length of vec macro is: {}", v.len());
}
