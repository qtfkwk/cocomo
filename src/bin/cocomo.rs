use clap::Parser;
use cocomo::*;

#[derive(Parser)]
#[command(
    about = "\
COCOMO (Constructive Cost Model) CLI utility and library

<https://crates.io/crates/cocomo> / <https://github.com/qtfkwk/cocomo>

See also: <https://en.wikipedia.org/wiki/COCOMO>

---",
    version,
    max_term_width = 80
)]
struct Cli {
    /// Source lines of code [default: *calculate from Files / Directories argument(s)*]
    #[arg(long, value_name = "N", conflicts_with = "paths")]
    sloc: Option<f64>,

    /// Average Wage
    #[arg(long, value_name = "f64", default_value = "56286.0")]
    average_wage: f64,

    /// Inflation multiplier
    #[arg(long, value_name = "f64", default_value = "1.0")]
    inflation_multiplier: f64,

    /// Inflation year (1995-2024) [default: 1995]
    #[arg(long, value_name = "usize")]
    inflation_year: Option<usize>,

    /// Overhead
    #[arg(long, value_name = "f64", default_value = "2.4")]
    overhead: f64,

    /// Effort Adjustment Factor (EAF); typically 0.9 - 1.4
    #[arg(long, value_name = "f64", default_value = "1.0")]
    eaf: f64,

    /**
    Project type (organic: "--custom 2.4,1.05,0.38", embedded: "--custom 3.6,1.20,0.32",
    semi-detached: "--custom 3.0,1.12,0.35")
    */
    #[arg(long, value_name = "TYPE", default_value = "organic")]
    project_type: ProjectType,

    /// Custom parameters (a, b, c) [default: "2.4,1.05,0.38" ("--project-type organic")]
    #[arg(long, value_name = "f64,f64,f64", conflicts_with = "project_type")]
    custom: Option<String>,

    /// Development time (d)
    #[arg(long, value_name = "f64", default_value = "2.5")]
    development_time: f64,

    /// Currency symbol
    #[arg(long, value_name = "STRING", default_value = "$")]
    currency_symbol: String,

    /// Output format
    #[arg(short, long, value_name = "FORMAT", default_value = "markdown-table")]
    output_format: OutputFormat,

    /// Files / Directories
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<std::path::PathBuf>,
}

fn main() {
    // Process command line options
    let cli = Cli::parse();
    let params = if let Some(custom) = cli.custom {
        let s: Vec<f64> = custom
            .split(',')
            .map(|x| {
                x.parse::<f64>().unwrap_or_else(|e| {
                    eprintln!("{e}: {x:?}");
                    std::process::exit(1);
                })
            })
            .collect();
        if s.len() == 3 {
            (s[0], s[1], s[2])
        } else {
            eprintln!("Invalid custom project parameters: {custom:?}");
            std::process::exit(1);
        }
    } else {
        cli.project_type.to_params()
    };

    // Calculate COCOMO estimates
    let project = Cocomo::new(
        &cli.currency_symbol,
        cli.eaf,
        cli.average_wage,
        cli.overhead,
        &params,
        cli.development_time,
        &cli.paths,
        &cli.sloc,
        cli.inflation_multiplier,
        &cli.inflation_year,
    );

    // Print report
    println!("{}", project.report(&cli.output_format));
}
