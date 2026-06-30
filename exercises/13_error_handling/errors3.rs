// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// The main problem is that we must return a Result in main,
// and also provide an error that implements `std::error::Error`.
// Use `Box<dyn std::error::Error>` for the error case.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
        return Err("You can't afford that many!".into());
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
    Ok(())
}