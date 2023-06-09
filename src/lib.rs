use migformatting::Formatting;
use std::{env};

mod argument;
use argument::Content;
pub use argument::{Argument, ArgumentOption, ArgumentType, DataType, ExtractFromContents, ListType, ContentList};

#[derive(Clone)]
pub struct ArgumentParser {
    arguments: Vec<Argument>,
    positional_cursor: i32,
}

impl ArgumentParser {
    /* ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    /* Creation +++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    pub fn new() -> ArgumentParser {
        ArgumentParser {
            arguments: vec![],
            positional_cursor: 1,
        }
    }

    /* ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    /* Arguments ++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    fn add_flag(
        &mut self,
        name: String,
        identifiers: Vec<String>,
        options: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>,
    ) -> Result<(), String> {
        self.arguments
            .push(Argument::new_flag(&name, identifiers, options, default_val));
        Ok(())
    }

    fn add_positional(
        &mut self,
        name: String,
        identifiers: Vec<String>,
        data_type: DataType,
        options: Option<Vec<ArgumentOption>>,
        index: i32,
    ) -> Result<(), String> {
        self.arguments.push(Argument::new_positional(
            &name,
            identifiers,
            data_type,
            options,
            index,
        ));
        Ok(())
    }

    fn add_optional(
        &mut self,
        name: String,
        identifiers: Vec<String>,
        data_type: DataType,
        options: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>,
        n_args_: usize
    ) -> Result<(), String> {
        self.arguments.push(Argument::new_optional(
            &name,
            identifiers,
            data_type,
            options,
            default_val,
            n_args_
        ));
        Ok(())
    }

    /* ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    /* Parsing aux. +++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    fn parse_text<T: std::str::FromStr>(text: &String) -> Option<T> {
        let parsed = text.parse::<T>().ok();
        match parsed {
            Some(c) => Some(c),
            None => None,
        }
    }

    fn parse_value(text: &String, type_: &DataType) -> Option<Content> {
        let res: Option<Content> = match type_ {
            DataType::Int => {
                let parsed = ArgumentParser::parse_text::<i32>(text);
                if let Some(c) = parsed {
                    return Some(Content::Int(c));
                } else {
                    return None;
                }
            }
            DataType::Uint => {
                let parsed = ArgumentParser::parse_text::<u32>(text);
                if let Some(c) = parsed {
                    return Some(Content::Uint(c));
                } else {
                    return None;
                }
            }
            DataType::Bool => {
                let parsed = ArgumentParser::parse_text::<bool>(text);
                if let Some(c) = parsed {
                    return Some(Content::Bool(c));
                } else {
                    return None;
                }
            }
            DataType::String => Some(Content::String(text.clone())),
            DataType::Float => {
                let parsed = ArgumentParser::parse_text::<f32>(text);
                if let Some(c) = parsed {
                    return Some(Content::Float(c));
                } else {
                    return None;
                }
            },
            DataType::List(t) => {
                let values_txt = text.trim().split(' ').collect::<Vec<&str>>();
                let mut result = ContentList::new(t.clone());
                match t {
                    ListType::Bool => {
                        for i in values_txt {
                            let v = ArgumentParser::parse_text::<bool>(&i.to_owned()).unwrap();
                            result.data.push(Content::Bool(v));
                        }
                    },
                    ListType::Int => {
                        for i in values_txt {
                            let v = ArgumentParser::parse_text::<i32>(&i.to_owned()).unwrap();
                            result.data.push(Content::Int(v));
                        }
                    },
                    ListType::Uint => {
                        for i in values_txt {
                            let v = ArgumentParser::parse_text::<u32>(&i.to_owned()).unwrap();
                            result.data.push(Content::Uint(v));
                        }
                    },
                    ListType::String => {
                        for i in values_txt {
                            let v = ArgumentParser::parse_text::<String>(&i.to_owned()).unwrap();
                            result.data.push(Content::String(v));
                        }
                    },
                    ListType::Float => {
                        for i in values_txt {
                            let v = ArgumentParser::parse_text::<f32>(&i.to_owned()).unwrap();
                            result.data.push(Content::Float(v));
                        }
                    },
                }
                Some(Content::List(result))
            }
        };

        res
    }

    fn parse_arg(
        &mut self,
        cl_arguments: &Vec<String>,
        used_cl_args: &mut Vec<bool>,
        argument_ix: usize,
    ) {
        /* Loop on cl arguments */
        let mut argument = self.arguments[argument_ix].clone();
        let arg_name = argument.name.clone();
        let arg_type = argument.get_type();
        let data_type = argument.data_type.clone();
        let cl_n_args: usize = cl_arguments.len();
        for (i, arg) in cl_arguments.iter().enumerate() {
            if i != 0 {
                match arg_type {
                    ArgumentType::Flag => {
                        if argument.has_identifier(arg) && !used_cl_args[i]
                        {   
                            if argument.has_option(ArgumentOption::StoreTrue) {
                                argument.set_data(Content::Bool(true));
                            } else {
                                argument.set_data(Content::Bool(false));
                            }
                            argument.set_parsed();
                        }
                    }
                    ArgumentType::Positional => {
                        if i32::try_from(i).unwrap() == argument.get_index() && !used_cl_args[i] {
                            let data = ArgumentParser::parse_value(
                                &cl_arguments[i],
                                &data_type,
                            );
                            if let Some(d) = data {
                                argument.set_data(d);
                            }
                            argument.set_parsed();
                        } else if i32::try_from(i).unwrap() > argument.get_index() {
                            break;
                        }
                    }
                    ArgumentType::Optional => {
                        if argument.has_identifier(arg) && !used_cl_args[i]
                        {
                            let n_args = argument.n_args.clone();
                            if i + n_args > cl_n_args {
                                println!("Not enough arguments for list ({n_args},{cl_n_args})! ");
                                panic!();
                            }
                            let data_args = cl_arguments[i + 1 .. i + argument.n_args + 1].to_vec();
                            let mut data_txt: String = String::new();
                            
                            for i in data_args {
                                data_txt.push_str(&i);
                                data_txt.push(' ');
                            }
                            
                            let data =
                                ArgumentParser::parse_value(&data_txt, &data_type);
                            if let Some(d) = data {
                                argument.set_data(d);
                            }
                            argument.set_parsed();
                            for j in 0..n_args {
                                used_cl_args[i + j] = true;
                            }
                            
                        }
                    }
                }
                if argument.is_parsed(){
                    used_cl_args[i] = true;
                    self.arguments[argument_ix] = argument.clone();
                    break;
                }
            }
        }

        if !argument.is_parsed() && argument.has_option(ArgumentOption::Necessary) {
            println!(
                "{}",
                format!("Necessary argument '{}' is not present", arg_name).error()
            );
            panic!();
        }
    }

    /* ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    /* User API +++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
     pub fn add_argument(
        &mut self,
        name: &str,
        alias: Option<Vec<String>>,
        data_type: DataType,
        options_: Option<Vec<ArgumentOption>>,
        default_value: Option<Content>,
    ) -> Result<(), String> {
        /* Set-up*/
        let mut data: Option<Content> = default_value.clone();
        let mut options = options_.unwrap_or_default();

        /* Bool */
        if data_type == DataType::Bool {
            if options.contains(&ArgumentOption::StoreFalse) {
                data = Some(Content::Bool(true));
            } else {
                /* StoreTrue by default */
                data = Some(Content::Bool(false));

                if !options.contains(&ArgumentOption::StoreFalse) {
                    options.push(ArgumentOption::StoreTrue);
                }
            }
        }

        let arg_name = match Argument::parse_name(name) {
            Some(n) => n,
            None => name.into(),
        };
        let cl_name = name.to_owned(); // keeping --arg if present
        let mut identifiers = vec![cl_name.clone()];
        if let Some(mut i) = alias {
            identifiers.append(&mut i);
        }
        let n_args = Argument::get_n_args(&options);

        /* Positional - Optional - Flags */
        let argument_type = Argument::guess_type(name, &options, &data_type);
        match argument_type {
            Some(t) => {
                match t {
                    ArgumentType::Optional => {
                        self.add_optional(arg_name, identifiers, data_type, Some(options), data, n_args)?;
                    }
                    ArgumentType::Positional => {
                        // Add the necesary option if not already
                        if !options.contains(&ArgumentOption::Necessary) {
                            options.push(ArgumentOption::Necessary);
                        }
                        let index = self.positional_cursor;
                        self.positional_cursor = self.positional_cursor + 1;
                        self.add_positional(
                            arg_name,
                            identifiers,
                            data_type,
                            Some(options),
                            index,
                        )?;
                    }
                    ArgumentType::Flag => {
                        self.add_flag(arg_name, identifiers, Some(options), data)?;
                    }
                }
            }
            None => {
                return Err("Invalid name for arg!".to_owned());
            }
        }
        Ok(())
    }

    pub fn parse_arguments(&mut self) {
        let arguments: Vec<String> = env::args().collect();
        println!("{}", format!("Arguments: \n {arguments:?}"));
        let mut used_arguments: Vec<bool> = vec![false; arguments.len()];
        for arg_ix in 0..self.arguments.len() { 
            self.parse_arg(&arguments, &mut used_arguments, arg_ix);
        }
    }

    pub fn parse_arguments_from_text(&mut self, text: String) {
        let mut arguments: Vec<String> = text.split(" ").map(|f| f.to_owned()).collect();
        arguments.insert(0, "program_name".to_owned());
        let mut used_arguments: Vec<bool> = vec![false; arguments.len()];
        for arg_ix in 0..self.arguments.len() { 
            self.parse_arg(&arguments, &mut used_arguments, arg_ix);
        }
    }

    pub fn get_value<T: ExtractFromContents>(&self, arg: &str) -> Option<T> {
        let mut ret: Option<Content> = None;
        for a in self.arguments.iter() {
            if &a.name == arg {
                ret = a.get_data();
            }
        }

        match ret {
            Some(c) => c.get_value(),
            None => None,
        }
    }

    /* ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    /* API Aux. +++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
    pub fn print_data(&self) {
        println!("##### Arguments");
        for d in self.arguments.iter() {
            let data = if let Some(c) = d.get_data() {
                c.get_value_str()
            } else {
                "None".to_string()
            };
            println!("{:?} [{:?}] : {:?}", d.name, d.data_type, data);
        }
        println!("------");
    }
}
