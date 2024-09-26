use structopt::StructOpt;

use rendro::cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rendro = cli::Rendro::from_args();

    match rendro.cmd {
        None => {
            println!("You didn't tell me to do anything...");
            Ok(())
        }
        Some(subcommand) => match subcommand {
            cli::SubCommand::Render {
                input_dir,
                output_dir,
                file_extension,
            } => cli::render(input_dir, output_dir, file_extension),
        },
    }
}
