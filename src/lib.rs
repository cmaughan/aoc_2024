use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};
// CM: Note:  I got these macros from here:
// https://github.com/AxlLind/AdventOfCode2022

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("../../inputs/{}.in", day.token()),
        _ => panic!("Expected one integer argument"),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
        use clap::Parser;
        use prettytable::{Cell, Row, Table};
        const INPUT: &str = include_str!(#input_path);

        #[derive(Parser)]
        struct Cli {
            #[arg(short, long, default_value_t = false)]
            perf: bool,
            #[arg(short, long, default_value_t = false)]
            uber: bool
        }

        fn format_ns(val: u64) -> String {
            if val < 1000 {
                format!("{:.1}ns", val)
            } else if val < 1000000 {
                format!("{:.1}μs", val as f64 / 1000.0)
            } else {
                format!("{:.1}ms", val as f64 / 1000000.0)
            }
        }

        #aoc_solution

        fn main() {
            let args = Cli::parse();
            let mut tot : f64 = 0.0;
            let mut out : usize = 0;
            let mut num_tries = 1;
            if args.perf == true {
                num_tries = 1000
            }

            let mut res = Vec::new();
            for part in 0..=1 {
                for _ in 0..num_tries {
                    let now = ::std::time::Instant::now();
                    out = aoc_solution(part, INPUT.trim_end());
                    let elapsed = now.elapsed();
                    tot += elapsed.as_nanos() as f64;
                }
                tot = tot / num_tries as f64;
                res.push((out, tot as u64));
            }

            if !args.uber {
                let mut table = Table::new();
                table.add_row(Row::new(vec![
                    Cell::new("Part"),
                    Cell::new("Result"),
                    Cell::new("Time"),
                ]));

                for (index, (r, t)) in res.iter().enumerate() {
                    table.add_row(Row::new(vec![
                        Cell::new(&format!("{}", index)),
                        Cell::new(&format!("{}", r)),
                        Cell::new(&format!("{}", format_ns(*t)))
                    ]));
                }
                table.printstd();
            }
            else {
                for a in res { 
                    print!("{},{},", a.0, a.1);
                }
            }
        }
    };
    TokenStream::from(tokens)
}
