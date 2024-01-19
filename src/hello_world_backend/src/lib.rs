use ic_cdk_macros::{query, export_candid};
use std::cmp::Ordering;


#[query]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}


#[query]
fn matching(a: i32,b:i32) -> String {

    match a.cmp(&b) {
        Ordering::Less => format!("{} is big",b),
        Ordering::Greater => format!("{} Too big!",a),
        Ordering::Equal => format!("You win!"),

    }

}

#[query]
fn condtion(number:i32)->String{

    if number % 4 == 0 {
        format!("number is divisible by 4")
    } else if number % 3 == 0 {
        format!("number is divisible by 3")
    } else if number % 2 == 0 {
        format!("number is divisible by 2")
    } else {
        format!("number is not divisible by 4, 3, or 2")
    }
}


// Generate did files

export_candid!();
