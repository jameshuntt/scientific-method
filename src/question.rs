// Copyright © 2026 James Hunt. All rights reserved.

/// The question an experiment is intended to answer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Question {
    pub text: String,
}

impl Question {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}
