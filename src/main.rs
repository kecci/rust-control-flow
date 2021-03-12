fn main() {
    /* if Expressions */
    let age = 13;
    if age >= 13 {
      println!("Yes, your age is in an acceptable range");
    } else {
      println!("Minimum age is 13");
    }

    // Multiple Conditions with else if Keyword
    let name = "John";

    if name == "Jane" {
        println!("Jane logged in");
    } else if name == "John" {
        println!("John logged in");
    } else {
        println!("I don't know this person. Call the security");
    }

    // else if keyword is a way to combine if expressions
    let name = "Ali";

    if name == "Jane" {
        println!("Jane logged in");
    } 

    if name == "John" {
        println!("John logged in");
    } 

    if name != "John" && name != "Jane" {
        println!("I don't know this person. Call the security");
    }

    // if with let keyword
    let is_valid = true;
    let the_page = if is_valid { "admin" } else { "user" };

    println!("You see the {} page", the_page);

    /* Loops in Rust */
    
    // loop keyword
    // loop {
    //     println!("I'm living here");
    // }

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 1000 {
        break counter;
        }
    };

    println!("The result is {}", result);

    // while loop
    let mut counter = 0;
    while counter != 3 {
        counter += 1;

        println!("Counter is {}", counter);
    }
    println!("Loop terminated!");

    // for loop
    let names = ["John", "Jane", "Ben", "Jack", "Burak", "Ali"];
    let mut index = 0;

    while index < 6 {
        println!("the name is: {}", names[index]);

        index += 1;
    }

    // iterate collections such as arrays with for loop in Rust
    let names = ["John", "Jane", "Ben", "Jack", "Burak", "Ali"];

    for name in names.iter() {
        println!("the name is: {}", name);
    }

    // let names = ["John", "Jane", "Ben"];
    // while index < 6 {
    //     println!("the name is: {}", names[index]);
    //     index += 1;
    // }
    // code will panic. 

    // loop in range with for loop
    let first_number = 1;
    let second_number = 5;

    for number in first_number..second_number {
        println!("the number is: {}", number);
    }
}