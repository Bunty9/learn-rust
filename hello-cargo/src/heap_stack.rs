// rust has clear rules for heap and stack memory management

// Stack:
// fast allocation and deallocation, Rust uses stact for most primitive data types
// fixed size, data is stored in a contiguous block of memory
// data is stored in a last-in-first-out (LIFO) manner

// Heap:
// dynamic allocation, used for data whose size is not known at compile time
// used for data that can grow or shrink in size in runtime, like vectors or strings
// slower than stack, as it requires more complex memory management,
// ex: after mutation and string grows but there is no contiguous memory in front of the string in heap then the os needs to move the entire heap to some other location

// you still store the address of the heap data on the stack as a pointer

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);

    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    // for _ in 0..50 {
    //     s.push_str(" and some additional text");
    //     println!(
    //         "Capacity: {}, Length: {}, Pointer: {:p}",
    //         s.capacity(),
    //         s.len(),
    //         s.as_ptr()
    //     );
    // }
}

pub fn demonstrate_stack_vs_heap() {
    println!("\n=== Stack vs Heap Memory Management ===");

    // Demonstrate stack memory
    stack_fn();

    // Demonstrate heap memory
    heap_fn();

    // Update a string on the heap
    update_string();
}
