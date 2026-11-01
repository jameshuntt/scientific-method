// Copyright © 2026 James Hunt. All rights reserved.

#![forbid(unsafe_code)]

//! Scientific method primitives for defining experiments.
//!
//! This crate models the method. It does not run experiments.

mod conclusion;
mod control;
mod criterion;
mod hypothesis;
mod measurement;
mod method;
mod observation;
mod prediction;
mod question;
mod variable;

pub use conclusion::{Conclusion, Outcome};
pub use control::Control;
pub use criterion::{Comparator, Criterion, CriterionError, CriterionResult, Direction, Operand};
pub use hypothesis::Hypothesis;
pub use measurement::{Measurement, Scalar};
pub use method::{MethodEvaluation, ScientificMethod};
pub use observation::{Observation, ObservationRef, Observations};
pub use prediction::Prediction;
pub use question::Question;
pub use variable::{DependentVariable, IndependentVariable};
