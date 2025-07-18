// // Immutable variables represent variables that cannot be changed once assigned
// by default, Rust variables are immutable
// to make a variable mutable, we need to use the 'mut' keyword
// knowing that a certain data will not change allows the compiler to optimize the code better

pub fn demonstrate_mutability() {
    println!("=== Mutability Examples ===");

    // Example 1: Immutable variable
    let x = 5;
    println!("Immutable x: {}", x);
    // x = 6; // This would cause a compile error!

    // Example 2: Mutable variable
    let mut y = 10;
    println!("Mutable y before change: {}", y);
    y += 5; // Now we can change it
    println!("Mutable y after change: {}", y);

    // Example 3: Mutable reference
    let mut z = String::from("Hello");
    change_string(&mut z);
    println!("String after mutation: {}", z);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!");
}
