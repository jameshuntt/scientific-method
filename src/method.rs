// Copyright © 2026 James Hunt. All rights reserved.

use crate::{
    Conclusion, Control, Criterion, CriterionError, CriterionResult, DependentVariable, Hypothesis,
    IndependentVariable, Observations, Outcome, Prediction, Question,
};

/// A complete statement of the scientific method for one experiment.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScientificMethod {
    pub question: Option<Question>,
    pub hypothesis: Hypothesis,
    pub predictions: Vec<Prediction>,
    pub independent_variables: Vec<IndependentVariable>,
    pub dependent_variables: Vec<DependentVariable>,
    pub controls: Vec<Control>,
    pub criteria: Vec<Criterion>,
}

impl ScientificMethod {
    pub fn new(hypothesis: Hypothesis) -> Self {
        Self {
            question: None,
            hypothesis,
            predictions: Vec::new(),
            independent_variables: Vec::new(),
            dependent_variables: Vec::new(),
            controls: Vec::new(),
            criteria: Vec::new(),
        }
    }

    pub fn question(mut self, question: Question) -> Self {
        self.question = Some(question);
        self
    }

    pub fn prediction(mut self, prediction: Prediction) -> Self {
        self.predictions.push(prediction);
        self
    }

    pub fn independent(mut self, variable: IndependentVariable) -> Self {
        self.independent_variables.push(variable);
        self
    }

    pub fn dependent(mut self, variable: DependentVariable) -> Self {
        self.dependent_variables.push(variable);
        self
    }

    pub fn control(mut self, control: Control) -> Self {
        self.controls.push(control);
        self
    }

    pub fn criterion(mut self, criterion: Criterion) -> Self {
        self.criteria.push(criterion);
        self
    }

    /// Evaluate every declared criterion against the supplied observations.
    pub fn evaluate(
        &self,
        observations: &Observations,
    ) -> Result<MethodEvaluation, CriterionError> {
        if self.criteria.is_empty() {
            return Ok(MethodEvaluation {
                criteria: Vec::new(),
                conclusion: Conclusion::new(Outcome::Inconclusive),
            });
        }

        let criteria = self
            .criteria
            .iter()
            .map(|criterion| criterion.evaluate(observations))
            .collect::<Result<Vec<_>, _>>()?;

        let outcome = if criteria.iter().all(|result| result.passed) {
            Outcome::Supported
        } else {
            Outcome::NotSupported
        };

        Ok(MethodEvaluation {
            criteria,
            conclusion: Conclusion::new(outcome),
        })
    }
}

/// The result of applying a scientific method to observations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodEvaluation {
    pub criteria: Vec<CriterionResult>,
    pub conclusion: Conclusion,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Comparator, Measurement, Observation, Operand};

    #[test]
    fn no_criteria_is_inconclusive() {
        let method = ScientificMethod::new(Hypothesis::new("something changes"));
        assert_eq!(
            method
                .evaluate(&Observations::new())
                .unwrap()
                .conclusion
                .outcome,
            Outcome::Inconclusive
        );
    }

    #[test]
    fn all_declared_criteria_must_pass() {
        let method = ScientificMethod::new(Hypothesis::new("candidate is valid"))
            .criterion(Criterion::compare(
                Operand::observation("candidate", "tests_pass"),
                Comparator::Equal,
                Operand::literal(true),
            ))
            .criterion(Criterion::compare(
                Operand::observation("candidate", "checksum_matches"),
                Comparator::Equal,
                Operand::literal(true),
            ));

        let observations = Observations::new()
            .with(Observation::new(
                "candidate",
                Measurement::new("tests_pass", true),
            ))
            .with(Observation::new(
                "candidate",
                Measurement::new("checksum_matches", false),
            ));

        assert_eq!(
            method.evaluate(&observations).unwrap().conclusion.outcome,
            Outcome::NotSupported
        );
    }
}
