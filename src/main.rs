use std::env;

mod terminal;
mod parse_files;

fn help() {
    println!("Usage: listinfo [OPTIONS] [COMMAND]
    Parse some Linux files in a table.

    Available commands:
        passwd      parse the /etc/passwd file
        group       parse the /etc/group file
        shells      parse the /etc/shells file

    Available options:
        -f [FILE]   specify the file to parse
        --help      display this help
        --version   output version information
    ");
}

fn main() {
    let mut overwritten_file_to_parse: String = "".to_string();
    let mut command_to_execute: &str = "";
    let args: Vec<String> = env::args().skip(1).collect();
    
    if !args.is_empty()
    {
        let mut now_parameter: bool = false;
        let mut what_parameter_is: &str = "";
        for arg in args
        {
            if now_parameter
            {
                match what_parameter_is {
                    "-f" => {
                        overwritten_file_to_parse = arg;
                        now_parameter = false;
                    }
                    _ => now_parameter = false
                }
            }
            else
            {
                match arg.as_ref() {
                    "-f" => {
                        now_parameter = true;
                        what_parameter_is = "-f";
                    }
                    "--help" => return help(),
                    "--version" => return println!("listinfo 1.0.1"),
                    "passwd" => command_to_execute = "passwd",
                    "group" => command_to_execute = "group",
                    "shells" => command_to_execute = "shells",
                    _ => return help()
                }
            }
        }
    }

    if command_to_execute == "" {
        return help();
    }
    match command_to_execute {
        "passwd" => terminal::render_passwd(overwritten_file_to_parse),
        "group" => terminal::render_group(overwritten_file_to_parse),
        "shells" => terminal::render_shells(overwritten_file_to_parse),
        _ => {}
    }
    
    //terminal::render(file_to_parse);
}
