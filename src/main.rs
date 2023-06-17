use clap::Parser;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PARAMS: HashMap<String, (f64, f64, f64, f64)> = [
        (String::from("organic"), (2.4, 1.05, 2.5, 0.38)),
        (String::from("semi-detached"), (3.0, 1.12, 2.5, 0.35)),
        (String::from("embedded"), (3.6, 1.20, 2.5, 0.32)),
    ].into_iter().collect();
}

fn estimate_cost(effort: f64, avg_wage: f64, overhead: f64) -> f64 {
    effort * avg_wage / 12.0 * overhead
}

fn estimate_effort(sloc: f64, eaf: f64, project_type: &String) -> f64 {
    let p = PARAMS.get(project_type).expect("invalid project type");
    p.0 * (sloc / 1000.0).powf(p.1) * eaf
}

fn estimate_months(effort: f64, project_type: &String) -> f64 {
    let p = PARAMS.get(project_type).expect("invalid project type");
    p.2 * effort.powf(p.3)
}

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Average Wage
    #[arg(long, value_name = "f64", default_value = "56286.0")]
    average_wage: f64,

    /// Overhead
    #[arg(long, value_name = "f64", default_value = "2.4")]
    overhead: f64,

    /// Effort Adjustment Factor
    #[arg(long, value_name = "f64", default_value = "1.0")]
    eaf: f64,

    /// Project type (organic, semi-detached, embedded)
    #[arg(long, value_name = "TYPE", default_value = "organic")]
    project_type: String,

    /// Currency symbol
    #[arg(long, value_name = "STRING", default_value = "$")]
    currency_symbol: String,

    /// Files / Directories
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<std::path::PathBuf>,
}

fn main() {
    // Process command line options
    let cli = Cli::parse();
    let cur = &cli.currency_symbol;
    let project_type = &cli.project_type;
    if !PARAMS.contains_key(project_type) {
        eprintln!("Invalid project type");
        std::process::exit(1);
    }

    // Get total SLOC
    let config = tokei::Config::default();
    let mut languages = tokei::Languages::new();
    languages.get_statistics(&cli.paths, &[], &config);
    let sum = languages.total();
    let sloc = sum.code as f64;

    // Calculate COCOMO estimates
    let effort = estimate_effort(sloc, cli.eaf, project_type);
    let cost = estimate_cost(effort, cli.average_wage, cli.overhead);
    let months = estimate_months(effort, project_type);
    let people = effort / months;

    // Print report
    println!("\
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | {sloc:.0}
Estimated Cost to Develop  | {cur}{cost:.2}
Estimated Schedule Effort  | {months:.2} months
Estimated People Required  | {people:.2}
\
    ");
}
