use std::env;

const TEMPLATE: &str = r#"
fn main() {
    let inputs = shared::parse_input(|s| {
        todo!("Parse input")
    });

    shared::solution_fn(1, &inputs, todo!("Part 1 sample solution"), |input| {
        todo!("Part 1")
    });

    shared::solution_fn(2, &inputs, todo!("Part 2 sample solution"), |input| {
        todo!("Part 2")
    });
}

shared::runner!();
"#;

const CARGO_TOML_TEMPLATE: &str = r#"
[[bin]]
name = "*YEAR*_day-*DAY*"
path = "src/bin/day-*DAY*.rs"
"#;

const MAIN_RS_TEMPLATE: &str = r#"
#[path = "bin/day-*DAY*.rs"]
mod day_*DAY*;
"#;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let year = &args[1];
    let day = &args[2];

    let path = format!("{year}/src/bin/day-{day}.rs");
    std::fs::write(path, TEMPLATE).unwrap();

    let cargo_toml_template = CARGO_TOML_TEMPLATE
        .replace("*YEAR*", year)
        .replace("*DAY*", day);

    let cargo_toml_path = format!("{year}/Cargo.toml");
    let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path).unwrap();
    cargo_toml.push_str(&cargo_toml_template);
    std::fs::write(cargo_toml_path, cargo_toml).unwrap();

    let main_rs_template = MAIN_RS_TEMPLATE.replace("*DAY*", day);

    let main_rs_path = format!("{year}/src/main.rs");
    let mut main_rs = std::fs::read_to_string(&main_rs_path).unwrap();
    main_rs.push_str(&main_rs_template);
    let mut lines = main_rs.lines().map(ToString::to_string).collect::<Vec<_>>();
    for line in &mut lines {
        if line.contains("const DAYS: &[fn()]") {
            line.remove(line.len() - 2);
            line.remove(line.len() - 1);
            line.push_str(&format!(", day_{day}::run];"));
        }

        line.push('\n');
    }
    let main_rs = lines.iter().flat_map(|c| c.chars()).collect::<String>();
    std::fs::write(main_rs_path, main_rs).unwrap();
}
