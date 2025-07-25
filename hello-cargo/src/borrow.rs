//the issue with ownership was that we needed to completely pass the ownership of the variable to the function
// this is not always what we want, sometimes we just want to use the variable without taking
// also we needed to write some funky code to return the variable back to the caller
// this is where borrowing comes in, we can borrow a variable without taking ownership of it

// borrowing
// borrowing is a way to allow a function to use a variable without taking ownership of it
// we can borrow a variable by using a reference, which is a pointer to the variable
// references are created using the '&' symbol

pub fn demonstrate_borrowing() {
    let s1 = String::from("Hello");

    let s2 = &s1; // Borrowing s1, s2 is a reference to s1
    let s3 = &s1; // We can borrow s1 multiple times
    let s4 = &s1; // Another reference to s1
    println!("s1: {}", s1);
    println!("s2: {}", s2); // s2 can be used without taking ownership of s1
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    //example of passing vars to functions by borrowing
    takes_ownership(&s1); // Passing a reference to s1
    println!("s1: {}", s1); // s1 can still be used here

    //mutable references
    let mut s5 = String::from("Hello, Rust!");
    println!("Before mutation: {}", s5);
    update_string(&mut s5); // Passing as a mutable reference to s5
    println!("After mutation: {}", s5); // s5 can still be used here
}

fn takes_ownership(some_string: &String) {
    println!("Received string: {}", some_string);
}

fn update_string(some_string: &mut String) {
    some_string.push_str(", world!"); // Mutating the borrowed string
}

//rules of borrowing
// 1.. You can have either one mutable reference or any number of immutable references to a variable at a time.
// 2.. References must always be valid, meaning they cannot outlive the data they point to.
// 3.. Mutable references cannot coexist with immutable references, ensuring data integrity.
// 4.. References are immutable by default, meaning you cannot change the value they point to.
