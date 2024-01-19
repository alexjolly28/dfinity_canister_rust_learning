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
fn condtion(a:i32)->String{

    if a < 5 {
        format!("condition was true")
    } else {
        format!("condition was false")
    }
}


// Generate did files

export_candid!();
