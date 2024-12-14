use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0},
    multi::separated_list0,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        delimited(multispace0, take_until("="), multispace0),
        char('='),
        delimited(multispace0, take_until(","), multispace0),
    )(input)
}

fn parse_key_value_pairs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list0(char(','), terminated(parse_key_value, multispace0))(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, _) = take_until("!")(input)?;
    let (input, _) = tag("!")(input)?;
    parse_key_value_pairs(input)
}
