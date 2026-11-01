// Copyright © 2026 James Hunt. All rights reserved.

use crate::Measurement;

/// The address of one measurement under one experimental condition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObservationRef {
    pub condition: String,
    pub measurement: String,
}

impl ObservationRef {
    pub fn new(condition: impl Into<String>, measurement: impl Into<String>) -> Self {
        Self {
            condition: condition.into(),
            measurement: measurement.into(),
        }
    }
}

/// A measurement made under a named condition such as `baseline` or `candidate`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Observation {
    pub condition: String,
    pub measurement: Measurement,
}

impl Observation {
    pub fn new(condition: impl Into<String>, measurement: Measurement) -> Self {
        Self {
            condition: condition.into(),
            measurement,
        }
    }

    pub fn reference(&self) -> ObservationRef {
        ObservationRef::new(&self.condition, &self.measurement.name)
    }
}

/// The observations available when criteria are evaluated.
///
/// A vector is used intentionally so the public representation serializes
/// cleanly in JSON, TOML, YAML, and other formats without special map-key rules.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Observations(Vec<Observation>);

impl Observations {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, observation: Observation) -> Option<Observation> {
        let reference = observation.reference();
        if let Some(index) = self
            .0
            .iter()
            .position(|existing| existing.reference() == reference)
        {
            Some(std::mem::replace(&mut self.0[index], observation))
        } else {
            self.0.push(observation);
            None
        }
    }

    pub fn with(mut self, observation: Observation) -> Self {
        self.insert(observation);
        self
    }

    pub fn get(&self, reference: &ObservationRef) -> Option<&Measurement> {
        self.0
            .iter()
            .find(|observation| observation.reference() == *reference)
            .map(|observation| &observation.measurement)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Observation> {
        self.0.iter()
    }
}
