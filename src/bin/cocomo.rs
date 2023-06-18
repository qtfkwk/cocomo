use clap::Parser;
use cocomo::*;

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

    /// Project type
    #[arg(
        long,
        value_name = "TYPE",
        default_value = "organic",
    )]
    project_type: ProjectType,

    /// Currency symbol
    #[arg(long, value_name = "STRING", default_value = "$")]
    currency_symbol: String,

    /// Use SLOCCount-style output format
    #[arg(long)]
    sloccount: bool,

    /// Files / Directories
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<std::path::PathBuf>,
}

fn main() {
    // Process command line options
    let cli = Cli::parse();
    let cur = &cli.currency_symbol;
    let params = cli.project_type.to_params();

    // Get total SLOC
    let config = tokei::Config::default();
    let mut languages = tokei::Languages::new();
    languages.get_statistics(&cli.paths, &[], &config);
    let sum = languages.total();
    let sloc = sum.code as f64;

    // Calculate COCOMO estimates
    let effort = estimate_effort(sloc, cli.eaf, &params);
    let cost = estimate_cost(effort, cli.average_wage, cli.overhead);
    let months = estimate_months(effort, &params);
    let people = effort / months;

    // Print report
    if cli.sloccount {
        println!(
            "\
Total Physical Source Lines of Code (SLOC)                    = {sloc:.0}
Development Effort Estimate, Person-Years (Person-Months)     = {:.2} ({effort:.2})
  (Basic COCOMO model, Person-Months = {:.2}*(KSLOC**{:.2})*{:.2})
Schedule Estimate, Years (Months)                             = {:.2} ({months:.2})
  (Basic COCOMO model, Months = {:.2}*(person-months**{:.2}))
Estimated Average Number of Developers (Effort/Schedule)      = {people:.2}
Total Estimated Cost to Develop                               = {cur}{cost:.0}
  (average salary = {cur}{:.0}/year, overhead = {:.2})
\
            ",
            effort / 12.0,
            &params.0,
            &params.1,
            cli.eaf,
            months / 12.0,
            &params.2,
            &params.3,
            cli.average_wage,
            cli.overhead,
        );
    } else {
        println!(
            "\
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | {sloc:.0}
Estimated Cost to Develop  | {cur}{cost:.2}
Estimated Schedule Effort  | {months:.2} months
Estimated People Required  | {people:.2}
\
            "
        );
    }
}
