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

    let path = format!("{}/src/bin/day-{}.rs", year, day);
    std::fs::write(path, TEMPLATE).unwrap();

    let cargo_toml_template = CARGO_TOML_TEMPLATE
        .replace("*YEAR*", &year)
        .replace("*DAY*", &day);

    let cargo_toml_path = format!("{}/Cargo.toml", year);
    let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path).unwrap();
    cargo_toml.extend(cargo_toml_template.chars());
    std::fs::write(cargo_toml_path, cargo_toml).unwrap();

    let main_rs_template = MAIN_RS_TEMPLATE.replace("*DAY*", &day);

    let main_rs_path = format!("{}/src/main.rs", year);
    let mut main_rs = std::fs::read_to_string(&main_rs_path).unwrap();
    main_rs.extend(main_rs_template.chars());
    let mut lines = main_rs.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    for line in &mut lines {
        if line.contains("const DAYS: &[fn()]") {
            line.remove(line.len() - 2);
            line.remove(line.len() - 1);
            line.push_str(&format!(", day_{}::run];", day));
        }

        line.push('\n');
    }
    let main_rs = lines
        .iter()
        .map(|c| c.chars())
        .flatten()
        .collect::<String>();
    std::fs::write(main_rs_path, main_rs).unwrap();
}
