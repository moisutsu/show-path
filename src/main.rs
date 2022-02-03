use anyhow::Result;
use clap::Parser;
use show_path::Args;
use std::path::Path;

fn main() -> Result<()> {
    let args = Args::parse();

    let path_name = args.path.unwrap_or_else(|| ".".to_string());

    let path = Path::new(&path_name);

    let absolute_path = path.canonicalize()?;

    println!("{}", absolute_path.display());

    Ok(())
}
