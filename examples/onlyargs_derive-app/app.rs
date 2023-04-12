use onlyargs::CliError;
use onlyargs_derive::OnlyArgs;

/// Demonstrates argument parsing with `onlyargs_derive`.
#[derive(Debug, OnlyArgs)]
struct Args {
    /// Sets a number.
    number: u32,

    /// Sets an optional number.
    opt_number: Option<u32>,

    /// Sets width.
    #[default(10)]
    width: u32,

    /// Input paths.
    input: Vec<std::path::PathBuf>,
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
