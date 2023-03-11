# migparser
This crates implements a very simple argument parser inspired by the Python one. It allows adding arguments of different types (int, uint, bool, string, float) and customize the behaviour with different options (necessary, store-true, store-false, ...).

## Example
```rust
use migparser::ArgumentParser;
use migparser::ContentsTypes;
use migparser::ArgumentOptions;

fn main () {
    let mut parser = ArgumentParser::new();

    // The "--" prefix makes it optional.
    parser.add_argument("--number", ContentsTypes::Int, None, None);

    // No prefix means necessary by default.
    parser.add_argument("name", ContentsTypes::String, None, None);

    // Options used to make it necessary and have the store-true 
    // behaviour.
    parser.add_argument("--flag", ContentsTypes::Bool, 
            Some(vec![ArgumentOptions::STORE_TRUE, ArgumentOptions::NECESSARY]), None);
    parser.print_data();    

    parser.parse_arguments();
    parser.print_data();

    let name : String = parser.get_value("name").unwrap();
    println!("The name is {name}");

    match parser.get_value::<i32>("number") {
        Some(i) => {println!("The number is {i}");},
        None => {},
    }
    
}
```


Run it
```bash
cargo run name miguel --number 8 --flag
```
