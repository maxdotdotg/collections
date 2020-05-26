fn main() {
    /* 
     * Vectors allow you to store more than one value in a single data structure
     * that puts all the values next to each other in memory. Vectors can only
     * store values of the same type. They are useful when you have a list of
     * items, such as the lines of text in a file or the prices of items 
     * in a shopping cart.
     * ch08-01-vectors
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
        // the get method with the index passed as an argument, which gives us
        // an Option<&T>
        // ch08-01-vectors
        Some(third) => println!("match on vector.get(index) sez this is the third value: {}", third),
        None => println!("there is no third element"),
    }

    let v3 = vec!(1, 2);
    match v3.get(2) {
        Some(third) => println!("{} is the third value", third),
        None => println!("there's no third value"), // yep, this arm is executed
        // and it gets executed without panicking
    }
}
