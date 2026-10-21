use std::io::Read;
use std::io::Write;


fn main() {
    let announce = "Hi, welcome to the text\n";
    let dept = "Department of Software Engineering";

    let mut file = std::fs::File::create("welcome_message.txt").expect("create failed");
    file.write_all("Welcome!!\n"
        .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");


    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
