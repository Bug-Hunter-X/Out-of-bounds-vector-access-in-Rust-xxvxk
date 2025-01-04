fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    // Safe way to access the element at index 10 
    match vec.get(10) {
        Some(tenth) => println!("Tenth element: {}", tenth),
        None => println!("Index out of bounds")
    };
} 