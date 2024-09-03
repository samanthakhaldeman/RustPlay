#![deny(clippy::all)]

const MY_AGE:u8 = 21;

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


}
