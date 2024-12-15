fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Check if the index is within bounds before accessing the element
    let index = 10;
    if index < vec.len() {
        println!("Value at index {}: {}", index, vec[index]);
    } else {
        println!("Index {} is out of bounds", index);
    }

    // Another approach using the get() method which returns an Option
    let value_at_index_10 = vec.get(10);
    match value_at_index_10 {
        Some(value) => println!("Value at index 10: {}", value),
        None => println!("Index 10 is out of bounds")
    }
} 