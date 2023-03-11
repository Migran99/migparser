/// Implements content data structure and associated functionality.
/// 
/// The ExtractFromContents allows to directly extract the value of the enum
/// Any new types in Contents enum should also implement this trait.

// Supported types
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum DataType {
    Int,
    Uint,
    String,
    Bool,
    Float,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Content {
    Int(i32),
    Uint(u32),
    String(String),
    Bool(bool),
    Float(f32)
}
impl Content {
    pub fn get_value_str(&self) -> String{
        match self {
            Content::Bool(c) => c.to_string(),
            Content::Int(c) => c.to_string(),
            Content::Uint(c) => c.to_string(),
            Content::String(c) => c.to_string(),
            Content::Float(c) => c.to_string(),
        }
    }
    pub fn get_value<T: ExtractFromContents>(&self) -> Option<T> {
        T::extract(self)
    }
    pub fn get_type(&self) -> DataType {
        match self {
            Content::Bool(c) => DataType::Bool,
            Content::Int(c) => DataType::Int,
            Content::Uint(c) => DataType::Uint,
            Content::String(c) => DataType::String,
            Content::Float(c) => DataType::Float,
        }
    }
}

pub trait ExtractFromContents {
    fn extract(object: &Content) -> Option<Self> where Self: Sized;
}

impl ExtractFromContents for i32 {
    fn extract(object: &Content) -> Option<Self> {
        match object {
            Content::Int(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}

impl ExtractFromContents for bool {
    fn extract(object: &Content) -> Option<Self> {
        match object {
            Content::Bool(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}

impl ExtractFromContents for u32 {
    fn extract(object: &Content) -> Option<Self> {
        match object {
            Content::Uint(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}

impl ExtractFromContents for String {
    fn extract(object: &Content) -> Option<Self> {
        match object {
            Content::String(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}

impl ExtractFromContents for f32 {
    fn extract(object: &Content) -> Option<Self> {
        match object {
            Content::Float(i) => {Some(i.to_owned())},
            _ => None
        }
    }
}