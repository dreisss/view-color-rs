fn main() {
    let mut args = std::env::args();
    let _default_arg = args.next();

    let color = args.collect::<String>();
    let r = i64::from_str_radix(&color[1..3], 16).unwrap();
    let g = i64::from_str_radix(&color[3..5], 16).unwrap();
    let b = i64::from_str_radix(&color[5..7], 16).unwrap();

    print!("\x1b[0m\x1b[48;2;{};{};{}m {} \x1b[0m", r, g, b, color);
    println!("\x1b[0m\x1b[38;2;{};{};{}m {} \x1b[0m", r, g, b, color);
}
