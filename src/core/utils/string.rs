

const line_terminator: &'static str = "\r\n";

pub fn remove_line_terminator(command: &str) -> &str {
    let command_len = command.len();
    if command_len < 2 {
        return command;
    }

    let possible_terminator = command_len - 2;
    if &command[possible_terminator..command_len] == "\r\n" {
        return &command[0..possible_terminator];
    }
    return command;
}

pub fn remove_line_terminator_string(mut line: String) -> String {
    if line.len() < line_terminator.len() {
        return line
    }

    let line_terminator_position = line.len() - line_terminator.len();
    if &line[line_terminator_position..line.len()] == line_terminator {
        line.truncate(line_terminator_position)
    }

    return line
}