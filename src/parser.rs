use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::bytes::streaming::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::{pair, tuple};
use nom::IResult;

#[derive(Debug)]
pub struct SysCall<'a> {
    name: &'a str,
}

pub fn c_identifier(s: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(s)
}

pub fn syscall_name(s: &str) -> IResult<&str, &str> {
    take_while1(|c| c != '(')(s)
}

pub fn parse_log_line(s: &str) -> IResult<&str, SysCall> {
    map(tuple((syscall_name, tag("("))), |(name, _)| SysCall {
        name,
    })(s)
}
