 fn main() {
    let v = vec![101, 250, 330, 400];
    //vector v owns the object in heap

    let v2 = v.clone();
    //only a signle variable can own an objecct at a time
    //rust detects a race condition as two variables point to the same group.

    println!("{:?}", v2);
}
