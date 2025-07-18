// Whenever you run a program (C++, Rust, JS), it allocates and deallocates memory on the RAM.
// 3 popular ways of doing memory management:
// garbage collection (GC)
//   usually written by smart people, no dangling pointers, no leaks, you cant do manual memory management. ex: Java, Python, JS
// manual memory management
//  you allocate and deallocate memory yourself, can lead to dangling pointers, memory leaks, race conditions, buffer overflow (ctf attack vector), learning curve is high, examples: C, C++ (malloc, calloc, free)
// The Rust way
// Rust has its own ownership model, for memory management, makes it extremely safe, no dangling pointers, no memory leaks, no garbage collection.
// rust forces you to write code such that you cannot have these common memory management issues.

pub fn demonstrate_ownership() {
    println!("=== Ownership Examples ===");
    
    // Example 1: Basic ownership
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This would cause a compile error!
    
    // Example 2: Clone to avoid move
    let s3 = String::from("World");
    let s4 = s3.clone(); // Deep copy
    println!("s3: {}, s4: {}", s3, s4);
    
    // Example 3: Function ownership
    let name = String::from("Rust");
    take_ownership(name);
    // println!("name: {}", name); // This would cause a compile error!
    
    let x = 5;
    make_copy(x); // Copy types (like integers) don't move
    println!("x is still valid: {}", x);
}

fn take_ownership(some_string: String) {
    println!("Taking ownership of: {}", some_string);
} // some_string goes out of scope and is dropped

fn make_copy(some_integer: i32) {
    println!("Copy of integer: {}", some_integer);
} // some_integer goes out of scope, but nothing special happens

pub fn demonstrate_borrowing() {
    println!("\n=== Borrowing Examples ===");
    
    // Example 1: Immutable borrowing
    let s1 = String::from("Hello, borrowing!");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
    
    // Example 2: Mutable borrowing
    let mut s2 = String::from("Hello");
    change_string(&mut s2);
    println!("After modification: {}", s2);
    
    // Example 3: Multiple immutable references
    let s3 = String::from("Multiple refs");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {}, r2: {}", r1, r2);
    
    // Example 4: Borrowing rules demonstration
    let mut s4 = String::from("Borrowing rules");
    {
        let r1 = &mut s4;
        println!("Mutable reference: {}", r1);
    } // r1 goes out of scope
    let r2 = &s4; // Now we can create an immutable reference
    println!("Immutable reference: {}", r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it doesn't have ownership, nothing happens

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!");
}

pub fn demonstrate_slices() {
    println!("\n=== Slice Examples ===");
    
    let s = String::from("Hello world programming");
    
    // String slices
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("First word: {}, Second word: {}", hello, world);
    
    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("Array slice: {:?}", slice);
    
    // Using slices to find first word
    let first_word = get_first_word(&s);
    println!("First word using slice: {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

pub fn demonstrate_stack_vs_heap() {
    println!("\n=== Stack vs Heap Examples ===");
    
    // Stack allocated data (fixed size, fast)
    let stack_var = 42; // i32 stored on stack
    let stack_array = [1, 2, 3, 4, 5]; // Array stored on stack
    println!("Stack variable: {}", stack_var);
    println!("Stack array: {:?}", stack_array);
    
    // Heap allocated data (dynamic size, slower)
    let heap_string = String::from("This is on the heap");
    let mut heap_vec = Vec::new();
    heap_vec.push(1);
    heap_vec.push(2);
    heap_vec.push(3);
    
    println!("Heap string: {}", heap_string);
    println!("Heap vector: {:?}", heap_vec);
    
    // Box: explicit heap allocation
    let boxed_value = Box::new(100);
    println!("Boxed value on heap: {}", boxed_value);
}

pub fn demonstrate_lifetimes() {
    println!("\n=== Lifetime Examples ===");
    
    let string1 = String::from("long string is long");
    let result;
    
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("The longest string is: {}", result);
    }
    
    // Lifetime with structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("Important excerpt: {}", excerpt.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn demonstrate_smart_pointers() {
    println!("\n=== Smart Pointer Examples ===");
    
    // Box<T> - heap allocation
    let b = Box::new(5);
    println!("Boxed value: {}", b);
    
    // Reference counting with Rc<T>
    use std::rc::Rc;
    let rc_string = Rc::new(String::from("Shared ownership"));
    let rc_clone1 = Rc::clone(&rc_string);
    let _rc_clone2 = Rc::clone(&rc_string);
    
    println!("RC count: {}", Rc::strong_count(&rc_string));
    println!("Value: {}", rc_string);
    
    drop(rc_clone1);
    println!("RC count after dropping clone1: {}", Rc::strong_count(&rc_string));
}
