// Copyright © 2026 James Hunt. All rights reserved.

/// A proposition an experiment is intended to test.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hypothesis {
    pub statement: String,
    pub rationale: Option<String>,
}

impl Hypothesis {
    pub fn new(statement: impl Into<String>) -> Self {
        Self {
            statement: statement.into(),
            rationale: None,
        }
    }

    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }
}
