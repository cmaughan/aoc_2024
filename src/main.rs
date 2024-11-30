use clap::Parser;
use itertools::Itertools;
use prettytable::{Cell, Row, Table};
use std::{error::Error, fs, process::Command};

fn format_ns(val: u64) -> String {
    if val < 1000 {
        format!("{:.1}ns", val)
    } else if val < 1000000 {
        format!("{:.1}μs", val as f64 / 1000.0)
    } else {
        format!("{:.1}ms", val as f64 / 1000000.0)
    }
}
fn extract_microseconds(output: &str) -> (u64, &str, u64, &str) {
    let out = output
        .lines()
        .last()
        .unwrap()
        .split(",")
        .collect::<Vec<_>>();

    let r1 = out[0];
    let p1 = out[1].parse::<u64>().unwrap();
    let r2 = out[2];
    let p2 = out[3].parse::<u64>().unwrap();

    (p1, r1, p2, r2)
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    perf: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let clargs = Cli::parse();
    let mut days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();
    let mut total_time = 0;

    days.sort_by(|a, b| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Day"),
        Cell::new("Result 1"),
        Cell::new("Result 2"),
        Cell::new("Time 1"),
        Cell::new("Time 2"),
        Cell::new("Total Time"),
    ]));

    for day in &days {
        let mut args = vec!["run", "--release", "--bin", day];
        args.push("--");
        if clargs.perf {
            args.push("--perf");
        }
        args.push("--uber");
        let cmd = Command::new("cargo").args(args).output()?;
        let output = String::from_utf8(cmd.stdout)?;
        let res = extract_microseconds(&output);
        table.add_row(Row::new(vec![
            Cell::new(day),
            Cell::new(res.1),
            Cell::new(res.3),
            Cell::new(&format_ns(res.0)),
            Cell::new(&format_ns(res.2)),
            Cell::new(&format_ns(res.0 + res.2)),
        ]));
        total_time += res.0 + res.2;
    }

    table.printstd();
    println!("\nTotal time: {}", format_ns(total_time));
    Ok(())
}
