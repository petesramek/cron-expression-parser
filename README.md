# cron-expression-parser

A small Rust library for parsing standard 5-field cron expressions and computing the next matching UTC occurrence after a given Unix timestamp.

## Supported cron format

This library supports the standard 5-field cron format:

```text
┌───────────── minute (0-59)
│ ┌───────────── hour (0-23)
│ │ ┌───────────── day of month (1-31)
│ │ │ ┌───────────── month (1-12)
│ │ │ │ ┌───────────── day of week (0-6, 0 = Sunday)
│ │ │ │ │
* * * * *
```

## Supported syntax

The parser currently supports the syntax required by the task specification:

- Literal value: `5`
- Wildcard: `*`
- List of exact values: `1,15,30`
- Range: `1-5`
- Step: `*/15`
- Range + step: `1-30/5`

## Public API

The crate exposes the following public API:

- `parse(expression: &str) -> Result<Cron, Error>`
- `next_occurrence(parsed: &Cron, reference_timestamp: i64) -> Result<i64, Error>`

## Build

Build the library with:

```bash
cargo build
```

Build an optimized release version with:

```bash
cargo build --release
```

## Run tests

Run the full test suite with:

```bash
cargo test
```

## Optional checks

Format the code with:

```bash
cargo fmt
```

Run lints with:

```bash
cargo clippy -- -D warnings
```

## Example usage

```rust
use cron_expression_parser::{next_occurrence, parse};

fn main() -> Result<(), cron_expression_parser::Error> {
  let parsed = parse("*/15 * * * *")?;
  let next = next_occurrence(&parsed, 1_700_000_000)?;

  println!("Next occurrence: {next}");
  Ok(())
}
```

## Behavior

- All matching is performed in **UTC**
- `next_occurrence` returns the first matching timestamp **after** the reference timestamp
- Field ranges are validated during parsing
- Invalid expressions return `Error::InvalidExpression`
- Invalid timestamps return `Error::InvalidTimestamp`
- Calendar-impossible schedules return `Error::NoOccurrenceFound`

## Notes on supported behavior

This implementation intentionally supports the subset of cron syntax required by the task.

### Supported forms

- `*`
- exact values
- comma-separated lists of exact values
- ranges
- wildcard-based steps
- range-based steps

### Not supported in this version

- named days or months (`MON`, `JAN`)
- special strings (`@yearly`, `@hourly`)
- seconds or year fields
- timezone handling
- mixed list/range expressions such as `1,5-10`
- list-based step expressions such as `1,5,10/2`
- exact-value step expressions such as `5/10`

## Range behavior

Ranges are parsed as inclusive bounds:

- `1-5` matches `1`, `2`, `3`, `4`, `5`
- `5-5` is accepted and represents a range containing exactly one value

Reversed ranges such as `10-5` are rejected as invalid.

## Step behavior

Step expressions are interpreted relative to the appropriate anchor:

- `*/15` in the minute field matches `0, 15, 30, 45`
- `*/2` in a one-based field such as month matches values starting from `1`
- `9-17/2` matches `9, 11, 13, 15, 17`

## Evaluator strategy

The evaluator performs a forward minute-by-minute search in UTC until it finds the next matching timestamp.

To avoid unbounded loops for impossible schedules, the search is bounded. If no matching occurrence is found within the configured search window, the evaluator returns:

- `Error::NoOccurrenceFound`

This also covers schedules that are syntactically valid but calendar-impossible, such as:

```text
0 0 31 2 *
```

## Design overview

The implementation is structured around a small internal model:

- `Field` represents a single parsed cron field
- `Cron` represents a full parsed 5-field cron expression
- `next_occurrence` is responsible only for iterating forward through UTC time until a match is found

This keeps parsing, field matching, full-expression matching, and time iteration separated and easy to test.

## Test coverage

The test suite includes coverage for the categories required by the task:

- basic expressions such as:
  - `* * * * *`
  - `0 0 * * *`
  - `30 8 * * 1`
- step and range-step expressions such as:
  - `*/15 * * * *`
  - `0 9-17/2 * * *`
- invalid input handling
- month rollover
- day-of-month overflow
- year rollover
- impossible schedules that produce no occurrence