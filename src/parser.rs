use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0},
    error::Error,
    multi::separated_list0,
    sequence::{delimited, separated_pair, terminated},
    Err, IResult,
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

fn parse_line(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, _) = take_until("!")(input)?;
    let (input, _) = tag("!")(input)?;
    parse_key_value_pairs(input)
}

pub fn parse(input: &str) -> Result<Vec<(&str, &str)>, Err<Error<String>>> {
    parse_line(input)
        .map(|(_, result)| result)
        .map_err(Err::<Error<&str>>::to_owned)
}
