mod borrow;
mod enums;
mod hashmaps;
mod heap_stack;
mod mem_mgmt;
mod mutability;
mod option_enum;
mod ownership;
mod structs;
mod vectors;

fn main() {
    println!("Hello, cargoo!!!");

    let x = 5;
    let y = -10;
    let z = 100.345;

    print!("x:{}, y:{}, z:{}\n", x, y, z);

    let s = "Hello, Rust!";
    println!("s:{}", s);

    let is_male = true;
    let is_above_18 = true;

    if is_male {
        println!("male");
    } else {
        println!("not a male");
    }

    if is_male && is_above_18 {
        println!("legal male");
    }

    let greeting = String::from("hello, welcome to Rust programming!");
    println!("{}", greeting);

    let index = 7;
    let _char1 = greeting.chars().nth(index);

    match _char1 {
        Some(c) => println!("character at index {} is: {}", index, c),
        None => println!("No character found"),
    }

    for i in 0..10 {
        println!("{}", i);
    }

    //iterate over arrays, maps, strings, etc.
    let sentence = String::from("Hello Rust programming language");
    let first_word_result = first_word(sentence);
    println!("First word: {}", first_word_result);

    let word = String::from("Rust");
    let reversed_word = reverse_string(word);
    println!("Reversed word: {}", reversed_word);

    println!("\n{}", "=".repeat(50));
    println!("MEMORY MANAGEMENT EXAMPLES");
    println!("{}", "=".repeat(50));

    mem_mgmt::demonstrate_ownership();
    mem_mgmt::demonstrate_borrowing();
    mem_mgmt::demonstrate_slices();
    mem_mgmt::demonstrate_stack_vs_heap();
    mem_mgmt::demonstrate_lifetimes();
    mem_mgmt::demonstrate_smart_pointers();

    println!("\n{}", "=".repeat(50));
    println!("MUTABILITY EXAMPLES");
    println!("{}", "=".repeat(50));

    mutability::demonstrate_mutability();

    println!("\n{}", "=".repeat(50));
    println!("STACK AND HEAP EXAMPLES");
    println!("{}", "=".repeat(50));

    heap_stack::demonstrate_stack_vs_heap();

    println!("\n{}", "=".repeat(50));
    println!("OWNERSHIP EXAMPLES");
    println!("{}", "=".repeat(50));

    ownership::vars_in_functions();
    ownership::scope_in_function();
    ownership::strings_in_functions();
    ownership::strings_in_funtions_2();

    borrow::demonstrate_borrowing();

    structs::demonstrate_structs();
    structs::demonstrate_methods();
    structs::demonstrate_debug();
    structs::demonstrate_unit_structs();

    println!("\n{}", "=".repeat(50));
    println!("ENUMS EXAMPLES");
    println!("{}", "=".repeat(50));
    enums::demonstrate_enums();
    enums::demonstrate_enum_game();

    println!("\n{}", "=".repeat(50));
    println!("VECTORS EXAMPLES");
    println!("{}", "=".repeat(50));
    vectors::demostrate_vectors();

    println!("\n{}", "=".repeat(50));
    println!("HASHMAPS EXAMPLES");
    println!("{}", "=".repeat(50));

    hashmaps::demonstrate_hashmaps();
}

//this is where ownership comes into picture
fn first_word(sentence: String) -> String {
    let mut end_index = 0;
    for (i, c) in sentence.chars().enumerate() {
        if c == ' ' {
            end_index = i;
            break;
        }
    }
    sentence[0..end_index].to_string()
}

fn reverse_string(word: String) -> String {
    let mut reversed = String::new();
    for c in word.chars().rev() {
        reversed.push(c);
    }
    reversed
}
