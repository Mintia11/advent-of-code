#![feature(exit_status_error)]

use std::{env, process::Command, sync::Arc};

use regex::Regex;
use reqwest::{
    blocking::Client,
    cookie::Jar,
    header::{ACCEPT, COOKIE, REFERER, USER_AGENT},
    Url,
};

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

const CARGO_TOML_RUNNER_TEMPLATE: &str = r#"shared = { workspace = true, features = ["runner"] }

[dev-dependencies]
shared = { workspace = true }
"#;

const MAIN_RS_RUNNER_TEMPLATE: &str = r#"
shared::runner!(DAYS);

#[rustfmt::skip]
const DAYS: &[fn()] = &[];
"#;

fn main() {
    dotenvy::dotenv().unwrap();

    let args = env::args().collect::<Vec<_>>();

    let year = &args[1];
    let day = args.get(2);

    if let Some(day) = day {
        day_template(year, day);
    } else {
        year_template(year);
    }
}

fn day_template(year: &str, day: &str) {
    println!("Making template for {} day {}", year.replace('-', " "), day);

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

    println!("Downloading input and test-input for {}/{}", year, day);

    let session = env::var("session").expect("No session in dotenv");

    let cookies = Jar::default();
    cookies.add_cookie_str(
        &format!("session={session}"),
        &"https://adventofcode.com/".parse::<Url>().unwrap(),
    );
    let client = Client::builder()
        .cookie_provider(Arc::new(cookies))
        .build()
        .unwrap();

    let index = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}",
            year.strip_prefix("year-").unwrap(),
            day,
        ))
        .header(ACCEPT, "text/html")
        .header(
            REFERER,
            format!(
                "https://adventofcode.com/{}",
                year.strip_prefix("year-").unwrap(),
            ),
        )
        .header(
            USER_AGENT,
            r"Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
        )
        .build()
        .unwrap();

    let index = client.execute(index).unwrap();

    let index = index.bytes().unwrap();
    let index = std::str::from_utf8(&index).unwrap();

    let code_regex = Regex::new(r"<code>(.*?)<\/code>").unwrap();
    let codes = code_regex
        .captures_iter(index)
        .map(|c| c[1].to_string())
        .collect::<Vec<_>>();

    if (codes.len() - 1) > 1 {
        println!("Found more than one code entry, using the last one");
    }

    let code = codes[codes.len() - 2].as_str();
    std::fs::write(format!("{year}/test-input/day-{day}.txt"), code).unwrap();

    let input = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year.strip_prefix("year-").unwrap(),
            day,
        ))
        .header(ACCEPT, "text/html")
        .header(
            REFERER,
            format!(
                "https://adventofcode.com/{}",
                year.strip_prefix("year-").unwrap(),
            ),
        )
        .header(
            USER_AGENT,
            r"Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
        )
        .build()
        .unwrap();

    let input = client.execute(input).unwrap();

    let input = input.bytes().unwrap();
    let input = std::str::from_utf8(&input).unwrap();

    std::fs::write(format!("{year}/input/day-{day}.txt"), input.trim()).unwrap();
}

fn year_template(year: &str) {
    println!("Initializing crate for {}", year.replace('-', " "));

    Command::new("cargo")
        .args(&["new", year])
        .output()
        .unwrap()
        .status
        .exit_ok()
        .unwrap();

    std::fs::create_dir(format!("{year}/src/bin/")).unwrap();
    std::fs::create_dir(format!("{year}/input/")).unwrap();
    std::fs::create_dir(format!("{year}/test-input/")).unwrap();

    std::fs::write(format!("{year}/src/main.rs"), MAIN_RS_RUNNER_TEMPLATE).unwrap();

    let mut cargo_toml = std::fs::read_to_string(format!("{year}/Cargo.toml")).unwrap();
    cargo_toml.push_str(CARGO_TOML_RUNNER_TEMPLATE);
    std::fs::write(format!("{year}/Cargo.toml"), cargo_toml).unwrap();

    let mut git_attributes = std::fs::read_to_string(".gitattributes").unwrap();
    git_attributes.push_str(&format!(
        "\n{year}/input/** filter=git-crypt diff=git-crypt"
    ));
    std::fs::write(".gitattributes", git_attributes).unwrap();
}
