/// This function will throw a cause the compiler to panic under certain
/// if the standard library has not been imported into the module. This
/// function will not cause the same panic if it is moved to main.rs.
pub fn foo() -> Vec<String> {
    std::env::args()
        .skip(1)   // Compiler does not panic if this is commented out.
        .collect()
}


// /// This is the same function without the missing `::` prefix. This will
// /// compile and run correctly.
// pub fn collect_args() -> Vec<String> {
//     ::std::env::args()
//         .skip(1)   // Compiler panics if this line is not commented out.
//         .collect()
// }

