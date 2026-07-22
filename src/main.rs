use std::io;

fn main() {
    println!("-------------------------------- ");
    println!("----- Working with Collection -----");
    println!("-------------------------------- ");
    println!("I want to create a vector, define the lengh of it, please.");
    println!("Please input your value: ");


    let mut v: Vec<i32> = Vec::new();

    let mut v_length  = String::new();

    io::stdin()
        .read_line(&mut v_length)
        .expect("Failed to read line");
        
    print!("Your vector length: {}", v_length);

    println!("-------------------------------- ");
    println!("----- Adding new values to the vector -----");
    v.push(13);
    v.push(56);
    v.push(87);

    println!("---- Showing each vector element -----");
    for i in &mut v {
        println!("The value is: {}", i);
    }

    // READING VECTION ELEMENTS USING THE INDEXING SYNTAX
    // & [] GIVERS A REFERENCE TO THE ELEMENT
    println!("---- Making reference to a vector element [OPT 1]-----");
    let third_opt1 = &v[2];  
    
    println!("The third element is: {}", third_opt1);
    

    // READING VECTION ELEMENTS USING THE get() METHOD
    // .get() RETURNS AN OPTION<&T> VALUE, WHICH IS EITHER SOME(&T) IF THE ELEMENT EXISTS OR NONE IF IT DOES NOT.
    println!("---- Making reference to a vector element [OPT 2]-----");
    let third_opt2 = v.get(2);
    match third_opt2{
        Some(third_opt2) => println!("The third element is: {}", third_opt2),
        None=> println!("There is not a third element in the vector"),
    }
    
}
