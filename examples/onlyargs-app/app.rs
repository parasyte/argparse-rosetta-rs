use onlyargs::{CliError, OnlyArgs};
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug)]
struct Args {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<PathBuf>,
}

impl OnlyArgs for Args {
    fn parse(args: Vec<OsString>) -> Result<Self, CliError> {
        use onlyargs::extensions::*;

        let mut number = None;
        let mut opt_number = None;
        let mut width = 10;
        let mut input = vec![];

        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.to_str() {
                Some(name @ "--number") => {
                    number = Some(args.next().parse_int(name)?);
                }
                Some(name @ "--opt-number") => {
                    opt_number = Some(args.next().parse_int(name)?);
                }
                Some(name @ "--width") => {
                    width = args.next().parse_int(name)?;
                }
                _ => {
                    input.push(arg.into());
                }
            }
        }

        Ok(Self {
            number: number.required("--number")?,
            opt_number,
            width,
            input,
        })
    }
}

fn main() -> Result<(), CliError> {
    let args: Args = onlyargs::parse()?;

    println!("{:#?}", args.number);
    println!("{:#?}", args.opt_number);
    println!("{:#?}", args.width);
    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }

    Ok(())
}
