use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
///example module
///
pub mod myexamplemodule {
    pub fn my_example(x: f64) -> f64 {
        //note this is integer power,
        //more efficient than powf
        //(which takes a float)
        x.powi(4)
    }
}

///data structure examples
///
pub struct MyDataExample {
    some_num: f64,
    some_int: i32,
}
/// add functions to the struct (as close to a "class" as you will get in rust)
impl MyDataExample {
    //`init` isn't special, could be named anything.  Idiomatic to name it `init` though
    pub fn init(some_num: f64, some_int: i32) -> Self {
        MyDataExample { some_num, some_int }
    }
}

/// SERDE example
#[derive(Serialize, Deserialize)]
pub struct SerdeExample {
    some_num: f64,
    some_vec: Vec<f64>,
}

//example of serde and implementing trait
impl fmt::Display for SerdeExample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, [{}])",
            self.some_num,
            self.some_vec //example vector operations + lazy iterators
                .iter()
                .map(|v| v.to_string()) //note that functions can also return iterators, so you can chain iterators without intermediary execution
                .fold(String::from(""), |accum, v| format!("{}, {}", accum, v))
        )
    }
}

///example par_iter
///
fn example_par_iter(some_vec: &[&str]) {
    some_vec.par_iter().for_each(|v| println!("{}", v))
}

///example lifetimes
///Note that if returning iterators or functions special care needs to be taken care for lifetimes
fn example_lifetime<'a, 'b: 'a>(
    //explicitly state that 'b will last longer than 'a
    some_vector_that_will_be_around_for_longer_than_input_function: &'b [f64],
    some_function_that_takes_the_vector_as_an_argument: &'a impl Fn(&[f64]),
    //note, impl Fn is required because the function can be a closure.
    //If you know that this will not be a closure, you can use `fn` instead of `impl Fn`,
    //which will have mild performance boosts
) -> impl Fn() + 'a {
    return || {
        some_function_that_takes_the_vector_as_an_argument(
            some_vector_that_will_be_around_for_longer_than_input_function,
        )
    };
}

/// SERDE example with str
#[derive(Serialize, Deserialize)]
pub struct SerdeExampleStr<'a> {
    some_str: &'a str,
}

///example borrow vs reference
fn example_borrow(v: Vec<i32>) {
    v.iter().for_each(|v| println!("{}", v));
}
///example  borrow vs reference
fn example_reference(v: &[i32]) {
    v.iter().for_each(|v| println!("{}", v));
}

///`cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_computes_power() {
        assert_eq!(myexamplemodule::my_example(2.0), 16.0);
    }
    #[test]
    fn it_initiates_struct() {
        let data_struct = MyDataExample::init(1.2, 3);
        assert_eq!(data_struct.some_num, 1.2);
    }

    #[test]
    fn it_deserializes() {
        let serialized = "{\"some_num\": 32.0, \"some_vec\":[1.0, 2.0]}";
        let deserialized: SerdeExample = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.some_vec[0], 1.0);
    }
    #[test]
    fn it_does_not_deserialize() {
        let serialized = "{\"some_num\": 32.0, \"some_vec\":4.0}";
        let maybe_deserialized: Result<SerdeExample, serde_json::Error> =
            serde_json::from_str(&serialized);
        match maybe_deserialized {
            Ok(deserialized) => println!("{}", deserialized),
            Err(err) => println!("{}", err),
        }
    }

    #[test]
    fn it_executes_in_parallel() {
        example_par_iter(&vec!["hello", "world", "goodbye", "cruel", "earth"]);
    }
    #[test]
    fn it_does_lifetime() {
        let f = |myvec: &[f64]| {
            myvec.iter().for_each(|v| println!("{}", v));
        };
        let v = vec![3.0, 5.0];
        let my_closure = example_lifetime(&v, &f);
        my_closure(); //both f and v are still around here

        // can't do this!  v doesn't live long enough!  compiler will be mad!
        //let my_closure_two = example_lifetime(&vec![3.0, 3.0], &f);
    }
    #[test]
    fn it_deserializes_str() {
        let serialized = "{\"some_str\": \"hello\"}";
        let deserialized: SerdeExampleStr = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.some_str, "hello");
    }
    #[test]
    fn it_serializes_str() {
        let deserialized: SerdeExampleStr = SerdeExampleStr { some_str: "hello" };
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, "{\"some_str\": \"hello\"}");
    }
    #[test]
    fn it_does_borrow_and_reference() {
        let x = vec![2, 3, 4];
        example_borrow(x); //note that in rust you have to EXPLICITLY copy non-primitives...unlike C/C++ this is a "move" not a copy

        //uh oh, can't do this!  x was "moved" into example_reference_vs_borrow
        //x.iter().for_each(|v| println!("{}", v));

        let y = vec![2, 3, 4];
        example_reference(&y); //note that this is an immutable reference.

        //this is fine, because example_reference didn't take ownership of y, just the reference
        y.iter().for_each(|v| println!("{}", v));

        let mut z = vec![2, 3, 4];
        example_reference(&mut z); //note that this is a mutable reference, and z can be updated within the function
        z[0] = 4; //I can also update it here
    }
}
