use crate::parse_files;

use cli_table::{format::Justify, Cell, Style, Table, print_stdout};

pub fn render_passwd(mut overwritten_file_to_parse: String) {
    if overwritten_file_to_parse == "" {
        overwritten_file_to_parse = "/etc/passwd".to_string();
    }

    if let Ok(lines) = parse_files::read_lines(overwritten_file_to_parse) {
        let mut lines_table: Vec<Vec<cli_table::CellStruct>> = vec![];
        for line in lines.flatten() {
            let splitted_line: Vec<&str> = line.split(':').collect();

            lines_table.push(vec![
                splitted_line[0].cell(), if splitted_line[1] == "x" { "ENCRYPTED".cell() } else { splitted_line[1].cell() }, splitted_line[2].cell(), splitted_line[3].cell(),
                splitted_line[4].cell(), splitted_line[5].cell(), splitted_line[6].cell()
            ]);
        }
        
        let table: cli_table::TableStruct = lines_table
            .table()
            .title(vec![
                "User".cell().bold(true),
                "Password".cell().bold(true).justify(Justify::Center),
                "UID".cell().bold(true).justify(Justify::Center),
                "GID".cell().bold(true).justify(Justify::Center),
                "GECOS".cell().bold(true).justify(Justify::Center),
                "Home directory".cell().bold(true).justify(Justify::Center),
                "Shell".cell().bold(true).justify(Justify::Center),
            ])
            .bold(true);
        
        assert!(print_stdout(table).is_ok());
    }
}

pub fn render_group(mut overwritten_file_to_parse: String) {
    if overwritten_file_to_parse == "" {
        overwritten_file_to_parse = "/etc/group".to_string();
    }

    if let Ok(lines) = parse_files::read_lines(overwritten_file_to_parse) {
        let mut lines_table: Vec<Vec<cli_table::CellStruct>> = vec![];
        for line in lines.flatten() {
            let splitted_line: Vec<&str> = line.split(':').collect();

            lines_table.push(vec![
                splitted_line[0].cell(), if splitted_line[1] == "x" || splitted_line[1] == "*" { "CANNOT LOGIN".cell() } else { splitted_line[1].cell() }, splitted_line[2].cell(), splitted_line[3].cell()
            ]);
        }
        
        let table: cli_table::TableStruct = lines_table
            .table()
            .title(vec![
                "Group".cell().bold(true),
                "Password".cell().bold(true).justify(Justify::Center),
                "GID".cell().bold(true).justify(Justify::Center),
                "Group list".cell().bold(true).justify(Justify::Center),
            ])
            .bold(true);
        
        assert!(print_stdout(table).is_ok());
    }
}

pub fn render_shells(mut overwritten_file_to_parse: String) {
    if overwritten_file_to_parse == "" {
        overwritten_file_to_parse = "/etc/shells".to_string();
    }
    if let Ok(lines) = parse_files::read_lines(overwritten_file_to_parse) {
        let mut lines_table: Vec<Vec<cli_table::CellStruct>> = vec![];
        for line in lines.flatten() {
            if !line.starts_with("#") {
                let splitted_line: Vec<&str> = line.split("\n").collect();

                lines_table.push(vec![splitted_line[0].cell()]);
            }
        }
        
        let table: cli_table::TableStruct = lines_table
            .table()
            .title(vec![
                "Shell".cell().bold(true),
            ])
            .bold(true);
        
        assert!(print_stdout(table).is_ok());
    }
}
