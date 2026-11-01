// Copyright © 2026 James Hunt. All rights reserved.

use crate::{ObservationRef, Observations, Scalar};
use std::fmt;

/// Numeric direction for an improvement or regression criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Direction {
    HigherIsBetter,
    LowerIsBetter,
}

/// A comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Comparator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

/// A literal value or a reference to an observed measurement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Operand {
    Literal { value: Scalar },
    Observation { reference: ObservationRef },
}

impl Operand {
    pub fn literal(value: impl Into<Scalar>) -> Self {
        Self::Literal {
            value: value.into(),
        }
    }

    pub fn observation(condition: impl Into<String>, measurement: impl Into<String>) -> Self {
        Self::Observation {
            reference: ObservationRef::new(condition, measurement),
        }
    }
}

/// A machine-evaluable acceptance rule.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Criterion {
    Compare {
        left: Operand,
        comparator: Comparator,
        right: Operand,
    },
    Improvement {
        baseline: ObservationRef,
        candidate: ObservationRef,
        direction: Direction,
        minimum_fraction: f64,
    },
    RegressionLimit {
        baseline: ObservationRef,
        candidate: ObservationRef,
        direction: Direction,
        maximum_fraction: f64,
    },
    All {
        criteria: Vec<Criterion>,
    },
    Any {
        criteria: Vec<Criterion>,
    },
    Not {
        criterion: Box<Criterion>,
    },
}

impl Criterion {
    pub fn compare(left: Operand, comparator: Comparator, right: Operand) -> Self {
        Self::Compare {
            left,
            comparator,
            right,
        }
    }

    pub fn improvement(
        baseline: ObservationRef,
        candidate: ObservationRef,
        direction: Direction,
        minimum_fraction: f64,
    ) -> Self {
        Self::Improvement {
            baseline,
            candidate,
            direction,
            minimum_fraction,
        }
    }

    pub fn regression_limit(
        baseline: ObservationRef,
        candidate: ObservationRef,
        direction: Direction,
        maximum_fraction: f64,
    ) -> Self {
        Self::RegressionLimit {
            baseline,
            candidate,
            direction,
            maximum_fraction,
        }
    }

    pub fn all(criteria: impl IntoIterator<Item = Criterion>) -> Self {
        Self::All {
            criteria: criteria.into_iter().collect(),
        }
    }

    pub fn any(criteria: impl IntoIterator<Item = Criterion>) -> Self {
        Self::Any {
            criteria: criteria.into_iter().collect(),
        }
    }

    pub fn negate(criterion: Criterion) -> Self {
        Self::Not {
            criterion: Box::new(criterion),
        }
    }

    pub fn evaluate(&self, observations: &Observations) -> Result<CriterionResult, CriterionError> {
        match self {
            Self::Compare {
                left,
                comparator,
                right,
            } => {
                let left = resolve(left, observations)?;
                let right = resolve(right, observations)?;
                let passed = compare(left, *comparator, right)?;
                Ok(CriterionResult::leaf(
                    passed,
                    format!("comparison {:?}", comparator),
                ))
            }
            Self::Improvement {
                baseline,
                candidate,
                direction,
                minimum_fraction,
            } => {
                require_fraction("minimum_fraction", *minimum_fraction)?;
                let (before, after) = numeric_pair(baseline, candidate, observations)?;
                let fraction = improvement_fraction(before, after, *direction)?;
                Ok(CriterionResult::leaf(
                    fraction >= *minimum_fraction,
                    format!("improvement {fraction:.6} >= {minimum_fraction:.6}"),
                ))
            }
            Self::RegressionLimit {
                baseline,
                candidate,
                direction,
                maximum_fraction,
            } => {
                require_fraction("maximum_fraction", *maximum_fraction)?;
                let (before, after) = numeric_pair(baseline, candidate, observations)?;
                let improvement = improvement_fraction(before, after, *direction)?;
                let regression = (-improvement).max(0.0);
                Ok(CriterionResult::leaf(
                    regression <= *maximum_fraction,
                    format!("regression {regression:.6} <= {maximum_fraction:.6}"),
                ))
            }
            Self::All { criteria } => {
                if criteria.is_empty() {
                    return Err(CriterionError::EmptyComposite("all"));
                }
                let children = criteria
                    .iter()
                    .map(|c| c.evaluate(observations))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(CriterionResult::branch(
                    children.iter().all(|r| r.passed),
                    "all",
                    children,
                ))
            }
            Self::Any { criteria } => {
                if criteria.is_empty() {
                    return Err(CriterionError::EmptyComposite("any"));
                }
                let children = criteria
                    .iter()
                    .map(|c| c.evaluate(observations))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(CriterionResult::branch(
                    children.iter().any(|r| r.passed),
                    "any",
                    children,
                ))
            }
            Self::Not { criterion } => {
                let child = criterion.evaluate(observations)?;
                Ok(CriterionResult::branch(!child.passed, "not", vec![child]))
            }
        }
    }
}

/// The structured result of evaluating a criterion.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CriterionResult {
    pub passed: bool,
    pub detail: String,
    pub children: Vec<CriterionResult>,
}

impl CriterionResult {
    fn leaf(passed: bool, detail: impl Into<String>) -> Self {
        Self {
            passed,
            detail: detail.into(),
            children: Vec::new(),
        }
    }

    fn branch(passed: bool, detail: impl Into<String>, children: Vec<CriterionResult>) -> Self {
        Self {
            passed,
            detail: detail.into(),
            children,
        }
    }
}

/// Why a criterion could not be evaluated.
#[derive(Debug, Clone, PartialEq)]
pub enum CriterionError {
    MissingObservation(ObservationRef),
    NonNumericObservation(ObservationRef),
    NonFiniteNumber,
    UndefinedRelativeChange,
    InvalidFraction { name: &'static str, value: f64 },
    IncompatibleOperands,
    UnsupportedOrdering,
    EmptyComposite(&'static str),
}

impl fmt::Display for CriterionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingObservation(r) => {
                write!(f, "missing observation {}:{}", r.condition, r.measurement)
            }
            Self::NonNumericObservation(r) => write!(
                f,
                "observation {}:{} is not numeric",
                r.condition, r.measurement
            ),
            Self::NonFiniteNumber => write!(f, "a numeric value is not finite"),
            Self::UndefinedRelativeChange => {
                write!(f, "relative change is undefined for a zero baseline")
            }
            Self::InvalidFraction { name, value } => {
                write!(f, "{name} must be finite and non-negative, got {value}")
            }
            Self::IncompatibleOperands => write!(f, "the operands have incompatible scalar types"),
            Self::UnsupportedOrdering => write!(f, "ordering comparisons require numeric operands"),
            Self::EmptyComposite(kind) => {
                write!(f, "{kind} criterion must contain at least one criterion")
            }
        }
    }
}

impl std::error::Error for CriterionError {}

fn require_fraction(name: &'static str, value: f64) -> Result<(), CriterionError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(CriterionError::InvalidFraction { name, value })
    }
}

fn resolve<'a>(
    operand: &'a Operand,
    observations: &'a Observations,
) -> Result<&'a Scalar, CriterionError> {
    match operand {
        Operand::Literal { value } => Ok(value),
        Operand::Observation { reference } => observations
            .get(reference)
            .map(|m| &m.value)
            .ok_or_else(|| CriterionError::MissingObservation(reference.clone())),
    }
}

fn numeric_pair(
    baseline: &ObservationRef,
    candidate: &ObservationRef,
    observations: &Observations,
) -> Result<(f64, f64), CriterionError> {
    let a = observations
        .get(baseline)
        .ok_or_else(|| CriterionError::MissingObservation(baseline.clone()))?;
    let b = observations
        .get(candidate)
        .ok_or_else(|| CriterionError::MissingObservation(candidate.clone()))?;
    let before = match &a.value {
        Scalar::Number(v) if v.is_finite() => *v,
        Scalar::Number(_) => return Err(CriterionError::NonFiniteNumber),
        _ => return Err(CriterionError::NonNumericObservation(baseline.clone())),
    };
    let after = match &b.value {
        Scalar::Number(v) if v.is_finite() => *v,
        Scalar::Number(_) => return Err(CriterionError::NonFiniteNumber),
        _ => return Err(CriterionError::NonNumericObservation(candidate.clone())),
    };
    Ok((before, after))
}

fn compare(left: &Scalar, comparator: Comparator, right: &Scalar) -> Result<bool, CriterionError> {
    use Comparator::*;
    match comparator {
        Equal | NotEqual => {
            let equal = match (left, right) {
                (Scalar::Number(a), Scalar::Number(b)) => {
                    if !a.is_finite() || !b.is_finite() {
                        return Err(CriterionError::NonFiniteNumber);
                    }
                    a == b
                }
                (Scalar::Boolean(a), Scalar::Boolean(b)) => a == b,
                (Scalar::Text(a), Scalar::Text(b)) => a == b,
                _ => return Err(CriterionError::IncompatibleOperands),
            };
            Ok(if comparator == Equal { equal } else { !equal })
        }
        GreaterThan | GreaterThanOrEqual | LessThan | LessThanOrEqual => {
            let (a, b) = match (left, right) {
                (Scalar::Number(a), Scalar::Number(b)) => {
                    if !a.is_finite() || !b.is_finite() {
                        return Err(CriterionError::NonFiniteNumber);
                    }
                    (*a, *b)
                }
                (Scalar::Number(_), _) | (_, Scalar::Number(_)) => {
                    return Err(CriterionError::IncompatibleOperands);
                }
                _ => return Err(CriterionError::UnsupportedOrdering),
            };
            Ok(match comparator {
                GreaterThan => a > b,
                GreaterThanOrEqual => a >= b,
                LessThan => a < b,
                LessThanOrEqual => a <= b,
                Equal | NotEqual => unreachable!(),
            })
        }
    }
}

fn improvement_fraction(
    before: f64,
    after: f64,
    direction: Direction,
) -> Result<f64, CriterionError> {
    if before == 0.0 {
        return Err(CriterionError::UndefinedRelativeChange);
    }
    let magnitude = before.abs();
    Ok(match direction {
        Direction::HigherIsBetter => (after - before) / magnitude,
        Direction::LowerIsBetter => (before - after) / magnitude,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Measurement, Observation};

    fn observations(before: f64, after: f64) -> Observations {
        Observations::new()
            .with(Observation::new(
                "baseline",
                Measurement::new("latency", before),
            ))
            .with(Observation::new(
                "candidate",
                Measurement::new("latency", after),
            ))
    }

    #[test]
    fn lower_is_better_improvement_passes() {
        let c = Criterion::improvement(
            ObservationRef::new("baseline", "latency"),
            ObservationRef::new("candidate", "latency"),
            Direction::LowerIsBetter,
            0.05,
        );
        assert!(c.evaluate(&observations(100.0, 90.0)).unwrap().passed);
    }

    #[test]
    fn lower_is_better_improvement_fails() {
        let c = Criterion::improvement(
            ObservationRef::new("baseline", "latency"),
            ObservationRef::new("candidate", "latency"),
            Direction::LowerIsBetter,
            0.05,
        );
        assert!(!c.evaluate(&observations(100.0, 97.0)).unwrap().passed);
    }

    #[test]
    fn regression_limit_passes_on_improvement() {
        let c = Criterion::regression_limit(
            ObservationRef::new("baseline", "latency"),
            ObservationRef::new("candidate", "latency"),
            Direction::LowerIsBetter,
            0.02,
        );
        assert!(c.evaluate(&observations(100.0, 90.0)).unwrap().passed);
    }

    #[test]
    fn regression_limit_catches_regression() {
        let c = Criterion::regression_limit(
            ObservationRef::new("baseline", "latency"),
            ObservationRef::new("candidate", "latency"),
            Direction::LowerIsBetter,
            0.02,
        );
        assert!(!c.evaluate(&observations(100.0, 103.0)).unwrap().passed);
    }

    #[test]
    fn all_composes() {
        let o = observations(100.0, 90.0).with(Observation::new(
            "candidate",
            Measurement::new("tests_pass", true),
        ));
        let c = Criterion::all([
            Criterion::improvement(
                ObservationRef::new("baseline", "latency"),
                ObservationRef::new("candidate", "latency"),
                Direction::LowerIsBetter,
                0.05,
            ),
            Criterion::compare(
                Operand::observation("candidate", "tests_pass"),
                Comparator::Equal,
                Operand::literal(true),
            ),
        ]);
        assert!(c.evaluate(&o).unwrap().passed);
    }

    #[test]
    fn missing_observation_is_error() {
        let c = Criterion::compare(
            Operand::observation("candidate", "missing"),
            Comparator::Equal,
            Operand::literal(true),
        );
        assert!(matches!(
            c.evaluate(&Observations::new()),
            Err(CriterionError::MissingObservation(_))
        ));
    }

    #[test]
    fn zero_baseline_refuses_relative_change() {
        let c = Criterion::improvement(
            ObservationRef::new("baseline", "latency"),
            ObservationRef::new("candidate", "latency"),
            Direction::LowerIsBetter,
            0.05,
        );
        assert_eq!(
            c.evaluate(&observations(0.0, 1.0)).unwrap_err(),
            CriterionError::UndefinedRelativeChange
        );
    }
}
