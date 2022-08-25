use anyhow::{anyhow, Result};
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Arg<'a> {
    Null,
    String(&'a str),
    Flags(Vec<&'a str>),
}

#[derive(Debug, PartialEq)]
pub struct SysCall<'a> {
    name: &'a str,
    args: Vec<Arg<'a>>,
    result: i8,
}

pub fn arg_raw_str(s: &str) -> IResult<&str, Arg> {
    map(take_till(|c| c == ',' || c == ')'), |s: &str| {
        Arg::String(s)
    })(s)
}

pub fn syscall_name(s: &str) -> IResult<&str, &str> {
    take_until("(")(s)
}

pub fn syscall_args(s: &str) -> IResult<&str, Vec<Arg>> {
    delimited(tag("("), separated_list0(tag(","), arg_raw_str), tag(")"))(s)
}

pub fn syscall_result(s: &str) -> IResult<&str, i8> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), |out: &str| {
        out.parse()
    })(s)
}

pub fn parse_syscall_line(s: &str) -> IResult<&str, SysCall> {
    map(
        tuple((
            syscall_name,
            syscall_args,
            delimited(multispace0, tag("="), multispace0),
            syscall_result,
        )),
        |(name, args, _, result)| SysCall { name, args, result },
    )(s)
}

pub fn parse_log_line(s: &str) -> Result<SysCall> {
    match parse_syscall_line(s) {
        Ok((_, v)) => Ok(v),
        Err(e) => Err(anyhow!("Could not parse {s}: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_syscall_line, Arg, SysCall};

    #[test]
    fn parse_openat() {
        let s = r#"openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3"#;
        let res = parse_syscall_line(s);
        dbg!(&res);
        assert_eq!(
            res,
            Ok((
                "",
                SysCall {
                    name: "openat",
                    args: vec!(
                        Arg::String("AT_FDCWD"),
                        Arg::String(" \"/etc/ld.so.cache\""),
                        Arg::String(" O_RDONLY|O_CLOEXEC"),
                    ),
                    result: 3
                }
            ))
        )
    }
}
