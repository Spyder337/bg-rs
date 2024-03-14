use std::error::Error;

use bg_lib::{generators::RustGenerator, Generator, ProjectBuilder, ProjectType};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'b', value_enum, default_value="rust")]
    build_system: ProjectBuilder,
    #[arg(short = 't', value_enum, default_value="bin")]
    build_type: ProjectType,
    #[arg(required=true)]
    p_name: String,
    #[arg(required=false)]
    libs: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>>{
    //  Note --help returns after this function.
    let cli = Cli::parse();

    let gen: Box<dyn Generator>;
    match cli.build_system {
        ProjectBuilder::Build2 => todo!(),
        ProjectBuilder::Python => todo!(),
        ProjectBuilder::Rust => gen = Box::new(RustGenerator),
    }
    
    gen.create(cli.build_type, &cli.p_name, &cli.libs)?;

    return Ok(());
}
