fn main() {
    /* 
     * Vectors allow you to store more than one value in a single data structure
     * that puts all the values next to each other in memory. Vectors can only
     * store values of the same type. They are useful when you have a list of
     * items, such as the lines of text in a file or the prices of items 
     * in a shopping cart.
     * ch08-01-vectors.html
    */
    // Vec requires a type signature
    let mut v: Vec<i32> = Vec::new(); 

    // vec also has a macro
    let v2 = vec!(2, 3, 4);

    // add items to vectors with push
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // read some items in a vector
    // don't forget to borrow the reference in the type signature!
    let third: &i32 = &v[2]; 
    println!("accessing by brackets for index, the third value in v is {}", third);

    match v.get(2) {
        Some(third) => println!("match on vector.get(index) sez this is the third value: {}", third),
        None => println!("there is no third element"),
    }
}
