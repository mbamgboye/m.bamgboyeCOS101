 use std::io::Write;

 fn main() {
    let lager = "33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n";
    let stout = "Legend, Turbo Kings, Williams\n";
    let nonalcoholic = "Maltina, Amstel Malta, Malta Gold, Fayrouz";

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all("The Nigerian Brewries Drinks:\n"
        .as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(nonalcoholic.as_bytes()).expect("write failed");
    println!("\nData written to file.");
    
 }