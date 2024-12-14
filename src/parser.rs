use nom::{
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::char,
    error::Error,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Err, IResult,
};

pub struct Pair<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn value(input: &str) -> IResult<&str, &str> {
    take_till1(|c: char| c.is_whitespace() || c == ',')(input)
}

fn key_value_pair(input: &str) -> IResult<&str, Pair> {
    separated_pair(identifier, char('='), value)(input)
        .map(|(input, (key, value))| (input, Pair { key, value }))
}

fn key_value_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list0(char(','), key_value_pair)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Pair>> {
    preceded(tag("!"), key_value_pairs)(input)
}

pub fn parse(input: &str) -> Result<Vec<Pair>, Err<Error<String>>> {
    parse_line(input)
        .map(|(_, result)| result)
        .map_err(Err::<Error<&str>>::to_owned)
}
