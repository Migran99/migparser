mod contents;
pub use contents::Content;
pub use contents::{DataType, ExtractFromContents};
/*
TODO:
   - ArgumentOptions -> to enum
   - cl_name -> to cl_indetifiers ?
   - new() : and different creators for different argument types
   - encapsulation of parsed, index, data (protected components)

*/

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
        default_val: Option<Content>
    ) -> Self {
        Argument {
            name: name_.to_owned(),
            cl_identifiers: cl_identifiers_,
            data_type: data_type_,
            data: default_val,
            options: options_.unwrap_or_default(),
            parsed: false,
            index: index_, /* Only settable at instantiation new_positional */
        }
    }
    pub fn new_optional(
        name_: &str,
        cl_identifiers_: Vec<String>,
        data_type_: DataType,
        options_: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>
    ) -> Self {
        Argument::new(name_, cl_identifiers_, data_type_, options_, -1, default_val)
    }

    pub fn new_positional(
        name_: &str,
        cl_identifiers_: Vec<String>,
        data_type_: DataType,
        options_: Option<Vec<ArgumentOption>>,
        index_: i32,
    ) -> Self {
        Argument::new(name_, cl_identifiers_, data_type_, options_, index_, None)
    }

    pub fn new_flag(
        name_: &str,
        cl_identifiers_: Vec<String>,
        options_: Option<Vec<ArgumentOption>>,
        default_val: Option<Content>
    ) -> Self {
        Argument::new(name_, cl_identifiers_, DataType::Bool, options_, -1, default_val)
    }

    /* AUX */
    pub fn get_data(&self) -> Option<Content> {
        self.data.clone()
    }
    pub fn set_data(&mut self, data: Content) -> bool {
        if self.is_parsed() {
            return false;
        }
        if self.data_type != data.get_type() {
            return false;
        }
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

    pub fn get_type(name: &str, options: &Vec<ArgumentOption>, data_type_: &DataType) -> Option<ArgumentType> {
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
