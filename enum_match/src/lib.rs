use ic_cdk_macros::{query, export_candid};
use ic_cdk::export::candid::{CandidType, Deserialize};


#[derive(CandidType, Deserialize)]
enum MathOperation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
}


#[query]
fn perform_math_operation(operation:MathOperation) -> i32 {

    match operation {
        MathOperation::Add(a, b) => a + b,
        MathOperation::Subtract(a, b) => a - b,
        MathOperation::Multiply(a, b) => a * b,
    }
}


// Generate did files

export_candid!();
