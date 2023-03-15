/// Implements content data structure and associated functionality.
/// 
/// The ExtractFromContents allows to directly extract the value of the enum
/// Any new types in Contents enum should also implement this trait.
/// 
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum ListType {
    Int,
    Uint,
    String,
    Bool,
    Float,
}
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
    List(ListType)
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct ContentList {
    data_type: ListType,
    pub data: Vec<Content>
}
impl ContentList {
    pub fn new(data_t: ListType) -> Self {
        ContentList { data_type: data_t, data: vec![] }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Content {
    Int(i32),
    Uint(u32),
    String(String),
    Bool(bool),
    Float(f32),
    List(ContentList)
}

fn vec2string(obj: &ContentList) -> String{
    let mut text: String = String::new();
    let data = &obj.data;
    text.push('[');
    for d in data {
        text.push_str(format!(" {} ", d.get_value_str().as_str()).as_str());
    }
    text.push(']');

    text
}
impl Content {
    pub fn get_value_str(&self) -> String{
        match self {
            Content::Bool(c) => c.to_string(),
            Content::Int(c) => c.to_string(),
            Content::Uint(c) => c.to_string(),
            Content::String(c) => c.to_string(),
            Content::Float(c) => c.to_string(),
            Content::List(c) => vec2string(c),
        }
    }
    pub fn get_value<T: ExtractFromContents>(&self) -> Option<T> {
        T::extract(self)
    }
    pub fn get_type(&self) -> DataType {
        match self {
            Content::Bool(_) => DataType::Bool,
            Content::Int(_) => DataType::Int,
            Content::Uint(_) => DataType::Uint,
            Content::String(_) => DataType::String,
            Content::Float(_) => DataType::Float,
            Content::List(t) => {
                match &t.data_type {
                    ListType::Bool => {DataType::Bool},
                    ListType::Int => {DataType::Int},
                    ListType::Uint => {DataType::Uint},
                    ListType::String => {DataType::String},
                    ListType::Float => {DataType::Float},
                }
            },
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

impl <T: ExtractFromContents> ExtractFromContents for Vec<T> {
    fn extract(object: &Content) -> Option<Vec<T>> {
        match object {
            Content::List(l) => 
            {
                let mut list: Vec<T> = vec![];
                for i in &l.data {
                    let val: T = i.get_value().unwrap();
                    list.push(val);
                }
                Some(list)
            }
            _ => None,
        }
    }
}