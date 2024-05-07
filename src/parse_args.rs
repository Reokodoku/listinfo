use std::env;

pub struct ParsedArgs {
    pub command: Command,
    pub overwritten_file_to_parse: Option<String>,
}

pub enum Command {
    Passwd,
    Group,
    Shells,
}

fn help() {
    println!(
        "Usage: listinfo [OPTIONS] [COMMAND]
    Parse some Linux files in a table.

    Available commands:
        passwd      parse the /etc/passwd file
        group       parse the /etc/group file
        shells      parse the /etc/shells file

    Available options:
        -f [FILE]   specify the file to parse
        --help      display this help
        --version   output version information
    "
    );
    std::process::exit(0);
}

fn version() {
    println!("listinfo 1.0.1");
    std::process::exit(0);
}

pub fn parse() -> ParsedArgs {
    let mut pargs = ParsedArgs {
        command: unsafe { std::mem::zeroed() },
        overwritten_file_to_parse: None,
    };

    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        let mut now_parameter: bool = false;
        let mut what_parameter_is: &str = "";
        for arg in args {
            if now_parameter {
                match what_parameter_is {
                    "-f" => {
                        pargs.overwritten_file_to_parse = Some(arg);
                        now_parameter = false;
                    }
                    _ => now_parameter = false,
                }
            } else {
                match arg.as_ref() {
                    "-f" => {
                        now_parameter = true;
                        what_parameter_is = "-f";
                    }
                    "passwd" => pargs.command = Command::Passwd,
                    "group" => pargs.command = Command::Group,
                    "shells" => pargs.command = Command::Shells,
                    "--version" => version(),
                    _ => help(),
                }
            }
        }
    } else {
        help();
    }

    pargs
}
