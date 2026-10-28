# geospatial-analytics-ai999x — offline, verified geospatial analytics solver with step-by-step answers

[![tests](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geospatial-analytics-ai999x)](https://pypi.org/project/geospatial-analytics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geospatial-analytics-ai999x)

**[Install](https://pypi.org/project/geospatial-analytics-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/geospatial-analytics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/geospatial-analytics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.1

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
| Package | `geospatial-analytics-ai999x` 1.0.1 (import `geospatial_analytics_ai999x`) |
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

Command line: `geospatial-analytics-ai999x "<question>"`, `geospatial-analytics-ai999x --capabilities`. The outputs above were produced by running this exact code on geospatial-analytics-ai999x 1.0.1.

## Find the right capability, and draw it

`geospatial_analytics.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "geospatial-analytics-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import geospatial_analytics_ai999x as geospatial_analytics

geospatial_analytics.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `geospatial-analytics-ai999x`; the answer is the one it returned, with the steps it took.

**Geodesic destination point on the ellipsoid (direct problem)** · _Ellipsoidal geodesy_

> Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84

→ **lat2 = 52.83083647, lon2 = 1.169145951, azimuth2 = 45.83253655**

<details><summary>Steps</summary>

- Geodesic destination point on the ellipsoid (direct problem): lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84 → lat2 = 52.83083647, lon2 = 1.169145951, azimuth2 = 45.83253655

</details>

**Latitude and longitude to UTM coordinates (with scale factor and convergence)** · _Map projections_

> UTM forward: lat = 51.5, lon = -0.13, ellipsoid = WGS84

→ **easting = 699195.8759 m, northing = 5709335.019 m, k = 1.000087189, gamma = 2.246819165, zone = 30, hemisphere = N**

<details><summary>Steps</summary>

- Latitude and longitude to UTM coordinates (with scale factor and convergence): lat = 51.5, lon = -0.13, ellipsoid = WGS84 → easting = 699195.8759 m, northing = 5709335.019 m, k = 1.000087189 m, gamma = 2.246819165 m, zone = 30 m, hemisphere = N m

</details>

**Area and perimeter of a geodesic polygon on the ellipsoid** · _Ellipsoidal geodesy_

> Geodesic polygon area: lats = [48.0, 49.5, 47.2], lons = [2.0, 4.5, 6.1], ellipsoid = GRS80

→ **area = 3.36697348e+10 m^2, perimeter = 850957.6213 m**

<details><summary>Steps</summary>

- Area and perimeter of a geodesic polygon on the ellipsoid: lats = [48, 49.5, 47.2], lons = [2, 4.5, 6.1], ellipsoid = GRS80 → area = 3.36697348e+10 m^2, perimeter = 850957.6213 m

</details>

**Vegetation indices: NDVI, EVI, SAVI, OSAVI, MSAVI and EVI2 from band reflectances** · _Remote sensing_

> Vegetation indices: red = 0.08, nir = 0.45, blue = 0.04, L = 0.5

→ **NDVI = 0.6981132075, EVI = 0.5674846626, SAVI = 0.5388349515, OSAVI = 0.5362318841, MSAVI = 0.5468871126, EVI2 = 0.5633373934**

<details><summary>Steps</summary>

- Vegetation indices: NDVI, EVI, SAVI, OSAVI, MSAVI and EVI2 from band reflectances: red = 0.08, nir = 0.45, blue = 0.04, L = 0.5 → NDVI = 0.6981132075, EVI = 0.5674846626, SAVI = 0.5388349515, OSAVI = 0.5362318841, MSAVI = 0.5468871126, EVI2 = 0.5633373934

</details>

**Global Moran's I spatial autocorrelation with normal and randomization inference** · _Spatial statistics_

`m.solve("geo_morans_i", y=[8, 6, 6, 3, 2, 1], W=[[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]], transform="row")`

→ **I = 0.65625, EI = -0.2, VI_norm = 0.140952381, z_norm = 2.280680117, p_norm = 0.02256738258, VI_rand = 0.1817926233, z_rand = 2.008225402, p_rand = 0.04461934483**

<details><summary>Steps</summary>

- Global Moran's I spatial autocorrelation with normal and randomization inference: y = [8, 6, 6, 3, 2, 1], W = [0 1 0 0 0 0; 1 0 1 0 0 0; 0 1 0 1 0 0; 0 0 1 0 1 0; 0 0 0 1 0 1; 0 0 0 0 1 0], transform = row → I = 0.65625, EI = -0.2, VI_norm = 0.140952381, z_norm = 2.280680117, p_norm = 0.02256738258, VI_rand = 0.1817926233, z_rand = 2.008225402, p_rand = 0.04461934483

</details>

**Map scale (representative fraction) from a map and a ground distance** · _Cartography_

> Map scale from distances: map_distance = 5 cm, ground_distance = 2.5 km

→ **scale = 50000, rf = 2e-05**

<details><summary>Steps</summary>

- Map scale (representative fraction) from a map and a ground distance: map_distance = 5 cm, ground_distance = 2.5 km → scale = 50000, rf = 2e-05

</details>

## Figures

Every point was computed by `geospatial-analytics-ai999x`.

**Great circle against rhumb line, London to New York**

![Great circle against rhumb line, London to New York — computed by geospatial-analytics-ai999x](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/figures/geo-geodesic-destination-map.svg)

_London to New York on a Mercator map (WGS84): the geodesic leaves at azimuth 288.373° and is 5585.81 km long; the rhumb line keeps bearing 258.088° and is 5810.37 km. Each path is 40 destination points computed by the package and projected by it; the rhumb line is straight on this map. Marked: the geodesic midpoint, latitude 52.385°, longitude -41.2786°._

**World graticule in the Mollweide equal-area projection**

![World graticule in the Mollweide equal-area projection — computed by geospatial-analytics-ai999x](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/figures/geo-mollweide-forward-projection.svg)

_The world graticule in the Mollweide equal-area projection (sphere of radius 6371 km): every parallel and meridian is a chain of points projected by the package (576 points); marked: latitude 45°, longitude 90° → x = 7261.19 km, y = 5334.27 km._

**The world from London: azimuth and distance**

![The world from London: azimuth and distance — computed by geospatial-analytics-ai999x](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/figures/geo-azimuthal-equidistant-forward-compass.svg)

_Eight cities seen from London as the package computes them on the WGS84 ellipsoid — each arrow points along the initial azimuth, clockwise from north, with length the geodesic distance (an azimuthal equidistant view): New York 5585.81 km at 288.373°; Tokyo 9583.99 km at 31.6564°; Cape Town 9635.07 km at 164.613°; Rio de Janeiro 9251.66 km at 219.388°; Moscow 2509.19 km at 64.2336°; Mumbai 7199.57 km at 90.6929°; Reykjavík 1895.1 km at 326.304°; Nairobi 6805.98 km at 136.642°._

**Mean centre and standard distance of a point pattern**

![Mean centre and standard distance of a point pattern — computed by geospatial-analytics-ai999x](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/figures/geo-mean-center-standard-distance-map.svg)

_A pattern of 11 points with the mean centre and standard distance computed by the package: marked: x = 52.4, y = 49.5982; the circle has radius 40.2158, the standard distance (root-mean-square distance of the points from the centre)._

**Line simplification at two tolerances**

![Line simplification at two tolerances — computed by geospatial-analytics-ai999x](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/figures/geo-douglas-peucker-simplification-shape.svg)

_A 12-vertex line simplified by the Douglas–Peucker algorithm at two tolerances, the kept vertices as returned by the package: tolerance 0.4 keeps 8 vertices (0, 2, 4, 6, 7, 9, 10, 11); tolerance 1 keeps 4 vertices (0, 2, 4, 11)._

**Elevation grid and its flow accumulation**

![Elevation grid and its flow accumulation — computed by geospatial-analytics-ai999x](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/figures/geo-d8-flow-accumulation-heatmap.svg)

_A 4 × 4 elevation grid (left, metres, cell size 10 m) and the D8 flow accumulation the package computes from it (right): each cell shows how many cells drain through it, itself included; the maximum, 16, is the outlet in the lowest corner. Each panel is shaded by its own maximum._

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
@software{geospatial_analytics_ai999x_1_0_1,
  title  = {geospatial-analytics-ai999x: an offline, verified geospatial analytics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.1},
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
