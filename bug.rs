fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // this will cause a panic if you try to access vec[10] because the index is out of bounds
    println!("Value at index 10: {}", vec[10]);
}