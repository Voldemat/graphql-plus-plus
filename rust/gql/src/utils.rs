pub fn format_line(
    line: &str,
    current_line: u32,
    location: &Location,
    exc: &str,
) -> String {
    let linestr = current_line.to_string();
    let mut buffer = format!("{}: {}\n", linestr, line);

    if current_line == location.get_line() {
        let mut underline = String::new();

        // spaces before the underline
        let spaces = location.get_start() as usize + 2 + linestr.len();
        for _ in 0..spaces {
            underline.push(' ');
        }

        // underline itself
        for _ in location.get_start()..=location.get_end() {
            underline.push('~');
        }

        // error message
        underline.push_str(format!(" Error: {}\n", exc).as_str());

        buffer.push_str(&underline);
    }

    buffer
}

pub fn format_error(
    exc: &str,
    location: Location,
    source: Rc<SourceFile>,
) -> String {
    let mut buffer = format!("{}\n", source.filepath.display());

    let line_num = location.get_line() as i32;

    let first_line_to_show = std::cmp::max(line_num - 4, 1) as u32;
    let last_line_to_show = (line_num + 4) as u32;

    let mut current_line: u32 = 1;

    for line in source.buffer.lines() {
        if first_line_to_show <= current_line
            && current_line <= last_line_to_show
        {
            buffer.push_str(&format_line(line, current_line, &location, exc));
        }

        current_line += 1;
    }

    buffer
}

