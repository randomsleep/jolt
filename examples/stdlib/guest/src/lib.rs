#![no_main]

#[jolt::provable(std)]
fn int_to_string(n: i32) -> String {
    n.to_string()
}
