// https://doc.rust-lang.org/rust-by-example/std/option.html

// An integer division that doesn't `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    #[allow(unused_variables)]
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    // println!("{:?} unwraps to {:?}", none, none.unwrap()); // panic! - cannot call None.unwrap()


    let quotient = checked_division(5,2);
    if quotient.is_some() {  // same as quotient != None
        println!("quotient ={:?}", quotient);  // quotient =Some(2)
        println!("quotient.unwrap() ={:?}", quotient.unwrap());  // quotient.unwrap =2
    }
}
