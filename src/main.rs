mod parse_args;

mod parse_files;
mod terminal;

use parse_args::{Command, ParsedArgs};

fn main() {
    let parsed_args: ParsedArgs = parse_args::parse();

    match parsed_args.command {
        Command::Passwd => terminal::render_passwd(parsed_args.overwritten_file_to_parse),
        Command::Group => terminal::render_group(parsed_args.overwritten_file_to_parse),
        Command::Shells => terminal::render_shells(parsed_args.overwritten_file_to_parse),
    };
}
