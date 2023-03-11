use std::{env};
use migformatting::Formatting;

mod argument;
pub use argument::{DataType, ExtractFromContents, ArgumentOption, Argument, ArgumentType};
use argument::{Content};
/* TODO

    - Positionals
    - Shortcuts --car -> -c

 */

pub struct ArgumentParser {
    arguments: Vec<Argument>,
    positional_cursor: i32,
}

impl ArgumentParser {

    /* Creation */
    pub fn new() -> ArgumentParser {
        ArgumentParser {arguments: vec![], positional_cursor: 0}
    }

    /* Argument type handler */
    fn add_flag(&mut self, 
        name: String, 
        identifiers: Vec<String>, 
        options: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>) -> Result<(), String> {

        self.arguments.push(Argument::new_flag(&name, identifiers, options, default_val));
        Ok(())
    }

    fn add_positional(&mut self, 
        name: String, 
        identifiers: Vec<String>,
        data_type: DataType, 
        options: Option<Vec<ArgumentOption>>,
        index: i32) -> Result<(), String> {
        
        self.arguments.push(Argument::new_positional(&name, identifiers, data_type, options, index));
        Ok(())
    }

    fn add_optional(&mut self, 
        name: String, 
        identifiers: Vec<String>,
        data_type: DataType, 
        options: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>) -> Result<(), String> {
        
        self.arguments.push(Argument::new_optional(&name, identifiers, data_type, options, default_val));
        Ok(())
    }

    /* Argument API */
    pub fn add_argument(&mut self, name: &str, alias: Option<Vec<String>>, data_type: DataType ,options_: Option<Vec<ArgumentOption>>, default_value: Option<Content>) -> Result<(), String> {
        /* Set-up*/
        let mut data: Option<Content> = default_value.clone();
        let mut options = options_.unwrap_or_default();

        /* Bool */
        if data_type == DataType::Bool
        {
            if options.contains(&ArgumentOption::StoreFalse) {
                data = Some(Content::Bool(true));
            }
            else { /* StoreTrue by default */
                data = Some(Content::Bool(false));

                if !options.contains(&ArgumentOption::StoreFalse) {
                    options.push(ArgumentOption::StoreTrue);
                }
            }
        }

        let arg_name = match Argument::parse_name(name) {
            Some(n) => { n },
            None => { name.into() },
        };
        let cl_name = name.to_owned(); // keeping --arg if present
        let mut identifiers = vec![cl_name.clone()];
        if let Some(mut i) =  alias{
            identifiers.append(&mut i);
        }

        /* Positional - Optional - Flags */
        let argument_type = Argument::get_type(name, &options, &data_type);
        match argument_type {
            Some(t) => {
                match t {
                    ArgumentType::Optional => {
                        self.add_optional(arg_name, identifiers, data_type, Some(options), data)?;
                    },
                    ArgumentType::Positional => {
                        // Add the necesary option if not already
                        if !options.contains(&ArgumentOption::Necessary) {
                                options.push(ArgumentOption::Necessary); 
                        }
                        let index = self.positional_cursor;
                        self.positional_cursor = self.positional_cursor + 1;
                        self.add_positional(arg_name, identifiers, data_type, Some(options), index)?;
                    },
                    ArgumentType::Flag => {
                        self.add_flag(arg_name, identifiers, Some(options), data)?;
                    }
                }

            },
            None => {return Err("Invalid name for arg!".to_owned());},
        }
        Ok(())
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
                    if opt.has_option(ArgumentOption::StoreTrue) && opt.data_type == DataType::Bool{
                        opt.set_data(Content::Bool(true));
                    }
                    else if opt.has_option(ArgumentOption::StoreFalse) && opt.data_type == DataType::Bool {
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
            if !opt.is_parsed() && opt.has_option(ArgumentOption::Necessary) {
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