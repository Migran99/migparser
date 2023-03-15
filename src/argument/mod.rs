mod contents;
pub use contents::Content;
pub use contents::{DataType, ExtractFromContents, ListType, ContentList};
/*
TODO:
   - ArgumentOptions -> to enum
   - cl_name -> to cl_indetifiers ?
   - new() : and different creators for different argument types
   - encapsulation of parsed, index, data (protected components)

*/
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum ArgumentType {
    Positional,
    Optional,
    Flag,
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum ArgumentOption {
    StoreTrue,
    StoreFalse,
    Necessary,
    NArgs(usize),
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub cl_identifiers: Vec<String>,
    pub data_type: DataType,
    data: Option<Content>,
    pub options: Vec<ArgumentOption>,
    parsed: bool,
    index: i32,
    arg_type: ArgumentType,
    pub n_args: usize
}
impl Argument {
    /* Creators
        - new (private) 
        - new_optional
        - new_positional
        - new_flag

    */
    fn new(
        name_: &str,
        cl_identifiers_: Vec<String>,
        data_type_: DataType,
        options_: Option<Vec<ArgumentOption>>,
        index_: i32,
        default_val: Option<Content>,
        arg_type_: ArgumentType,
        n_args_ : usize
    ) -> Self {
        Argument {
            name: name_.to_owned(),
            cl_identifiers: cl_identifiers_,
            data_type: data_type_,
            data: default_val,
            options: options_.unwrap_or_default(),
            parsed: false,
            index: index_, /* Only settable at instantiation new_positional */
            arg_type: arg_type_,
            n_args: n_args_
        }
    }
    pub fn new_optional(
        name_: &str,
        cl_identifiers_: Vec<String>,
        data_type_: DataType,
        options_: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>,
        n_args_ : usize
    ) -> Self {
        Argument::new(name_, cl_identifiers_, data_type_, options_, -1, default_val, ArgumentType::Optional, n_args_)
    }

    pub fn new_positional(
        name_: &str,
        cl_identifiers_: Vec<String>,
        data_type_: DataType,
        options_: Option<Vec<ArgumentOption>>,
        index_: i32,
    ) -> Self {
        Argument::new(name_, cl_identifiers_, data_type_, options_, index_, None, ArgumentType::Positional, 1)
    }

    pub fn new_flag(
        name_: &str,
        cl_identifiers_: Vec<String>,
        options_: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>
    ) -> Self {
        Argument::new(name_, cl_identifiers_, DataType::Bool, options_, -1, default_val, ArgumentType::Flag, 1)
    }

    /* AUX */
    pub fn get_data(&self) -> Option<Content> {
        self.data.clone()
    }
    pub fn set_data(&mut self, data: Content) -> bool {
        if self.is_parsed() {
            return false;
        }
        // if self.data_type != data.get_type() {
        //     return false;
        // }
        println!("{:?}", data);
        self.data = Some(data);
        return true;
    }
    pub fn has_option(&self, option: ArgumentOption) -> bool {
        self.options.contains(&option)
    }
    pub fn is_parsed(&self) -> bool {
        self.parsed
    }

    pub fn set_parsed(&mut self) {
        self.parsed = true;
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }
    pub fn get_n_args(opts: &Vec<ArgumentOption>) -> usize{
        let mut ret: usize = 1;
        for o in opts {
            match o {
                ArgumentOption::NArgs(i) => {if *i > 0 {ret = *i;}},
                _ => {}
            }
        }
        ret
    }

    pub fn guess_type(name: &str, options: &Vec<ArgumentOption>, data_type_: &DataType) -> Option<ArgumentType> {
        if name.is_empty() {
            return None;
        }
        
        if name.starts_with("-") {
            /* Flag or optional */
            if *data_type_ == DataType::Bool && 
                (options.contains(&ArgumentOption::StoreFalse) || options.contains(&ArgumentOption::StoreTrue))
            {
                return Some(ArgumentType::Flag);
            }
            return Some(ArgumentType::Optional);
        }
        
        return Some(ArgumentType::Positional);
    }

    pub fn get_type(&self) -> ArgumentType{
        self.arg_type
    }

    pub fn parse_name(name: &str) -> Option<String> {
        if name.is_empty() {
            return None;
        }

        let mut ret_name: String = name.clone().into();
        loop {
            let minus_start = ret_name.starts_with("-");
            if minus_start {
                ret_name = ret_name.strip_prefix("-").unwrap().into();
            } else {
                break;
            }
        }

        Some(ret_name)
    }

    pub fn has_identifier(&self, id: &str) -> bool {
        self.cl_identifiers.contains(&id.to_owned())
    }
}
