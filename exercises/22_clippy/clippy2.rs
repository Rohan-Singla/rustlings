fn main() {
    let mut res = 42;
    let option = Some(12);
    // Fixed the Clippy lint by not using a `while let` loop with a non-changing option.

    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
