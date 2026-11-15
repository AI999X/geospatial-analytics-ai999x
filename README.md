# Geospatial Analytics (geospatial-analytics-ai999x)

[![tests](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geospatial-analytics-ai999x)](https://pypi.org/project/geospatial-analytics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geospatial-analytics-ai999x)

Geospatial Analytics covers map projections and coordinates, geodesic distance and area, spatial statistics, remote-sensing indices, solar position and terrain, and urban and network analysis: 99 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Geospatial Analytics desktop app: Inverse Albers equal-area conic projection, with its worked example and live scene](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/hero.webp)](https://www.ai999x.com/models/geospatial-analytics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `geospatial-analytics` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/geospatial-analytics-ai999x/#try) |
| **Python** 3.10+ | `pip install geospatial-analytics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import geospatial_analytics_ai999x as geospatial_analytics
a = geospatial_analytics.ask("Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84")
print(a.outcome, a.value)
```

Output:

```text
SOLVED 52.8308364668
```

Every calculation, with its formula, a worked example and its figures: **[Geospatial Analytics on ai999x.com](https://www.ai999x.com/models/geospatial-analytics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/geospatial-analytics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/geospatial-analytics-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `geospatial-analytics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/geospatial-analytics`) and its data (`rust/models/geospatial_analytics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p geospatial-analytics-ai999x
./target/release/geospatial-analytics --help
cargo test -p geospatial-analytics-ai999x
```
