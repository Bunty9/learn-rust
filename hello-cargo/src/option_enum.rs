// Option Enum and its advantages over Null values
// option enum encodes the very common scenario in which the value could be something or nothing in runtime
// rust doesnot have the null feature that most other languages include
// a null is a value that is currently invalid or absent for some reason.
// rust does not have null as such, But has a enum that can encode this concept of absent values
// that enum is Option<T> defined by the standard library as

// enum Option<T> {
//     Some(T),
//     None,
// }

//

use std::collections::HashMap;

// Example User management

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    age: Option<u32>,
}

impl User {
    fn new(id: u32, name: String) -> Self {
        User {
            id,
            name,
            email: None,
            phone: None,
            age: None,
        }
    }

    // Method to return option - common implementation
    fn get_display_name(&self) -> Option<String> {
        if self.name.trim().is_empty() {
            None
        } else {
            Some(format!("User Name:{}", self.name))
        }
    }

    // Chaining Option operations
    fn get_email_domain(&self) -> Option<String> {
        self.email
            .as_ref()? // early return if None
            .split('@') // split at @
            .nth(1) // get the second element
            .map(|s| s.to_string()) // convert to string, here if we dont use ';' then the default expression output is returned
    }

    // Working with multiple Options
    fn can_send_marketing(&self) -> bool {
        match (&self.email, &self.age) {
            (Some(_), Some(age)) if *age >= 18 => true, // for some email and some age if age is 18+ return true
            _ => false,                                 // return false for all other cases
        }
    }

    //
}

//Example Database system configuration

#[derive(Debug)]
struct Config {
    database_url: Option<String>,
    port: Option<u16>,
    api_key: Option<String>,
    debug_mode: Option<bool>,
}

impl Config {
    fn new() -> Self {
        Config {
            database_url: None,
            port: None,
            api_key: None,
            debug_mode: None,
        }
    }

    // using unwrap_or and unwrap_or_else
    fn get_database_port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }

    fn get_database_url(&self) -> String {
        self.database_url.clone().unwrap_or_else(|| {
            println!("No databse url provided");
            "localhost:5432".to_string()
        })
    }

    // Validation using option
    fn is_valid(&self) -> Option<String> {
        if self.database_url.is_none() {
            //error condition1 if on fb url retun some error
            return Some("No database url provided".to_string());
        }
        if self.api_key.as_ref().map_or(true, |key| key.len() < 10) {
            // error condition2 if key len less than 10 return some error
            return Some("Invalid Api Key".to_string());
        }
        None // default return none error = valid config will pass if all conditions above pass
    }
}

// Example 3 shopping cart system

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: Option<String>,
    discount: Option<f64>,
}

struct Store {
    products: HashMap<u32, Product>,
    featured_product: Option<u32>,
}

impl Store {
    fn new() -> Self {
        Store {
            products: HashMap::new(),
            featured_product: None,
        }
    }

    fn add_product(&mut self, product: Product) {
        // get self as mutable coz we are going to edit and insert into products
        self.products.insert(product.id, product);
    }

    // returning option from lookup
    fn find_product(&self, id: u32) -> Option<Product> {
        self.products.get(&id).cloned()
    }

    // chaining options and handling multiple lookups
    fn get_featured_product_name(&self) -> Option<String> {
        self.featured_product
            .and_then(|id| self.find_product(id))
            .map(|product| product.clone().name)
    }

    // complex option operations with filtering
    fn find_products_by_category(&self, category: &String) -> Vec<&Product> {
        self.products
            .values()
            .filter(|product| product.category.as_ref().map_or(false, |c| c == category))
            .collect()
    }

    // calculate discounted price
    fn get_discounted_price(&self, id: u32) -> Option<f64> {
        self.find_product(id).and_then(|product| {
            product
                .discount
                .map(|discount| product.price * (1.0 - discount))
        })
    }

    //using option in calculations
    fn calculate_category_average_price(&self, category: &String) -> Option<f64> {
        let prices: Vec<f64> = self
            .find_products_by_category(category)
            .iter()
            .map(|p| p.price)
            .collect();
        if prices.is_empty() {
            None
        } else {
            Some(prices.iter().sum::<f64>() / prices.len() as f64)
        }
    }
}

// Example4 File processing system
fn parse_number(s: &str) -> Option<i32> {
    s.trim().parse().ok() // convert result to option
}

fn safe_divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}

//combining multiple option operations

fn process_calculation(input1: &str, input2: &str) -> Option<f64> {
    let num1 = parse_number(input1)?;
    let num2 = parse_number(input2)?;

    safe_divide(num1, num2)
}


//Example5 cache system

