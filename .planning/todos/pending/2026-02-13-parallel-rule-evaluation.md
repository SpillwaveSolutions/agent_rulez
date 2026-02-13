---
created: 2026-02-13T02:50:11.716Z
title: Parallel Rule Evaluation
area: tooling
files:
  - cch_cli/src/hooks.rs
---

## Problem

As the rule set grows, sequential evaluation in `evaluate_rules` may become a bottleneck.

## Solution

Consider using `tokio::join_all` or `rayon` for parallel rule evaluation, especially for rules that involve external script execution (validators).
