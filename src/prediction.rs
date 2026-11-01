// Copyright © 2026 James Hunt. All rights reserved.

/// A concrete expected observation implied by a hypothesis.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prediction {
    pub statement: String,
}

impl Prediction {
    pub fn new(statement: impl Into<String>) -> Self {
        Self {
            statement: statement.into(),
        }
    }
}
