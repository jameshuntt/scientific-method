// Copyright © 2026 James Hunt. All rights reserved.

/// A scalar value observed by an experiment.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "type", content = "value", rename_all = "snake_case")
)]
pub enum Scalar {
    Number(f64),
    Boolean(bool),
    Text(String),
}

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for Scalar {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for Scalar {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for Scalar {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

/// One named measurement and its optional unit.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Measurement {
    pub name: String,
    pub value: Scalar,
    pub unit: Option<String>,
}

impl Measurement {
    pub fn new(name: impl Into<String>, value: impl Into<Scalar>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            unit: None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}
