use crate::parse_files;

use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

pub fn render_passwd(overwritten_file_to_parse: Option<String>) {
    if let Ok(lines) =
        parse_files::read_lines(overwritten_file_to_parse.unwrap_or("/etc/passwd".to_string()))
    {
        let mut lines_table: Vec<Vec<cli_table::CellStruct>> = vec![];
        for line in lines.flatten() {
            let splitted_line: Vec<&str> = line.split(':').collect();

            lines_table.push(vec![
                splitted_line[0].cell(),
                if splitted_line[1] == "x" {
                    "ENCRYPTED".cell()
                } else {
                    splitted_line[1].cell()
                },
                splitted_line[2].cell(),
                splitted_line[3].cell(),
                splitted_line[4].cell(),
                splitted_line[5].cell(),
                splitted_line[6].cell(),
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

pub fn render_group(overwritten_file_to_parse: Option<String>) {
    if let Ok(lines) =
        parse_files::read_lines(overwritten_file_to_parse.unwrap_or("/etc/group".to_string()))
    {
        let mut lines_table: Vec<Vec<cli_table::CellStruct>> = vec![];
        for line in lines.flatten() {
            let splitted_line: Vec<&str> = line.split(':').collect();

            lines_table.push(vec![
                splitted_line[0].cell(),
                if splitted_line[1] == "x" || splitted_line[1] == "*" {
                    "CANNOT LOGIN".cell()
                } else {
                    splitted_line[1].cell()
                },
                splitted_line[2].cell(),
                splitted_line[3].cell(),
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

pub fn render_shells(overwritten_file_to_parse: Option<String>) {
    if let Ok(lines) =
        parse_files::read_lines(overwritten_file_to_parse.unwrap_or("/etc/shells".to_string()))
    {
        let mut lines_table: Vec<Vec<cli_table::CellStruct>> = vec![];
        for line in lines.flatten() {
            if !line.starts_with("#") {
                let splitted_line: Vec<&str> = line.split("\n").collect();

                lines_table.push(vec![splitted_line[0].cell()]);
            }
        }

        let table: cli_table::TableStruct = lines_table
            .table()
            .title(vec!["Shell".cell().bold(true)])
            .bold(true);

        assert!(print_stdout(table).is_ok());
    }
}
