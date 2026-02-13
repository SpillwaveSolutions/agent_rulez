---
created: 2026-02-13T02:50:11.716Z
title: Implement Regex and Config Caching
area: tooling
files:
  - cch_cli/src/hooks.rs
  - cch_cli/src/config.rs
---

## Problem

Currently, regexes are compiled inside the evaluation loop, and project-level configurations are re-read from disk for every hook event.

## Solution

Add a global LRU cache for compiled regexes and cache the `Config` object, invalidating it only when file-watchers detect changes to the YAML/JSON configuration files.
