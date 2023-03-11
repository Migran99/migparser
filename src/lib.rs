use std::{env};
use migformatting::Formatting;

mod argument;
pub use argument::{DataType, ExtractFromContents, ArgumentOptions, Argument};
use argument::{Content, ArgumentType};
/* TODO

    - Positionals
    - Shortcuts --car -> -c

 */

pub struct ArgumentParser {
    arguments: Vec<Argument>,
    positional_cursor: i32,
}

impl ArgumentParser {

    // Instantiation
    pub fn new() -> ArgumentParser {
        ArgumentParser {arguments: vec![], positional_cursor: 0}
    }

    // Arguments functions
    pub fn add_argument(&mut self, name: &str, data_type: DataType ,options_: Option<Vec<&str>>, default_value: Option<Content>) {
        /* Set-up*/
        let mut data: Option<Content> = default_value;
        let mut options = match options_ {
            Some(c) => c.iter().map(|&f| String::from(f)).collect(),
            None => vec![]
        };

        /* Bool */
        if data_type == DataType::Bool
        {
            if options.contains(&ArgumentOptions::STORE_FALSE.to_owned()) {
                data = Some(Content::Bool(true));
            }
            else { /* STORE_TRUE by default */
                data = Some(Content::Bool(false));

                if !options.contains(&ArgumentOptions::STORE_FALSE.to_owned()) {
                    options.push(ArgumentOptions::STORE_TRUE.to_owned());
                }
            }
        }

        /* Positional - Optional - Flags */
        let argument_type = Argument::get_type(name);
        let mut index: i32 = -1;
        match argument_type {
            Some(t) => {
                match t {
                    ArgumentType::Optional => {
                        // Do nothing
                    },
                    ArgumentType::Positional => {
                        // Add the necesary option if not already
                        if !options.contains(&ArgumentOptions::NECESSARY.to_owned()) {
                                options.push(ArgumentOptions::NECESSARY.to_owned()); 
                        }
                        index = self.positional_cursor;
                    },
                    ArgumentType::Flag => {

                    }
                }

            },
            None => {},
        }

        let arg_name = match Argument::parse_name(name) {
            Some(n) => { n },
            None => { name.into() },
        };
        let cl_name = name.to_owned(); // keeping --arg if present
        
        self.arguments.push(Argument::new_optional(
                            arg_name.as_str(), 
                            vec![cl_name], 
                            data_type, 
                            Some(options)));
    }

    fn parse_text<T: std::str::FromStr>(text: &String) -> Option<T>{
        let parsed = text.parse::<T>().ok(); 
        match parsed {
            Some(c) => Some(c),
            None => None
        }
    }

    fn parse_value(text: &String, type_ : &DataType) -> Option<Content> {
        let res: Option<Content> = match type_ {
            DataType::Int => {
                let parsed = ArgumentParser::parse_text::<i32>(text);
                if let Some(c) = parsed{
                    return Some(Content::Int(c));
                }
                else {
                    return None;
                }
            },
            DataType::Uint => {
                let parsed = ArgumentParser::parse_text::<u32>(text);
                if let Some(c) = parsed{
                    return Some(Content::Uint(c));
                }
                else {
                    return None;
                }
            },
            DataType::Bool => {
                let parsed = ArgumentParser::parse_text::<bool>(text);
                if let Some(c) = parsed{
                    return Some(Content::Bool(c));
                }
                else {
                    return None;
                }
            },
            DataType::String => Some(Content::String(text.clone())),
            DataType::Float => {
                let parsed = ArgumentParser::parse_text::<f32>(text);
                if let Some(c) = parsed{
                    return Some(Content::Float(c));
                }
                else {
                    return None;
                }
            },
        };

        res
    }

    pub fn parse_arguments(&mut self) {
        let arguments: Vec<String> = env::args().collect();
        let mut used_arguments: Vec<bool> = vec![false; arguments.len()];

        for opt in self.arguments.iter_mut() {
            for (i,arg) in arguments.iter().enumerate() {
                if opt.has_identifier(arg) && !used_arguments[i] && !opt.is_parsed() {
                    // Get value
                    used_arguments[i] = true;

                    /* TODO: match per data_type */
                    if opt.has_option(ArgumentOptions::STORE_TRUE) && opt.data_type == DataType::Bool{
                        opt.set_data(Content::Bool(true));
                    }
                    else if opt.has_option(ArgumentOptions::STORE_FALSE) && opt.data_type == DataType::Bool {
                        opt.set_data(Content::Bool(false));
                    }
                    else {
                        let data = ArgumentParser::parse_value(&arguments[i+1], &opt.data_type);
                        if let Some(d) = data {
                            opt.set_data(d);
                        }
                        used_arguments[i+1] = true;
                    }

                    opt.set_parsed();
                }
            }
            if !opt.is_parsed() && opt.has_option(ArgumentOptions::NECESSARY) {
                println!("{}", format!("Necessary argument '{}' is not present", opt.name).error());
                panic!();
            }
        }
    }

    pub fn get_value<T: ExtractFromContents>(&self, arg: &str) -> Option<T>{
        let mut ret: Option<Content> = None;
        for a in self.arguments.iter() {
            if &a.name == arg {
                ret = a.get_data();
            }
        }

        match ret {
            Some(c) => c.get_value(),
            None => None
        }
    }



    // Display functions
    pub fn print_data(&self) {
        println!("##### Arguments");
        for d in self.arguments.iter() {
            let data = if let Some(c) = d.get_data() {
                c.get_value_str()
            }
            else {
                "None".to_string()
            };
            println!("{:?} [{:?}] : {:?}", d.name, d.data_type, data);
        }
        println!("------");
    }
}