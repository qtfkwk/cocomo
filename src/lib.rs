use std::fmt;

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
    */
    SemiDetached,

    /**
    A software project with requiring the highest level of complexity, creativity, and experience
    requirement fall under this category. Such software requires a larger team size than the other
    two models and also the developers need to be sufficiently experienced and creative to develop
    such complex models.
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