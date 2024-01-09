fn main() {
    let input = receive_input();

    let color = match input {
        Some(input) => get_color(input),
        None => panic!("No input provided!"),
    };

    match color {
        Ok(color) => print_color(color),
        Err(error) => panic!("{}", error),
    }
}

fn receive_input() -> Option<String> {
    let mut args = std::env::args();
    let _default_arg = args.next();
    let input: String = args.collect();

    if input.is_empty() {
        return None;
    }

    Some(input)
}

struct Color {
    hex: String,
    rgb: [i16; 3],
}

fn get_color(input: String) -> Result<Color, String> {
    let mut start = 0;

    if input.starts_with("#") {
        start = 1;
    }

    if input.len() != start + 6 {
        return Err("Invalid color.".to_string());
    }

    let r = match i16::from_str_radix(&input[start..(start + 2)], 16) {
        Ok(r) => r,
        Err(_) => return Err("Error parsing color! Please, review it.".to_string()),
    };

    let g = match i16::from_str_radix(&input[(start + 2)..(start + 4)], 16) {
        Ok(g) => g,
        Err(_) => return Err("Error parsing color! Please, review it.".to_string()),
    };

    let b = match i16::from_str_radix(&input[(start + 4)..(start + 6)], 16) {
        Ok(b) => b,
        Err(_) => return Err("Error parsing color! Please, review it.".to_string()),
    };

    Ok(Color {
        hex: format!("{:#>7}", input),
        rgb: [r, g, b],
    })
}

fn print_color(color: Color) {
    let [r, g, b] = color.rgb;

    print!("\x1b[0m\x1b[48;2;{};{};{}m {} \x1b[0m", r, g, b, color.hex);
    println!("\x1b[0m\x1b[38;2;{};{};{}m {} \x1b[0m", r, g, b, color.hex);
}
