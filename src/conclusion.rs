// Copyright © 2026 James Hunt. All rights reserved.

/// The terminal interpretation of an experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Outcome {
    Accepted,
    Rejected,
    Inconclusive,
}

/// A scientific conclusion and the criterion evaluations supporting it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Conclusion {
    pub outcome: Outcome,
    pub summary: Option<String>,
}

impl Conclusion {
    pub fn new(outcome: Outcome) -> Self {
        Self {
            outcome,
            summary: None,
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }
}
