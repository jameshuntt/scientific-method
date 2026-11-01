// Copyright © 2026 James Hunt. All rights reserved.

/// A variable deliberately changed by an experiment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndependentVariable {
    pub name: String,
    pub description: Option<String>,
}

impl IndependentVariable {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// A variable observed in response to an experimental change.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DependentVariable {
    pub name: String,
    pub unit: Option<String>,
    pub description: Option<String>,
}

impl DependentVariable {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            unit: None,
            description: None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}
