use migparser::{ArgumentOption, ArgumentParser, DataType, ListType};

fn main() -> Result<(), String> {
    let mut parser = ArgumentParser::new();
    /* The type of argument (flag, positional, ...) is identified
       by the name of the argument and data type.
    */
    parser.add_argument("positionalarg", None, DataType::String, None, None)?;

    parser.add_argument(
        "--necessaryarg",
        Some(vec!["-na".to_owned()]),
        DataType::Int,
        Some(vec![ArgumentOption::Necessary]),
        None,
    )?;
    parser.add_argument(
        "--optionalarg",
        Some(vec!["-oa".to_owned()]),
        DataType::Float,
        None,
        None,
    )?;
    parser.add_argument(
        "--flagarg",
        Some(vec!["-f".to_owned()]),
        DataType::Bool,
        Some(vec![ArgumentOption::StoreTrue]), /* StoreTrue or StoreFalse for flag */
        None,
    )?;

    parser.add_argument(
        "--listarg",
        Some(vec!["-la".to_owned()]),
        DataType::List(ListType::Int),
        Some(vec![ArgumentOption::NArgs(4)]), /* StoreTrue or StoreFalse for flag */
        None,
    )?;

    let mut parser2 = parser.clone();

    parser.print_data();
    parser.parse_arguments();
    parser.print_data();

    parser2.parse_arguments_from_text("miguel -na 1 -f 1 -oa 2.3 -la 12 34 78 23".to_owned());
    parser2.print_data();

    Ok(())
}
