mod lib;
use lib::collect_args as collect_args_in_mod;

/// This function will not cause the compiler to panic.
pub fn collect_args() -> Vec<String> {
    let args = std::env::args()
        .skip(1)
        .collect();
    args
}

fn main() {
    // Collect the arguments using the implementation in this file.
    let args: Vec<String> = collect_args();
    // Collect the arguments using the implementation in the helper module.
    let other_args: Vec<String> = collect_args_in_mod();
}

