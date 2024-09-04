#![deny(clippy::all)]

const MY_AGE:u8 = 21;


// wouldn't want to do just String because that tells it we want to drop the variable right away
// which we usually don't want to do, so use the ampersand
fn greet(name: &String) {
    println!("Hello, {}", name);
}

// This won't work because we are saying we want to borrow the variable, and then are acting like  
// we own it
// fn empty_string(name: &String) {
//     value.clear();
// }

// This will work because we said it is mutable. we now have a mutable reference
fn empty_string(value: &mut String) {
    value.clear();
}

// This complains because we are trying to return something we create in the function, so rust is 
// confused at how it will live past the lifetime of the function 
// fn get_name() -> &String {
//     &"Sam".to_string()
// }

// Don't have to have a return statement. last statement without a semicolon is returned
fn say_hello_world() -> String {
    String::from("Hello, world!")
}

// This will yell at you because you don't need the return statement
// fn say_hello_world2() -> String {
//     return String::from("Hello, world!");
// }

// don't have to specify when it isn't returning something
fn print_hello_world() {
    let message = String::from("Hello, world!");
    println!("{}", message);
}

fn say_hello_to(to_person: &String) -> String {
    format!("Hello, {}!", to_person)
}

// function that takes a function pointer as a parameter
fn process_name(num: i32, callback: fn(i32) -> i32) {
    println!("{}", callback(num));
}

fn main() {
    let mut name = "Jane";
    println!("Hello {}", name);
    name = "Sam";
    println!("Hello {}", name);

    let age: u8 = MY_AGE;
    let big_number = 60_000_000;
    println!("{} is {} years old!", name, age);
    println!("{} is less than {} years old", name, big_number);

    let _red = 0xFA;
    let _rgb = 0xFF0000;

    let distance1 = 5.5f32;
    let distance2 = 6.2f32;
    let distance3 = 10.2;

    let total = distance1 + distance2 + distance3;

    println!("total distance is {}", total);

    let _ex_data = "foo";
    let _ex_data = 10;

    let data = "foo";
    {
        let data = data.to_string();
        println!("data: {}", data);
    }

    let personal_tuple: (i32, &str) = (21, "Sam");
    let age = personal_tuple.0;
    let name = personal_tuple.1;
    println!("{} is {} years old", name, age);

    // let name1 = "Sam".to_string();
    // let name2 = name1;
    // println!("Hello, {}", name1);
    // println!("Hello, {}", name2);

    // This block does not work. strings exist in stack AND heap
    // ptr, len, and capacity are in stack, ptr points to data in the heap. 
    // when program is over, name1 gets deallocated, and then it tries to deallocate name2, which
    // has already been deallocated. that's the error. double deallocation. 
    // deallocation decisions happen at compiletime

    let age1 = 21;
    let age2 = age1;
    println!("You are {} years old", age1);
    println!("You are {} years old", age2);
    // but this block does work. age1 and age2 are both in the stack and are copied by default
    // not like moving or borrowing like with the strings. 

    let name1 = String::from("Sam");
    let name2 = &name1;
    println!("Hello, {}", name1);
    println!("Hello, {}", name2);
    // this block does work though because it is now a reference and behaves differently
    // name2 will point to the pointer in name1 which points to the data in the heap

    let name1 = String::from("Sam");
    let name2 = &name1;
    //greet(name1); wouldn't work because different types
    greet(&name1);
    greet(name2);

    let mut another_name = String::from("George");  // needs the mut here for type matching
    empty_string(&mut another_name);

    // let mut no_mult_ref = String::from("test");
    // let mut no_mult_ref2 = &mut no_mult_ref;
    // let mut no_mult_ref3 = &mut no_mult_ref;
    // empty_string(&mut no_mult_ref2);
    // this doesn't work because you can't have more than one mutable reference to the same variable

    // let mut mut_var = String::from("test");
    // let no_mut_var = & mut_var;
    // let mut mut_var3 = &mut mut_var;
    // println!("{}", mut_var)
    // println!("{}", no_mut_var)
    // println!("{}", mut_var3)
    // This doesn't work because you can't have a mutable reference and an immutable reference of 
    // the same variable

    let greeting = say_hello_world();
    println!("{}", greeting);

    print_hello_world();

    let person = String::from("Sam");
    println!("{}", say_hello_to(&person));

    // in line function
    let say_hello_to_inline = |name: &str| format!("Hello, {}!", name);
    println!("{}", say_hello_to_inline(&person.to_string()));

    let first_name = "Sam".to_string();
    let last_name = "Haldeman".to_string();
    let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
    println!("{}", full_name(&first_name, &last_name));

    let multiply_by_2 = |x: i32| x*2;
    println!("8 doubled: {}", multiply_by_2(8));

    // multiple line function
    let ask_for_age = || {
        // ask user for age
        // calculate how old in 10 years
        10
    };
    println!("{}", ask_for_age());

    // can have pointers to functions as well, easier than in C
    let ptr = multiply_by_2;
    let result = ptr(10);
    println!("{}", result);
    process_name(17, ptr);
}
