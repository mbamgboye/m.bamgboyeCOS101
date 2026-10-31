 fn main() {
    let v = vec![20, 30, 40, 50,60];
    //vector v owns the object in heap

    let v2 = v; 
    let v2_return = display(v2);
    println!("In main {:?}", v2_return);
}

fn display(v:Vec<i32>)->Vec<i32> {
    //returning same vector 
    println!("inside display {:?}",v);
    return v;
}
