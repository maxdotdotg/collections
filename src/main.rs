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

    // can't do mutable and immutable references in the same scope
    // this wouldn't work
    /* 
     * let mut vec![1,2,3,4,5];
     * let first = &v[0]; // immutable borrow
     * v.push(6); // mutable borrow, updating v
     * println!("the first element is {}", first); 
     * // crash, both refs are in the same scope
     */

    // iterate
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    // iterate and modify
    // doing a mutable borrow, since values are getting modified
    //
    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        // To change the value that the mutable reference refers to, we have 
        // to use the dereference operator (*) to get to the value in i 
        // before we can use the += operator.
        // ch08-01-vectors
        *i += 50;
    }
    // verify the changes by printing the contents of the vec
    for i in &v5 {
        println!("{}", i);
    }

    // a vec can have only one type
    // but that type can be an enum (including it's variants)

    // define the enum,
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // make a vec with it!
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
