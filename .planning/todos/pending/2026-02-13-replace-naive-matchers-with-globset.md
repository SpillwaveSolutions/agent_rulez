---
created: 2026-02-13T02:50:11.716Z
title: Replace Naive Matchers with globset
area: tooling
files:
  - cch_cli/src/hooks.rs
---

## Problem

The current directory matching in `cch_cli/src/hooks.rs` uses a "simple" `contains` check. This is brittle and does not support standard glob patterns (e.g., `**/tests/*.rs`).

## Solution

Integrate the `globset` crate to provide correct, high-performance glob matching that matches user expectations.
