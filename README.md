# Geospatial Analytics (geospatial-analytics-ai999x)

[![tests](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geospatial-analytics-ai999x)](https://pypi.org/project/geospatial-analytics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geospatial-analytics-ai999x)

Geospatial Analytics covers map projections and coordinates, geodesic distance and area, spatial statistics, remote-sensing indices, solar position and terrain, and urban and network analysis: 99 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Geospatial Analytics desktop app: Inverse Albers equal-area conic projection, with its worked example and live scene](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/hero.webp)](https://www.ai999x.com/models/geospatial-analytics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `geospatial-analytics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/geospatial-analytics-ai999x/#try) |
| **Python** 3.10+ | `pip install geospatial-analytics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import geospatial_analytics_ai999x as geospatial_analytics

a = geospatial_analytics.ask(
    "Map scale ground distance: map_distance = 4.2 cm, scale = 50000"
)
print(a.outcome, a.values)
# SOLVED {'ground_distance': 2100.0, 'ground_km': 2.1}

a = geospatial_analytics.ask(
    "Parallel arc length: lat = 60, lon1 = 0, lon2 = 1, ellipsoid = "
    "WGS84"
)
print(a.outcome, a.values)
# SOLVED {'distance': 55800.0015724, 'radius': 3197104.58692}

a = geospatial_analytics.ask(
    "Frechet distance: x1 = [0, 1, 2], y1 = [0, 0, 0], x2 = [0, 2], y2 "
    "= [1, 1]"
)
print(a.outcome, a.values)
# SOLVED {'distance': 1.4142135623730951}

a = geospatial_analytics.ask(
    "Kernel density: x = [0, 1, 2, 5], y = [0, 1, 0, 5], px = 1, py = "
    "0.5, bandwidth = 2, kernel = gaussian"
)
print(a.outcome, a.values)
# SOLVED {'density': 0.0267647992836, 'intensity': 0.107059197135}

a = geospatial_analytics.ask(
    "Brightness temperature: radiance = 10.5, K1 = 774.8853, K2 = "
    "1321.0789"
)
print(a.outcome, a.values)
# SOLVED {'T': 306.173902537}

a = geospatial_analytics.ask(
    "Bivariate Moran's I: x = [8, 6, 6, 3, 2, 1], y = [7, 7, 5, 4, 1, "
    "2], W = [[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, "
    "0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]]"
)
print(a.outcome, a.values)
# SOLVED {'I_xy': 0.891760344576}

a = geospatial_analytics.ask(
    "Map scale from distances: map_distance = 5 cm, ground_distance = "
    "2.5 km"
)
print(a.outcome, a.values)
# SOLVED {'scale': 50000.0, 'rf': 2e-05}

a = geospatial_analytics.ask(
    "ECEF to geodetic: X = 4517590.879 m, Y = 796576.168 m, Z = "
    "4487348.409 m, ellipsoid = WGS84"
)
print(a.outcome, a.values)
# SOLVED {'lat': 44.5599648566, 'lon': 10.0000370024, 'h': 49468.6669929}

a = geospatial_analytics.ask(
    "Geodesic polygon area: lats = [0, 0, 1, 1], lons = [0, 1, 1, 0], "
    "ellipsoid = WGS84"
)
print(a.outcome, a.values)
# SOLVED {'area': 12308778361.5, 'perimeter': 443770.917248}

a = geospatial_analytics.ask(
    "Maximal covering location: edges = [0-1 2; 1-2 3; 0-2 6; 2-3 1; "
    "3-4 4; 4-5 2; 1-5 7], demand = [10, 20, 15, 30, 5, 25], "
    "candidates = [0, 1, 2, 3, 4, 5], p = 2, coverage = 3"
)
print(a.outcome, a.values)
# SOLVED {'covered': 95.0, 'fraction': 0.904761904762}

a = geospatial_analytics.ask(
    "Mean center and standard distance: x = [0, 4, 4, 0], y = [0, 0, "
    "3, 3]"
)
print(a.outcome, a.values)
# SOLVED {'mean_x': 2.0, 'mean_y': 1.5, 'std_distance': 2.5}

a = geospatial_analytics.ask(
    "Ground sample distance: pixel_pitch = 0.0024 mm, focal_length = "
    "8.8 mm, altitude = 120 m, pixels = 5472"
)
print(a.outcome, a.values)
# SOLVED {'gsd': 0.0327272727273, 'swath': 179.083636364}

a = geospatial_analytics.ask(
    "DMS to decimal degrees: degrees = 51, minutes = 28, seconds = 38, "
    "hemisphere = N"
)
print(a.outcome, a.values)
# SOLVED {'decimal': 51.477222222222224, 'radians': 0.898447017559}

a = geospatial_analytics.ask(
    "Geodetic to ECEF: lat = 45, lon = 10, h = 100 m, ellipsoid = "
    "WGS84"
)
print(a.outcome, a.values)
# SOLVED {'X': 4449028.15885, 'Y': 784483.702337, 'Z': 4487419.11954}

a = geospatial_analytics.ask(
    "IDW interpolation: x = [0, 10, 0, 10], y = [0, 0, 10, 10], z = "
    "[1, 2, 3, 4], px = 2, py = 3, power = 2"
)
print(a.outcome, a.values)
# SOLVED {'estimate': 1.65897994017}
```

## Screenshots

[![The Geospatial Analytics desktop app: Inverse Albers equal-area conic projection, with its worked example and live scene](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/hero-dark.webp)](https://www.ai999x.com/models/geospatial-analytics-ai999x/)

[![Arc length along a parallel of latitude: the live scene in the Geospatial Analytics desktop app](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/see-1.webp)](https://www.ai999x.com/models/geospatial-analytics-ai999x/)

*Arc length along a parallel of latitude*

[![D8 flow direction of every cell of a small DEM: the live scene in the Geospatial Analytics desktop app](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/see-2.webp)](https://www.ai999x.com/models/geospatial-analytics-ai999x/)

*D8 flow direction of every cell of a small DEM*

[![Local east-north-up (ENU) to geodetic coordinates: the live scene in the Geospatial Analytics desktop app](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/see-3.webp)](https://www.ai999x.com/models/geospatial-analytics-ai999x/)

*Local east-north-up (ENU) to geodetic coordinates*

Every calculation, with its formula, a worked example and its figures: **[Geospatial Analytics on ai999x.com](https://www.ai999x.com/models/geospatial-analytics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/geospatial-analytics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/geospatial-analytics-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `geospatial-analytics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/geospatial-analytics`) and its data (`rust/models/geospatial_analytics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p geospatial-analytics-ai999x
./target/release/geospatial-analytics --help
cargo test -p geospatial-analytics-ai999x
```
