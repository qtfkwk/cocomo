use std::fmt;
use std::path::PathBuf;

//--------------------------------------------------------------------------------------------------

/**
Basic COCOMO Params from Boehm
*/
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ProjectType {
    /**
    A software project is said to be an organic type if the team size required is adequately small,
    the problem is well understood and has been solved in the past and also the team members have a
    nominal experience regarding the problem.
    Equivalent to `--custom 2.4,1.05,2.5,0.38`.
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
    Equivalent to `--custom 3.0,1.12,2.5,0.35`.
    */
    SemiDetached,

    /**
    A software project with requiring the highest level of complexity, creativity, and experience
    requirement fall under this category. Such software requires a larger team size than the other
    two models and also the developers need to be sufficiently experienced and creative to develop
    such complex models.
    Equivalent to: `--custom 3.6,1.20,2.5,0.32`.
    */
    Embedded,
}

pub use ProjectType::*;

impl ProjectType {
    /**
    ```
    # use cocomo::*;
    assert_eq!(Organic.to_params(), (2.4, 1.05, 2.5, 0.38));
    assert_eq!(SemiDetached.to_params(), (3.0, 1.12, 2.5, 0.35));
    assert_eq!(Embedded.to_params(), (3.6, 1.20, 2.5, 0.32));
    ```
    */
    pub fn to_params(&self) -> (f64, f64, f64, f64) {
        match self {
            Organic => (2.4, 1.05, 2.5, 0.38),
            SemiDetached => (3.0, 1.12, 2.5, 0.35),
            Embedded => (3.6, 1.20, 2.5, 0.32),
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//--------------------------------------------------------------------------------------------------

/**
COCOMO estimate
*/
pub struct Cocomo {
    pub cur: String,
    pub eaf: f64,
    pub avg_wage: f64,
    pub overhead: f64,
    pub params: (f64, f64, f64, f64),
    pub paths: Vec<PathBuf>,
    pub sloc: f64,
    pub effort: f64,
    pub cost: f64,
    pub months: f64,
    pub people: f64,
}

impl Cocomo {
    /**
    Calculate COCOMO estimates
    */
    pub fn new(
        cur: &str,
        eaf: f64,
        avg_wage: f64,
        overhead: f64,
        params: &(f64, f64, f64, f64),
        paths: &[PathBuf],
    ) -> Cocomo {
        let sloc = total_sloc(paths);
        let (effort, cost, months, people) = cocomo(sloc, eaf, avg_wage, overhead, params);
        Cocomo {
            cur: cur.to_string(),
            eaf,
            avg_wage,
            overhead,
            params: *params,
            paths: paths.to_vec(),
            sloc,
            effort,
            cost,
            months,
            people,
        }
    }

    /**
    Create a report
    */
    pub fn report(&self) -> String {
        format!(
            "\
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | {:.0}
Estimated Cost to Develop  | {}{:.2}
Estimated Schedule Effort  | {:.2} months
Estimated People Required  | {:.2}
\
            ",
            self.sloc, self.cur, self.cost, self.months, self.people,
        )
    }

    /**
    Create a SLOCCount-style report
    */
    pub fn sloccount_report(&self) -> String {
        format!(
            "\
Total Physical Source Lines of Code (SLOC)                    = {:.0}
Development Effort Estimate, Person-Years (Person-Months)     = {:.2} ({:.2})
  (Basic COCOMO model, Person-Months = {:.2}*(KSLOC**{:.2})*{:.2})
Schedule Estimate, Years (Months)                             = {:.2} ({:.2})
  (Basic COCOMO model, Months = {:.2}*(person-months**{:.2}))
Estimated Average Number of Developers (Effort/Schedule)      = {:.2}
Total Estimated Cost to Develop                               = {}{:.0}
  (average salary = {}{:.0}/year, overhead = {:.2})
\
            ",
            self.sloc,
            self.effort / 12.0,
            self.effort,
            self.params.0,
            self.params.1,
            self.eaf,
            self.months / 12.0,
            self.months,
            self.params.2,
            self.params.3,
            self.people,
            self.cur,
            self.cost,
            self.cur,
            self.avg_wage,
            self.overhead,
        )
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
pub fn estimate_effort(sloc: f64, eaf: f64, params: &(f64, f64, f64, f64)) -> f64 {
    params.0 * (sloc / 1000.0).powf(params.1) * eaf
}

/**
Calculate COCOMO time estimate in months
*/
pub fn estimate_months(effort: f64, params: &(f64, f64, f64, f64)) -> f64 {
    params.2 * effort.powf(params.3)
}

/**
Calculate COCOMO effort, cost, time (months), and people estimates
*/
pub fn cocomo(
    sloc: f64,
    eaf: f64, // Effort Adjustment Factor
    avg_wage: f64,
    overhead: f64,
    params: &(f64, f64, f64, f64),
) -> (f64, f64, f64, f64) {
    let effort = estimate_effort(sloc, eaf, params);
    let cost = estimate_cost(effort, avg_wage, overhead);
    let months = estimate_months(effort, params);
    let people = effort / months;
    (effort, cost, months, people)
}
