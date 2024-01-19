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



// Generate did files

export_candid!();
