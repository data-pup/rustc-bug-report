mod lib;
use lib::collect_args as collect_args_in_mod;

fn main() {
    // Collect the arguments using the implementation in the helper module.
    let _ = collect_args_in_mod();
}

