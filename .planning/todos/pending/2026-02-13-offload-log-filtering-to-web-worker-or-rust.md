---
created: 2026-02-13T02:50:11.716Z
title: Offload Log Filtering to Web Worker or Rust
area: ui
files:
  - rulez_ui/src/stores/logStore.ts
---

## Problem

The `logStore.ts` performs filtering (text search and severity) on the main UI thread. For 100K+ entries, this will cause noticeable UI lag.

## Solution

Move log filtering to a Web Worker or implement a Tauri command to perform the filtering in Rust, leveraging Rust's superior string processing speed.
