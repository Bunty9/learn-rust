//vectors allow you to store more than one value
//in a single data structure that puts
//all the values next to each other in memory

//example 1
//functoin that takes vectors as input and responds a vector with even values

use std::vec;

pub fn demostrate_vectors() {
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);
    vec1.push(5);
    println!("vec1: {:?}", vec1);
    let vec2 = even_filter(vec1);
    println!("vec2 with only even values: {:?}", vec2);
    // println!("vec1: {:?}", vec1);  // here you cannot use vec1 because the even_filter funtion has taken ownership and after completion of the function the vec1 is disposed

    //in order to use the vec1 again we need to pass the vec1 as a reference to the even_filter
    let mut vec3 = Vec::new();
    vec3.push(1);
    vec3.push(2);
    vec3.push(3);
    vec3.push(4);
    vec3.push(5);
    println!("vec3: {:?}", vec3);
    let vec4 = even_filter_by_ref(&vec3);
    println!("vec4 with only even values: {:?}", vec4);
    println!("vec3: {:?}", vec3); // here we can use the vec3 again as we only passed it as a refence to the fn

    even_filter_inplace(&mut vec3); // transfer the ownership as mutable and get back the vec3 after the operation is complete
    println!("vec3 with only even values: {:?}", vec3); // the vec3 is changed in place (memory optimised operation)
    
    //vectors using macros
    let vec5 = vec![1, 2, 3, 4, 5]; //just another way to initialize the vector
    println!("vec5: {:?}", vec5);

    
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    //required to specify a generic argument of<i32> to vec
    let mut new_vec = Vec::new();

    for i in vec {
        if i % 2 == 0 {
            new_vec.push(i);
        }
    }
    return new_vec;
}

fn even_filter_by_ref(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for i in vec {
        if i % 2 == 0 {
            new_vec.push(*i); // here we pass *i to dereference the pointed values
        }
    }
    return new_vec;
}

fn even_filter_inplace(vec: &mut Vec<i32>) {
    vec.retain(|x| x % 2 == 0);
}
