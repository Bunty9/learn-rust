// enums, starter generics, result and error handling, pattern matching

// Enums allow you to define types by enumerating its possible variants
// to use enums and match code blocks to the specific case of an enum, rust provides pattern matching in the same form of a switch case as in javascript
// while using pattern matching we need to match code blocks for every case if any is left out code panics
#[derive(Debug)]
enum Directions {
    North,
    South,
    East,
    West,
}

pub fn demonstrate_enums() {
    let direction1 = Directions::North;
    turn_to_direction(direction1);
    let direction2 = Directions::South;
    turn_to_direction(direction2);
    let direction3 = Directions::East;
    turn_to_direction(direction3);
    let direction4 = Directions::West;
    turn_to_direction(direction4);
}

fn turn_to_direction(direction: Directions) {
    match direction {
        Directions::North => println!("Moving North"),
        Directions::South => println!("Moving South"),
        Directions::East => println!("Moving East"),
        Directions::West => println!("Moving West"),
    }
}

// there is on esimilarity in structs and enums
// just as we could define methods in structs by using implementation (impl)
// we are also able to define methods in enums using impl
#[derive(Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
enum Moves {
    Jump { h: u32 },
    Duck,
    Peek { k: Side },
    Walk { d: Directions, s: u32, v: u32 },
    Run { s: u32, v: u32 },
}

impl Moves {
    fn charge(&self) {
        match self {
            Moves::Jump { h } => {
                println!("Jumping with height: {}", h);
            }
            Moves::Duck => {
                println!("Ducking");
            }
            Moves::Peek { k } => match k {
                Side::Left => println!("Peeking Left"),
                Side::Right => println!("Peeking Right"),
            },
            Moves::Walk { d, s, v } => {
                println!("Walking: {:?}, speed: {}, velocity: {}", d, s, v);
            }
            Moves::Run { s, v } => {
                println!("Running: speed: {}, velocity: {}", s, v);
            }
        }
    }
    // Method that calculates energy cost
    fn energy_cost(&self) -> u32 {
        match self {
            Moves::Jump { h } => h * 2,         // Higher jumps cost more energy
            Moves::Duck => 1,                   // Low energy cost
            Moves::Peek { .. } => 1,            // Very low cost (.. ignores the data)
            Moves::Walk { s, v, .. } => s + v,  // Walking cost depends on speed and vigor
            Moves::Run { s, v } => (s + v) * 2, // Running costs the most
        }
    }
    // Method that returns a description
    fn describe(&self) -> String {
        match self {
            Moves::Jump { h } => format!("A {} meter high jump", h),
            Moves::Duck => "Ducking down low".to_string(),
            Moves::Peek { k } => format!("Peeking to the {:?}", k),
            Moves::Walk { d, s, v } => format!("Walking {:?} with speed {} and vigor {}", d, s, v),
            Moves::Run { s, v } => format!("Running with speed {} and vigor {}", s, v),
        }
    }
    // Method that checks if the move is fast
    fn is_fast_move(&self) -> bool {
        match self {
            Moves::Run { s, .. } if *s > 5 => true, // Fast if speed > 5
            _ => false,                             // Everything else is not considered fast
        }
    }
}

impl Side {
    fn opposite(&self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

pub fn demonstrate_enum_game() {
    // Create some move instances
    let jump = Moves::Jump { h: 3 };
    let duck = Moves::Duck;
    let peek_left = Moves::Peek { k: Side::Left };
    let peek_right = Moves::Peek { k: Side::Right };

    let run_fast = Moves::Run { s: 8, v: 6 };
    let walk_north = Moves::Walk {
        d: Directions::North,
        s: 3,
        v: 4,
    };

    // Test our methods
    println!("=== Testing our enum methods ===\n");

    //charge methods
    jump.charge();
    duck.charge();
    peek_left.charge();
    peek_right.charge();

    run_fast.charge();
    walk_north.charge();

    //energy cost for moves
    println!("Energy cost for jump: {}", jump.energy_cost());
    println!("Energy cost for duck: {}", duck.energy_cost());
    println!("Energy cost for peek: {}", peek_left.energy_cost());
    println!("Energy cost for run: {}", run_fast.energy_cost());
    println!("Energy cost for walk: {}", walk_north.energy_cost());

    //describe moves
    println!("Description for jump: {}", jump.describe());
    println!("Description for duck: {}", duck.describe());
    println!("Description for peek: {}", peek_left.describe());
    println!("Description for run: {}", run_fast.describe());
    println!("Description for walk: {}", walk_north.describe());

    //is_fast_move
    println!("Is jump fast? {}", jump.is_fast_move());
    println!("Is duck fast? {}", duck.is_fast_move());
    println!("Is peek fast? {}", peek_left.is_fast_move());
    println!("Is run fast? {}", run_fast.is_fast_move());
    println!("Is walk fast? {}", walk_north.is_fast_move());

    let leftside = Side::Left;
    println!("Opposite of {:?} is {:?}", leftside, leftside.opposite());
}
