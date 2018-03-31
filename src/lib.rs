// use std; // No compile issue at all if this line is not commented out.

/// This function will throw a cause the compiler to panic under certain
/// if the standard library has not been imported into the module. This
/// function will not cause the same panic if it is moved to main.rs.
pub fn collect_args() -> Vec<String> {
    std::env::args()
        .skip(1)   // Compiler panics if this line is not commented out.
        .collect()
}

