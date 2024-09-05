use {
    lazy_static::lazy_static,
    std::{collections::BTreeMap, fmt, path::PathBuf},
};

//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref NUM: format_num::NumberFormat = format_num::NumberFormat::new();
    pub static ref USA_YEAR_INFLATION_MULTIPLIER: BTreeMap<usize, (f64, f64)> = [
        (1995, (2.83, 1.0000)),
        (1996, (2.64, 1.0283)),
        (1997, (2.57, 1.0547)),
        (1998, (2.14, 1.0804)),
        (1999, (2.23, 1.1018)),
        (2000, (3.38, 1.1241)),
        (2001, (2.77, 1.1579)),
        (2002, (2.44, 1.1856)),
        (2003, (2.27, 1.2100)),
        (2004, (3.38, 1.2327)),
        (2005, (3.39, 1.2665)),
        (2006, (2.54, 1.3004)),
        (2007, (2.85, 1.3258)),
        (2008, (3.84, 1.3543)),
        (2009, (-0.36, 1.3927)),
        (2010, (1.64, 1.3891)),
        (2011, (3.16, 1.4055)),
        (2012, (2.07, 1.4371)),
        (2013, (1.46, 1.4578)),
        (2014, (1.62, 1.4724)),
        (2015, (0.12, 1.4886)),
        (2016, (1.26, 1.4898)),
        (2017, (2.13, 1.5024)),
        (2018, (2.44, 1.5237)),
        (2019, (2.29, 1.5481)),
        (2020, (1.23, 1.5710)),
        (2021, (4.70, 1.5833)),
        (2022, (6.49, 1.6303)),
        (2023, (6.56, 1.6952)),
        (2024, (2.90, 1.7608)), // as of July 2024, with the next update scheduled for September 11, 2024
    ]
    .into_iter()
    .collect();
}

//--------------------------------------------------------------------------------------------------

/**
COCOMO Parameters from Boehm
*/
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ProjectType {
    /**
    A software project with requiring the highest level of complexity, creativity, and experience
    requirement fall under this category. Such software requires a larger team size than the other
    two models and also the developers need to be sufficiently experienced and creative to develop
    such complex models.
    Equivalent to: `--custom 3.6,1.20,0.32`.
    */
    Embedded,

    /**
    A software project is said to be an organic type if the team size required is adequately small,
    the problem is well understood and has been solved in the past and also the team members have a
    nominal experience regarding the problem.
    Equivalent to `--custom 2.4,1.05,0.38`.
    */
    Organic,

    /**
    A software project is said to be a Semi-detached type if the vital characteristics such as
    team-size, experience, knowledge of the various programming environment lie in between that of
    organic and Embedded.
    The projects classified as Semi-Detached are comparatively less familiar and difficult to
    develop compared to the organic ones and require more experience and better guidance and
    creativity.
    Eg: Compilers or different Embedded Systems can be considered of Semi-Detached type.
    Equivalent to `--custom 3.0,1.12,0.35`.
    */
    SemiDetached,
}

pub use ProjectType::*;

impl ProjectType {
    /**
    ```
    # use cocomo::*;
    assert_eq!(Embedded.to_params(), (3.6, 1.20, 0.32));
    assert_eq!(Organic.to_params(), (2.4, 1.05, 0.38));
    assert_eq!(SemiDetached.to_params(), (3.0, 1.12, 0.35));
    ```
    */
    pub fn to_params(&self) -> (f64, f64, f64) {
        match self {
            Embedded => (3.6, 1.20, 0.32),
            Organic => (2.4, 1.05, 0.38),
            SemiDetached => (3.0, 1.12, 0.35),
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    MarkdownTable,
    Sloccount,
    SloccountInflation,
}

pub use OutputFormat::*;

//--------------------------------------------------------------------------------------------------

/**
COCOMO estimate
*/
pub struct Cocomo {
    pub cur: String,
    pub eaf: f64,
    pub avg_wage: f64,
    pub overhead: f64,
    pub params: (f64, f64, f64),
    pub dev_time: f64,
    pub paths: Vec<PathBuf>,
    pub sloc: f64,
    pub effort: f64,
    pub cost: f64,
    pub months: f64,
    pub people: f64,
    pub inflation_multiplier: f64,
    pub inflation_year: Option<usize>,
}

impl Cocomo {
    /**
    Calculate COCOMO estimates
    */
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cur: &str,
        eaf: f64,
        avg_wage: f64,
        overhead: f64,
        params: &(f64, f64, f64),
        dev_time: f64,
        paths: &[PathBuf],
        sloc: &Option<f64>,
        inflation_multiplier: f64,
        inflation_year: &Option<usize>,
    ) -> Cocomo {
        let sloc = if let Some(n) = sloc {
            *n
        } else {
            total_sloc(paths)
        };
        let inflation_multiplier = if let Some(year) = inflation_year {
            USA_YEAR_INFLATION_MULTIPLIER
                .get(year)
                .unwrap_or(&(0.0, inflation_multiplier))
                .1
        } else {
            inflation_multiplier
        };
        let avg_wage = avg_wage * inflation_multiplier;
        let (effort, cost, months, people) =
            cocomo(sloc, eaf, avg_wage, overhead, params, dev_time);
        Cocomo {
            cur: cur.to_string(),
            eaf,
            avg_wage,
            overhead,
            params: *params,
            dev_time,
            paths: paths.to_vec(),
            sloc,
            effort,
            cost,
            months,
            people,
            inflation_multiplier,
            inflation_year: *inflation_year,
        }
    }

    /**
    Create a report
    */
    pub fn report(&self, fmt: &OutputFormat) -> String {
        match fmt {
            MarkdownTable => {
                format!(
                    "\
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | {}
Estimated Cost to Develop  | {}{}
Estimated Schedule Effort  | {} months
Estimated People Required  | {}
\
                    ",
                    integer(self.sloc),
                    self.cur,
                    float(self.cost),
                    float(self.months),
                    float(self.people),
                )
            }
            Sloccount => {
                format!(
                    "\
Total Physical Source Lines of Code (SLOC)                    = {}
Development Effort Estimate, Person-Years (Person-Months)     = {} ({})
  (Basic COCOMO model, Person-Months = {}*(KSLOC**{})*{})
Schedule Estimate, Years (Months)                             = {} ({})
  (Basic COCOMO model, Months = {}*(person-months**{}))
Estimated Average Number of Developers (Effort/Schedule)      = {}
Total Estimated Cost to Develop                               = {}{}
  (average salary = {}{}/year, overhead = {})
\
                    ",
                    integer(self.sloc),
                    float(self.effort / 12.0),
                    float(self.effort),
                    float(self.params.0),
                    float(self.params.1),
                    float(self.eaf),
                    float(self.months / 12.0),
                    float(self.months),
                    float(self.dev_time),
                    float(self.params.2),
                    float(self.people),
                    self.cur,
                    integer(self.cost),
                    self.cur,
                    integer(self.avg_wage),
                    float(self.overhead),
                )
            }
            SloccountInflation => {
                format!(
                    "\
Total Physical Source Lines of Code (SLOC)                    = {}
Development Effort Estimate, Person-Years (Person-Months)     = {} ({})
  (Basic COCOMO model, Person-Months = {}*(KSLOC**{})*{})
Schedule Estimate, Years (Months)                             = {} ({})
  (Basic COCOMO model, Months = {}*(person-months**{}))
Estimated Average Number of Developers (Effort/Schedule)      = {}
Total Estimated Cost to Develop                               = {}{}
  (average salary {}@{}*1995 = {}{}/year, overhead = {})
\
                    ",
                    integer(self.sloc),
                    float(self.effort / 12.0),
                    float(self.effort),
                    float(self.params.0),
                    float(self.params.1),
                    float(self.eaf),
                    float(self.months / 12.0),
                    float(self.months),
                    float(self.dev_time),
                    float(self.params.2),
                    float(self.people),
                    self.cur,
                    integer(self.cost),
                    if let Some(year) = self.inflation_year {
                        format!("{year}")
                    } else {
                        String::new()
                    },
                    float(self.inflation_multiplier),
                    self.cur,
                    integer(self.avg_wage),
                    float(self.overhead),
                )
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------

/**
Calculate total source lines of code (SLOC) via tokei
*/
pub fn total_sloc(paths: &[PathBuf]) -> f64 {
    let config = tokei::Config::default();
    let mut languages = tokei::Languages::new();
    languages.get_statistics(paths, &[], &config);
    let sum = languages.total();
    sum.code as f64
}

/**
Calculate COCOMO cost estimate
*/
pub fn estimate_cost(effort: f64, avg_wage: f64, overhead: f64) -> f64 {
    effort * avg_wage / 12.0 * overhead
}

/**
Calculate COCOMO effort estimate
*/
pub fn estimate_effort(sloc: f64, eaf: f64, params: &(f64, f64, f64)) -> f64 {
    params.0 * (sloc / 1000.0).powf(params.1) * eaf
}

/**
Calculate COCOMO time estimate in months
*/
pub fn estimate_months(effort: f64, params: &(f64, f64, f64), dev_time: f64) -> f64 {
    dev_time * effort.powf(params.2)
}

/**
Calculate COCOMO effort, cost, time (months), and people estimates
*/
pub fn cocomo(
    sloc: f64,
    eaf: f64, // Effort Adjustment Factor
    avg_wage: f64,
    overhead: f64,
    params: &(f64, f64, f64),
    dev_time: f64,
) -> (f64, f64, f64, f64) {
    let effort = estimate_effort(sloc, eaf, params);
    let cost = estimate_cost(effort, avg_wage, overhead);
    let months = estimate_months(effort, params, dev_time);
    let people = effort / months;
    (effort, cost, months, people)
}

fn integer(n: f64) -> String {
    NUM.format(",d", n)
}

fn float(n: f64) -> String {
    NUM.format(",.2f", n)
}
