use gp::Float;
use ndarray::{Array1, Array2, ArrayView2};

pub const SQRT_2PI: f64 = 2.5066282746310007;

pub trait ObjFunc: Send + Sync + 'static + Fn(&[f64]) -> f64 {}
impl<T> ObjFunc for T where T: Send + Sync + 'static + Fn(&[f64]) -> f64 {}

#[derive(Debug)]
pub struct OptimResult<F: Float> {
    pub x_opt: Array1<F>,
    pub y_opt: Array1<F>,
}

#[derive(Debug, PartialEq)]
pub enum InfillStrategy {
    EI,
    WB2,
    WB2S,
}

#[derive(Debug, PartialEq)]
pub enum QEiStrategy {
    KrigingBeliever,
    KrigingBelieverLowerBound,
    KrigingBelieverUpperBound,
    ConstantLiarMinimum,
}

pub struct ObjData<F> {
    pub scale_obj: F,
    pub scale_cstr: Array1<F>,
    pub scale_wb2: F,
}

pub trait GroupFunc: Send + Sync + 'static + Fn(&ArrayView2<f64>) -> Array2<f64> {}
impl<T> GroupFunc for T where T: Send + Sync + 'static + Fn(&ArrayView2<f64>) -> Array2<f64> {}

pub enum CstrStatus {
    Respected,
    Violated,
    Active,
}

pub struct Constraint {
    pub value: f64,
    pub status: CstrStatus,
}

pub enum InfillOptimizer {
    Slsqp,
    Cobyla,
}
