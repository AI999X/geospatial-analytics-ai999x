# geospatial-analytics-ai999x — offline, verified geospatial analytics solver with step-by-step answers

[![tests](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geospatial-analytics-ai999x)](https://pypi.org/project/geospatial-analytics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geospatial-analytics-ai999x)

**[Install](https://pypi.org/project/geospatial-analytics-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/geospatial-analytics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/geospatial-analytics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Geospatial** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

geospatial-analytics-ai999x is an offline, deterministic Python solver that answers geospatial analytics questions with verified results from 99 certified capabilities, gives 615 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**geospatial-analytics-ai999x** is a compact, deterministic geospatial analytics engine for Python. It reads a question in plain English (or a typed call), picks a certified geospatial analytics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified geospatial analytics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 615 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** ellipsoidal geodesy, coordinates and map projections, spatial statistics and geostatistics, remote sensing, terrain analysis, and urban and network analysis.

## Model overview

| Property | Value |
|---|---|
| Package | `geospatial-analytics-ai999x` 1.0.0 (import `geospatial_analytics_ai999x`) |
| Type | Deterministic neuro-symbolic geospatial analytics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 99 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 615 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install geospatial-analytics-ai999x
```

```python
import geospatial_analytics_ai999x as geospatial_analytics

geospatial_analytics.ask("Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84").show()
# -> SOLVED — lat2 = 52.8308, lon2 = 1.16915, azimuth2 = 45.8325  (geo_geodesic_destination)
geospatial_analytics.ask("UTM forward: lat = 51.5, lon = -0.13, ellipsoid = WGS84").show()
# -> SOLVED — easting = 699196, northing = 5.70934e+06, k = 1.00009, gamma = 2.24682  (geo_utm_forward)
geospatial_analytics.ask("Geodesic polygon area: lats = [48.0, 49.5, 47.2], lons = [2.0, 4.5, 6.1], ellipsoid = GRS80").show()
# -> SOLVED — area = 3.36697e+10, perimeter = 850958  (geo_geodesic_polygon_area)
geospatial_analytics.ask("Vegetation indices: red = 0.08, nir = 0.45, blue = 0.04, L = 0.5").show()
# -> SOLVED — NDVI = 0.698113, EVI = 0.567485, SAVI = 0.538835, OSAVI = 0.536232  (geo_vegetation_indices)
geospatial_analytics.solve("geo_morans_i", y=[8, 6, 6, 3, 2, 1], W=[[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]], transform="row").show()
# -> SOLVED — I = 0.65625, EI = -0.2, VI_norm = 0.140952, z_norm = 2.28068  (geo_morans_i)
geospatial_analytics.ask("Who created you?").message
# -> geospatial-analytics-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `geospatial-analytics-ai999x "<question>"`, `geospatial-analytics-ai999x --capabilities`. The outputs above were produced by running this exact code on geospatial-analytics-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

99 certified procedures. A few of them:

- **Procedures:** Albers equal-area conic projection: coordinates and scale factors; Arc length along a parallel of latitude; Area and perimeter of a geodesic polygon on the ellipsoid; Area of a latitude-longitude cell on the ellipsoid; At-sensor brightness temperature from thermal radiance; Auxiliary latitudes: geocentric, reduced, conformal, authalic and rectifying; Azimuthal equidistant projection (ellipsoidal, geodesic); Bivariate Moran's I (spatial cross-correlation); Closest facility for every node of a network; D8 flow accumulation of a small DEM; D8 flow direction of every cell of a small DEM; Decimal degrees to degrees, minutes and seconds; Decode a geohash: cell centre and size; Degrees, minutes and seconds to decimal degrees; Destination along a rhumb line (loxodrome) on the ellipsoid; Discrete Frechet distance between two polylines; Distance-band spatial weights; Douglas-Peucker line simplification; Empirical (Matheron) semivariogram in distance bins; Equirectangular (equidistant cylindrical / plate carree) projection; Fit an exponential or Gaussian variogram model (nugget, partial sill, range); Geary's C spatial autocorrelation with normal and randomization inference; Geocentric (ECEF) to geodetic coordinates; Geodesic destination point on the ellipsoid (direct problem) …

`geospatial_analytics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified geospatial analytics capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (ellipsoidal geodesy, coordinates and map projections, spatial statistics and geostatistics, remote sensing, terrain analysis, and urban and network analysis); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{geospatial_analytics_ai999x_1_0_0,
  title  = {geospatial-analytics-ai999x: an offline, verified geospatial analytics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/geospatial-analytics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/geospatial-analytics-ai999x/discussions or visit https://ai999x.com.

## Support the project

geospatial-analytics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/geospatial-analytics-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/geospatial-analytics-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/geospatial-analytics-ai999x/tree/main/onnx).
