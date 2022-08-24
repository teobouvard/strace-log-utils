use anyhow::{anyhow, Result};
use nom::bytes::complete::take_until;
use nom::bytes::streaming::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res, opt, recognize};
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct SysCall<'a> {
    name: &'a str,
    args: &'a str,
    result: i8,
}

pub fn syscall_name(s: &str) -> IResult<&str, &str> {
    take_until("(")(s)
}

pub fn syscall_args(s: &str) -> IResult<&str, &str> {
    take_until(")")(s)
}

pub fn syscall_result(s: &str) -> IResult<&str, i8> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), |out| {
        i8::from_str_radix(out, 10)
    })(s)
}

pub fn parse_syscall_line(s: &str) -> IResult<&str, SysCall> {
    map(
        tuple((
            syscall_name,
            delimited(tag("("), syscall_args, tag(")")),
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
    use crate::parser::{parse_syscall_line, SysCall};

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
                    args: r#"AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC"#,
                    result: 3
                }
            ))
        )
    }
}
