# geospatial-analytics-ai999x — offline, verified geospatial analytics solver with step-by-step answers

[![tests](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geospatial-analytics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geospatial-analytics-ai999x)](https://pypi.org/project/geospatial-analytics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geospatial-analytics-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/geospatial-analytics-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/geospatial-analytics-ai999x/)** · **[Guides](https://huggingface.co/AI999X/geospatial-analytics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/geospatial-analytics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Geospatial** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

geospatial-analytics-ai999x is an offline, deterministic Python solver that answers geospatial analytics questions with verified results from 99 certified capabilities, gives 615 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**geospatial-analytics-ai999x** is a compact, deterministic geospatial analytics engine for Python. It reads a question in plain English (or a typed call), picks a certified geospatial analytics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** geospatial-analytics-ai999x comes as the AI999X desktop app and as the command-line program `geospatial-analytics` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Geospatial Analytics desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-geospatial-analytics-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-geospatial-analytics-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`geospatial-analytics`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/geospatial-analytics-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install geospatial-analytics-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Geospatial Analytics also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.3](https://github.com/AI999X/geospatial-analytics-ai999x/releases/tag/v1.0.3) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Geospatial Analytics) | Windows x86-64 | `ai999x-geospatial-analytics-1.2.0-windows-x86_64.zip` | 8.1 MB | [Hub](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/rust/ai999x-geospatial-analytics-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/geospatial-analytics-ai999x/releases/download/v1.0.3/ai999x-geospatial-analytics-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-geospatial-analytics-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-geospatial-analytics.exe` (no installer) |
| Desktop app (Geospatial Analytics) | Linux x86-64 | `ai999x-geospatial-analytics-1.2.0-linux-x86_64.tar.gz` | 9.0 MB | [Hub](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/rust/ai999x-geospatial-analytics-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/geospatial-analytics-ai999x/releases/download/v1.0.3/ai999x-geospatial-analytics-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-geospatial-analytics-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-geospatial-analytics` (X11 or Wayland desktop) |
| Command line (`geospatial-analytics`) | Windows x86-64 | `ai999x-geospatial-analytics-cli-1.2.0-windows-x86_64.zip` | 3.6 MB | [Hub](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/rust/ai999x-geospatial-analytics-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/geospatial-analytics-ai999x/releases/download/v1.0.3/ai999x-geospatial-analytics-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `geospatial-analytics.exe ask "<question>"` or `geospatial-analytics.exe solve <capability> name=value` |
| Command line (`geospatial-analytics`) | Linux x86-64 | `ai999x-geospatial-analytics-cli-1.2.0-linux-x86_64.tar.gz` | 3.5 MB | [Hub](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/rust/ai999x-geospatial-analytics-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/geospatial-analytics-ai999x/releases/download/v1.0.3/ai999x-geospatial-analytics-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./geospatial-analytics ask "<question>"` or `./geospatial-analytics solve <capability> name=value` |

The command-line archive holds `geospatial-analytics` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/geospatial-analytics-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Geospatial Analytics** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Geospatial Analytics among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-geospatial-analytics-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-geospatial-analytics-1.2.0-linux-x86_64.tar.gz
./ai999x-geospatial-analytics-1.2.0-linux-x86_64/ai999x-geospatial-analytics
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
geospatial-analytics --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
geospatial-analytics --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ geospatial-analytics ask "Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84"
Read as: Geodesic destination point on the ellipsoid (direct problem)
lat2 = 52.83083647
lon2 = 1.169145951
azimuth2 = 45.83253655
  by: Geodesic destination point on the ellipsoid (direct problem)  (Bessel's auxiliary sphere: solve b*I1(sigma) = s by Newton steps with the arc-length integral by Gauss-Legendre quadrature, then the longitude integral)
  azimuth = 45
  distance = 100000 m
  lat1 = 52.2
  lon1 = 0.12
  ellipsoid = WGS84
  assumes: start latitude strictly between -90 and 90; 0 < distance <= 15000 km
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ geospatial-analytics info geo_geodesic_destination
Geodesic destination point on the ellipsoid (direct problem)
  Bessel's auxiliary sphere: solve b*I1(sigma) = s by Newton steps with the arc-length integral by Gauss-Legendre quadrature, then the longitude integral
  in   lat1     latitude of the start point (decimal degrees) []
  in   lon1     longitude of the start point (decimal degrees) []
  in   azimuth  initial azimuth, clockwise from north (decimal degrees) []
  in   distance geodesic distance travelled [m]
  in   ellipsoid reference ellipsoid (choice) []
  out  lat2     latitude of the destination (degrees) []
  out  lon2     longitude of the destination (degrees) []
  out  azimuth2 forward azimuth at the destination (degrees clockwise from north) []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ geospatial-analytics solve geo_geodesic_destination lat1=52.2 lon1=0.12 azimuth=45.0 "distance=100 km" ellipsoid=WGS84
lat2 = 52.83083647
lon2 = 1.169145951
azimuth2 = 45.83253655
  by: Geodesic destination point on the ellipsoid (direct problem)  (Bessel's auxiliary sphere: solve b*I1(sigma) = s by Newton steps with the arc-length integral by Gauss-Legendre quadrature, then the longitude integral)
  lat1 = 52.2
  lon1 = 0.12
  azimuth = 45
  distance = 100000 m
  ellipsoid = WGS84
  assumes: start latitude strictly between -90 and 90; 0 < distance <= 15000 km
```

**Draw the figures** — The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```console
$ geospatial-analytics figures geo_geodesic_destination lat1=52.2 lon1=0.12 azimuth=45.0 "distance=100 km" ellipsoid=WGS84 --out .
[
  {
    "slug": "geo-geodesic-destination-live-1",
    "title": "Latitude of the destination as latitude of the start point (decimal degrees) varies",
    "caption": "Latitude of the destination as lat1 runs from 13.05 to 89.39, every other input as you entered it (40 solutions). Marked: your lat1 = 52.2 gives lat2 = 52.83. Your inputs: lat1 = 52.2, lon1 = 0.12, azimuth = 45.0, distance = 100 km, ellipsoid = WGS84.",
    "file": "./geo-geodesic-destination-live-1.svg"
  },
  {
    "slug": "geo-geodesic-destination-live-2",
    "title": "Latitude of the destination as initial azimuth, clockwise from north (decimal... varies",
    "caption": "Latitude of the destination as azimuth runs from 11.25 to 78.75, every other input as you entered it (29 solutions). Marked: your azimuth = 45 gives lat2 = 52.83. Your inputs: lat1 = 52.2, lon1 = 0.12, azimuth = 45.0, distance = 100 km, ellipsoid = WGS84.",
    "file": "./geo-geodesic-destination-live-2.svg"
  }
]
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | geospatial-analytics batch

`cases.jsonl`:

```json
{"key": "geo_geodesic_destination", "inputs": {"lat1": 52.2, "lon1": 0.12, "azimuth": 45.0, "distance": "100 km", "ellipsoid": "WGS84"}}
{"key": "geo_utm_forward", "inputs": {"lat": 51.5, "lon": -0.13, "ellipsoid": "WGS84"}}
```

```console
$ geospatial-analytics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":52.8308364668,"unit":"","solved_for":"lat2","si":[["lat1",52.2],["lon1",0.12],["azimuth",45.0],["distance",100000.0]],"values":[["lat2",52.8308364668,""],["lon2",1.16914595053,""],["azimuth2",45.832536554,""]],"target":"lat2","phrase":"latitude of the destination (degrees)","assumptions":["start latitude strictly between -90 and 90","0 < distance <= 15000 km"],"name":"Geodesic destination point on the ellipsoid (direct problem)","equation":"Bessel's auxiliary sphere: solve b*I1(sigma) = s by Newton steps with the arc-length integral by Gauss-Legendre quadrature, then the longitude integral"}
{"outcome":"SOLVED","message":"","value":699195.875854,"unit":"m","solved_for":"easting","si":[["lat",51.5],["lon",-0.13]],"values":[["easting",699195.875854,"m"],["northing",5709335.01926,"m"],["k",1.00008718886,""],["gamma",2.24681916539,""],["zone",30,""],["hemisphere","N",""]],"target":"easting","phrase":"UTM easting","assumptions":["80 S <= latitude <= 84 N"],"name":"Latitude and longitude to UTM coordinates (with scale factor and convergence)","equation":"zone from the longitude (Norway and Svalbard exceptions), then transverse Mercator by Krueger's series with k0 = 0.9996, false easting 500 km, false northing 10000 km in the south"}
```

### In your browser

Open [Geospatial Analytics on ai999x.com](https://www.ai999x.com/models/geospatial-analytics-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `geospatial_analytics_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install geospatial-analytics-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Ground distance from a map distance and a representative fraction

The real-world distance represented by a distance measured on a map of scale 1 : scale.

Command line:

```console
$ geospatial-analytics solve geo_map_scale_ground_distance "map_distance=4.2 cm" scale=50000
ground_distance = 2100 m
ground_km = 2.1
  by: Ground distance from a map distance and a representative fraction  (ground distance = map distance x scale denominator)
  map_distance = 0.042 m
  scale = 50000
  assumes: positive map distance and scale
```

Python:

```python
import geospatial_analytics_ai999x as m

a = m.solve("geo_map_scale_ground_distance", map_distance="4.2 cm", scale="50000")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geospatial-analytics-ai999x/#cap-geo_map_scale_ground_distance) and press **Compute in your browser**.

_What this means:_ This calculation determines the real-world distance represented by a measurement taken on a map. Multiplying the measured map distance of 4.2 cm by the scale denominator of 50000 shows that the actual ground distance is 2100 m. This result can also be expressed as 2.1 for easier interpretation in kilometers.

### Map scale (representative fraction) from a map and a ground distance

The scale denominator and representative fraction of a map on which a known ground distance measures a given map distance.

Command line:

```console
$ geospatial-analytics solve geo_map_scale_from_distances "map_distance=5 cm" "ground_distance=2.5 km"
scale = 50000
rf = 2e-05
  by: Map scale (representative fraction) from a map and a ground distance  (S = ground distance / map distance in the same unit)
  map_distance = 0.05 m
  ground_distance = 2500 m
  assumes: ground distance > map distance > 0
```

Python:

```python
import geospatial_analytics_ai999x as m

a = m.solve("geo_map_scale_from_distances", map_distance="5 cm", ground_distance="2.5 km")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geospatial-analytics-ai999x/#cap-geo_map_scale_from_distances) and press **Compute in your browser**.

_What this means:_ This calculation determines the map's scale by comparing the measured distance on the map to the corresponding actual ground distance. Given a map distance of 5 cm and a ground distance of 2.5 km, the resulting scale denominator is 50000. This indicates that one unit on the map represents 50000 units on the ground, expressed as a representative fraction of 2e-05.

### Decimal degrees to degrees, minutes and seconds

A signed latitude or longitude in decimal degrees written as whole degrees, whole minutes, seconds and a hemisphere letter.

Command line:

```console
$ geospatial-analytics solve geo_decimal_to_dms decimal=51.477222 axis=latitude
degrees = 51
minutes = 28
seconds = 37.9992
hemisphere = N
  by: Decimal degrees to degrees, minutes and seconds  (exact decimal arithmetic: degrees = floor(|x|), minutes = floor(60 frac), seconds = the remainder times 3600)
  decimal = 51.477222
  axis = latitude
  assumes: |latitude| <= 90, |longitude| <= 180
```

Python:

```python
import geospatial_analytics_ai999x as m

a = m.solve("geo_decimal_to_dms", decimal="51.477222", axis="latitude")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geospatial-analytics-ai999x/#cap-geo_decimal_to_dms) and press **Compute in your browser**.

_What this means:_ This calculation transforms a single signed decimal value representing geographic position into traditional Degrees, Minutes, and Seconds notation. It takes the input angle 51.4772 associated with the latitude coordinate system and decomposes it into whole degrees 51, remaining minutes 28, and fractional seconds 37.9992. The sign of the original value determines the hemisphere letter N, indicating whether the location is North, South, East, or West relative to the equator or prime meridian.

### Decode a geohash: cell centre and size

The centre latitude and longitude of a geohash cell and its half-height and half-width in degrees.

Command line:

```console
$ geospatial-analytics solve geo_geohash_decode geohash=u4pruydqqvj
lat_center = 57.64911063
lon_center = 10.40743969
lat_error = 6.705522537e-07
lon_error = 6.705522537e-07
  by: Decode a geohash: cell centre and size  (each character gives five bits that halve the longitude and latitude intervals alternately (longitude first))
  geohash = u4pruydqqvj
  assumes: 1 to 12 valid characters
```

Python:

```python
import geospatial_analytics_ai999x as m

a = m.solve("geo_geohash_decode", geohash="u4pruydqqvj")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geospatial-analytics-ai999x/#cap-geo_geohash_decode) and press **Compute in your browser**.

_What this means:_ This process decodes the provided geohash string to determine the geographic location it represents. The algorithm iteratively halves the global coordinate intervals based on the binary values of each character, starting with longitude. The final result provides the precise center coordinates as 57.6491 and 10.4074, along with the uncertainty bounds defined by the half-height 6.70552e-07 and half-width 6.70552e-07. These values describe the exact spatial footprint of the cell within the standard geohash grid system.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/geospatial-analytics-ai999x/blob/main/docs/geospatial_analytics_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/geospatial-analytics-ai999x/blob/main/docs/geospatial_analytics_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/geospatial-analytics-ai999x/blob/main/docs/geospatial_analytics_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/geospatial-analytics-ai999x/blob/main/docs/geospatial_analytics_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/geospatial-analytics-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified geospatial analytics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 615 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `geospatial-analytics` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** ellipsoidal geodesy, coordinates and map projections, spatial statistics and geostatistics, remote sensing, terrain analysis, and urban and network analysis.

## Model overview

| Property | Value |
|---|---|
| Package | `geospatial-analytics-ai999x` 1.0.3 (import `geospatial_analytics_ai999x`) |
| Type | Deterministic neuro-symbolic geospatial analytics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 99 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 615 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `geospatial-analytics-ai999x "<question>"`; for the native program use `geospatial-analytics` (above). The outputs above were produced by running this exact code on geospatial-analytics-ai999x 1.0.3.

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

## Python worked examples, with their steps

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
@software{geospatial_analytics_ai999x_1_0_3,
  title  = {geospatial-analytics-ai999x: an offline, verified geospatial analytics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
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
