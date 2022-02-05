mod myexamplemodule;
use crate::myexamplemodule::myexamplemodule::my_example;
///
/// Basic function.  Note the exclamation point after println.  This indicates a macro call
///
///I can create doc tests right from these comments.
/// # Examples
///
/// ```
/// main(); //this will run when I run `cargo doc`.  `cargo doc` generates documentation AND runs doc tests.  `cargo doc --open`
///
/// ```
///
fn main() {
    //main returns
    println!("Hello, world!");
    let result = my_example(2.0);
    println!("{}", result);

    //can't do this! variables are immutable by default
    //result = 5.0;

    let result = 4.0; // I can do this, because "let" is a re-assignment.

    let mut result1 = 3.0;

    result1 = 4.0; //I can do this because its mutable
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
