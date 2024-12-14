use nom::{
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::char,
    error::Error,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Err, IResult,
};

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn value(input: &str) -> IResult<&str, &str> {
    take_till1(|c: char| c.is_whitespace() || c == ',')(input)
}

fn key_value_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(identifier, char('='), value)(input)
}

fn key_value_pairs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list0(char(','), key_value_pair)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    preceded(tag("!"), key_value_pairs)(input)
}

pub fn parse(input: &str) -> Result<Vec<(&str, &str)>, Err<Error<String>>> {
    parse_line(input)
        .map(|(_, result)| result)
        .map_err(Err::<Error<&str>>::to_owned)
}
