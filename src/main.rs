use std::env;

mod terminal;
mod parse_passwd;

fn help() {
    println!("Usage: listpasswd [OPTION]...
    Parse the /etc/passwd file in a table.
    
    Available options:
        -f [FILE]   specify the file to parse
        --help      display this help
        --version   output version information
    ");
}

fn main() {
    let mut passwd_file_path: String = "/etc/passwd".to_string();
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty()
    {
        let mut now_parameter: bool = false;
        let mut what_parameter_is: &str = "";
        for arg in args
        {
            if now_parameter
            {
                match what_parameter_is {
                    "-f" => {
                        passwd_file_path = arg;
                        now_parameter = false;
                    }
                    _ => {
                        now_parameter = false;
                    }
                }
            }
            else
            {
                match arg.as_ref() {
                    "-f" => {
                        now_parameter = true;
                        what_parameter_is = "-f";
                    }
                    "--help" => {
                        return help();
                    }
                    "--version" => {
                        return println!("listpasswd 1.0.0");
                    }
                    _ => return help()
                }   
            }
        }
    }
    
    terminal::render(passwd_file_path);
}
