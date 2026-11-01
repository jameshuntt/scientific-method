# scientific-method

The parts of the scientific method used to define and evaluate experiments.

`scientific-method` models questions, hypotheses, predictions, independent and dependent variables, controls, observations, measurements, criteria, and conclusions. It does not run experiments or know about repositories, commands, files, or benchmarks.

Criteria are machine-evaluable and can be composed. A method evaluates its declared criteria against observations and concludes `accepted`, `rejected`, or `inconclusive`.

```rust
use scientific_method::{
    Criterion, Direction, Hypothesis, Measurement, Observation, ObservationRef,
    Observations, ScientificMethod,
};

let method = ScientificMethod::new(Hypothesis::new("the candidate lowers latency"))
    .criterion(Criterion::improvement(
        ObservationRef::new("baseline", "latency"),
        ObservationRef::new("candidate", "latency"),
        Direction::LowerIsBetter,
        0.05,
    ));

let observations = Observations::new()
    .with(Observation::new("baseline", Measurement::new("latency", 100.0)))
    .with(Observation::new("candidate", Measurement::new("latency", 90.0)));

let evaluation = method.evaluate(&observations)?;
assert!(evaluation.criteria[0].passed);
# Ok::<(), scientific_method::CriterionError>(())
```

## Serde

Serialization is optional:

```toml
scientific-method = { version = "0.1", features = ["serde"] }
```

The crate keeps serialization separate from execution so experiment frameworks can choose their own file formats and storage.

## Releasing

Bump `version` in `Cargo.toml`, add a dated `## <version> (YYYY-MM-DD)` entry to `CHANGELOG.md`, and push to `main`. CI tests stable and Rust 1.89, runs clippy, and publishes a version that is not already on crates.io through trusted publishing.

## Ownership

Developed and published by James Hunt. Repository: https://github.com/jameshuntt/scientific-method.

## License

MIT OR Apache-2.0
