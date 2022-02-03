use anyhow::Result;
use clap::Parser;
use show_path::Args;
use std::path::Path;

fn main() -> Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.path);

    let absolute_path = path.canonicalize()?;

    println!("{}", absolute_path.display());

    Ok(())
}
