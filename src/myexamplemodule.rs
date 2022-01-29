use serde::{Deserialize, Serialize};
use std::fmt;
///example module
///
pub mod myexamplemodule {
    pub fn my_example(x: f64) -> f64 {
        x.powi(4)
    }
}

///data structure examples
///
pub struct MyDataExample {
    some_num: f64,
    some_int: i32,
}

impl MyDataExample {
    pub fn init(some_num: f64, some_int: i32) -> Self {
        MyDataExample { some_num, some_int }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdeExample {
    some_num: f64,
    some_vec: Vec<f64>,
}

impl fmt::Display for SerdeExample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, [{}])",
            self.some_num,
            self.some_vec
                .iter()
                .map(|v| v.to_string())
                .fold(String::from(""), |accum, v| concat!(accum, ", ", v))
        )
    }
}

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
        //assert_eq!(deserialized.some_vec[0], 1.0);
    }
}
