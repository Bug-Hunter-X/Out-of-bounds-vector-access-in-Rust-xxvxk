fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    //Try to access the element at index 10
    let tenth = vec[10];
    println!("Tenth element: {}", tenth);
}