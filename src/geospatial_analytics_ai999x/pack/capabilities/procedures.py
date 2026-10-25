"""Geospatial analytics procedures: deterministic capabilities with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics (the geospatial view): ellipsoidal geodesy (direct problem and midpoint of a geodesic, geodesic polygon area
and perimeter, rhumb lines, meridian and parallel arcs, graticule-cell areas, radii of curvature), coordinates
(geodetic, geocentric ECEF and local ENU frames, look angles, auxiliary latitudes, UTM zones, MGRS and geohash cells,
web-map tiles, degrees-minutes-seconds, map scale), map projections (transverse Mercator and UTM with scale factor and
grid convergence, Mercator, Web Mercator, Lambert conformal conic, Albers equal-area, polar stereographic, Lambert
azimuthal equal-area, azimuthal equidistant, sinusoidal, Mollweide, orthographic, gnomonic, equirectangular and
cylindrical equal-area projections, Tissot's indicatrix), planar analysis (line simplification, Frechet distance,
compactness indices), spatial statistics (Moran's I, Geary's C, Getis-Ord G and local Gi, local and bivariate Moran,
join counts, spatial lag and weights, nearest-neighbour index, quadrat test, Ripley's K, centrography, kernel density,
inverse-distance weighting, semivariograms, variogram fits, ordinary kriging, trend surfaces, rank-size and density
gradients, a Poisson gravity model), remote sensing (vegetation and water indices, top-of-atmosphere reflectance,
brightness temperature, ground sample distance, classification accuracy), terrain (Horn slope and aspect,
hillshade, curvature, ruggedness, D8 flow direction and accumulation, watersheds, line-of-sight profiles) and urban
and network analysis (gravity accessibility, Huff model, two-step floating catchment, service areas, closest
facility, p-median and maximal covering location, detour index, connectivity indices, Shimbel index, segregation
indices).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, a reference computation (used only by certification; the checking libraries are imported inside the
reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re, fractions, itertools, heapq) plus mpmath where stated.

How the references differ from the shipped code:
- geodesy: the shipped code solves geodesics on Bessel's auxiliary sphere with the two exact integrals evaluated by
  Gauss-Legendre quadrature; pyproj's Geod (Karney's series algorithms) checks direct, inverse, midpoint, polygon
  area and perimeter; meridian arcs (Helmert's series here) are checked by pyproj geodesics along a meridian and scipy
  quadrature; rhumb lines by scipy quadrature (inverse) and scipy DOP853 integration of the rhumb ODE (direct);
- coordinates: pyproj (cart and topocentric conversions) for ECEF and ENU; auxiliary latitudes from pyproj's ECEF,
  Mercator, cylindrical equal-area projections and meridian geodesics; utm, mgrs, pygeohash and mercantile for zones,
  grid references, geohashes and tiles;
- projections: pyproj (PROJ) forward and inverse projections, and PROJ's own scale factors and meridian convergence
  (Proj.get_factors) for scale, convergence and Tissot parameters;
- planar: shapely (GEOS) for Douglas-Peucker simplification, area, length, convex hull and minimum bounding circle; a
  decision-and-search formulation (reachability on the free-space grid) for the discrete Frechet distance;
- spatial statistics: esda and libpysal (Moran, Geary, G, G_Local, Moran_Local, Moran_BV, Join_Counts, lag_spatial,
  KNN and DistanceBand weights), scipy cKDTree, pointpats (quadrat statistics, Ripley's K, centrography, ellipse),
  scikit-learn (KernelDensity, KNeighborsRegressor), gstools (vario_estimate), scipy curve_fit, pykrige
  (OrdinaryKriging), numpy least squares, scipy linregress, statsmodels Poisson GLM;
- remote sensing: spyndex (the Awesome Spectral Indices catalogue), pvlib (Spencer's Earth-Sun distance factor),
  scikit-learn (accuracy, kappa, recall, precision);
- terrain: a least-squares quadratic surface (numpy) for slope, aspect and the hillshade normal (vector geometry),
  networkx graph ancestors for flow accumulation and watersheds, shapely line-of-sight intersection for profiles;
- networks: networkx Dijkstra distances and components; scipy milp (HiGHS) for the p-median and maximal covering
  location problems; segregation (pysal) for the segregation indices.

Conventions (fixed, so run and reference agree exactly):
- Ellipsoids are named (a required choice): WGS84 (a = 6378137 m, 1/f = 298.257223563) or GRS80 (a = 6378137 m,
  1/f = 298.257222101); these constants are facts of the published datums. Spherical formulas take a stated radius R.
- Latitudes and longitudes are decimal degrees (north and east positive); output longitudes lie in (-180, 180];
  azimuths and bearings are degrees clockwise from true north in [0, 360). Heights are ellipsoidal, in metres.
- Projected coordinates and lists of coordinates are in metres (or in the stated unit of the input); x is easting,
  y is northing. Grid convergence is positive when grid north lies east of true north (east of the central meridian in
  the northern hemisphere). Scale factors are point scale factors.
- Transverse Mercator is Krueger's series to sixth order in n (as PROJ's Poder/Engsager algorithm); UTM uses k0 =
  0.9996, false easting 500 km, false northing 0 (north) or 10000 km (south), and zones with the Norway and Svalbard
  exceptions. MGRS references are truncated to 1 m.
- Rasters are matrices with row 0 to the north and column 0 to the west; cells are square with the stated cell size.
  Horn's method gives slope and aspect (aspect is the downslope direction clockwise from north; a flat window has no
  aspect); D8 directions use the ESRI codes 1 E, 2 SE, 4 S, 8 SW, 16 W, 32 NW, 64 N, 128 NE, distances sqrt(2) on
  diagonals, 0 for a cell with no lower neighbour inside the raster; accumulation counts the cell itself.
- Spatial weights are given as full matrices (zero diagonal); "row" standardizes each row to sum 1. Variogram models
  use pykrige's parameterization (exponential 1 - exp(-3h/a), gaussian 1 - exp(-(7h/4a)^2), spherical), with the
  sill being nugget + partial sill.
- Network nodes are numbered 0 .. n-1 in the edge list ('0-1 4; 1-2 2.5'); edges are undirected.
- Results built from trigonometric, exponential or logarithmic functions are rounded to 12 significant digits
  (identical on every platform and Python version); values that are zero by construction are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84`). Degrees may
be written with or without a degree sign; physical values carry a unit; lists and matrices are written in brackets;
graphs as `edges = [0-1 4; 1-2 2.5]`; choices are words. Any other number, an unknown name, a name given twice or free
prose is refused, never guessed. The reading guard (tools/ask_sweep.py, 2026-10-23):
  1. every stated number is bound: the question is split into 'name = value' pairs and any other text or number
     refuses the whole question;
  2. the capability phrase must open the question and be followed by the colon, so wording that names a different
     capability (destination vs midpoint, geodesic vs rhumb, forward vs inverse projection) never reaches this reader;
  3. a convention is an input (ellipsoid, hemisphere, weights transform, kernel, variogram model, impedance function):
     required, never assumed;
  4. physical values are passed to the runtime with their units, so a value is converted exactly once.
"""
from __future__ import annotations

import heapq
import itertools
import math
import re
from fractions import Fraction

# ================================================================================================ constants
ELLIPSOIDS = {"WGS84": (6378137.0, 1.0 / 298.257223563), "GRS80": (6378137.0, 1.0 / 298.257222101)}
WEB_MERCATOR_A = 6378137.0
DEG = math.pi / 180.0


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)) or a is None or b is None:
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    if not (math.isfinite(a) and math.isfinite(b)):
        return 0.0 if a == b else math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails.
    The recomputation of one input set is remembered (the procedure is deterministic)."""
    memo = {}

    def verify(inputs, outputs):
        key = repr(sorted(inputs.items()))
        if key not in memo:
            try:
                memo[key] = fn(**inputs)
            except Exception:
                memo[key] = None
            if len(memo) > 64:
                memo.pop(next(iter(memo)))
        want = memo[key]
        if want is None:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), f"{name} must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v > 0,
              f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v >= 0,
              f"{name} must be zero or positive")


def _lat(name, v, polar_ok=False):
    _fin(name, v)
    if polar_ok:
        _need(-90 <= v <= 90, f"{name} must lie between -90 and 90 degrees")
    else:
        _need(-90 < v < 90, f"{name} must lie strictly between -90 and 90 degrees")


def _lon(name, v):
    _fin(name, v)
    _need(-540 <= v <= 540, f"{name} must be a longitude in degrees (-540 to 540)")


def _nlon(lon):
    """A longitude in (-180, 180]."""
    x = math.fmod(lon, 360.0)
    if x > 180.0:
        x -= 360.0
    elif x <= -180.0:
        x += 360.0
    return x + 0.0


def _az(a):
    """An azimuth in degrees in [0, 360)."""
    x = math.fmod(a, 360.0)
    if x < 0:
        x += 360.0
    if x >= 360.0:
        x -= 360.0
    return x + 0.0


def _ellipsoid(name):
    _need(name in ELLIPSOIDS, "the ellipsoid must be WGS84 or GRS80")
    a, f = ELLIPSOIDS[name]
    e2 = f * (2 - f)
    return {"a": a, "f": f, "b": a * (1 - f), "e2": e2, "e": math.sqrt(e2), "ep2": e2 / (1 - e2), "n": f / (2 - f)}


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=6):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


def _solve_lin(A, b):
    """Gaussian elimination with partial pivoting; singular -> ValueError."""
    n = len(A)
    M = [list(map(float, row)) + [float(b[i])] for i, row in enumerate(A)]
    scale = max((abs(x) for row in A for x in row), default=1.0) or 1.0
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        _need(abs(M[p][c]) > 1e-13 * scale, "the equations are singular (the data do not determine the parameters)")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f != 0:
                for k in range(c, n + 1):
                    M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for r in reversed(range(n)):
        x[r] = (M[r][n] - math.fsum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


def _ols(X, y):
    """Least squares by the normal equations (X given as a list of columns)."""
    p = len(X)
    A = [[math.fsum(X[i][k] * X[j][k] for k in range(len(y))) for j in range(p)] for i in range(p)]
    b = [math.fsum(X[i][k] * y[k] for k in range(len(y))) for i in range(p)]
    return _solve_lin(A, b)


def _linfit(x, y):
    """Least-squares line y = c0 + c1 x: (c0, c1, R^2)."""
    n = len(x)
    mx, my = math.fsum(x) / n, math.fsum(y) / n
    sxx = math.fsum((a - mx) ** 2 for a in x)
    syy = math.fsum((b - my) ** 2 for b in y)
    sxy = math.fsum((a - mx) * (b - my) for a, b in zip(x, y))
    _need(sxx > 0, "the x values must not all be equal")
    c1 = sxy / sxx
    r2 = sxy * sxy / (sxx * syy) if syy > 0 else 1.0
    return my - c1 * mx, c1, r2


def _norm_cdf(z):
    return 0.5 * math.erfc(-z / math.sqrt(2.0))


def _p_two(z):
    return math.erfc(abs(z) / math.sqrt(2.0))


# ------------------------------------------------------------------ Gauss-Legendre quadrature (shipped)
def _gl_nodes(n):
    xs, ws = [], []
    for i in range(1, n + 1):
        x = math.cos(math.pi * (i - 0.25) / (n + 0.5))
        for _ in range(100):
            p0, p1 = 1.0, x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1.0)
            dx = p1 / dp
            x -= dx
            if abs(dx) < 1e-16:
                break
        p0, p1 = 1.0, x
        for k in range(2, n + 1):
            p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
        dp = n * (x * p1 - p0) / (x * x - 1.0)
        xs.append(x)
        ws.append(2.0 / ((1.0 - x * x) * dp * dp))
    return xs, ws


_GLX, _GLW = _gl_nodes(16)


def _integ(f, x0, x1):
    """Integral of a smooth function from x0 to x1: 16-point Gauss-Legendre on pieces of at most pi/4."""
    if x1 == x0:
        return 0.0
    pieces = max(1, int(math.ceil(abs(x1 - x0) / (math.pi / 4))))
    h = (x1 - x0) / pieces
    terms = []
    for p in range(pieces):
        a = x0 + p * h
        c, r = a + h / 2, h / 2
        for x, w in zip(_GLX, _GLW):
            terms.append(w * r * f(c + r * x))
    return math.fsum(terms)


# ------------------------------------------------------------------ reference helpers (used only by references)
def _quad(f, a, b, **kw):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=500, **kw)
    return float(v)


def _geod(ell):
    import pyproj
    return pyproj.Geod(ellps=ell)


def _tr(src, dst):
    import pyproj
    return pyproj.Transformer.from_crs(pyproj.CRS(src), pyproj.CRS(dst), always_xy=True)


def _ll(ell):
    return f"+proj=longlat +ellps={ell} +no_defs +type=crs"


# ================================================================================================ units
_KU = {"length": "m", "area": "m^2", "number": ""}


def _written_unit(num: str, unit: str, kind: str):
    return f"{num} {unit.strip()}"


_ATOM_SI = {"m": 1.0, "s": 1.0}
_ATOM_FIXED = {"km": 1e3, "cm": 1e-2, "mm": 1e-3, "1": 1.0}


def _si(text):
    """SI value of a written example quantity ('100 km' -> 1e5; '2.5 km^2' -> 2.5e6)."""
    m = re.fullmatch(r"\s*([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?)\s*(.*?)\s*", text)
    v, u = float(m.group(1)), m.group(2)
    mu = re.fullmatch(r"([a-z]+)(?:\^(\d))?", u)
    if not mu:
        raise ValueError(f"unit {u!r} not known to the example reader")
    f = _ATOM_FIXED.get(mu.group(1), _ATOM_SI.get(mu.group(1)))
    if f is None:
        raise ValueError(f"unit {u!r} not known to the example reader")
    return v * f ** int(mu.group(2) or 1)


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_TOKEN = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([0-9A-Za-z]+)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\](?:\s*;\s*\[[^\]]*\])*|\[\[.*\]\])\s*$")
_DEGU = ("°", "deg", "degree", "degrees")


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard (tools/ask_sweep.py, 2026-10-23: question-reader errors in finished models) is built in:
      1. every stated number is bound: the question is split into 'name = value' pairs and any other text or number
         refuses the whole question (a stray number can never be ignored);
      2. the capability phrase must open the question and be followed by the colon, so wording that names a different
         capability never reaches this reader;
      3. a convention is an input (ellipsoid, hemisphere, transform, kernel, model): required, never assumed; a value
         without its unit (a physical quantity) or with a unit on a pure number is refused (degrees may carry a
         degree sign);
      4. physical values are passed to the runtime with their units, so a value is converted exactly once.
    """
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names, by = {}, {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    lowers = {}
    for i in inputs:
        lowers.setdefault(i["symbol"].lower(), []).append(i["symbol"])
    for lo, syms in lowers.items():         # a lower-case spelling only where it names exactly one input
        if len(syms) == 1:
            names.setdefault(lo, syms[0])
    for a, s in (aliases or {}).items():
        names[a] = s
        names.setdefault(a.lower(), s)

    def key_of(word):
        return names.get(word, names.get(word.lower()))

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = key_of(mv.group(1))
                if key is None or key in out or by[key].get("type") not in ("vector", "matrix", "edges"):
                    return None
                text = mv.group(2).strip()
                if by[key]["type"] == "matrix":
                    if text.startswith("[["):
                        text = text[1:-1]
                    rows = re.findall(r"\[([^\]]*)\]", text)
                    text = "; ".join(r.strip() for r in rows)
                elif by[key]["type"] == "edges":
                    text = text[1:-1].strip()
                    if not text or "[" in text:
                        return None
                out[key] = text
                continue
            mw = _WORD.match(seg)
            if mw:
                key = key_of(mw.group(1))
                if key is not None and by[key].get("type") in ("choice", "boolean"):
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip().replace(" ", "_").replace("-", "_")
                    continue
                if key is not None and by[key].get("type") == "text" and _TOKEN.match(seg):
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip()
                    continue
                return None
            mt = _TOKEN.match(seg)
            if mt:
                key = key_of(mt.group(1))
                if key is not None and by[key].get("type") == "text":
                    if key in out:
                        return None
                    out[key] = mt.group(2)
                    continue
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = key_of(ms.group(1))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "matrix", "choice", "boolean", "text", "edges"):
                return None
            kind = spec.get("kind", "number")
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            if kind != "number":
                if not unit:
                    return None
                out[key] = _written_unit(num, unit, kind)
                continue
            if unit:
                if spec.get("deg") and unit.lower() in _DEGU:
                    out[key] = float(num)
                    continue
                return None
            out[key] = float(num)
        return out
    return parse


def _fmt(x):
    x = float(x)
    if x == int(x) and abs(x) < 1e15:
        return str(int(x))
    return repr(x)


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_fmt(x) for x in v) + "]"
    if t == "matrix":
        return "[" + ", ".join("[" + ", ".join(_fmt(x) for x in row) + "]" for row in v) + "]"
    if t == "edges":
        if isinstance(v, str):
            return "[" + v + "]"
        return "[" + "; ".join(f"{e[0]}-{e[1]}" + (f" {_fmt(e[2])}" if len(e) == 3 else "") for e in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        return f"{_fmt(v)} {_KU[kind]}"
    return _fmt(v)


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, **extra):
    _KU[kind]                                   # every physical kind has its SI unit here
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required, **extra}


def _numin(sym, meaning, required=True, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}


def _degin(sym, meaning, required=True):
    return {"symbol": sym, "kind": "number", "meaning": meaning + " (decimal degrees)", "required": required, "deg": True}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=200, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _matin(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": required}


def _edgein(sym="edges", meaning="undirected network as weighted edges between nodes numbered 0 .. n-1", lo=1, hi=200,
            required=True):
    return {"symbol": sym, "type": "edges", "meaning": meaning, "required": required, "min_length": lo, "max_length": hi}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _textin(sym, meaning, required=True, lo=1, hi=40):
    return {"symbol": sym, "type": "text", "meaning": meaning, "required": required, "min_length": lo, "max_length": hi}


def _ellin():
    return _choice("ellipsoid", ["WGS84", "GRS80"], "reference ellipsoid")


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _ex(written, outputs=None):
    """A worked example given as a user writes it: physical values as '100 km' strings (converted to SI here for
    the example's inputs, kept as written for its question), lists as lists, choices as words."""
    inputs, shown = {}, {}
    for k, v in written.items():
        if isinstance(v, str) and re.match(r"\s*[-+]?[\d.]", v) and re.search(r"[A-Za-z%°µ]", v) and " " in v.strip():
            inputs[k] = _si(v)
            shown[k] = v
        else:
            inputs[k] = v
    return {"inputs": inputs, "outputs": outputs or {}, "_shown": shown}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None, weak=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex, drop=None):
        shown = ex.get("_shown", {})
        return f"{phrases[0]}: " + ", ".join(f"{k} = {shown[k] if k in shown else _written(by[k], v)}"
                                             for k, v in ex["inputs"].items() if k != drop)
    templates = [ask_text(e) for e in examples[:2]]
    questions = [{"q": ask_text(e), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        questions.append({"q": ask_text(examples[0], drop=req[0]), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    exs = [{"inputs": e["inputs"], "outputs": e["outputs"]} for e in examples]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": exs, "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


# ================================================================================================ GEODESY
_GEO = "ellipsoidal geodesy"


def _reduced(E, lat):
    """sin and cos of the reduced (parametric) latitude, normalized."""
    p = lat * DEG
    sb, cb = (1 - E["f"]) * math.sin(p), math.cos(p)
    r = math.hypot(sb, cb)
    return sb / r, cb / r


def _omega(s_a0, sigma):
    """Longitude on the auxiliary sphere, continuous in sigma (Karney 2013, eq. 6)."""
    w = math.atan2(abs(s_a0) * math.sin(sigma), math.cos(sigma))
    w += 2 * math.pi * round((sigma - w) / (2 * math.pi))
    return math.copysign(1.0, s_a0) * w if s_a0 != 0 else 0.0


def _geodesic_frame(E, sb1, cb1, az1):
    """Constants of a geodesic leaving a point of reduced latitude b1 at azimuth az1 (radians)."""
    sa1, ca1 = math.sin(az1), math.cos(az1)
    sa0 = sa1 * cb1
    ca0 = math.hypot(ca1, sa1 * sb1)
    sig1 = math.atan2(sb1, ca1 * cb1)
    k2 = E["ep2"] * ca0 * ca0
    return sa0, ca0, sig1, k2


def _lam_integral(E, sa0, k2, s1, s2):
    f = E["f"]
    J = _integ(lambda t: (2 - f) / (1 + (1 - f) * math.sqrt(1 + k2 * math.sin(t) ** 2)), s1, s2)
    return _omega(sa0, s2) - _omega(sa0, s1) - f * sa0 * J


def _geod_direct(E, lat1, lon1, az1, s):
    """Direct geodesic problem: (lat2, lon2, azimuth at point 2) in degrees."""
    sb1, cb1 = _reduced(E, lat1)
    sa0, ca0, s1, k2 = _geodesic_frame(E, sb1, cb1, az1 * DEG)
    g = lambda t: math.sqrt(1 + k2 * math.sin(t) ** 2)
    target = s / E["b"]
    s2 = s1 + target
    for _ in range(60):
        F = _integ(g, s1, s2) - target
        d = F / g(s2)
        s2 -= d
        if abs(d) <= 1e-15 * max(1.0, abs(s2)):
            break
    sb2 = ca0 * math.sin(s2)
    cb2 = math.hypot(sa0, ca0 * math.cos(s2))
    lat2 = math.atan2(sb2, (1 - E["f"]) * cb2) / DEG
    az2 = math.atan2(sa0, ca0 * math.cos(s2)) / DEG
    lam = _lam_integral(E, sa0, k2, s1, s2)
    return lat2, _nlon(lon1 + lam / DEG), _az(az2)


def _geod_inverse(E, lat1, lon1, lat2, lon2):
    """Inverse geodesic problem by iteration on the auxiliary-sphere longitude (Vincenty's scheme with the exact
    integrals): returns distance, azimuth 1, azimuth 2 (degrees) and the frame of the line."""
    L = _nlon(lon2 - lon1) * DEG
    sb1, cb1 = _reduced(E, lat1)
    sb2, cb2 = _reduced(E, lat2)
    f = E["f"]
    w = L
    res = None
    for it in range(300):
        sw, cw = math.sin(w), math.cos(w)
        x = cb2 * sw
        y = cb1 * sb2 - sb1 * cb2 * cw
        ss = math.hypot(x, y)
        cs = sb1 * sb2 + cb1 * cb2 * cw
        sig = math.atan2(ss, cs)
        _need(ss > 0, "the two points must be distinct")
        az1 = math.atan2(x, y)
        sa0, ca0, s1, k2 = _geodesic_frame(E, sb1, cb1, az1)
        J = _integ(lambda t: (2 - f) / (1 + (1 - f) * math.sqrt(1 + k2 * math.sin(t) ** 2)), s1, s1 + sig)
        w_new = L + f * sa0 * J
        res = (az1, sa0, ca0, s1, k2, sig, w_new)
        if abs(w_new - w) <= 1e-15:
            w = w_new
            break
        w = w_new
    else:
        raise ValueError("the geodesic iteration did not converge (points nearly antipodal)")
    sw, cw = math.sin(w), math.cos(w)
    x = cb2 * sw
    y = cb1 * sb2 - sb1 * cb2 * cw
    sig = math.atan2(math.hypot(x, y), sb1 * sb2 + cb1 * cb2 * cw)
    az1 = math.atan2(x, y)
    sa0, ca0, s1, k2 = _geodesic_frame(E, sb1, cb1, az1)
    s = E["b"] * _integ(lambda t: math.sqrt(1 + k2 * math.sin(t) ** 2), s1, s1 + sig)
    az2 = math.atan2(cb1 * sw, -sb1 * cb2 + cb1 * sb2 * cw)
    return {"s": s, "az1": _az(az1 / DEG), "az2": _az(az2 / DEG), "sa0": sa0, "ca0": ca0, "s1": s1, "k2": k2, "sig": sig}


def _sph_sep(lat1, lon1, lat2, lon2):
    p1, p2, dl = lat1 * DEG, lat2 * DEG, (lon2 - lon1) * DEG
    return math.atan2(math.hypot(math.cos(p2) * math.sin(dl), math.cos(p1) * math.sin(p2) - math.sin(p1) * math.cos(p2) * math.cos(dl)),
                      math.sin(p1) * math.sin(p2) + math.cos(p1) * math.cos(p2) * math.cos(dl)) / DEG


def _two_points(lat1, lon1, lat2, lon2):
    _lat("lat1", lat1)
    _lat("lat2", lat2)
    _lon("lon1", lon1)
    _lon("lon2", lon2)
    sep = _sph_sep(lat1, lon1, lat2, lon2)
    _need(sep > 1e-7, "the two points must be distinct")
    _need(sep <= 150, "the points must be at most 150 degrees apart (the iteration converges slowly towards antipodes)")


def _not_near_north(az):
    return min(az, 360 - az) > 0.01


# ---------------------------------------------------------------- direct problem
def geodesic_destination(lat1, lon1, azimuth, distance, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat1", lat1)
    _lon("lon1", lon1)
    _fin("azimuth", azimuth)
    _pos("the distance", distance)
    _need(distance <= 15e6, "the distance must be at most 15000 km")
    lat2, lon2, az2 = _geod_direct(E, lat1, lon1, azimuth, distance)
    return _out(lat2=lat2, lon2=lon2, azimuth2=az2)


def _ref_geodesic_destination(lat1, lon1, azimuth, distance, ellipsoid):
    lon2, lat2, back = _geod(ellipsoid).fwd(lon1, lat1, azimuth, distance)
    return {"lat2": lat2, "lon2": _nlon(lon2), "azimuth2": _az(back + 180.0)}


def _rand_geodesic_destination(rng):
    for _ in range(100):
        x = {"lat1": _sig(rng.uniform(-80, 80)), "lon1": _sig(rng.uniform(-179, 179)), "azimuth": _sig(rng.uniform(0.5, 359.5)),
             "distance": _sig(_lu(rng, 1e3, 1e7)), "ellipsoid": rng.choice(["WGS84", "GRS80"])}
        o = geodesic_destination(**x)
        if _not_near_north(o["azimuth2"]) and abs(o["lat2"]) < 85 and abs(abs(o["lon2"]) - 180) > 0.01 and abs(o["lon2"]) > 1e-3:
            return x
    return x


_add("geo_geodesic_destination",
     name="Geodesic destination point on the ellipsoid (direct problem)", topic=_GEO,
     purpose="The point reached by travelling a given distance along a geodesic from a start point at a given initial azimuth on the WGS84 or GRS80 ellipsoid, and the azimuth of the geodesic on arrival.",
     inputs=[_degin("lat1", "latitude of the start point"), _degin("lon1", "longitude of the start point"),
             _degin("azimuth", "initial azimuth, clockwise from north"), _phys("distance", "length", "geodesic distance travelled"),
             _ellin()],
     outputs=[_o("lat2", "number", "latitude of the destination (degrees)"), _o("lon2", "number", "longitude of the destination (degrees)"),
              _o("azimuth2", "number", "forward azimuth at the destination (degrees clockwise from north)")],
     primary="lat2", conditions=["start latitude strictly between -90 and 90", "0 < distance <= 15000 km"],
     method="Bessel's auxiliary sphere: solve b*I1(sigma) = s by Newton steps with the arc-length integral by Gauss-Legendre quadrature, then the longitude integral",
     latex=r"\frac{s}{b} = \int_{\sigma_1}^{\sigma_2}\sqrt{1 + k^2\sin^2\sigma}\,d\sigma,\quad \lambda_{12} = \omega_{12} - f\sin\alpha_0\int_{\sigma_1}^{\sigma_2}\frac{2-f}{1 + (1-f)\sqrt{1+k^2\sin^2\sigma}}\,d\sigma",
     run=geodesic_destination, reference=_ref_geodesic_destination, rand=_rand_geodesic_destination,
     wrong=lambda lat1, lon1, azimuth, distance, ellipsoid: (lambda r: _out(lat2=r[0], lon2=r[1], azimuth2=r[2]))(
         _geod_direct(dict(_ellipsoid(ellipsoid), b=6378137.0), lat1, lon1, azimuth, distance)),
     phrases=("Geodesic destination", "Geodesic direct problem", "Destination point on the ellipsoid"),
     examples=[_ex({"lat1": 52.2, "lon1": 0.12, "azimuth": 45, "distance": "100 km", "ellipsoid": "WGS84"}),
               _ex({"lat1": -33.87, "lon1": 151.21, "azimuth": 240, "distance": "2000 km", "ellipsoid": "GRS80"})],
     level="intermediate",
     extra_q=[{"q": "Geodesic destination: lat1 = 52.2°, lon1 = 0.12°, azimuth = 45°, distance = 100000 m, ellipsoid = WGS84", "like": 0},
              {"q": "Geodesic destination: lat1 = 95, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84", "outcome": "INVALID_INPUT"},
              {"q": "Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100, ellipsoid = WGS84", "outcome": "UNRECOGNIZED_FORMAT"}])


# ---------------------------------------------------------------- midpoint
def geodesic_midpoint(lat1, lon1, lat2, lon2, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _two_points(lat1, lon1, lat2, lon2)
    inv = _geod_inverse(E, lat1, lon1, lat2, lon2)
    lat, lon, az = _geod_direct(E, lat1, lon1, inv["az1"], inv["s"] / 2)
    return _out(lat_mid=lat, lon_mid=lon, distance=inv["s"])


def _ref_geodesic_midpoint(lat1, lon1, lat2, lon2, ellipsoid):
    g = _geod(ellipsoid)
    az, _, s = g.inv(lon1, lat1, lon2, lat2)
    lon, lat, _ = g.fwd(lon1, lat1, az, s / 2)
    return {"lat_mid": lat, "lon_mid": _nlon(lon), "distance": s}


def _rand_two_points(rng, near=None):
    for _ in range(200):
        lat1, lon1 = rng.uniform(-75, 75), rng.uniform(-179, 179)
        sep = near or _lu(rng, 0.05, 120)
        az = rng.uniform(0, 360)
        lat2 = math.asin(math.sin(lat1 * DEG) * math.cos(sep * DEG) + math.cos(lat1 * DEG) * math.sin(sep * DEG) * math.cos(az * DEG)) / DEG
        lon2 = lon1 + math.atan2(math.sin(az * DEG) * math.sin(sep * DEG) * math.cos(lat1 * DEG),
                                 math.cos(sep * DEG) - math.sin(lat1 * DEG) * math.sin(lat2 * DEG)) / DEG
        if abs(lat2) < 80:
            return {"lat1": _sig(lat1), "lon1": _sig(lon1), "lat2": _sig(lat2), "lon2": _sig(_nlon(lon2))}
    return {"lat1": 10.0, "lon1": 20.0, "lat2": 30.0, "lon2": 40.0}


def _rand_geodesic_midpoint(rng):
    for _ in range(100):
        x = dict(_rand_two_points(rng), ellipsoid=rng.choice(["WGS84", "GRS80"]))
        o = geodesic_midpoint(**x)
        if abs(o["lon_mid"]) > 1e-3 and abs(o["lat_mid"]) > 1e-3 and abs(abs(o["lon_mid"]) - 180) > 0.01:
            return x
    return x


_add("geo_geodesic_midpoint",
     name="Midpoint of a geodesic on the ellipsoid", topic=_GEO,
     purpose="The point halfway along the shortest path (geodesic) between two points on the WGS84 or GRS80 ellipsoid, with the geodesic length.",
     inputs=[_degin("lat1", "latitude of the first point"), _degin("lon1", "longitude of the first point"),
             _degin("lat2", "latitude of the second point"), _degin("lon2", "longitude of the second point"), _ellin()],
     outputs=[_o("lat_mid", "number", "latitude of the midpoint (degrees)"), _o("lon_mid", "number", "longitude of the midpoint (degrees)"),
              _o("distance", "length", "length of the geodesic between the two points")],
     primary="lat_mid", conditions=["distinct points at most 150 degrees apart", "latitudes strictly between -90 and 90"],
     method="inverse problem (iteration on the auxiliary-sphere longitude, exact integrals) for the azimuth and length, then the direct problem over half the length",
     latex=r"P_{mid} = \mathrm{direct}\left(P_1,\ \alpha_1,\ s_{12}/2\right)",
     run=geodesic_midpoint, reference=_ref_geodesic_midpoint, rand=_rand_geodesic_midpoint,
     wrong=lambda lat1, lon1, lat2, lon2, ellipsoid: _out(lat_mid=(lat1 + lat2) / 2, lon_mid=_nlon(lon1 + _nlon(lon2 - lon1) / 2),
                                                         distance=_geod_inverse(_ellipsoid(ellipsoid), lat1, lon1, lat2, lon2)["s"]),
     phrases=("Geodesic midpoint", "Midpoint on the ellipsoid", "Midpoint of a geodesic"),
     examples=[_ex({"lat1": 51.5, "lon1": -0.13, "lat2": 40.71, "lon2": -74.01, "ellipsoid": "WGS84"}),
               _ex({"lat1": 35.68, "lon1": 139.69, "lat2": -33.87, "lon2": 151.21, "ellipsoid": "GRS80"})],
     level="intermediate",
     extra_q=[{"q": "Geodesic midpoint: lat1 = 10, lon1 = 20, lat2 = 10, lon2 = 20, ellipsoid = WGS84", "outcome": "INVALID_INPUT"},
              {"q": "Geodesic midpoint: lat1 = 51.5, lon1 = -0.13, lat2 = 40.71, lon2 = -74.01, ellipsoid = Clarke", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- polygon area and perimeter
def _q_auth(E, phi):
    s = math.sin(phi)
    e = E["e"]
    return (1 - E["e2"]) * (s / (1 - E["e2"] * s * s) + math.atanh(e * s) / e)


def geodesic_polygon_area(lats, lons, ellipsoid):
    E = _ellipsoid(ellipsoid)
    n = len(lats)
    _need(n == len(lons), "give one longitude per latitude")
    _need(3 <= n <= 50, "a polygon needs 3 to 50 vertices")
    for v in lats:
        _lat("each latitude", v)
        _need(abs(v) <= 88, "vertices must lie within 88 degrees of the equator (polygons around a pole are not supported)")
    for v in lons:
        _lon("each longitude", v)
    a2 = E["a"] ** 2
    qc = _q_auth(E, lats[0] * DEG)
    f = E["f"]
    total, perim, dlam_sum = [], [], 0.0
    for i in range(n):
        j = (i + 1) % n
        _need(_sph_sep(lats[i], lons[i], lats[j], lons[j]) > 1e-7, "consecutive vertices must be distinct")
        _need(_sph_sep(lats[i], lons[i], lats[j], lons[j]) <= 90, "each edge must span at most 90 degrees")
        inv = _geod_inverse(E, lats[i], lons[i], lats[j], lons[j])
        perim.append(inv["s"])
        sa0, ca0, s1, k2, sig = inv["sa0"], inv["ca0"], inv["s1"], inv["k2"], inv["sig"]

        def integrand(t):
            sb = ca0 * math.sin(t)
            cb2 = sa0 * sa0 + (ca0 * math.cos(t)) ** 2
            phi = math.atan2(sb, (1 - f) * math.sqrt(cb2))
            dl = sa0 / cb2 - f * sa0 * (2 - f) / (1 + (1 - f) * math.sqrt(1 + k2 * math.sin(t) ** 2))
            return a2 / 2 * (_q_auth(E, phi) - qc) * dl
        total.append(_integ(integrand, s1, s1 + sig))
        dlam_sum += _nlon(lons[j] - lons[i])
    _need(abs(dlam_sum) < 1.0, "the polygon must not enclose a pole")
    area = -math.fsum(total)
    _need(abs(area) > 0, "the polygon has no area")
    return _out(area=abs(area), perimeter=math.fsum(perim))


def _ref_geodesic_polygon_area(lats, lons, ellipsoid):
    area, perim = _geod(ellipsoid).polygon_area_perimeter(list(lons), list(lats))
    return {"area": abs(area), "perimeter": perim}


def _rand_polygon(rng):
    n = rng.randint(3, 9)
    lat0, lon0 = rng.uniform(-70, 70), rng.uniform(-175, 175)
    rad = _lu(rng, 0.2, 8.0)
    angs = sorted(rng.uniform(0, 2 * math.pi) for _ in range(n))
    if max(b - a for a, b in zip(angs, angs[1:] + [angs[0] + 2 * math.pi])) > 2.5:
        angs = [2 * math.pi * k / n + rng.uniform(-0.3, 0.3) for k in range(n)]
    lats, lons = [], []
    for a in angs:
        r = rad * rng.uniform(0.5, 1.0)
        lats.append(_sig(lat0 + r * math.sin(a)))
        lons.append(_sig(_nlon(lon0 + r * math.cos(a) / math.cos(lat0 * DEG))))
    if rng.random() < 0.5:
        lats.reverse()
        lons.reverse()
    return {"lats": lats, "lons": lons, "ellipsoid": rng.choice(["WGS84", "GRS80"])}


_add("geo_geodesic_polygon_area",
     name="Area and perimeter of a geodesic polygon on the ellipsoid", topic=_GEO,
     purpose="The area enclosed by a polygon whose edges are geodesics on the WGS84 or GRS80 ellipsoid, and its perimeter (vertices given as latitude and longitude lists, in either orientation).",
     inputs=[_vecin("lats", "latitudes of the vertices in order (decimal degrees)", 3, 50),
             _vecin("lons", "longitudes of the vertices in the same order (decimal degrees)", 3, 50), _ellin()],
     outputs=[_o("area", "area", "enclosed area on the ellipsoid"), _o("perimeter", "length", "perimeter (sum of the geodesic edge lengths)")],
     primary="area", conditions=["a simple polygon not enclosing a pole", "vertices within 88 degrees of the equator", "edges at most 90 degrees long"],
     method="Green's theorem on the ellipsoid: the authalic strip area a^2 q(phi)/2 integrated against the longitude along each geodesic edge (Gauss-Legendre in the auxiliary-sphere arc), summed over the edges",
     latex=r"S = -\oint \frac{a^2}{2}\left[q(\varphi) - q(\varphi_c)\right]d\lambda,\quad q(\varphi) = (1-e^2)\left[\frac{\sin\varphi}{1-e^2\sin^2\varphi} + \frac{\operatorname{artanh}(e\sin\varphi)}{e}\right]",
     run=geodesic_polygon_area, reference=_ref_geodesic_polygon_area, rand=_rand_polygon,
     wrong=lambda lats, lons, ellipsoid: (lambda r: _out(area=r["area"] * (1 - _ellipsoid(ellipsoid)["e2"]), perimeter=r["perimeter"]))(
         geodesic_polygon_area(lats, lons, ellipsoid)),
     phrases=("Geodesic polygon area", "Ellipsoidal polygon area", "Area of a polygon on the ellipsoid"),
     examples=[_ex({"lats": [0, 0, 1, 1], "lons": [0, 1, 1, 0], "ellipsoid": "WGS84"}),
               _ex({"lats": [48.0, 49.5, 47.2], "lons": [2.0, 4.5, 6.1], "ellipsoid": "GRS80"})],
     level="expert",
     extra_q=[{"q": "Geodesic polygon area: lats = [0, 0, 1], lons = [0, 1], ellipsoid = WGS84", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- meridian arc (Helmert's series)
def _merid(E, phi):
    n = E["n"]
    n2, n3, n4 = n * n, n ** 3, n ** 4
    return E["a"] / (1 + n) * ((1 + n2 / 4 + n4 / 64) * phi - 1.5 * (n - n3 / 8) * math.sin(2 * phi)
                               + 15.0 / 16 * (n2 - n4 / 4) * math.sin(4 * phi) - 35.0 / 48 * n3 * math.sin(6 * phi)
                               + 315.0 / 512 * n4 * math.sin(8 * phi))


def _M(E, phi):
    return E["a"] * (1 - E["e2"]) / (1 - E["e2"] * math.sin(phi) ** 2) ** 1.5


def _N(E, phi):
    return E["a"] / math.sqrt(1 - E["e2"] * math.sin(phi) ** 2)


def _merid_inv(E, m):
    phi = m / _merid(E, math.pi / 2) * math.pi / 2
    for _ in range(50):
        d = (_merid(E, phi) - m) / _M(E, phi)
        phi -= d
        if abs(d) < 1e-16:
            break
    return phi


def meridian_arc(lat1, lat2, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat1", lat1, True)
    _lat("lat2", lat2, True)
    _need(lat1 != lat2, "the two latitudes must differ")
    m1, m2 = _merid(E, lat1 * DEG), _merid(E, lat2 * DEG)
    return _out(distance=abs(m2 - m1), m1=m1, m2=m2)


def _ref_meridian_arc(lat1, lat2, ellipsoid):
    g = _geod(ellipsoid)
    s = g.inv(10.0, lat1, 10.0, lat2)[2]
    E = _ellipsoid(ellipsoid)
    m = lambda p: math.copysign(g.inv(10.0, 0.0, 10.0, abs(p))[2], p) if p != 0 else 0.0
    return {"distance": s, "m1": m(lat1), "m2": m(lat2)}


_add("geo_meridian_arc",
     name="Meridian arc length between two latitudes", topic=_GEO,
     purpose="The distance along a meridian of the WGS84 or GRS80 ellipsoid between two latitudes, and the meridian distance of each latitude from the equator.",
     inputs=[_degin("lat1", "first latitude"), _degin("lat2", "second latitude"), _ellin()],
     outputs=[_o("distance", "length", "meridian arc length between the latitudes"), _o("m1", "length", "meridian distance from the equator to lat1 (negative south)"),
              _o("m2", "length", "meridian distance from the equator to lat2 (negative south)")],
     primary="distance", conditions=["latitudes between -90 and 90 and different"],
     method="Helmert's series for the meridian distance in the third flattening n (to n^4)",
     latex=r"m(\varphi) = \frac{a}{1+n}\left[\left(1+\tfrac{n^2}{4}+\tfrac{n^4}{64}\right)\varphi - \tfrac32\left(n-\tfrac{n^3}{8}\right)\sin2\varphi + \tfrac{15}{16}\left(n^2-\tfrac{n^4}{4}\right)\sin4\varphi - \tfrac{35}{48}n^3\sin6\varphi + \tfrac{315}{512}n^4\sin8\varphi\right]",
     run=meridian_arc, reference=_ref_meridian_arc,
     rand=lambda rng: (lambda a, b: {"lat1": a, "lat2": b if abs(b - a) > 0.05 else a + 1.0, "ellipsoid": rng.choice(["WGS84", "GRS80"])})(
         _sig(rng.uniform(-89, 89)), _sig(rng.uniform(-89, 89))),
     wrong=lambda lat1, lat2, ellipsoid: (lambda E: _out(distance=E["a"] * abs(lat2 - lat1) * DEG, m1=E["a"] * lat1 * DEG, m2=E["a"] * lat2 * DEG))(_ellipsoid(ellipsoid)),
     phrases=("Meridian arc length", "Meridian arc", "Meridian distance"),
     examples=[_ex({"lat1": 0, "lat2": 90, "ellipsoid": "WGS84"}), _ex({"lat1": 40, "lat2": 41, "ellipsoid": "GRS80"})],
     level="beginner")


# ---------------------------------------------------------------- arc of a parallel
def parallel_arc(lat, lon1, lon2, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat", lat)
    _lon("lon1", lon1)
    _lon("lon2", lon2)
    dl = abs(_nlon(lon2 - lon1))
    _need(dl > 0, "the two longitudes must differ")
    p = _N(E, lat * DEG) * math.cos(lat * DEG)
    return _out(distance=p * dl * DEG, radius=p)


def _ref_parallel_arc(lat, lon1, lon2, ellipsoid):
    t = _tr(_ll(ellipsoid), f"+proj=geocent +ellps={ellipsoid} +type=crs")
    X, Y, Z = t.transform(lon1, lat, 0.0)
    p = math.hypot(X, Y)
    return {"distance": p * abs(_nlon(lon2 - lon1)) * DEG, "radius": p}


_add("geo_parallel_arc",
     name="Arc length along a parallel of latitude", topic=_GEO,
     purpose="The distance along a parallel of latitude between two longitudes (the shorter way round) on the WGS84 or GRS80 ellipsoid, and the radius of the parallel.",
     inputs=[_degin("lat", "latitude of the parallel"), _degin("lon1", "first longitude"), _degin("lon2", "second longitude"), _ellin()],
     outputs=[_o("distance", "length", "arc length along the parallel"), _o("radius", "length", "radius of the parallel N cos(phi)")],
     primary="distance", conditions=["latitude strictly between -90 and 90", "different longitudes"],
     method="radius of the parallel N(phi) cos(phi) times the longitude difference in radians",
     latex=r"s = N(\varphi)\cos\varphi\,|\Delta\lambda|,\quad N = \frac{a}{\sqrt{1-e^2\sin^2\varphi}}",
     run=parallel_arc, reference=_ref_parallel_arc,
     rand=lambda rng: {"lat": _sig(rng.uniform(-85, 85)), "lon1": _sig(rng.uniform(-180, 180)), "lon2": _sig(rng.uniform(-180, 180)),
                       "ellipsoid": rng.choice(["WGS84", "GRS80"])},
     wrong=lambda lat, lon1, lon2, ellipsoid: (lambda E: _out(distance=E["a"] * math.cos(lat * DEG) * abs(_nlon(lon2 - lon1)) * DEG,
                                                             radius=E["a"] * math.cos(lat * DEG)))(_ellipsoid(ellipsoid)),
     phrases=("Parallel arc length", "Arc along a parallel", "Distance along a parallel"),
     examples=[_ex({"lat": 60, "lon1": 0, "lon2": 1, "ellipsoid": "WGS84"}), _ex({"lat": -23.5, "lon1": 170, "lon2": -170, "ellipsoid": "GRS80"})],
     level="beginner", weak="pyproj's geodetic-to-geocentric conversion gives the radius of the parallel; the arc is that radius times the longitude difference (a definition)")


# ---------------------------------------------------------------- area of a graticule cell
def graticule_cell_area(lat1, lat2, lon1, lon2, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat1", lat1, True)
    _lat("lat2", lat2, True)
    _need(lat2 > lat1, "lat2 must be north of lat1")
    _lon("lon1", lon1)
    _lon("lon2", lon2)
    dl = lon2 - lon1
    _need(0 < dl <= 360, "lon2 must lie east of lon1 by at most 360 degrees")
    qa = E["a"] ** 2 / 2 * (_q_auth(E, lat2 * DEG) - _q_auth(E, lat1 * DEG))
    total = 2 * math.pi * E["a"] ** 2 / 2 * _q_auth(E, math.pi / 2) * 2
    area = qa * dl * DEG
    return _out(area=area, fraction_of_ellipsoid=area / total)


def _ref_graticule_cell_area(lat1, lat2, lon1, lon2, ellipsoid):
    E = _ellipsoid(ellipsoid)
    band = _quad(lambda p: _M(E, p) * _N(E, p) * math.cos(p), lat1 * DEG, lat2 * DEG)
    total = 4 * math.pi * _quad(lambda p: _M(E, p) * _N(E, p) * math.cos(p), 0.0, math.pi / 2)
    return {"area": band * (lon2 - lon1) * DEG, "fraction_of_ellipsoid": band * (lon2 - lon1) * DEG / total}


_add("geo_graticule_cell_area",
     name="Area of a latitude-longitude cell on the ellipsoid", topic=_GEO,
     purpose="The area of the region between two parallels and two meridians (a graticule cell or grid square) on the WGS84 or GRS80 ellipsoid, and its share of the whole ellipsoid's surface.",
     inputs=[_degin("lat1", "southern latitude"), _degin("lat2", "northern latitude"), _degin("lon1", "western longitude"),
             _degin("lon2", "eastern longitude (east of lon1)"), _ellin()],
     outputs=[_o("area", "area", "area of the cell"), _o("fraction_of_ellipsoid", "number", "area as a fraction of the ellipsoid's total surface")],
     primary="area", conditions=["lat1 < lat2", "0 < lon2 - lon1 <= 360"],
     method="authalic function: area = (a^2/2) [q(phi2) - q(phi1)] (lambda2 - lambda1)",
     latex=r"A = \frac{a^2}{2}\left[q(\varphi_2) - q(\varphi_1)\right](\lambda_2 - \lambda_1)",
     run=graticule_cell_area, reference=_ref_graticule_cell_area,
     rand=lambda rng: (lambda a, w, l: {"lat1": a, "lat2": _sig(a + w), "lon1": l, "lon2": _sig(l + _lu(rng, 0.01, 40)),
                                         "ellipsoid": rng.choice(["WGS84", "GRS80"])})(_sig(rng.uniform(-89, 60)), _lu(rng, 0.01, 29), _sig(rng.uniform(-180, 170))),
     wrong=lambda lat1, lat2, lon1, lon2, ellipsoid: (lambda E: _out(area=E["a"] ** 2 * (math.sin(lat2 * DEG) - math.sin(lat1 * DEG)) * (lon2 - lon1) * DEG,
                                                                    fraction_of_ellipsoid=(math.sin(lat2 * DEG) - math.sin(lat1 * DEG)) * (lon2 - lon1) * DEG / (4 * math.pi)))(_ellipsoid(ellipsoid)),
     phrases=("Graticule cell area", "Area of a latitude-longitude cell", "Grid cell area on the ellipsoid"),
     examples=[_ex({"lat1": 0, "lat2": 1, "lon1": 0, "lon2": 1, "ellipsoid": "WGS84"}),
               _ex({"lat1": 50, "lat2": 51, "lon1": 4, "lon2": 5, "ellipsoid": "GRS80"})],
     level="intermediate")


# ---------------------------------------------------------------- radii of curvature
def radii_of_curvature(lat, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat", lat, True)
    M, N = _M(E, lat * DEG), _N(E, lat * DEG)
    return _out(M=M, N=N, R_gauss=math.sqrt(M * N))


def _ref_radii(lat, ellipsoid):
    """M as the radius of curvature of the parametric meridian ellipse x = a cos u, z = b sin u (generic curvature of a
    plane curve), N from pyproj's geocentric conversion (N = p / cos phi) or the polar value."""
    E = _ellipsoid(ellipsoid)
    a, b = E["a"], E["b"]
    u = math.atan2((1 - E["f"]) * math.sin(lat * DEG), math.cos(lat * DEG))
    x1, z1 = -a * math.sin(u), b * math.cos(u)
    x2, z2 = -a * math.cos(u), -b * math.sin(u)
    M = (x1 * x1 + z1 * z1) ** 1.5 / abs(x1 * z2 - z1 * x2)
    if abs(lat) < 89.9:
        t = _tr(_ll(ellipsoid), f"+proj=geocent +ellps={ellipsoid} +type=crs")
        X, Y, Z = t.transform(0.0, lat, 0.0)
        N = math.hypot(X, Y) / math.cos(lat * DEG)
    else:
        N = a * a / math.hypot(a * math.cos(lat * DEG), b * math.sin(lat * DEG))
    return {"M": M, "N": N, "R_gauss": math.sqrt(M * N)}


_add("geo_radii_of_curvature",
     name="Radii of curvature of the ellipsoid at a latitude", topic=_GEO,
     purpose="The meridional radius of curvature M, the prime-vertical radius of curvature N and the Gaussian mean radius sqrt(MN) of the WGS84 or GRS80 ellipsoid at a latitude.",
     inputs=[_degin("lat", "latitude"), _ellin()],
     outputs=[_o("M", "length", "meridional radius of curvature"), _o("N", "length", "prime-vertical radius of curvature"),
              _o("R_gauss", "length", "Gaussian mean radius sqrt(M N)")],
     primary="M", conditions=["latitude between -90 and 90"],
     method="M = a(1-e^2)/(1-e^2 sin^2 phi)^(3/2), N = a/sqrt(1-e^2 sin^2 phi)",
     latex=r"M = \frac{a(1-e^2)}{(1-e^2\sin^2\varphi)^{3/2}},\quad N = \frac{a}{\sqrt{1-e^2\sin^2\varphi}},\quad R_G = \sqrt{MN}",
     run=radii_of_curvature, reference=_ref_radii,
     rand=lambda rng: {"lat": _sig(rng.uniform(-89.5, 89.5)), "ellipsoid": rng.choice(["WGS84", "GRS80"])},
     wrong=lambda lat, ellipsoid: (lambda E: (lambda M, N: _out(M=M, N=N, R_gauss=math.sqrt(M * N)))(
         E["a"] * (1 - E["e2"]) / (1 - E["e2"] * math.sin(lat * DEG) ** 2), _N(E, lat * DEG)))(_ellipsoid(ellipsoid)),
     phrases=("Radii of curvature", "Ellipsoid radii of curvature", "Radius of curvature"),
     examples=[_ex({"lat": 45, "ellipsoid": "WGS84"}), _ex({"lat": 0, "ellipsoid": "GRS80"})],
     level="intermediate")


# ---------------------------------------------------------------- rhumb lines
def _psi(E, phi):
    return math.asinh(math.tan(phi)) - E["e"] * math.atanh(E["e"] * math.sin(phi))


def rhumb_inverse(lat1, lon1, lat2, lon2, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat1", lat1)
    _lat("lat2", lat2)
    _lon("lon1", lon1)
    _lon("lon2", lon2)
    _need(abs(lat1) <= 89 and abs(lat2) <= 89, "latitudes must lie within 89 degrees of the equator")
    dl = _nlon(lon2 - lon1) * DEG
    p1, p2 = lat1 * DEG, lat2 * DEG
    if lat1 == lat2:
        _need(dl != 0, "the two points must be distinct")
        s = abs(dl) * _N(E, p1) * math.cos(p1)
        brg = 90.0 if dl > 0 else 270.0
        return _out(distance=s, bearing=brg)
    dpsi = _psi(E, p2) - _psi(E, p1)
    dm = _merid(E, p2) - _merid(E, p1)
    brg = _az(math.atan2(dl, dpsi) / DEG)
    s = math.hypot(dl, dpsi) * dm / dpsi
    return _out(distance=s, bearing=brg)


def _ref_rhumb_inverse(lat1, lon1, lat2, lon2, ellipsoid):
    E = _ellipsoid(ellipsoid)
    dl = _nlon(lon2 - lon1) * DEG
    p1, p2 = lat1 * DEG, lat2 * DEG
    dm = _quad(lambda p: _M(E, p), p1, p2)
    dpsi = _quad(lambda p: _M(E, p) / (_N(E, p) * math.cos(p)), p1, p2)
    return {"distance": math.hypot(dl, dpsi) * dm / dpsi, "bearing": _az(math.atan2(dl, dpsi) / DEG)}


def _rand_rhumb_inverse(rng):
    for _ in range(100):
        x = {"lat1": _sig(rng.uniform(-80, 80)), "lon1": _sig(rng.uniform(-180, 180)), "ellipsoid": rng.choice(["WGS84", "GRS80"])}
        x["lat2"] = _sig(max(-85.0, min(85.0, x["lat1"] + rng.choice([-1, 1]) * _lu(rng, 0.1, 40))))
        x["lon2"] = _sig(_nlon(x["lon1"] + rng.uniform(-60, 60)))
        if abs(x["lat2"] - x["lat1"]) >= 0.05 and _not_near_north(rhumb_inverse(**x)["bearing"]):
            return x
    return x


_add("geo_rhumb_line_inverse",
     name="Rhumb-line (loxodrome) distance and bearing on the ellipsoid", topic=_GEO,
     purpose="The constant bearing and the length of the rhumb line (loxodrome) between two points on the WGS84 or GRS80 ellipsoid, the shorter way round in longitude.",
     inputs=[_degin("lat1", "latitude of the start point"), _degin("lon1", "longitude of the start point"),
             _degin("lat2", "latitude of the end point"), _degin("lon2", "longitude of the end point"), _ellin()],
     outputs=[_o("distance", "length", "rhumb-line length"), _o("bearing", "number", "constant bearing (degrees clockwise from north)")],
     primary="distance", conditions=["latitudes within 89 degrees of the equator", "distinct points"],
     method="bearing = atan2(dlambda, dpsi) with the isometric latitude psi; length = dm / cos(bearing) with Helmert's meridian distance (N cos(phi) |dlambda| along a parallel)",
     latex=r"\tan\alpha = \frac{\Delta\lambda}{\Delta\psi},\quad s = \frac{\Delta m}{\cos\alpha},\quad \psi = \operatorname{arsinh}(\tan\varphi) - e\operatorname{artanh}(e\sin\varphi)",
     run=rhumb_inverse, reference=_ref_rhumb_inverse, rand=_rand_rhumb_inverse,
     wrong=lambda lat1, lon1, lat2, lon2, ellipsoid: (lambda E: (lambda dl, dpsi, dm: _out(distance=math.hypot(dl, dpsi) * dm / dpsi, bearing=_az(math.atan2(dl, dpsi) / DEG)))(
         _nlon(lon2 - lon1) * DEG, math.asinh(math.tan(lat2 * DEG)) - math.asinh(math.tan(lat1 * DEG)), _merid(E, lat2 * DEG) - _merid(E, lat1 * DEG)))(_ellipsoid(ellipsoid)),
     phrases=("Rhumb line distance and bearing", "Rhumb line inverse", "Loxodrome distance and bearing"),
     examples=[_ex({"lat1": 50.0, "lon1": -5.0, "lat2": 58.0, "lon2": 3.0, "ellipsoid": "WGS84"}),
               _ex({"lat1": -34.0, "lon1": 18.4, "lat2": -31.9, "lon2": 115.9, "ellipsoid": "GRS80"})],
     level="intermediate", weak="scipy quadrature of the meridian and isometric-latitude integrals; the bearing and length are then composed with the same definitions (atan2 and dm/cos)")


def rhumb_destination(lat1, lon1, bearing, distance, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat1", lat1)
    _need(abs(lat1) <= 89, "the start latitude must lie within 89 degrees of the equator")
    _lon("lon1", lon1)
    _fin("bearing", bearing)
    _pos("the distance", distance)
    a = bearing * DEG
    p1 = lat1 * DEG
    ca = math.cos(a)
    if abs(ca) < 1e-12:
        dl = distance * math.sin(a) / (_N(E, p1) * math.cos(p1))
        return _out(lat2=lat1, lon2=_nlon(lon1 + dl / DEG))
    m2 = _merid(E, p1) + distance * ca
    _need(abs(m2) < _merid(E, 89 * DEG), "the rhumb line would reach within 1 degree of a pole")
    p2 = _merid_inv(E, m2)
    dl = math.tan(a) * (_psi(E, p2) - _psi(E, p1))
    return _out(lat2=p2 / DEG, lon2=_nlon(lon1 + dl / DEG))


def _ref_rhumb_destination(lat1, lon1, bearing, distance, ellipsoid):
    """The rhumb ODE dphi/ds = cos(a)/M, dlambda/ds = sin(a)/(N cos(phi)) integrated by scipy DOP853."""
    from scipy.integrate import solve_ivp
    E = _ellipsoid(ellipsoid)
    a = bearing * DEG
    sol = solve_ivp(lambda s, y: [math.cos(a) / _M(E, y[0]), math.sin(a) / (_N(E, y[0]) * math.cos(y[0]))],
                    (0.0, distance), [lat1 * DEG, 0.0], method="DOP853", rtol=1e-13, atol=1e-15)
    return {"lat2": sol.y[0, -1] / DEG, "lon2": _nlon(lon1 + sol.y[1, -1] / DEG)}


def _rand_rhumb_destination(rng):
    for _ in range(100):
        x = {"lat1": _sig(rng.uniform(-75, 75)), "lon1": _sig(rng.uniform(-180, 180)), "bearing": _sig(rng.uniform(0, 360)),
             "distance": _sig(_lu(rng, 1e4, 3e6)), "ellipsoid": rng.choice(["WGS84", "GRS80"])}
        if abs(math.cos(x["bearing"] * DEG)) < 0.1:
            continue
        try:
            o = rhumb_destination(**x)
        except ValueError:
            continue
        if abs(o["lat2"]) < 85 and abs(o["lon2"]) > 1e-3 and abs(o["lat2"]) > 1e-3 and abs(abs(o["lon2"]) - 180) > 1e-3:
            return x
    return x


_add("geo_rhumb_line_destination",
     name="Destination along a rhumb line (loxodrome) on the ellipsoid", topic=_GEO,
     purpose="The point reached by travelling a given distance on a constant bearing (rhumb line) from a start point on the WGS84 or GRS80 ellipsoid.",
     inputs=[_degin("lat1", "latitude of the start point"), _degin("lon1", "longitude of the start point"),
             _degin("bearing", "constant bearing, clockwise from north"), _phys("distance", "length", "distance travelled"), _ellin()],
     outputs=[_o("lat2", "number", "latitude of the destination (degrees)"), _o("lon2", "number", "longitude of the destination (degrees)")],
     primary="lat2", conditions=["the line stays more than 1 degree from the poles", "distance > 0"],
     method="meridian distance advances by s cos(bearing) (Helmert's series, inverted by Newton steps); the longitude by tan(bearing) times the change of isometric latitude",
     latex=r"m_2 = m_1 + s\cos\alpha,\quad \Delta\lambda = \tan\alpha\,(\psi_2 - \psi_1)",
     run=rhumb_destination, reference=_ref_rhumb_destination, rand=_rand_rhumb_destination,
     wrong=lambda lat1, lon1, bearing, distance, ellipsoid: (lambda E: (lambda p2: _out(lat2=p2 / DEG, lon2=_nlon(lon1 + math.tan(bearing * DEG) * (p2 - lat1 * DEG) / DEG)))(
         _merid_inv(E, _merid(E, lat1 * DEG) + distance * math.cos(bearing * DEG))))(_ellipsoid(ellipsoid)),
     phrases=("Rhumb line destination", "Loxodrome destination", "Destination on a constant bearing"),
     examples=[_ex({"lat1": 50.0, "lon1": -5.0, "bearing": 30, "distance": "500 km", "ellipsoid": "WGS84"}),
               _ex({"lat1": -10.0, "lon1": 100.0, "bearing": 200, "distance": "1200 km", "ellipsoid": "GRS80"})],
     level="advanced")


# ================================================================================================ COORDINATES
_COORD = "coordinate systems and conversions"


def _sincos_deg(angle_deg):
    """(sin, cos) of an angle in degrees, correctly rounded through mpmath at 30 digits: the platform's libm differs in
    the last bit between Linux and Windows, and ENU differences of ECEF positions (~6e6 m) magnify that a millionfold
    (geo_geodetic_to_enu differed across platforms, 2026-10-24)."""
    import mpmath
    with mpmath.workdps(30):
        x = mpmath.mpf(angle_deg) * mpmath.pi / 180
        return float(mpmath.sin(x)), float(mpmath.cos(x))


def _ecef(E, lat, lon, h):
    sp, cp = _sincos_deg(lat)
    sl, cl = _sincos_deg(lon)
    N = E["a"] / math.sqrt(1 - E["e2"] * sp * sp)
    return ((N + h) * cp * cl, (N + h) * cp * sl, (N * (1 - E["e2"]) + h) * sp)


def _geodetic(E, X, Y, Z):
    p = math.hypot(X, Y)
    _need(p > 1e-3, "the point lies on the polar axis: its longitude is undefined")
    phi = math.atan2(Z, p * (1 - E["e2"]))
    for _ in range(100):
        N = _N(E, phi)
        nphi = math.atan2(Z + E["e2"] * N * math.sin(phi), p)
        d = nphi - phi
        phi = nphi
        if abs(d) < 1e-16:
            break
    N = _N(E, phi)
    h = p * math.cos(phi) + Z * math.sin(phi) - E["a"] ** 2 / N
    return phi / DEG, math.atan2(Y, X) / DEG + 0.0, h


def _geo_point(lat, lon, h, pre=""):
    _lat(pre + "lat", lat, True)
    _lon(pre + "lon", lon)
    _fin(pre + "h", h)
    _need(-1e5 <= h <= 1e8, "the height must lie between -100 km and 100000 km")


def geodetic_to_ecef(lat, lon, h, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _geo_point(lat, lon, h)
    X, Y, Z = _ecef(E, lat, lon, h)
    return _out(X=X, Y=Y, Z=Z)


def _ref_geodetic_to_ecef(lat, lon, h, ellipsoid):
    X, Y, Z = _tr(f"+proj=longlat +ellps={ellipsoid} +no_defs +type=crs", f"+proj=geocent +ellps={ellipsoid} +type=crs").transform(lon, lat, h)
    return {"X": X, "Y": Y, "Z": Z}


def _rand_geo(rng, pre=""):
    return {pre + "lat": _sig(rng.uniform(-89, 89)), pre + "lon": _sig(rng.uniform(-179.9, 179.9)), pre + "h": _sig(rng.uniform(-400, 9000), 5)}


_add("geo_geodetic_to_ecef",
     name="Geodetic to geocentric (ECEF) coordinates", topic=_COORD,
     purpose="Earth-centred, Earth-fixed Cartesian coordinates X, Y, Z of a point given by geodetic latitude, longitude and ellipsoidal height on the WGS84 or GRS80 ellipsoid.",
     inputs=[_degin("lat", "geodetic latitude"), _degin("lon", "longitude"), _phys("h", "length", "ellipsoidal height"), _ellin()],
     outputs=[_o("X", "length", "X (towards latitude 0, longitude 0)"), _o("Y", "length", "Y (towards longitude 90 E)"), _o("Z", "length", "Z (towards the north pole)")],
     primary="X", conditions=["latitude between -90 and 90", "height between -100 km and 100000 km"],
     method="X = (N + h) cos(phi) cos(lambda), Y = (N + h) cos(phi) sin(lambda), Z = (N(1 - e^2) + h) sin(phi)",
     latex=r"X = (N+h)\cos\varphi\cos\lambda,\ Y = (N+h)\cos\varphi\sin\lambda,\ Z = \left(N(1-e^2)+h\right)\sin\varphi",
     run=geodetic_to_ecef, reference=_ref_geodetic_to_ecef, rand=lambda rng: dict(_rand_geo(rng), ellipsoid=rng.choice(["WGS84", "GRS80"])),
     wrong=lambda lat, lon, h, ellipsoid: (lambda E: (lambda N: _out(X=(N + h) * math.cos(lat * DEG) * math.cos(lon * DEG), Y=(N + h) * math.cos(lat * DEG) * math.sin(lon * DEG),
                                                                    Z=(N + h) * math.sin(lat * DEG)))(_N(E, lat * DEG)))(_ellipsoid(ellipsoid)),
     phrases=("Geodetic to ECEF", "Geodetic to geocentric coordinates", "Latitude longitude to ECEF"),
     examples=[_ex({"lat": 45, "lon": 10, "h": "100 m", "ellipsoid": "WGS84"}), _ex({"lat": -33.8568, "lon": 151.2153, "h": "50 m", "ellipsoid": "GRS80"})],
     level="beginner")


def ecef_to_geodetic(X, Y, Z, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _fin("X, Y and Z", X, Y, Z)
    r = math.sqrt(X * X + Y * Y + Z * Z)
    _need(6.2e6 <= r <= 1e8, "the point must lie between 6200 km and 100000 km from the Earth's centre")
    lat, lon, h = _geodetic(E, X, Y, Z)
    return _out(lat=lat, lon=lon, h=h)


def _refine(fwd, target, x0):
    """Polish an inverse by solving fwd(x) = target with scipy's fsolve (PROJ's closed-form inverses lose digits far
    from the ellipsoid); fwd is the library's own forward conversion."""
    from scipy.optimize import fsolve
    import numpy as np
    x = fsolve(lambda v: (np.array(fwd(*v)) - np.array(target)) / max(1.0, float(np.max(np.abs(target)))), np.array(x0, dtype=float),
               xtol=1e-15, maxfev=400)
    return [float(v) for v in x]


def _ref_ecef_to_geodetic(X, Y, Z, ellipsoid):
    t = _tr(f"+proj=longlat +ellps={ellipsoid} +no_defs +type=crs", f"+proj=geocent +ellps={ellipsoid} +type=crs")
    lon, lat, h = t.transform(X, Y, Z, direction="INVERSE")
    lon, lat, h = _refine(lambda a, b, c: t.transform(a, b, c), (X, Y, Z), (lon, lat, h))
    return {"lat": lat, "lon": _nlon(lon), "h": h}


def _rand_ecef(rng):
    E = rng.choice(["WGS84", "GRS80"])
    x = _rand_geo(rng)
    x["h"] = _sig(rng.choice([rng.uniform(-400, 9000), rng.uniform(2e5, 4e7)]), 6)
    X, Y, Z = _ecef(_ellipsoid(E), x["lat"], x["lon"], x["h"])
    return {"X": round(X, 3), "Y": round(Y, 3), "Z": round(Z, 3), "ellipsoid": E}


_add("geo_ecef_to_geodetic",
     name="Geocentric (ECEF) to geodetic coordinates", topic=_COORD,
     purpose="Geodetic latitude, longitude and ellipsoidal height of a point given by its Earth-centred, Earth-fixed coordinates X, Y, Z on the WGS84 or GRS80 ellipsoid.",
     inputs=[_phys("X", "length", "geocentric X"), _phys("Y", "length", "geocentric Y"), _phys("Z", "length", "geocentric Z"), _ellin()],
     outputs=[_o("lat", "number", "geodetic latitude (degrees)"), _o("lon", "number", "longitude (degrees)"), _o("h", "length", "ellipsoidal height")],
     primary="lat", conditions=["6200 km <= distance from the centre <= 100000 km", "not on the polar axis"],
     method="fixed-point iteration phi = atan2(Z + e^2 N sin(phi), p) to convergence; h = p cos(phi) + Z sin(phi) - a^2/N",
     latex=r"\varphi = \operatorname{atan2}\!\left(Z + e^2N\sin\varphi,\ \sqrt{X^2+Y^2}\right),\quad h = p\cos\varphi + Z\sin\varphi - \frac{a^2}{N}",
     run=ecef_to_geodetic, reference=_ref_ecef_to_geodetic, rand=_rand_ecef,
     wrong=lambda X, Y, Z, ellipsoid: (lambda E: (lambda p: _out(lat=math.atan2(Z, p) / DEG, lon=math.atan2(Y, X) / DEG,
                                                              h=math.sqrt(X * X + Y * Y + Z * Z) - E["a"]))(math.hypot(X, Y)))(_ellipsoid(ellipsoid)),
     phrases=("ECEF to geodetic", "Geocentric to geodetic coordinates", "ECEF to latitude longitude"),
     examples=[_ex({"X": "4517590.879 m", "Y": "796576.168 m", "Z": "4487348.409 m", "ellipsoid": "WGS84"}),
               _ex({"X": "-4646678.4 m", "Y": "2553002.1 m", "Z": "-3534200.6 m", "ellipsoid": "GRS80"})],
     level="intermediate")


def _enu_rot(lat0, lon0):
    sp, cp = _sincos_deg(lat0)
    sl, cl = _sincos_deg(lon0)
    return [[-sl, cl, 0.0], [-sp * cl, -sp * sl, cp], [cp * cl, cp * sl, sp]]


def geodetic_to_enu(lat0, lon0, h0, lat, lon, h, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _geo_point(lat0, lon0, h0, "the origin's ")
    _geo_point(lat, lon, h)
    _need(abs(lat0) < 90, "the origin must not be a pole")
    o = _ecef(E, lat0, lon0, h0)
    t = _ecef(E, lat, lon, h)
    d = [t[i] - o[i] for i in range(3)]
    R = _enu_rot(lat0, lon0)
    e, n, u = (math.fsum(R[i][j] * d[j] for j in range(3)) for i in range(3))
    return _out(e=e, n=n, u=u, slant_range=math.sqrt(math.fsum(x * x for x in d)))


def _topo(ell, lat0, lon0, h0):
    import pyproj
    return pyproj.Transformer.from_pipeline(f"+proj=pipeline +step +proj=unitconvert +xy_in=deg +xy_out=rad +step +proj=cart +ellps={ell} "
                                            f"+step +proj=topocentric +ellps={ell} +lat_0={lat0!r} +lon_0={lon0!r} +h_0={h0!r}")


def _ref_geodetic_to_enu(lat0, lon0, h0, lat, lon, h, ellipsoid):
    e, n, u = _topo(ellipsoid, lat0, lon0, h0).transform(lon, lat, h)
    t = _tr(f"+proj=longlat +ellps={ellipsoid} +no_defs +type=crs", f"+proj=geocent +ellps={ellipsoid} +type=crs")
    a, b = t.transform(lon0, lat0, h0), t.transform(lon, lat, h)
    import numpy as np
    return {"e": e, "n": n, "u": u, "slant_range": float(np.linalg.norm(np.array(b) - np.array(a)))}


def _rand_enu(rng):
    x = _rand_geo(rng, "")
    o = {"lat0": _sig(rng.uniform(-80, 80)), "lon0": _sig(rng.uniform(-179, 179)), "h0": _sig(rng.uniform(0, 3000), 5)}
    d = _lu(rng, 0.01, 20)
    x["lat"] = _sig(max(-89.0, min(89.0, o["lat0"] + rng.uniform(-d, d))))
    x["lon"] = _sig(_nlon(o["lon0"] + rng.uniform(-d, d)))
    if rng.random() < 0.3:
        x["h"] = _sig(rng.uniform(2e5, 2e7))
    return dict(o, **x, ellipsoid=rng.choice(["WGS84", "GRS80"]))


_ENU_IN = [_degin("lat0", "latitude of the local origin"), _degin("lon0", "longitude of the local origin"), _phys("h0", "length", "ellipsoidal height of the origin")]
_add("geo_geodetic_to_enu",
     name="Geodetic to local east-north-up (ENU) coordinates", topic=_COORD,
     purpose="Local tangent-plane coordinates east, north and up of a point relative to an origin (both given by latitude, longitude and ellipsoidal height), and the straight-line (slant) range.",
     inputs=_ENU_IN + [_degin("lat", "latitude of the point"), _degin("lon", "longitude of the point"), _phys("h", "length", "ellipsoidal height of the point"), _ellin()],
     outputs=[_o("e", "length", "east coordinate"), _o("n", "length", "north coordinate"), _o("u", "length", "up coordinate"),
              _o("slant_range", "length", "straight-line distance from the origin")],
     primary="e", conditions=["origin not at a pole"],
     method="ECEF difference rotated into the local frame: [e n u] = R(lat0, lon0) (X - X0)",
     latex=r"\begin{bmatrix}e\\n\\u\end{bmatrix} = \begin{bmatrix}-\sin\lambda_0 & \cos\lambda_0 & 0\\ -\sin\varphi_0\cos\lambda_0 & -\sin\varphi_0\sin\lambda_0 & \cos\varphi_0\\ \cos\varphi_0\cos\lambda_0 & \cos\varphi_0\sin\lambda_0 & \sin\varphi_0\end{bmatrix}\begin{bmatrix}\Delta X\\ \Delta Y\\ \Delta Z\end{bmatrix}",
     run=geodetic_to_enu, reference=_ref_geodetic_to_enu, rand=_rand_enu,
     wrong=lambda lat0, lon0, h0, lat, lon, h, ellipsoid: (lambda r: _out(e=r["n"], n=r["e"], u=r["u"], slant_range=r["slant_range"]))(geodetic_to_enu(lat0, lon0, h0, lat, lon, h, ellipsoid)),
     phrases=("Geodetic to ENU", "Local ENU coordinates", "East north up coordinates"),
     examples=[_ex({"lat0": 46.017, "lon0": 7.750, "h0": "1673 m", "lat": 45.976, "lon": 7.658, "h": "4531 m", "ellipsoid": "WGS84"}),
               _ex({"lat0": 0, "lon0": 0, "h0": "0 m", "lat": 0.5, "lon": 0.5, "h": "1000 m", "ellipsoid": "GRS80"})],
     level="intermediate")


def enu_to_geodetic(lat0, lon0, h0, e, n, u, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _geo_point(lat0, lon0, h0, "the origin's ")
    _need(abs(lat0) < 90, "the origin must not be a pole")
    _fin("e, n and u", e, n, u)
    _need(max(abs(e), abs(n), abs(u)) <= 5e7, "the local coordinates must be at most 50000 km")
    o = _ecef(E, lat0, lon0, h0)
    R = _enu_rot(lat0, lon0)
    v = (e, n, u)
    X, Y, Z = (o[j] + math.fsum(R[i][j] * v[i] for i in range(3)) for j in range(3))
    _need(math.sqrt(X * X + Y * Y + Z * Z) >= 6.2e6, "the point would lie deep inside the Earth")
    lat, lon, h = _geodetic(E, X, Y, Z)
    return _out(lat=lat, lon=lon, h=h)


def _ref_enu_to_geodetic(lat0, lon0, h0, e, n, u, ellipsoid):
    t = _topo(ellipsoid, lat0, lon0, h0)
    lon, lat, h = t.transform(e, n, u, direction="INVERSE")
    lon, lat, h = _refine(lambda a, b, c: t.transform(a, b, c), (e, n, u), (lon, lat, h))
    return {"lat": lat, "lon": _nlon(lon), "h": h}


def _rand_enu_inv(rng):
    x = _rand_enu(rng)
    r = geodetic_to_enu(**x)
    return {"lat0": x["lat0"], "lon0": x["lon0"], "h0": x["h0"], "e": round(r["e"], 3), "n": round(r["n"], 3), "u": round(r["u"], 3), "ellipsoid": x["ellipsoid"]}


_add("geo_enu_to_geodetic",
     name="Local east-north-up (ENU) to geodetic coordinates", topic=_COORD,
     purpose="Latitude, longitude and ellipsoidal height of a point given by its local east, north and up coordinates relative to an origin on the WGS84 or GRS80 ellipsoid.",
     inputs=_ENU_IN + [_phys("e", "length", "east coordinate"), _phys("n", "length", "north coordinate"), _phys("u", "length", "up coordinate"), _ellin()],
     outputs=[_o("lat", "number", "latitude (degrees)"), _o("lon", "number", "longitude (degrees)"), _o("h", "length", "ellipsoidal height")],
     primary="lat", conditions=["origin not at a pole", "the point not deep inside the Earth"],
     method="rotate [e n u] back to an ECEF offset, add the origin's ECEF position, then convert ECEF to geodetic by iteration",
     latex=r"\mathbf{X} = \mathbf{X}_0 + R^{T}(\varphi_0,\lambda_0)\,[e\ n\ u]^{T}",
     run=enu_to_geodetic, reference=_ref_enu_to_geodetic, rand=_rand_enu_inv,
     wrong=lambda lat0, lon0, h0, e, n, u, ellipsoid: enu_to_geodetic(lat0, lon0, h0, n, e, u, ellipsoid),
     phrases=("ENU to geodetic", "Local ENU to geodetic coordinates", "East north up to latitude longitude"),
     examples=[_ex({"lat0": 46.017, "lon0": 7.75, "h0": "1673 m", "e": "-7123.1 m", "n": "-4557.3 m", "u": "2848.2 m", "ellipsoid": "WGS84"}),
               _ex({"lat0": 0, "lon0": 0, "h0": "0 m", "e": "1000 m", "n": "2000 m", "u": "10 m", "ellipsoid": "GRS80"})],
     level="advanced")


def look_angles(lat0, lon0, h0, lat, lon, h, ellipsoid):
    r = geodetic_to_enu(lat0, lon0, h0, lat, lon, h, ellipsoid)
    _need(r["slant_range"] > 0, "the target must differ from the observer")
    e, n, u = r["e"], r["n"], r["u"]
    E = _ellipsoid(ellipsoid)
    o = _ecef(E, lat0, lon0, h0)
    t = _ecef(E, lat, lon, h)
    d = [t[i] - o[i] for i in range(3)]
    R = _enu_rot(lat0, lon0)
    e, n, u = (math.fsum(R[i][j] * d[j] for j in range(3)) for i in range(3))
    return _out(azimuth=_az(math.atan2(e, n) / DEG), elevation=math.atan2(u, math.hypot(e, n)) / DEG, slant_range=r["slant_range"])


def _ref_look_angles(lat0, lon0, h0, lat, lon, h, ellipsoid):
    e, n, u = _topo(ellipsoid, lat0, lon0, h0).transform(lon, lat, h)
    return {"azimuth": _az(math.degrees(math.atan2(e, n))), "elevation": math.degrees(math.atan2(u, math.hypot(e, n))),
            "slant_range": math.sqrt(e * e + n * n + u * u)}


def _rand_look(rng):
    for _ in range(100):
        x = _rand_enu(rng)
        o = look_angles(**x)
        if _not_near_north(o["azimuth"]) and abs(o["elevation"]) > 0.01:
            return x
    return x


_add("geo_look_angles",
     name="Look angles: azimuth, elevation and range from an observer to a target", topic=_COORD,
     purpose="The azimuth (clockwise from true north), elevation angle above the local horizontal plane and slant range from an observer to a target, both given by latitude, longitude and ellipsoidal height (a satellite, aircraft or summit).",
     inputs=[_degin("lat0", "observer latitude"), _degin("lon0", "observer longitude"), _phys("h0", "length", "observer ellipsoidal height"),
             _degin("lat", "target latitude"), _degin("lon", "target longitude"), _phys("h", "length", "target ellipsoidal height"), _ellin()],
     outputs=[_o("azimuth", "number", "azimuth (degrees clockwise from north)"), _o("elevation", "number", "elevation angle (degrees, negative below the horizon)"),
              _o("slant_range", "length", "straight-line distance")],
     primary="elevation", conditions=["observer not at a pole", "target distinct from the observer"],
     method="ENU vector of the target; azimuth = atan2(e, n), elevation = atan2(u, sqrt(e^2 + n^2))",
     latex=r"A = \operatorname{atan2}(e, n),\quad \varepsilon = \operatorname{atan2}\!\left(u, \sqrt{e^2+n^2}\right)",
     run=look_angles, reference=_ref_look_angles, rand=_rand_look,
     wrong=lambda lat0, lon0, h0, lat, lon, h, ellipsoid: (lambda r: _out(azimuth=_az(math.atan2(r["n"], r["e"]) / DEG), elevation=math.atan2(r["u"], r["slant_range"]) / DEG * 0.5,
                                                                          slant_range=r["slant_range"]))(geodetic_to_enu(lat0, lon0, h0, lat, lon, h, ellipsoid)),
     phrases=("Look angles", "Azimuth and elevation", "Azimuth elevation and range"),
     examples=[_ex({"lat0": 40.0, "lon0": -105.0, "h0": "1600 m", "lat": 0.0, "lon": -100.0, "h": "35786000 m", "ellipsoid": "WGS84"}),
               _ex({"lat0": 46.017, "lon0": 7.75, "h0": "1673 m", "lat": 45.976, "lon": 7.658, "h": "4478 m", "ellipsoid": "WGS84"})],
     level="advanced", weak="pyproj's topocentric conversion gives the ENU vector; azimuth and elevation are then the same atan2 definitions")


# ---------------------------------------------------------------- auxiliary latitudes
def _taup(E, tau):
    e = E["e"]
    sig = math.sinh(e * math.atanh(e * tau / math.sqrt(1 + tau * tau)))
    return tau * math.sqrt(1 + sig * sig) - sig * math.sqrt(1 + tau * tau)


def _tau_of_taup(E, taup):
    e2 = E["e2"]
    tau = taup / (1 - e2)
    for _ in range(50):
        tp = _taup(E, tau)
        d = (taup - tp) / math.sqrt(1 + tp * tp) * (1 + (1 - e2) * tau * tau) / ((1 - e2) * math.sqrt(1 + tau * tau))
        tau += d
        if abs(d) <= 1e-15 * max(1.0, abs(tau)):
            break
    return tau


def auxiliary_latitudes(lat, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _lat("lat", lat)
    p = lat * DEG
    geoc = math.atan2((1 - E["e2"]) * math.sin(p), math.cos(p))
    red = math.atan2((1 - E["f"]) * math.sin(p), math.cos(p))
    conf = math.atan(_taup(E, math.tan(p)))
    auth = math.asin(max(-1.0, min(1.0, _q_auth(E, p) / _q_auth(E, math.pi / 2))))
    rect = math.pi / 2 * _merid(E, p) / _merid(E, math.pi / 2)
    return _out(geocentric=geoc / DEG, reduced=red / DEG, conformal=conf / DEG, authalic=auth / DEG, rectifying=rect / DEG)


def _ref_aux(lat, ellipsoid):
    import pyproj
    E = _ellipsoid(ellipsoid)
    X, Y, Z = _tr(_ll(ellipsoid), f"+proj=geocent +ellps={ellipsoid} +type=crs").transform(0.0, lat, 0.0)
    geoc = math.degrees(math.atan2(Z, X))
    red = math.degrees(math.atan2(Z / E["b"], X / E["a"]))
    y = pyproj.Proj(f"+proj=merc +ellps={ellipsoid}")(0.0, lat)[1]
    conf = math.degrees(math.atan(math.sinh(y / E["a"])))
    cea = pyproj.Proj(f"+proj=cea +ellps={ellipsoid}")
    auth = math.degrees(math.asin(cea(0.0, lat)[1] / cea(0.0, 90.0)[1]))
    g = _geod(ellipsoid)
    m = math.copysign(g.inv(0.0, 0.0, 0.0, abs(lat))[2], lat)
    rect = 90.0 * m / g.inv(0.0, 0.0, 0.0, 90.0)[2]
    return {"geocentric": geoc, "reduced": red, "conformal": conf, "authalic": auth, "rectifying": rect}


_add("geo_auxiliary_latitudes",
     name="Auxiliary latitudes: geocentric, reduced, conformal, authalic and rectifying", topic=_COORD,
     purpose="The five auxiliary latitudes of a geodetic latitude on the WGS84 or GRS80 ellipsoid (used by the sphere-based forms of map projections and geodesics).",
     inputs=[_degin("lat", "geodetic latitude"), _ellin()],
     outputs=[_o("geocentric", "number", "geocentric latitude (degrees)"), _o("reduced", "number", "reduced (parametric) latitude (degrees)"),
              _o("conformal", "number", "conformal latitude (degrees)"), _o("authalic", "number", "authalic latitude (degrees)"),
              _o("rectifying", "number", "rectifying latitude (degrees)")],
     primary="geocentric", conditions=["latitude strictly between -90 and 90"],
     method="tan(psi) = (1-e^2) tan(phi); tan(beta) = (1-f) tan(phi); chi = atan(sinh(isometric latitude)); xi = asin(q/q_p); mu = (pi/2) m(phi)/m(pi/2)",
     latex=r"\tan\psi = (1-e^2)\tan\varphi,\ \tan\beta = (1-f)\tan\varphi,\ \chi = \operatorname{gd}\psi_{iso},\ \xi = \arcsin\frac{q}{q_p},\ \mu = \frac{\pi}{2}\frac{m(\varphi)}{m(\pi/2)}",
     run=auxiliary_latitudes, reference=_ref_aux, rand=lambda rng: {"lat": _sig(rng.uniform(-89, 89)), "ellipsoid": rng.choice(["WGS84", "GRS80"])},
     wrong=lambda lat, ellipsoid: (lambda r: dict(r, geocentric=r["reduced"], authalic=r["rectifying"]))(auxiliary_latitudes(lat, ellipsoid)),
     phrases=("Auxiliary latitudes", "Geocentric and auxiliary latitudes"),
     examples=[_ex({"lat": 45, "ellipsoid": "WGS84"}), _ex({"lat": -30, "ellipsoid": "GRS80"})],
     level="expert")


# ---------------------------------------------------------------- UTM zones
_BANDS = "CDEFGHJKLMNPQRSTUVWXX"


def _utm_zone(lat, lon):
    lon = _nlon(lon)
    if lon == 180.0:
        lon = -180.0
    z = int(math.floor((lon + 180.0) / 6.0)) + 1
    if 56 <= lat < 64 and 3 <= lon < 12:
        z = 32
    if 72 <= lat <= 84 and lon >= 0:
        if lon < 9:
            z = 31
        elif lon < 21:
            z = 33
        elif lon < 33:
            z = 35
        elif lon < 42:
            z = 37
    return z


def utm_zone(lat, lon):
    _lat("lat", lat, True)
    _lon("lon", lon)
    _need(-80 <= lat <= 84, "UTM covers latitudes from 80 S to 84 N (polar regions use UPS)")
    z = _utm_zone(lat, lon)
    return {"zone": z, "band": _BANDS[int((lat + 80) // 8)], "hemisphere": "N" if lat >= 0 else "S", "central_meridian": float(6 * z - 183)}


def _ref_utm_zone(lat, lon):
    import utm
    z = int(utm.latlon_to_zone_number(lat, lon))
    return {"zone": z, "band": utm.latitude_to_zone_letter(lat), "hemisphere": "N" if lat >= 0 else "S", "central_meridian": float(utm.zone_number_to_central_longitude(z))}


_add("geo_utm_zone",
     name="UTM zone, latitude band and central meridian", topic=_COORD,
     purpose="The Universal Transverse Mercator zone number (with the Norway and Svalbard exceptions), the MGRS latitude band letter, the hemisphere and the zone's central meridian for a latitude and longitude.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude")],
     outputs=[_o("zone", "number", "UTM zone number (1-60)"), _o("band", "text", "latitude band letter (C-X)"), _o("hemisphere", "text", "N or S"),
              _o("central_meridian", "number", "central meridian of the zone (degrees)")],
     primary="zone", conditions=["80 S <= latitude <= 84 N"],
     method="zone = floor((lon + 180)/6) + 1 with zone 32 widened over south-west Norway and zones 31/33/35/37 over Svalbard; band = 8-degree letter from 80 S (X spans 72-84 N)",
     latex=r"z = \left\lfloor\frac{\lambda + 180^\circ}{6^\circ}\right\rfloor + 1,\quad \lambda_0 = 6z - 183^\circ",
     run=utm_zone, reference=_ref_utm_zone,
     rand=lambda rng: rng.choice([{"lat": _sig(rng.uniform(-80, 84)), "lon": _sig(rng.uniform(-180, 179.9))},
                                  {"lat": _sig(rng.uniform(56, 84)), "lon": _sig(rng.uniform(0, 42))}]),
     wrong=lambda lat, lon: (lambda r: dict(r, zone=r["zone"] - 1 if r["zone"] > 1 else 60, central_meridian=r["central_meridian"] - 6))(utm_zone(lat, lon)),
     phrases=("UTM zone", "UTM zone and band"),
     examples=[_ex({"lat": 51.5, "lon": -0.13}), _ex({"lat": 60.0, "lon": 5.3}), _ex({"lat": 78.2, "lon": 15.6}), _ex({"lat": -33.9, "lon": 18.4})],
     level="beginner",
     extra_q=[{"q": "UTM zone: lat = 85, lon = 10", "outcome": "INVALID_INPUT"}],
     weak="the utm library computes the same zone and band definitions (no different route exists for a lookup rule)")


# ---------------------------------------------------------------- transverse Mercator (Krueger series, 6th order)
def _kr(n):
    n2, n3, n4, n5, n6 = n ** 2, n ** 3, n ** 4, n ** 5, n ** 6
    alpha = [n / 2 - 2 * n2 / 3 + 5 * n3 / 16 + 41 * n4 / 180 - 127 * n5 / 288 + 7891 * n6 / 37800,
             13 * n2 / 48 - 3 * n3 / 5 + 557 * n4 / 1440 + 281 * n5 / 630 - 1983433 * n6 / 1935360,
             61 * n3 / 240 - 103 * n4 / 140 + 15061 * n5 / 26880 + 167603 * n6 / 181440,
             49561 * n4 / 161280 - 179 * n5 / 168 + 6601661 * n6 / 7257600,
             34729 * n5 / 80640 - 3418889 * n6 / 1995840,
             212378941 * n6 / 319334400]
    beta = [n / 2 - 2 * n2 / 3 + 37 * n3 / 96 - n4 / 360 - 81 * n5 / 512 + 96199 * n6 / 604800,
            n2 / 48 + n3 / 15 - 437 * n4 / 1440 + 46 * n5 / 105 - 1118711 * n6 / 3870720,
            17 * n3 / 480 - 37 * n4 / 840 - 209 * n5 / 4480 + 5569 * n6 / 90720,
            4397 * n4 / 161280 - 11 * n5 / 504 - 830251 * n6 / 7257600,
            4583 * n5 / 161280 - 108847 * n6 / 3991680,
            20648693 * n6 / 638668800]
    A = 1 / (1 + n) * (1 + n2 / 4 + n4 / 64 + n6 / 256)
    return alpha, beta, A


def _tm_fwd(E, phi, lam):
    """Krueger's series: (A xi, A eta, gamma (rad), k / k0) for latitude phi and longitude lam from the central meridian."""
    alpha, _, A1 = _kr(E["n"])
    A = A1 * E["a"]
    tau = math.tan(phi)
    taup = _taup(E, tau)
    cl, sl = math.cos(lam), math.sin(lam)
    xip = math.atan2(taup, cl)
    etap = math.asinh(sl / math.hypot(taup, cl))
    xi, eta, p, q = xip, etap, 1.0, 0.0
    for j, al in enumerate(alpha, start=1):
        c2, s2 = math.cos(2 * j * xip), math.sin(2 * j * xip)
        ch, sh = math.cosh(2 * j * etap), math.sinh(2 * j * etap)
        xi += al * s2 * ch
        eta += al * c2 * sh
        p += 2 * j * al * c2 * ch
        q += 2 * j * al * s2 * sh
    gamma = math.atan(taup / math.sqrt(1 + taup * taup) * math.tan(lam)) + math.atan2(q, p)
    k = A1 * math.sqrt(1 + (1 - E["e2"]) * tau * tau) / math.hypot(taup, cl) * math.hypot(p, q)
    return A * xi, A * eta, gamma, k


def _tm_inv(E, X, Y):
    """Inverse of _tm_fwd: (phi, lam) from (A xi, A eta)."""
    _, beta, A1 = _kr(E["n"])
    A = A1 * E["a"]
    xi, eta = X / A, Y / A
    xip, etap = xi, eta
    for j, be in enumerate(beta, start=1):
        xip -= be * math.sin(2 * j * xi) * math.cosh(2 * j * eta)
        etap -= be * math.cos(2 * j * xi) * math.sinh(2 * j * eta)
    taup = math.sin(xip) / math.hypot(math.sinh(etap), math.cos(xip))
    lam = math.atan2(math.sinh(etap), math.cos(xip))
    return math.atan(_tau_of_taup(E, taup)), lam


def tm_forward(lat, lon, lat0, lon0, k0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _lat("lat", lat)
    _lat("lat0", lat0)
    _lon("lon", lon)
    _lon("lon0", lon0)
    _pos("k0", k0)
    _fin("the false easting and northing", FE, FN)
    _need(abs(lat) <= 89, "the latitude must lie within 89 degrees of the equator")
    dl = _nlon(lon - lon0)
    _need(abs(dl) <= 10, "the point must lie within 10 degrees of longitude of the central meridian")
    X, Y, g, k = _tm_fwd(E, lat * DEG, dl * DEG)
    X0 = _tm_fwd(E, lat0 * DEG, 0.0)[0]
    return _out(x=FE + k0 * Y, y=FN + k0 * (X - X0), k=k0 * k, gamma=g / DEG)


def _tmerc(lat0, lon0, k0, ell, FE, FN):
    import pyproj
    return pyproj.Proj(f"+proj=tmerc +algo=poder_engsager +lat_0={lat0!r} +lon_0={lon0!r} +k_0={k0!r} +x_0={FE!r} +y_0={FN!r} +ellps={ell}")


def _ref_tm_forward(lat, lon, lat0, lon0, k0, ellipsoid, FE=0.0, FN=0.0):
    P = _tmerc(lat0, lon0, k0, ellipsoid, FE, FN)
    x, y = P(lon, lat)
    f = P.get_factors(lon, lat)
    return {"x": x, "y": y, "k": f.parallel_scale, "gamma": f.meridian_convergence}


def _rand_tm(rng):
    for _ in range(100):
        x = {"lat": _sig(rng.uniform(-80, 80)), "lat0": _sig(rng.uniform(-60, 60)), "lon0": _sig(rng.uniform(-180, 180)),
             "k0": rng.choice([1.0, 0.9996, 0.9999, _sig(rng.uniform(0.999, 1.0), 6)]), "ellipsoid": rng.choice(["WGS84", "GRS80"]),
             "FE": _sig(rng.choice([0.0, 500000.0, rng.uniform(0, 1e6)]), 7), "FN": _sig(rng.choice([0.0, rng.uniform(-1e6, 1e7)]), 7)}
        x["lon"] = _sig(_nlon(x["lon0"] + rng.uniform(-8, 8)))
        o = tm_forward(**x)
        if abs(o["gamma"]) > 0.05 and abs(o["x"]) > 1 and abs(o["y"]) > 1:
            return x
    return x


_add("geo_transverse_mercator_forward",
     name="Transverse Mercator projection: coordinates, scale factor and grid convergence", topic="map projections",
     purpose="Easting and northing of a point in a transverse Mercator projection with a given origin, central-meridian scale factor and false origin on the WGS84 or GRS80 ellipsoid, with the point scale factor and the grid convergence.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lat0", "latitude of origin"), _degin("lon0", "central meridian"),
             _numin("k0", "scale factor on the central meridian"), _ellin(),
             _phys("FE", "length", "false easting (default 0)", required=False), _phys("FN", "length", "false northing (default 0)", required=False)],
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing"), _o("k", "number", "point scale factor"),
              _o("gamma", "number", "grid convergence (degrees, positive east of the central meridian in the north)")],
     primary="x", conditions=["within 10 degrees of longitude of the central meridian", "latitude within 89 degrees of the equator"],
     method="Krueger's series to sixth order in the third flattening (conformal latitude, then the Gauss-Krueger sums); scale and convergence from the series' derivatives (Karney 2011)",
     latex=r"\xi = \xi' + \sum_{j=1}^{6}\alpha_j\sin2j\xi'\cosh2j\eta',\quad \eta = \eta' + \sum_{j=1}^{6}\alpha_j\cos2j\xi'\sinh2j\eta',\quad x = FE + k_0A\eta,\ y = FN + k_0A(\xi - \xi_0)",
     run=tm_forward, reference=_ref_tm_forward, rand=_rand_tm,
     wrong=lambda lat, lon, lat0, lon0, k0, ellipsoid, FE=0.0, FN=0.0: (lambda r: dict(r, gamma=-r["gamma"], x=2 * FE - r["x"]))(tm_forward(lat, lon, lat0, lon0, k0, ellipsoid, FE, FN)),
     phrases=("Transverse Mercator forward", "Transverse Mercator projection", "Gauss-Krueger projection"),
     examples=[_ex({"lat": 52.5, "lon": 13.4, "lat0": 0, "lon0": 15, "k0": 0.9996, "ellipsoid": "GRS80", "FE": "500000 m", "FN": "0 m"}),
               _ex({"lat": 51.5, "lon": -0.13, "lat0": 49, "lon0": -2, "k0": 0.9996012717, "ellipsoid": "WGS84", "FE": "400000 m", "FN": "-100000 m"})],
     level="advanced")


def utm_forward(lat, lon, ellipsoid):
    E = _ellipsoid(ellipsoid)
    z = utm_zone(lat, lon)
    _need(abs(_nlon(lon - z["central_meridian"])) <= 10, "the point lies too far from its zone's central meridian")
    X, Y, g, k = _tm_fwd(E, lat * DEG, _nlon(lon - z["central_meridian"]) * DEG)
    FN = 0.0 if lat >= 0 else 1e7
    return dict(_out(easting=500000.0 + 0.9996 * Y, northing=FN + 0.9996 * X, k=0.9996 * k, gamma=g / DEG), zone=z["zone"], hemisphere=z["hemisphere"])


def _ref_utm_forward(lat, lon, ellipsoid):
    import pyproj
    import utm
    z = int(utm.latlon_to_zone_number(lat, lon))
    P = pyproj.Proj(f"+proj=utm +algo=poder_engsager +zone={z} {'+south ' if lat < 0 else ''}+ellps={ellipsoid}")
    x, y = P(lon, lat)
    f = P.get_factors(lon, lat)
    return {"easting": x, "northing": y, "k": f.parallel_scale, "gamma": f.meridian_convergence, "zone": z, "hemisphere": "N" if lat >= 0 else "S"}


def _rand_utm(rng):
    for _ in range(100):
        x = {"lat": _sig(rng.uniform(-79.9, 83.9)), "lon": _sig(rng.uniform(-179.9, 179.9)), "ellipsoid": rng.choice(["WGS84", "GRS80"])}
        if rng.random() < 0.2:
            x = {"lat": _sig(rng.uniform(56, 83.9)), "lon": _sig(rng.uniform(0.1, 41.9)), "ellipsoid": x["ellipsoid"]}
        o = utm_forward(**x)
        if abs(o["gamma"]) > 0.05 and abs(o["northing"]) > 1 and abs(o["easting"] - 500000) > 1:
            return x
    return x


_add("geo_utm_forward",
     name="Latitude and longitude to UTM coordinates (with scale factor and convergence)", topic="map projections",
     purpose="UTM zone, hemisphere, easting and northing of a point on the WGS84 or GRS80 ellipsoid, with the point scale factor and grid convergence.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _ellin()],
     outputs=[_o("easting", "length", "UTM easting"), _o("northing", "length", "UTM northing"), _o("zone", "number", "UTM zone number"),
              _o("hemisphere", "text", "N or S"), _o("k", "number", "point scale factor"), _o("gamma", "number", "grid convergence (degrees)")],
     primary="easting", conditions=["80 S <= latitude <= 84 N"],
     method="zone from the longitude (Norway and Svalbard exceptions), then transverse Mercator by Krueger's series with k0 = 0.9996, false easting 500 km, false northing 10000 km in the south",
     latex=r"E = 500\,000 + 0.9996\,A\eta,\quad N = N_0 + 0.9996\,A\xi",
     run=utm_forward, reference=_ref_utm_forward, rand=_rand_utm,
     wrong=lambda lat, lon, ellipsoid: (lambda r: dict(r, easting=r["easting"] / 0.9996 - 500000 / 0.9996 + 500000, northing=r["northing"] / 0.9996, k=r["k"] / 0.9996))(utm_forward(lat, lon, ellipsoid)),
     phrases=("UTM forward", "Latitude longitude to UTM", "UTM coordinates"),
     examples=[_ex({"lat": 51.5, "lon": -0.13, "ellipsoid": "WGS84"}), _ex({"lat": -33.9, "lon": 18.4, "ellipsoid": "WGS84"}),
               _ex({"lat": 60.4, "lon": 5.3, "ellipsoid": "GRS80"})],
     level="intermediate")


def utm_inverse(easting, northing, zone, hemisphere, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _fin("easting and northing", easting, northing)
    _need(isinstance(zone, int) and 1 <= zone <= 60, "the zone must be a whole number from 1 to 60")
    _need(hemisphere in ("N", "S"), "the hemisphere must be N or S")
    _need(1e5 <= easting <= 9e5, "the easting must lie between 100 km and 900 km")
    _need(0 <= northing <= 1e7, "the northing must lie between 0 and 10000 km")
    FN = 0.0 if hemisphere == "N" else 1e7
    phi, lam = _tm_inv(E, (northing - FN) / 0.9996, (easting - 500000.0) / 0.9996)
    _need(abs(phi / DEG) <= 85, "the coordinates lie outside the UTM latitude range")
    return _out(lat=phi / DEG, lon=_nlon(6 * zone - 183 + lam / DEG))


def _ref_utm_inverse(easting, northing, zone, hemisphere, ellipsoid):
    import pyproj
    P = pyproj.Proj(f"+proj=utm +algo=poder_engsager +zone={zone} {'+south ' if hemisphere == 'S' else ''}+ellps={ellipsoid}")
    lon, lat = P(easting, northing, inverse=True)
    return {"lat": lat, "lon": _nlon(lon)}


def _rand_utm_inv(rng):
    for _ in range(100):
        x = _rand_utm(rng)
        o = utm_forward(**x)
        if 1e5 <= o["easting"] <= 9e5:
            return {"easting": round(o["easting"], 3), "northing": round(o["northing"], 3), "zone": o["zone"], "hemisphere": o["hemisphere"], "ellipsoid": x["ellipsoid"]}
    return {"easting": 500100.0, "northing": 100000.0, "zone": 31, "hemisphere": "N", "ellipsoid": "WGS84"}


_add("geo_utm_inverse",
     name="UTM coordinates to latitude and longitude", topic="map projections",
     purpose="Latitude and longitude on the WGS84 or GRS80 ellipsoid of a point given by its UTM easting, northing, zone and hemisphere.",
     inputs=[_phys("easting", "length", "UTM easting"), _phys("northing", "length", "UTM northing"), _intin("zone", "UTM zone number", 1, 60),
             _choice("hemisphere", ["N", "S"], "hemisphere of the zone (N: false northing 0; S: false northing 10000 km)"), _ellin()],
     outputs=[_o("lat", "number", "latitude (degrees)"), _o("lon", "number", "longitude (degrees)")],
     primary="lat", conditions=["100 km <= easting <= 900 km", "0 <= northing <= 10000 km"],
     method="inverse Krueger series to the conformal latitude, then Newton iteration for the geodetic latitude",
     latex=r"\xi' = \xi - \sum_{j=1}^{6}\beta_j\sin2j\xi\cosh2j\eta,\quad \eta' = \eta - \sum_{j=1}^{6}\beta_j\cos2j\xi\sinh2j\eta",
     run=utm_inverse, reference=_ref_utm_inverse, rand=_rand_utm_inv,
     wrong=lambda easting, northing, zone, hemisphere, ellipsoid: utm_inverse(easting, northing, zone, "S" if hemisphere == "N" else "N", ellipsoid) if 0 else
         (lambda E: (lambda r: _out(lat=r[0] / DEG, lon=_nlon(6 * zone - 180 + r[1] / DEG)))(_tm_inv(E, (northing - (0 if hemisphere == "N" else 1e7)) / 0.9996, (easting - 500000) / 0.9996)))(_ellipsoid(ellipsoid)),
     phrases=("UTM inverse", "UTM to latitude longitude", "UTM to geographic"),
     examples=[_ex({"easting": "699316.0 m", "northing": "5710164.0 m", "zone": 30, "hemisphere": "N", "ellipsoid": "WGS84"}),
               _ex({"easting": "261878.0 m", "northing": "6243186.0 m", "zone": 34, "hemisphere": "S", "ellipsoid": "WGS84"})],
     level="intermediate")


# ================================================================================================ MAP PROJECTIONS
_PROJ = "map projections"


def _proj(s):
    import pyproj
    return pyproj.Proj(s)


def _pt(lat, lon, lat_lim=89.9):
    _lat("lat", lat)
    _lon("lon", lon)
    _need(abs(lat) <= lat_lim, f"the latitude must lie within {lat_lim:g} degrees of the equator")


# ---------------------------------------------------------------- Web Mercator
def web_mercator_forward(lat, lon):
    _pt(lat, lon, 85.06)
    return _out(x=WEB_MERCATOR_A * _nlon(lon) * DEG, y=WEB_MERCATOR_A * math.asinh(math.tan(lat * DEG)))


def _ref_web_mercator_forward(lat, lon):
    x, y = _tr("EPSG:4326", "EPSG:3857").transform(lon, lat)
    return {"x": x, "y": y}


_add("geo_web_mercator_forward",
     name="Latitude and longitude to Web Mercator (EPSG:3857)", topic=_PROJ,
     purpose="Web Mercator (pseudo-Mercator, EPSG:3857) x and y of a WGS84 latitude and longitude, as used by web maps.",
     inputs=[_degin("lat", "latitude (WGS84)"), _degin("lon", "longitude (WGS84)")],
     outputs=[_o("x", "length", "Web Mercator x"), _o("y", "length", "Web Mercator y")],
     primary="x", conditions=["|latitude| <= 85.06 degrees (the square web-map extent)"],
     method="spherical Mercator formulas on a sphere of radius 6378137 m applied to WGS84 coordinates",
     latex=r"x = R\lambda,\quad y = R\ln\tan\left(\frac{\pi}{4} + \frac{\varphi}{2}\right),\quad R = 6378137\ \mathrm{m}",
     run=web_mercator_forward, reference=_ref_web_mercator_forward,
     rand=lambda rng: {"lat": _sig(rng.choice([rng.uniform(-85, -0.01), rng.uniform(0.01, 85)])), "lon": _sig(rng.choice([rng.uniform(-179.9, -0.01), rng.uniform(0.01, 179.9)]))},
     wrong=lambda lat, lon: _out(x=WEB_MERCATOR_A * lon * DEG, y=_ellipsoid("WGS84")["a"] * _psi(_ellipsoid("WGS84"), lat * DEG)),
     phrases=("Web Mercator forward", "Latitude longitude to Web Mercator", "EPSG:3857 coordinates"),
     examples=[_ex({"lat": 51.5, "lon": -0.13}), _ex({"lat": -33.87, "lon": 151.21})],
     level="beginner")


def web_mercator_inverse(x, y):
    _fin("x and y", x, y)
    lim = WEB_MERCATOR_A * math.pi
    _need(abs(x) <= lim * (1 + 1e-12) and abs(y) <= lim * 1.0001, "x and y must lie within the web-map square (about 20037.5 km)")
    return _out(lat=math.atan(math.sinh(y / WEB_MERCATOR_A)) / DEG, lon=x / WEB_MERCATOR_A / DEG)


def _ref_web_mercator_inverse(x, y):
    lon, lat = _tr("EPSG:4326", "EPSG:3857").transform(x, y, direction="INVERSE")
    return {"lat": lat, "lon": lon}


_add("geo_web_mercator_inverse",
     name="Web Mercator (EPSG:3857) to latitude and longitude", topic=_PROJ,
     purpose="WGS84 latitude and longitude of a point given by its Web Mercator (pseudo-Mercator) x and y.",
     inputs=[_phys("x", "length", "Web Mercator x"), _phys("y", "length", "Web Mercator y")],
     outputs=[_o("lat", "number", "latitude (degrees)"), _o("lon", "number", "longitude (degrees)")],
     primary="lat", conditions=["inside the web-map square"],
     method="inverse spherical Mercator: lat = atan(sinh(y/R)), lon = x/R",
     latex=r"\varphi = \arctan\sinh\frac{y}{R},\quad \lambda = \frac{x}{R}",
     run=web_mercator_inverse, reference=_ref_web_mercator_inverse,
     rand=lambda rng: {"x": _sig(rng.uniform(-2e7, 2e7), 9), "y": _sig(rng.choice([rng.uniform(-2e7, -1e3), rng.uniform(1e3, 2e7)]), 9)},
     wrong=lambda x, y: _out(lat=2 * math.atan(math.exp(y / WEB_MERCATOR_A)) / DEG, lon=x / WEB_MERCATOR_A / DEG),
     phrases=("Web Mercator inverse", "Web Mercator to latitude longitude", "EPSG:3857 to geographic"),
     examples=[_ex({"x": "-14471.5 m", "y": "6710219.1 m"}), _ex({"x": "16832556.0 m", "y": "-4011000.0 m"})],
     level="beginner")


# ---------------------------------------------------------------- web-map tiles
def web_map_tile(lat, lon, zoom):
    _pt(lat, lon, 85.0511)
    _need(isinstance(zoom, int) and 0 <= zoom <= 24, "the zoom must be a whole number from 0 to 24")
    Z = 2 ** zoom
    xf = (_nlon(lon) + 180.0) / 360.0
    yf = 0.5 - math.asinh(math.tan(lat * DEG)) / (2 * math.pi)
    tx = min(Z - 1, max(0, int(math.floor(xf * Z))))
    ty = min(Z - 1, max(0, int(math.floor(yf * Z))))
    qk = "".join(str((1 if tx & (1 << (i - 1)) else 0) + (2 if ty & (1 << (i - 1)) else 0)) for i in range(zoom, 0, -1))
    return {"tile_x": tx, "tile_y": ty, "quadkey": qk}


def _ref_web_map_tile(lat, lon, zoom):
    import mercantile
    t = mercantile.tile(lon, lat, zoom)
    return {"tile_x": t.x, "tile_y": t.y, "quadkey": mercantile.quadkey(t)}


def _rand_tile(rng):
    for _ in range(100):
        x = {"lat": _sig(rng.uniform(-85, 85), 8), "lon": _sig(rng.uniform(-179.99, 179.99), 8), "zoom": rng.randint(1, 20)}
        Z = 2 ** x["zoom"]
        xf = (x["lon"] + 180) / 360 * Z
        yf = (0.5 - math.asinh(math.tan(x["lat"] * DEG)) / (2 * math.pi)) * Z
        if min(xf % 1, 1 - xf % 1, yf % 1, 1 - yf % 1) > 1e-6:
            return x
    return x


_add("geo_web_map_tile",
     name="Web-map tile (slippy map / XYZ) and quadkey containing a point", topic=_PROJ,
     purpose="The x and y indices of the web-map (XYZ / slippy map) tile containing a WGS84 point at a zoom level, and the tile's quadkey.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _intin("zoom", "zoom level (0-24)", 0, 24)],
     outputs=[_o("tile_x", "number", "tile column (from 180 W)"), _o("tile_y", "number", "tile row (from the north edge)"),
              _o("quadkey", "text", "quadkey of the tile")],
     primary="tile_x", conditions=["|latitude| <= 85.0511", "zoom 0-24"],
     method="Web Mercator fractions x = (lon + 180)/360, y = 1/2 - asinh(tan lat)/(2 pi), scaled by 2^zoom and floored; quadkey digits interleave the bits of x and y",
     latex=r"x_t = \left\lfloor 2^z\,\frac{\lambda + 180^\circ}{360^\circ}\right\rfloor,\quad y_t = \left\lfloor 2^z\left(\frac12 - \frac{\operatorname{arsinh}(\tan\varphi)}{2\pi}\right)\right\rfloor",
     run=web_map_tile, reference=_ref_web_map_tile, rand=_rand_tile,
     wrong=lambda lat, lon, zoom: (lambda r: dict(r, tile_y=2 ** zoom - 1 - r["tile_y"]))(web_map_tile(lat, lon, zoom)),
     phrases=("Web map tile", "Slippy map tile", "XYZ tile"),
     examples=[_ex({"lat": 51.5, "lon": -0.13, "zoom": 12}), _ex({"lat": -33.87, "lon": 151.21, "zoom": 8})],
     level="beginner", weak="mercantile computes the same Web Mercator tile definition")


# ---------------------------------------------------------------- Mercator (ellipsoidal)
def _merc_k0(E, lat_ts):
    return math.cos(lat_ts * DEG) / math.sqrt(1 - E["e2"] * math.sin(lat_ts * DEG) ** 2)


def mercator_forward(lat, lon, lon0, lat_ts, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _pt(lat, lon, 89.5)
    _lon("lon0", lon0)
    _lat("lat_ts", lat_ts)
    _need(abs(lat_ts) <= 85, "the standard parallel must lie within 85 degrees of the equator")
    k0 = _merc_k0(E, lat_ts)
    p = lat * DEG
    return _out(x=k0 * E["a"] * _nlon(lon - lon0) * DEG, y=k0 * E["a"] * _psi(E, p), k=k0 * math.sqrt(1 - E["e2"] * math.sin(p) ** 2) / math.cos(p))


def _ref_mercator_forward(lat, lon, lon0, lat_ts, ellipsoid):
    P = _proj(f"+proj=merc +lon_0={lon0!r} +lat_ts={lat_ts!r} +ellps={ellipsoid}")
    x, y = P(lon, lat)
    return {"x": x, "y": y, "k": P.get_factors(lon, lat).parallel_scale}


def _rand_merc(rng):
    lon0 = _sig(rng.uniform(-180, 180))
    return {"lat": _sig(rng.choice([rng.uniform(-85, -0.01), rng.uniform(0.01, 85)])), "lon": _sig(_nlon(lon0 + rng.choice([-1, 1]) * rng.uniform(0.01, 179))),
            "lon0": lon0, "lat_ts": _sig(rng.choice([0.0, rng.uniform(-70, 70)])), "ellipsoid": rng.choice(["WGS84", "GRS80"])}


_add("geo_mercator_forward",
     name="Mercator projection (ellipsoidal): coordinates and scale factor", topic=_PROJ,
     purpose="x and y of a point in the ellipsoidal Mercator projection with a central meridian and standard parallel (lat_ts = 0 is World Mercator, EPSG:3395), with the point scale factor.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lon0", "central meridian"), _degin("lat_ts", "latitude of true scale (standard parallel)"), _ellin()],
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing"), _o("k", "number", "point scale factor")],
     primary="y", conditions=["|latitude| <= 89.5", "|lat_ts| <= 85"],
     method="x = k0 a (lambda - lambda0), y = k0 a psi with the isometric latitude psi; k0 = cos(lat_ts)/sqrt(1 - e^2 sin^2 lat_ts)",
     latex=r"x = k_0a(\lambda-\lambda_0),\quad y = k_0a\left[\operatorname{arsinh}(\tan\varphi) - e\operatorname{artanh}(e\sin\varphi)\right],\quad k = \frac{k_0\sqrt{1-e^2\sin^2\varphi}}{\cos\varphi}",
     run=mercator_forward, reference=_ref_mercator_forward, rand=_rand_merc,
     wrong=lambda lat, lon, lon0, lat_ts, ellipsoid: (lambda E: (lambda k0: _out(x=k0 * E["a"] * _nlon(lon - lon0) * DEG, y=k0 * E["a"] * math.asinh(math.tan(lat * DEG)),
                                                                                 k=k0 / math.cos(lat * DEG)))(_merc_k0(E, lat_ts)))(_ellipsoid(ellipsoid)),
     phrases=("Mercator forward", "Mercator projection", "Ellipsoidal Mercator"),
     examples=[_ex({"lat": 51.5, "lon": -0.13, "lon0": 0, "lat_ts": 0, "ellipsoid": "WGS84"}), _ex({"lat": -20.0, "lon": 120.0, "lon0": 110, "lat_ts": -15, "ellipsoid": "GRS80"})],
     level="intermediate")


def mercator_inverse(x, y, lon0, lat_ts, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _fin("x and y", x, y)
    _lon("lon0", lon0)
    _lat("lat_ts", lat_ts)
    _need(abs(lat_ts) <= 85, "the standard parallel must lie within 85 degrees of the equator")
    k0 = _merc_k0(E, lat_ts)
    psi = y / (k0 * E["a"])
    _need(abs(psi) <= 6, "the northing lies too far from the equator")
    lam = x / (k0 * E["a"]) / DEG
    _need(abs(lam) <= 540, "the easting lies outside the projection")
    return _out(lat=math.atan(_tau_of_taup(E, math.sinh(psi))) / DEG, lon=_nlon(lon0 + lam))


def _ref_mercator_inverse(x, y, lon0, lat_ts, ellipsoid):
    lon, lat = _proj(f"+proj=merc +lon_0={lon0!r} +lat_ts={lat_ts!r} +ellps={ellipsoid}")(x, y, inverse=True)
    return {"lat": lat, "lon": _nlon(lon)}


def _rand_merc_inv(rng):
    x = _rand_merc(rng)
    o = mercator_forward(**x)
    return {"x": round(o["x"], 3), "y": round(o["y"], 3), "lon0": x["lon0"], "lat_ts": x["lat_ts"], "ellipsoid": x["ellipsoid"]}


_add("geo_mercator_inverse",
     name="Inverse Mercator projection (ellipsoidal)", topic=_PROJ,
     purpose="Latitude and longitude of a point given by its x and y in the ellipsoidal Mercator projection with a central meridian and standard parallel.",
     inputs=[_phys("x", "length", "easting"), _phys("y", "length", "northing"), _degin("lon0", "central meridian"), _degin("lat_ts", "latitude of true scale"), _ellin()],
     outputs=[_o("lat", "number", "latitude (degrees)"), _o("lon", "number", "longitude (degrees)")],
     primary="lat", conditions=["|lat_ts| <= 85"],
     method="isometric latitude psi = y/(k0 a); conformal tangent sinh(psi); geodetic latitude by Newton iteration (Karney's tau method)",
     latex=r"\psi = \frac{y}{k_0a},\quad \tau' = \sinh\psi,\quad \tau = \tan\varphi:\ \tau\sqrt{1+\sigma^2} - \sigma\sqrt{1+\tau^2} = \tau'",
     run=mercator_inverse, reference=_ref_mercator_inverse, rand=_rand_merc_inv,
     wrong=lambda x, y, lon0, lat_ts, ellipsoid: (lambda E: (lambda k0: _out(lat=math.atan(math.sinh(y / (k0 * E["a"]))) / DEG, lon=_nlon(lon0 + x / (k0 * E["a"]) / DEG)))(_merc_k0(E, lat_ts)))(_ellipsoid(ellipsoid)),
     phrases=("Mercator inverse", "Inverse Mercator", "Mercator to latitude longitude"),
     examples=[_ex({"x": "-14471.5 m", "y": "6680752.0 m", "lon0": 0, "lat_ts": 0, "ellipsoid": "WGS84"}),
               _ex({"x": "1000000 m", "y": "-2000000 m", "lon0": 110, "lat_ts": -15, "ellipsoid": "GRS80"})],
     level="intermediate")


# ---------------------------------------------------------------- Lambert conformal conic (2SP)
def _lcc_setup(E, lat1, lat2, lat0):
    for nm, v in (("lat1", lat1), ("lat2", lat2), ("lat0", lat0)):
        _lat(nm, v)
        _need(abs(v) <= 89, f"{nm} must lie within 89 degrees of the equator")
    _need(abs(lat1 + lat2) > 1e-6, "the standard parallels must not be symmetric about the equator")
    m = lambda p: math.cos(p) / math.sqrt(1 - E["e2"] * math.sin(p) ** 2)
    p1, p2 = lat1 * DEG, lat2 * DEG
    if abs(lat1 - lat2) < 1e-10:
        n = math.sin(p1)
    else:
        n = (math.log(m(p1)) - math.log(m(p2))) / (_psi(E, p2) - _psi(E, p1))
    F = m(p1) * math.exp(n * _psi(E, p1)) / n
    rho = lambda p: E["a"] * F * math.exp(-n * _psi(E, p))
    return n, F, rho, m


def lcc_forward(lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _pt(lat, lon, 89)
    _lon("lon0", lon0)
    _fin("the false easting and northing", FE, FN)
    n, F, rho, m = _lcc_setup(E, lat1, lat2, lat0)
    _need(lat * n > -80 * abs(n), "the point lies too close to the pole opposite the cone's apex")
    r, r0 = rho(lat * DEG), rho(lat0 * DEG)
    th = n * _nlon(lon - lon0) * DEG
    return _out(x=FE + r * math.sin(th), y=FN + r0 - r * math.cos(th), k=r * n / (E["a"] * m(lat * DEG)), gamma=th / DEG)


def _lcc_str(lat1, lat2, lat0, lon0, ell, FE, FN):
    return f"+proj=lcc +lat_1={lat1!r} +lat_2={lat2!r} +lat_0={lat0!r} +lon_0={lon0!r} +x_0={FE!r} +y_0={FN!r} +ellps={ell}"


def _ref_lcc_forward(lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    P = _proj(_lcc_str(lat1, lat2, lat0, lon0, ellipsoid, FE, FN))
    x, y = P(lon, lat)
    f = P.get_factors(lon, lat)
    return {"x": x, "y": y, "k": f.parallel_scale, "gamma": f.meridian_convergence}


def _rand_conic(rng):
    for _ in range(100):
        s = rng.choice([1, -1])
        a, b = sorted([rng.uniform(15, 70), rng.uniform(15, 70)])
        lat1, lat2 = _sig(s * a), _sig(s * b) if rng.random() < 0.85 else _sig(s * a)
        lat0 = _sig(s * rng.uniform(10, 60))
        lon0 = _sig(rng.uniform(-180, 180))
        x = {"lat": _sig(s * rng.uniform(5, 80)), "lon": _sig(_nlon(lon0 + rng.choice([-1, 1]) * rng.uniform(0.5, 40))), "lat1": lat1, "lat2": lat2,
             "lat0": lat0, "lon0": lon0, "ellipsoid": rng.choice(["WGS84", "GRS80"]), "FE": _sig(rng.choice([0.0, rng.uniform(0, 5e6)]), 7),
             "FN": _sig(rng.choice([0.0, rng.uniform(0, 5e6)]), 7)}
        if abs(x["lat"] - x["lat0"]) > 0.5:
            return x
    return x


_CONIC_IN = [_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lat1", "first standard parallel"), _degin("lat2", "second standard parallel"),
             _degin("lat0", "latitude of origin"), _degin("lon0", "central meridian"), _ellin(),
             _phys("FE", "length", "false easting (default 0)", required=False), _phys("FN", "length", "false northing (default 0)", required=False)]
_add("geo_lambert_conformal_conic_forward",
     name="Lambert conformal conic projection (two standard parallels): coordinates, scale factor and convergence", topic=_PROJ,
     purpose="Easting and northing of a point in the ellipsoidal Lambert conformal conic projection with two standard parallels (EPSG method 9802), with the point scale factor and the grid convergence.",
     inputs=_CONIC_IN,
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing"), _o("k", "number", "point scale factor"), _o("gamma", "number", "grid convergence (degrees)")],
     primary="x", conditions=["parallels within 89 degrees of the equator and not symmetric about it"],
     method="n = (ln m1 - ln m2)/(ln t1 - ln t2), F = m1/(n t1^n), rho = a F t^n with t = exp(-psi); x = FE + rho sin(theta), y = FN + rho0 - rho cos(theta), theta = n (lambda - lambda0); k = rho n/(a m); convergence = theta",
     latex=r"n = \frac{\ln m_1 - \ln m_2}{\ln t_1 - \ln t_2},\ \rho = aFt^n,\ x = FE + \rho\sin\theta,\ y = FN + \rho_0 - \rho\cos\theta,\ k = \frac{\rho n}{am}",
     run=lcc_forward, reference=_ref_lcc_forward, rand=_rand_conic,
     wrong=lambda lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0: (lambda r: dict(r, gamma=r["gamma"] / max(1e-9, abs(math.sin(lat0 * DEG))) * 0 + _nlon(lon - lon0)))(
         lcc_forward(lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE, FN)),
     phrases=("Lambert conformal conic forward", "Lambert conformal conic projection", "LCC forward"),
     examples=[_ex({"lat": 40.0, "lon": -100.0, "lat1": 33, "lat2": 45, "lat0": 23, "lon0": -96, "ellipsoid": "GRS80"}),
               _ex({"lat": 46.8, "lon": 2.35, "lat1": 44, "lat2": 49, "lat0": 46.5, "lon0": 3, "ellipsoid": "GRS80", "FE": "700000 m", "FN": "6600000 m"})],
     level="advanced")


def lcc_inverse(x, y, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _fin("x and y", x, y)
    _lon("lon0", lon0)
    _fin("the false easting and northing", FE, FN)
    n, F, rho, m = _lcc_setup(E, lat1, lat2, lat0)
    r0 = rho(lat0 * DEG)
    xp, yp = x - FE, r0 - (y - FN)
    sg = 1.0 if n > 0 else -1.0
    r = sg * math.hypot(xp, yp)
    _need(r * sg > 0, "the point is the apex of the cone")
    th = math.atan2(sg * xp, sg * yp)
    psi = -math.log(r / (E["a"] * F)) / n
    lat = math.atan(_tau_of_taup(E, math.sinh(psi))) / DEG
    _need(abs(lat) <= 89, "the point lies too close to a pole")
    return _out(lat=lat, lon=_nlon(lon0 + th / n / DEG))


def _ref_lcc_inverse(x, y, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    lon, lat = _proj(_lcc_str(lat1, lat2, lat0, lon0, ellipsoid, FE, FN))(x, y, inverse=True)
    return {"lat": lat, "lon": _nlon(lon)}


def _rand_conic_inv(rng, fwd):
    x = _rand_conic(rng)
    o = fwd(**x)
    d = {k: v for k, v in x.items() if k not in ("lat", "lon")}
    return dict(x=round(o["x"], 3), y=round(o["y"], 3), **d)


_CONIC_INV_IN = [_phys("x", "length", "easting"), _phys("y", "length", "northing")] + _CONIC_IN[2:]
_add("geo_lambert_conformal_conic_inverse",
     name="Inverse Lambert conformal conic projection (two standard parallels)", topic=_PROJ,
     purpose="Latitude and longitude of a point given by its easting and northing in the ellipsoidal Lambert conformal conic projection with two standard parallels.",
     inputs=_CONIC_INV_IN,
     outputs=[_o("lat", "number", "latitude (degrees)"), _o("lon", "number", "longitude (degrees)")],
     primary="lat", conditions=["parallels within 89 degrees of the equator and not symmetric about it"],
     method="rho and theta from the offsets, t = (rho/(aF))^(1/n), isometric latitude -ln t, geodetic latitude by Newton iteration; lambda = lambda0 + theta/n",
     latex=r"\rho = \operatorname{sgn}(n)\sqrt{x'^2 + (\rho_0 - y')^2},\quad t = \left(\frac{\rho}{aF}\right)^{1/n},\quad \lambda = \lambda_0 + \frac{\theta}{n}",
     run=lcc_inverse, reference=_ref_lcc_inverse, rand=lambda rng: _rand_conic_inv(rng, lcc_forward),
     wrong=lambda x, y, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0: lcc_inverse(x, y, lat1, lat2, lat0, lon0, ellipsoid, -FE, -FN),
     phrases=("Lambert conformal conic inverse", "Inverse Lambert conformal conic", "LCC inverse"),
     examples=[_ex({"x": "-339545.0 m", "y": "1937380.0 m", "lat1": 33, "lat2": 45, "lat0": 23, "lon0": -96, "ellipsoid": "GRS80"}),
               _ex({"x": "652000 m", "y": "6862000 m", "lat1": 44, "lat2": 49, "lat0": 46.5, "lon0": 3, "ellipsoid": "GRS80", "FE": "700000 m", "FN": "6600000 m"})],
     level="advanced")


# ---------------------------------------------------------------- Albers equal-area conic
def _aea_setup(E, lat1, lat2, lat0):
    for nm, v in (("lat1", lat1), ("lat2", lat2), ("lat0", lat0)):
        _lat(nm, v)
        _need(abs(v) <= 89, f"{nm} must lie within 89 degrees of the equator")
    _need(abs(lat1 + lat2) > 1e-6, "the standard parallels must not be symmetric about the equator")
    m = lambda p: math.cos(p) / math.sqrt(1 - E["e2"] * math.sin(p) ** 2)
    q = lambda p: _q_auth(E, p)
    p1, p2 = lat1 * DEG, lat2 * DEG
    n = math.sin(p1) if abs(lat1 - lat2) < 1e-10 else (m(p1) ** 2 - m(p2) ** 2) / (q(p2) - q(p1))
    C = m(p1) ** 2 + n * q(p1)
    rho = lambda p: E["a"] * math.sqrt(C - n * q(p)) / n
    return n, C, rho, m, q


def albers_forward(lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _pt(lat, lon, 89)
    _lon("lon0", lon0)
    _fin("the false easting and northing", FE, FN)
    n, C, rho, m, q = _aea_setup(E, lat1, lat2, lat0)
    r, r0 = rho(lat * DEG), rho(lat0 * DEG)
    th = n * _nlon(lon - lon0) * DEG
    k = r * n / (E["a"] * m(lat * DEG))
    return _out(x=FE + r * math.sin(th), y=FN + r0 - r * math.cos(th), k=k, h=1 / k)


def _aea_str(lat1, lat2, lat0, lon0, ell, FE, FN):
    return f"+proj=aea +lat_1={lat1!r} +lat_2={lat2!r} +lat_0={lat0!r} +lon_0={lon0!r} +x_0={FE!r} +y_0={FN!r} +ellps={ell}"


def _ref_albers_forward(lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    P = _proj(_aea_str(lat1, lat2, lat0, lon0, ellipsoid, FE, FN))
    x, y = P(lon, lat)
    f = P.get_factors(lon, lat)
    return {"x": x, "y": y, "k": f.parallel_scale, "h": f.meridional_scale}


_add("geo_albers_equal_area_forward",
     name="Albers equal-area conic projection: coordinates and scale factors", topic=_PROJ,
     purpose="Easting and northing of a point in the ellipsoidal Albers equal-area conic projection with two standard parallels, with the scale factors along the parallel (k) and the meridian (h = 1/k).",
     inputs=_CONIC_IN,
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing"), _o("k", "number", "scale factor along the parallel"),
              _o("h", "number", "scale factor along the meridian")],
     primary="x", conditions=["parallels within 89 degrees of the equator and not symmetric about it"],
     method="n = (m1^2 - m2^2)/(q2 - q1), C = m1^2 + n q1, rho = a sqrt(C - n q)/n with the authalic function q; x = FE + rho sin(theta), y = FN + rho0 - rho cos(theta)",
     latex=r"n = \frac{m_1^2 - m_2^2}{q_2 - q_1},\quad \rho = \frac{a\sqrt{C - nq}}{n},\quad x = FE + \rho\sin\theta,\ y = FN + \rho_0 - \rho\cos\theta,\ k = \frac{\rho n}{am} = \frac1h",
     run=albers_forward, reference=_ref_albers_forward, rand=_rand_conic,
     wrong=lambda lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0: (lambda r: dict(r, y=r["y"] - 2 * (r["y"] - FN), k=r["h"], h=r["k"]))(albers_forward(lat, lon, lat1, lat2, lat0, lon0, ellipsoid, FE, FN)),
     phrases=("Albers equal-area forward", "Albers equal-area conic projection", "Albers projection"),
     examples=[_ex({"lat": 40.0, "lon": -100.0, "lat1": 29.5, "lat2": 45.5, "lat0": 23, "lon0": -96, "ellipsoid": "GRS80"}),
               _ex({"lat": -25.0, "lon": 135.0, "lat1": -18, "lat2": -36, "lat0": 0, "lon0": 132, "ellipsoid": "GRS80"})],
     level="advanced")


def albers_inverse(x, y, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _fin("x and y", x, y)
    _lon("lon0", lon0)
    _fin("the false easting and northing", FE, FN)
    n, C, rho, m, q = _aea_setup(E, lat1, lat2, lat0)
    r0 = rho(lat0 * DEG)
    xp, yp = x - FE, r0 - (y - FN)
    sg = 1.0 if n > 0 else -1.0
    r = math.hypot(xp, yp)
    th = math.atan2(sg * xp, sg * yp)
    qv = (C - (r * n / E["a"]) ** 2) / n
    qp = _q_auth(E, math.pi / 2)
    _need(abs(qv) < qp * (1 - 1e-12), "the point lies outside the projected area")
    phi = math.asin(max(-1.0, min(1.0, qv / 2)))
    for _ in range(60):
        s = math.sin(phi)
        d = (_q_auth(E, phi) - qv) * (1 - E["e2"] * s * s) ** 2 / (2 * (1 - E["e2"]) * math.cos(phi))
        phi -= d
        if abs(d) < 1e-16:
            break
    _need(abs(phi / DEG) <= 89, "the point lies too close to a pole")
    return _out(lat=phi / DEG, lon=_nlon(lon0 + th / n / DEG))


def _ref_albers_inverse(x, y, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    lon, lat = _proj(_aea_str(lat1, lat2, lat0, lon0, ellipsoid, FE, FN))(x, y, inverse=True)
    return {"lat": lat, "lon": _nlon(lon)}


_add("geo_albers_equal_area_inverse",
     name="Inverse Albers equal-area conic projection", topic=_PROJ,
     purpose="Latitude and longitude of a point given by its easting and northing in the ellipsoidal Albers equal-area conic projection with two standard parallels.",
     inputs=_CONIC_INV_IN,
     outputs=[_o("lat", "number", "latitude (degrees)"), _o("lon", "number", "longitude (degrees)")],
     primary="lat", conditions=["parallels within 89 degrees of the equator and not symmetric about it", "the point inside the projected area"],
     method="rho and theta from the offsets, q = (C - (rho n/a)^2)/n, latitude from q by Newton iteration; lambda = lambda0 + theta/n",
     latex=r"q = \frac{C - \rho^2n^2/a^2}{n},\quad \varphi:\ q(\varphi) = q,\quad \lambda = \lambda_0 + \frac{\theta}{n}",
     run=albers_inverse, reference=_ref_albers_inverse, rand=lambda rng: _rand_conic_inv(rng, albers_forward),
     wrong=lambda x, y, lat1, lat2, lat0, lon0, ellipsoid, FE=0.0, FN=0.0: albers_inverse(x, y, lat2, lat1, lat0 + 1, lon0, ellipsoid, FE, FN),
     phrases=("Albers equal-area inverse", "Inverse Albers projection", "Albers inverse"),
     examples=[_ex({"x": "-338796.0 m", "y": "1952260.0 m", "lat1": 29.5, "lat2": 45.5, "lat0": 23, "lon0": -96, "ellipsoid": "GRS80"}),
               _ex({"x": "300000 m", "y": "-2700000 m", "lat1": -18, "lat2": -36, "lat0": 0, "lon0": 132, "ellipsoid": "GRS80"})],
     level="advanced")


# ---------------------------------------------------------------- polar stereographic (variant B)
def polar_stereographic_forward(lat, lon, pole, lat_ts, lon0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _lat("lat", lat)
    _lon("lon", lon)
    _lon("lon0", lon0)
    _fin("the false easting and northing", FE, FN)
    _need(pole in ("north", "south"), "the pole must be north or south")
    s = 1.0 if pole == "north" else -1.0
    _need(0 < s * lat < 90, "the point must lie in the hemisphere of the pole")
    _lat("lat_ts", lat_ts)
    _need(30 <= s * lat_ts <= 89.99, "the standard parallel must lie 30 to 89.99 degrees from the equator in the pole's hemisphere")
    m = lambda p: math.cos(p) / math.sqrt(1 - E["e2"] * math.sin(p) ** 2)
    pc, p = s * lat_ts * DEG, s * lat * DEG
    rho = E["a"] * m(pc) * math.exp(-_psi(E, p)) / math.exp(-_psi(E, pc))
    dl = _nlon(lon - lon0) * DEG
    return _out(x=FE + rho * math.sin(dl), y=FN - s * rho * math.cos(dl), k=rho / (E["a"] * m(p)))


def _ref_polar_stereographic_forward(lat, lon, pole, lat_ts, lon0, ellipsoid, FE=0.0, FN=0.0):
    P = _proj(f"+proj=stere +lat_0={90 if pole == 'north' else -90} +lat_ts={lat_ts!r} +lon_0={lon0!r} +x_0={FE!r} +y_0={FN!r} +ellps={ellipsoid}")
    x, y = P(lon, lat)
    return {"x": x, "y": y, "k": P.get_factors(lon, lat).parallel_scale}


def _rand_stere(rng):
    s = rng.choice([1, -1])
    lon0 = _sig(rng.choice([0.0, -45.0, rng.uniform(-180, 180)]))
    return {"lat": _sig(s * rng.uniform(40, 89.5)), "lon": _sig(rng.uniform(-179.9, 179.9)), "pole": "north" if s > 0 else "south",
            "lat_ts": _sig(s * rng.choice([70.0, 71.0, rng.uniform(50, 89)])), "lon0": lon0, "ellipsoid": rng.choice(["WGS84", "GRS80"]),
            "FE": _sig(rng.choice([0.0, 2e6]), 7), "FN": _sig(rng.choice([0.0, 2e6]), 7)}


_add("geo_polar_stereographic_forward",
     name="Polar stereographic projection (standard parallel): coordinates and scale factor", topic=_PROJ,
     purpose="Easting and northing of a point in the ellipsoidal polar stereographic projection defined by a latitude of true scale (EPSG variant B, as in EPSG:3413 and EPSG:3031), with the point scale factor.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _choice("pole", ["north", "south"], "pole at the projection centre"),
             _degin("lat_ts", "latitude of true scale"), _degin("lon0", "longitude of the downward (north pole) or upward (south pole) y axis"), _ellin(),
             _phys("FE", "length", "false easting (default 0)", required=False), _phys("FN", "length", "false northing (default 0)", required=False)],
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing"), _o("k", "number", "point scale factor")],
     primary="x", conditions=["the point in the pole's hemisphere (not at the pole)", "30 <= |lat_ts| <= 89.99 in the same hemisphere"],
     method="rho = a m_c t/t_c with t = exp(-psi); x = FE + rho sin(dlambda), y = FN -/+ rho cos(dlambda); k = rho/(a m)",
     latex=r"\rho = a\,m_c\frac{t}{t_c},\quad t = e^{-\psi},\quad x = FE + \rho\sin\Delta\lambda,\ y = FN \mp \rho\cos\Delta\lambda,\quad k = \frac{\rho}{am}",
     run=polar_stereographic_forward, reference=_ref_polar_stereographic_forward, rand=_rand_stere,
     wrong=lambda lat, lon, pole, lat_ts, lon0, ellipsoid, FE=0.0, FN=0.0: (lambda r: dict(r, y=2 * FN - r["y"]))(polar_stereographic_forward(lat, lon, pole, lat_ts, lon0, ellipsoid, FE, FN)),
     phrases=("Polar stereographic forward", "Polar stereographic projection"),
     examples=[_ex({"lat": 75.0, "lon": -40.0, "pole": "north", "lat_ts": 70, "lon0": -45, "ellipsoid": "WGS84"}),
               _ex({"lat": -77.85, "lon": 166.67, "pole": "south", "lat_ts": -71, "lon0": 0, "ellipsoid": "WGS84"})],
     level="advanced")


# ---------------------------------------------------------------- Lambert azimuthal equal-area (ellipsoidal, oblique)
def laea_forward(lat, lon, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _pt(lat, lon, 89.9)
    _lat("lat0", lat0)
    _need(abs(lat0) <= 89, "the centre must lie within 89 degrees of the equator (the polar aspect is not supported)")
    _lon("lon0", lon0)
    _fin("the false easting and northing", FE, FN)
    _need(_sph_sep(lat0, lon0, lat, lon) <= 120, "the point must lie within 120 degrees of the centre")
    q = lambda p: _q_auth(E, p)
    qp = q(math.pi / 2)
    Rq = E["a"] * math.sqrt(qp / 2)
    b1 = math.asin(q(lat0 * DEG) / qp)
    b = math.asin(max(-1.0, min(1.0, q(lat * DEG) / qp)))
    m1 = math.cos(lat0 * DEG) / math.sqrt(1 - E["e2"] * math.sin(lat0 * DEG) ** 2)
    D = E["a"] * m1 / (Rq * math.cos(b1))
    dl = _nlon(lon - lon0) * DEG
    B = Rq * math.sqrt(2 / (1 + math.sin(b1) * math.sin(b) + math.cos(b1) * math.cos(b) * math.cos(dl)))
    return _out(x=FE + B * D * math.cos(b) * math.sin(dl), y=FN + B / D * (math.cos(b1) * math.sin(b) - math.sin(b1) * math.cos(b) * math.cos(dl)))


def _ref_laea_forward(lat, lon, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    x, y = _proj(f"+proj=laea +lat_0={lat0!r} +lon_0={lon0!r} +x_0={FE!r} +y_0={FN!r} +ellps={ellipsoid}")(lon, lat)
    return {"x": x, "y": y}


def _sph_dest(lat1, lon1, az, sep):
    p1 = lat1 * DEG
    lat2 = math.asin(math.sin(p1) * math.cos(sep * DEG) + math.cos(p1) * math.sin(sep * DEG) * math.cos(az * DEG)) / DEG
    lon2 = lon1 + math.atan2(math.sin(az * DEG) * math.sin(sep * DEG) * math.cos(p1), math.cos(sep * DEG) - math.sin(p1) * math.sin(lat2 * DEG)) / DEG
    return lat2, _nlon(lon2)


def _rand_azi(rng, maxsep=100):
    for _ in range(100):
        lat0, lon0 = _sig(rng.uniform(-80, 80)), _sig(rng.uniform(-180, 180))
        lat, lon = _sph_dest(lat0, lon0, rng.uniform(0, 360), _lu(rng, 0.1, maxsep * 0.98))
        x = {"lat": _sig(lat), "lon": _sig(lon), "lat0": lat0, "lon0": lon0}
        sep = _sph_dest and _sph_sep(lat0, lon0, x["lat"], x["lon"])
        if 0.05 < sep <= maxsep and abs(x["lat"]) < 85 and abs(x["lat"]) > 1e-3:
            return x
    return {"lat": 50.0, "lon": 10.0, "lat0": 52.0, "lon0": 10.0}


_add("geo_lambert_azimuthal_equal_area_forward",
     name="Lambert azimuthal equal-area projection (ellipsoidal)", topic=_PROJ,
     purpose="Easting and northing of a point in the ellipsoidal oblique Lambert azimuthal equal-area projection about a centre (as in ETRS89-LAEA, EPSG:3035).",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lat0", "latitude of the centre"), _degin("lon0", "longitude of the centre"), _ellin(),
             _phys("FE", "length", "false easting (default 0)", required=False), _phys("FN", "length", "false northing (default 0)", required=False)],
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing")],
     primary="x", conditions=["centre within 89 degrees of the equator", "point within 120 degrees of the centre"],
     method="authalic latitudes beta = asin(q/q_p) on the authalic sphere R_q, then the spherical azimuthal equal-area formulas with Snyder's D factor",
     latex=r"B = R_q\sqrt{\frac{2}{1 + \sin\beta_1\sin\beta + \cos\beta_1\cos\beta\cos\Delta\lambda}},\ x = BD\cos\beta\sin\Delta\lambda,\ y = \frac{B}{D}\left(\cos\beta_1\sin\beta - \sin\beta_1\cos\beta\cos\Delta\lambda\right)",
     run=laea_forward, reference=_ref_laea_forward,
     rand=lambda rng: dict(_rand_azi(rng), ellipsoid=rng.choice(["WGS84", "GRS80"]), FE=_sig(rng.choice([0.0, 4321000.0]), 7), FN=_sig(rng.choice([0.0, 3210000.0]), 7)),
     wrong=lambda lat, lon, lat0, lon0, ellipsoid, FE=0.0, FN=0.0: (lambda E: (lambda r: _out(x=FE + 2 * E["a"] * math.sin(r / 2) * 1.0, y=FN))(
         _sph_sep(lat0, lon0, lat, lon) * DEG))(_ellipsoid(ellipsoid)),
     phrases=("Lambert azimuthal equal-area forward", "Lambert azimuthal equal-area projection", "LAEA forward"),
     examples=[_ex({"lat": 50.0, "lon": 5.0, "lat0": 52, "lon0": 10, "ellipsoid": "GRS80", "FE": "4321000 m", "FN": "3210000 m"}),
               _ex({"lat": 40.0, "lon": -101.0, "lat0": 45, "lon0": -100, "ellipsoid": "WGS84"})],
     level="expert")


# ---------------------------------------------------------------- azimuthal equidistant (ellipsoidal, geodesic)
def aeqd_forward(lat, lon, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    E = _ellipsoid(ellipsoid)
    _fin("the false easting and northing", FE, FN)
    _two_points(lat0, lon0, lat, lon)
    _need(abs(lat0) <= 89, "the centre must lie within 89 degrees of the equator")
    inv = _geod_inverse(E, lat0, lon0, lat, lon)
    a = inv["az1"] * DEG
    return _out(x=FE + inv["s"] * math.sin(a), y=FN + inv["s"] * math.cos(a), distance=inv["s"], azimuth=inv["az1"])


def _ref_aeqd_forward(lat, lon, lat0, lon0, ellipsoid, FE=0.0, FN=0.0):
    x, y = _proj(f"+proj=aeqd +lat_0={lat0!r} +lon_0={lon0!r} +x_0={FE!r} +y_0={FN!r} +ellps={ellipsoid}")(lon, lat)
    az, _, s = _geod(ellipsoid).inv(lon0, lat0, lon, lat)
    return {"x": x, "y": y, "distance": s, "azimuth": _az(az)}


def _rand_aeqd(rng):
    for _ in range(100):
        x = dict(_rand_azi(rng, 120), ellipsoid=rng.choice(["WGS84", "GRS80"]))
        o = aeqd_forward(**x)
        if _not_near_north(o["azimuth"]) and abs(o["x"]) > 1 and abs(o["y"]) > 1:
            return x
    return x


_add("geo_azimuthal_equidistant_forward",
     name="Azimuthal equidistant projection (ellipsoidal, geodesic)", topic=_PROJ,
     purpose="Easting and northing of a point in the ellipsoidal azimuthal equidistant projection about a centre: the geodesic distance from the centre plotted at the geodesic azimuth (used for range rings and airline maps).",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lat0", "latitude of the centre"), _degin("lon0", "longitude of the centre"), _ellin(),
             _phys("FE", "length", "false easting (default 0)", required=False), _phys("FN", "length", "false northing (default 0)", required=False)],
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing"), _o("distance", "length", "geodesic distance from the centre"),
              _o("azimuth", "number", "geodesic azimuth from the centre (degrees)")],
     primary="x", conditions=["point distinct from the centre and at most 150 degrees from it", "centre within 89 degrees of the equator"],
     method="inverse geodesic problem from the centre (exact integrals), then x = s sin(alpha), y = s cos(alpha)",
     latex=r"x = FE + s_{12}\sin\alpha_1,\quad y = FN + s_{12}\cos\alpha_1",
     run=aeqd_forward, reference=_ref_aeqd_forward, rand=_rand_aeqd,
     wrong=lambda lat, lon, lat0, lon0, ellipsoid, FE=0.0, FN=0.0: (lambda r: dict(r, x=r["y"] - FN + FE, y=r["x"] - FE + FN))(aeqd_forward(lat, lon, lat0, lon0, ellipsoid, FE, FN)),
     phrases=("Azimuthal equidistant forward", "Azimuthal equidistant projection"),
     examples=[_ex({"lat": 40.71, "lon": -74.01, "lat0": 51.5, "lon0": -0.13, "ellipsoid": "WGS84"}),
               _ex({"lat": -33.87, "lon": 151.21, "lat0": -37.81, "lon0": 144.96, "ellipsoid": "GRS80"})],
     level="advanced")


# ---------------------------------------------------------------- sinusoidal (ellipsoidal)
def sinusoidal_forward(lat, lon, lon0, ellipsoid):
    E = _ellipsoid(ellipsoid)
    _pt(lat, lon, 89.9)
    _lon("lon0", lon0)
    p = lat * DEG
    return _out(x=E["a"] * _nlon(lon - lon0) * DEG * math.cos(p) / math.sqrt(1 - E["e2"] * math.sin(p) ** 2), y=_merid(E, p))


def _ref_sinusoidal_forward(lat, lon, lon0, ellipsoid):
    x, y = _proj(f"+proj=sinu +lon_0={lon0!r} +ellps={ellipsoid}")(lon, lat)
    return {"x": x, "y": y}


_add("geo_sinusoidal_forward",
     name="Sinusoidal equal-area projection (ellipsoidal)", topic=_PROJ,
     purpose="x and y of a point in the ellipsoidal sinusoidal (Sanson-Flamsteed) equal-area projection, as used for MODIS land grids.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lon0", "central meridian"), _ellin()],
     outputs=[_o("x", "length", "easting"), _o("y", "length", "northing")],
     primary="x", conditions=["|latitude| <= 89.9"],
     method="x = N cos(phi) (lambda - lambda0), y = meridian distance m(phi) (Helmert's series)",
     latex=r"x = \frac{a\cos\varphi}{\sqrt{1-e^2\sin^2\varphi}}(\lambda - \lambda_0),\quad y = m(\varphi)",
     run=sinusoidal_forward, reference=_ref_sinusoidal_forward,
     rand=lambda rng: (lambda lon0: {"lat": _sig(rng.choice([rng.uniform(-89, -0.01), rng.uniform(0.01, 89)])), "lon": _sig(_nlon(lon0 + rng.choice([-1, 1]) * rng.uniform(0.01, 179))),
                                     "lon0": lon0, "ellipsoid": rng.choice(["WGS84", "GRS80"])})(_sig(rng.uniform(-180, 180))),
     wrong=lambda lat, lon, lon0, ellipsoid: (lambda E: _out(x=E["a"] * _nlon(lon - lon0) * DEG * math.cos(lat * DEG), y=E["a"] * lat * DEG))(_ellipsoid(ellipsoid)),
     phrases=("Sinusoidal forward", "Sinusoidal projection"),
     examples=[_ex({"lat": 45.0, "lon": 10.0, "lon0": 0, "ellipsoid": "WGS84"}), _ex({"lat": -12.5, "lon": -60.0, "lon0": -50, "ellipsoid": "GRS80"})],
     level="intermediate")


# ---------------------------------------------------------------- spherical projections
def _sphere_R(R):
    _pos("the radius R", R)


def mollweide_forward(lat, lon, lon0, R):
    _pt(lat, lon, 90)
    _lon("lon0", lon0)
    _sphere_R(R)
    p = lat * DEG
    if abs(lat) == 90:
        th = math.copysign(math.pi / 2, lat)
    else:
        th = p
        for _ in range(100):
            d = (2 * th + math.sin(2 * th) - math.pi * math.sin(p)) / (2 + 2 * math.cos(2 * th))
            th -= d
            if abs(d) < 1e-15:
                break
    return _out(x=R * 2 * math.sqrt(2) / math.pi * _nlon(lon - lon0) * DEG * math.cos(th), y=R * math.sqrt(2) * math.sin(th))


def _ref_mollweide_forward(lat, lon, lon0, R):
    x, y = _proj(f"+proj=moll +lon_0={lon0!r} +R={R!r}")(lon, lat)
    return {"x": x, "y": y}


def _rand_sph(rng, latlim=85, sep=179):
    lon0 = _sig(rng.uniform(-180, 180))
    return {"lat": _sig(rng.choice([rng.uniform(-latlim, -0.01), rng.uniform(0.01, latlim)])), "lon": _sig(_nlon(lon0 + rng.choice([-1, 1]) * rng.uniform(0.01, sep))),
            "lon0": lon0, "R": _sig(rng.choice([6371000.0, 6378137.0, _lu(rng, 1.0, 1e7)]))}


_SPH_IN = [_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lon0", "central meridian"), _phys("R", "length", "radius of the sphere")]
_add("geo_mollweide_forward",
     name="Mollweide equal-area projection (sphere)", topic=_PROJ,
     purpose="x and y of a point in the Mollweide (homalographic) equal-area projection of a sphere of radius R.",
     inputs=_SPH_IN, outputs=[_o("x", "length", "x"), _o("y", "length", "y")],
     primary="x", conditions=["sphere of radius R > 0"],
     method="auxiliary angle theta from 2 theta + sin 2 theta = pi sin(phi) by Newton's method; x = (2 sqrt2/pi) R dlambda cos(theta), y = sqrt2 R sin(theta)",
     latex=r"2\theta + \sin2\theta = \pi\sin\varphi,\quad x = \frac{2\sqrt2}{\pi}R\,\Delta\lambda\cos\theta,\quad y = \sqrt2R\sin\theta",
     run=mollweide_forward, reference=_ref_mollweide_forward, rand=lambda rng: _rand_sph(rng, 89),
     wrong=lambda lat, lon, lon0, R: _out(x=R * 2 * math.sqrt(2) / math.pi * _nlon(lon - lon0) * DEG * math.cos(lat * DEG), y=R * math.sqrt(2) * math.sin(lat * DEG)),
     phrases=("Mollweide forward", "Mollweide projection"),
     examples=[_ex({"lat": 45.0, "lon": 90.0, "lon0": 0, "R": "6371000 m"}), _ex({"lat": -30.0, "lon": -60.0, "lon0": 10, "R": "1 m"})],
     level="intermediate")


def orthographic_forward(lat, lon, lat0, lon0, R):
    _pt(lat, lon, 90)
    _lat("lat0", lat0, True)
    _lon("lon0", lon0)
    _sphere_R(R)
    p, p0, dl = lat * DEG, lat0 * DEG, _nlon(lon - lon0) * DEG
    cc = math.sin(p0) * math.sin(p) + math.cos(p0) * math.cos(p) * math.cos(dl)
    _need(cc >= 0, "the point lies on the far side of the globe (not visible in the orthographic view)")
    return _out(x=R * math.cos(p) * math.sin(dl), y=R * (math.cos(p0) * math.sin(p) - math.sin(p0) * math.cos(p) * math.cos(dl)))


def _ref_orthographic_forward(lat, lon, lat0, lon0, R):
    x, y = _proj(f"+proj=ortho +lat_0={lat0!r} +lon_0={lon0!r} +R={R!r}")(lon, lat)
    return {"x": x, "y": y}


def _rand_ortho(rng, maxsep=85):
    return dict(_rand_azi(rng, maxsep), R=_sig(rng.choice([6371000.0, _lu(rng, 1.0, 1e7)])))


_AZ_IN = [_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lat0", "latitude of the centre"), _degin("lon0", "longitude of the centre"),
          _phys("R", "length", "radius of the sphere")]
_add("geo_orthographic_forward",
     name="Orthographic projection (sphere)", topic=_PROJ,
     purpose="x and y of a point in the orthographic projection (the view of a globe from infinitely far away) of a sphere of radius R centred on a point.",
     inputs=_AZ_IN, outputs=[_o("x", "length", "x"), _o("y", "length", "y")],
     primary="x", conditions=["the point on the visible hemisphere (cos c >= 0)"],
     method="x = R cos(phi) sin(dlambda), y = R (cos(phi0) sin(phi) - sin(phi0) cos(phi) cos(dlambda))",
     latex=r"x = R\cos\varphi\sin\Delta\lambda,\quad y = R\left(\cos\varphi_0\sin\varphi - \sin\varphi_0\cos\varphi\cos\Delta\lambda\right)",
     run=orthographic_forward, reference=_ref_orthographic_forward, rand=lambda rng: _rand_ortho(rng, 85),
     wrong=lambda lat, lon, lat0, lon0, R: (lambda p, p0, dl: _out(x=R * math.cos(p) * math.sin(dl), y=R * (math.cos(p0) * math.sin(p) + math.sin(p0) * math.cos(p) * math.cos(dl))))(lat * DEG, lat0 * DEG, _nlon(lon - lon0) * DEG),
     phrases=("Orthographic forward", "Orthographic projection"),
     examples=[_ex({"lat": 40.0, "lon": 10.0, "lat0": 45, "lon0": 0, "R": "6371000 m"}), _ex({"lat": -10.0, "lon": 100.0, "lat0": 0, "lon0": 80, "R": "1 m"})],
     extra_q=[{"q": "Orthographic forward: lat = -45, lon = 180, lat0 = 45, lon0 = 0, R = 6371000 m", "outcome": "INVALID_INPUT"}],
     level="beginner")


def gnomonic_forward(lat, lon, lat0, lon0, R):
    _pt(lat, lon, 90)
    _lat("lat0", lat0, True)
    _lon("lon0", lon0)
    _sphere_R(R)
    p, p0, dl = lat * DEG, lat0 * DEG, _nlon(lon - lon0) * DEG
    cc = math.sin(p0) * math.sin(p) + math.cos(p0) * math.cos(p) * math.cos(dl)
    _need(cc >= 0.1, "the point must lie within about 84 degrees of the centre")
    return _out(x=R * math.cos(p) * math.sin(dl) / cc, y=R * (math.cos(p0) * math.sin(p) - math.sin(p0) * math.cos(p) * math.cos(dl)) / cc)


def _ref_gnomonic_forward(lat, lon, lat0, lon0, R):
    x, y = _proj(f"+proj=gnom +lat_0={lat0!r} +lon_0={lon0!r} +R={R!r}")(lon, lat)
    return {"x": x, "y": y}


_add("geo_gnomonic_forward",
     name="Gnomonic projection (sphere)", topic=_PROJ,
     purpose="x and y of a point in the gnomonic projection (great circles become straight lines) of a sphere of radius R centred on a point.",
     inputs=_AZ_IN, outputs=[_o("x", "length", "x"), _o("y", "length", "y")],
     primary="x", conditions=["the point within about 84 degrees of the centre (cos c >= 0.1)"],
     method="the orthographic numerators divided by cos c, the cosine of the angular distance from the centre",
     latex=r"x = \frac{R\cos\varphi\sin\Delta\lambda}{\cos c},\quad y = \frac{R\left(\cos\varphi_0\sin\varphi - \sin\varphi_0\cos\varphi\cos\Delta\lambda\right)}{\cos c}",
     run=gnomonic_forward, reference=_ref_gnomonic_forward, rand=lambda rng: _rand_ortho(rng, 80),
     wrong=lambda lat, lon, lat0, lon0, R: orthographic_forward(lat, lon, lat0, lon0, R),
     phrases=("Gnomonic forward", "Gnomonic projection"),
     examples=[_ex({"lat": 40.0, "lon": 10.0, "lat0": 45, "lon0": 0, "R": "6371000 m"}), _ex({"lat": 60.0, "lon": -30.0, "lat0": 90, "lon0": 0, "R": "1 m"})],
     level="intermediate")


def equirectangular_forward(lat, lon, lat_ts, lat0, lon0, R):
    _pt(lat, lon, 90)
    _lat("lat_ts", lat_ts)
    _lat("lat0", lat0, True)
    _lon("lon0", lon0)
    _sphere_R(R)
    return _out(x=R * _nlon(lon - lon0) * DEG * math.cos(lat_ts * DEG), y=R * (lat - lat0) * DEG)


def _ref_equirectangular_forward(lat, lon, lat_ts, lat0, lon0, R):
    x, y = _proj(f"+proj=eqc +lat_ts={lat_ts!r} +lat_0={lat0!r} +lon_0={lon0!r} +R={R!r}")(lon, lat)
    return {"x": x, "y": y}


_add("geo_equirectangular_forward",
     name="Equirectangular (equidistant cylindrical / plate carree) projection", topic=_PROJ,
     purpose="x and y of a point in the equirectangular (equidistant cylindrical) projection of a sphere of radius R with a standard parallel (lat_ts = 0 is the plate carree).",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lat_ts", "standard parallel (true scale)"), _degin("lat0", "latitude of origin"),
             _degin("lon0", "central meridian"), _phys("R", "length", "radius of the sphere")],
     outputs=[_o("x", "length", "x"), _o("y", "length", "y")],
     primary="x", conditions=["sphere of radius R > 0"],
     method="x = R (lambda - lambda0) cos(lat_ts), y = R (phi - phi0)",
     latex=r"x = R(\lambda-\lambda_0)\cos\varphi_{ts},\quad y = R(\varphi-\varphi_0)",
     run=equirectangular_forward, reference=_ref_equirectangular_forward,
     rand=lambda rng: dict(_rand_sph(rng), lat_ts=_sig(rng.choice([0.0, rng.uniform(-60, 60)])), lat0=_sig(rng.choice([0.0, rng.uniform(-60, 60)]))),
     wrong=lambda lat, lon, lat_ts, lat0, lon0, R: _out(x=R * _nlon(lon - lon0) * DEG * math.cos(lat * DEG), y=R * (lat - lat0) * DEG),
     phrases=("Equirectangular forward", "Equirectangular projection", "Plate carree projection"),
     examples=[_ex({"lat": 45.0, "lon": 10.0, "lat_ts": 0, "lat0": 0, "lon0": 0, "R": "6371000 m"}),
               _ex({"lat": -20.0, "lon": 30.0, "lat_ts": 30, "lat0": 0, "lon0": 20, "R": "6371007 m"})],
     level="beginner", weak="PROJ's eqc implements the same two linear formulas")


def cea_forward(lat, lon, lat_ts, lon0, R):
    _pt(lat, lon, 90)
    _lat("lat_ts", lat_ts)
    _lon("lon0", lon0)
    _sphere_R(R)
    c = math.cos(lat_ts * DEG)
    return _out(x=R * _nlon(lon - lon0) * DEG * c, y=R * math.sin(lat * DEG) / c)


def _ref_cea_forward(lat, lon, lat_ts, lon0, R):
    x, y = _proj(f"+proj=cea +lat_ts={lat_ts!r} +lon_0={lon0!r} +R={R!r}")(lon, lat)
    return {"x": x, "y": y}


_add("geo_cylindrical_equal_area_forward",
     name="Lambert cylindrical equal-area projection (sphere)", topic=_PROJ,
     purpose="x and y of a point in the cylindrical equal-area projection of a sphere of radius R with a standard parallel (0 Lambert, 30 Behrmann, 45 Gall-Peters).",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _degin("lat_ts", "standard parallel"), _degin("lon0", "central meridian"),
             _phys("R", "length", "radius of the sphere")],
     outputs=[_o("x", "length", "x"), _o("y", "length", "y")],
     primary="y", conditions=["standard parallel strictly between -90 and 90"],
     method="x = R (lambda - lambda0) cos(lat_ts), y = R sin(phi)/cos(lat_ts)",
     latex=r"x = R(\lambda-\lambda_0)\cos\varphi_s,\quad y = \frac{R\sin\varphi}{\cos\varphi_s}",
     run=cea_forward, reference=_ref_cea_forward,
     rand=lambda rng: dict({k: v for k, v in _rand_sph(rng).items()}, lat_ts=_sig(rng.choice([0.0, 30.0, 45.0, rng.uniform(-60, 60)]))),
     wrong=lambda lat, lon, lat_ts, lon0, R: _out(x=R * _nlon(lon - lon0) * DEG * math.cos(lat_ts * DEG), y=R * lat * DEG / math.cos(lat_ts * DEG)),
     phrases=("Cylindrical equal-area forward", "Cylindrical equal-area projection", "Gall-Peters projection"),
     examples=[_ex({"lat": 45.0, "lon": 10.0, "lat_ts": 45, "lon0": 0, "R": "6371000 m"}), _ex({"lat": -30.0, "lon": 120.0, "lat_ts": 30, "lon0": 0, "R": "1 m"})],
     level="beginner")


# ---------------------------------------------------------------- Tissot's indicatrix
def _tissot_derivs(projection, p, l):
    if projection == "mercator":
        return 1.0, 0.0, 0.0, 1 / math.cos(p)
    if projection == "equirectangular":
        return 1.0, 0.0, 0.0, 1.0
    if projection == "cylindrical_equal_area":
        return 1.0, 0.0, 0.0, math.cos(p)
    if projection == "sinusoidal":
        return math.cos(p), 0.0, -l * math.sin(p), 1.0
    th = p
    for _ in range(100):
        d = (2 * th + math.sin(2 * th) - math.pi * math.sin(p)) / (2 + 2 * math.cos(2 * th))
        th -= d
        if abs(d) < 1e-15:
            break
    dth = math.pi * math.cos(p) / (4 * math.cos(th) ** 2)
    c = 2 * math.sqrt(2) / math.pi
    return c * math.cos(th), 0.0, -c * l * math.sin(th) * dth, math.sqrt(2) * math.cos(th) * dth


def tissot_indicatrix(projection, lat, lon):
    _need(projection in _TISSOT, "the projection must be one of " + ", ".join(_TISSOT))
    _lat("lat", lat)
    _need(abs(lat) <= 80, "the latitude must lie within 80 degrees of the equator")
    _fin("lon", lon)
    _need(-180 <= lon <= 180, "the longitude from the central meridian must lie between -180 and 180")
    p, l = lat * DEG, lon * DEG
    xl, yl, xp, yp = _tissot_derivs(projection, p, l)
    h = math.hypot(xp, yp)
    k = math.hypot(xl, yl) / math.cos(p)
    cross = abs(yp * xl - xp * yl) / (h * k * math.cos(p))
    dot = (xp * xl + yp * yl) / (h * k * math.cos(p))
    th = math.atan2(cross, abs(dot))
    st, ct = math.sin(th), math.cos(th)
    ap = math.sqrt(h * h + k * k + 2 * h * k * st)
    bp = math.sqrt((h - k) ** 2 + 2 * h * k * ct * ct / (1 + st))     # h^2 + k^2 - 2hk sin(theta'), without cancellation
    a, b = (ap + bp) / 2, (ap - bp) / 2
    omega = 2 * math.asin((a - b) / (a + b))
    if abs(a - b) <= 1e-12 * a:
        a = b = (a + b) / 2
        omega = 0.0
    return _out(h=h, k=k, theta_prime=th / DEG, a=a, b=b, omega=omega / DEG, s=h * k * st)


_TISSOT = ["mercator", "equirectangular", "cylindrical_equal_area", "sinusoidal", "mollweide"]
_TPROJ = {"mercator": "merc", "equirectangular": "eqc", "cylindrical_equal_area": "cea", "sinusoidal": "sinu", "mollweide": "moll"}


def _ref_tissot(projection, lat, lon):
    f = _proj(f"+proj={_TPROJ[projection]} +R=1").get_factors(lon, lat)
    # PROJ differentiates numerically: an angle of exactly 90 degrees comes back as 89.999999, and for a conformal
    # projection the split of a and b is sqrt-sensitive to that noise; both are snapped to their exact limits
    th = 90.0 if abs(f.meridian_parallel_angle - 90.0) < 1e-5 else f.meridian_parallel_angle
    om, a, b = f.angular_distortion, f.tissot_semimajor, f.tissot_semiminor
    if abs(om) < 1e-5:
        om, a, b = 0.0, (a + b) / 2, (a + b) / 2
    return {"h": f.meridional_scale, "k": f.parallel_scale, "theta_prime": th, "a": a, "b": b, "omega": om, "s": f.areal_scale}


_add("geo_tissot_indicatrix",
     name="Tissot's indicatrix: scale factors and distortion of a projection at a point", topic=_PROJ,
     purpose="The meridian and parallel scale factors h and k, the angle between meridian and parallel, the semi-axes a and b of Tissot's indicatrix, the maximum angular distortion and the areal scale of a world projection of the sphere at a point.",
     inputs=[_choice("projection", _TISSOT, "projection (spherical form, central meridian 0)"), _degin("lat", "latitude"),
             _degin("lon", "longitude measured from the central meridian")],
     outputs=[_o("h", "number", "scale factor along the meridian"), _o("k", "number", "scale factor along the parallel"),
              _o("theta_prime", "number", "angle between the projected meridian and parallel (degrees, at most 90)"),
              _o("a", "number", "maximum scale factor (semi-major axis)"), _o("b", "number", "minimum scale factor (semi-minor axis)"),
              _o("omega", "number", "maximum angular distortion (degrees)"), _o("s", "number", "areal scale factor")],
     primary="h", conditions=["|latitude| <= 80"],
     method="partial derivatives of the projection: h = |d(x,y)/dphi|, k = |d(x,y)/dlambda|/cos(phi), sin(theta') from their cross product; a' = sqrt(h^2+k^2+2hk sin theta'), b' = sqrt(h^2+k^2-2hk sin theta'), a = (a'+b')/2, b = (a'-b')/2, omega = 2 asin((a-b)/(a+b)), s = hk sin theta'",
     latex=r"a' = \sqrt{h^2+k^2+2hk\sin\theta'},\ b' = \sqrt{h^2+k^2-2hk\sin\theta'},\ a = \tfrac{a'+b'}{2},\ b = \tfrac{a'-b'}{2},\ \omega = 2\arcsin\frac{a-b}{a+b},\ s = hk\sin\theta'",
     run=tissot_indicatrix, reference=_ref_tissot,
     rand=lambda rng: {"projection": rng.choice(_TISSOT), "lat": _sig(rng.choice([rng.uniform(-79, -20), rng.uniform(20, 79)])),
                       "lon": _sig(rng.choice([rng.uniform(-170, -20), rng.uniform(20, 170)]))},
     wrong=lambda projection, lat, lon: (lambda r: dict(r, k=r["k"] * math.cos(lat * DEG), s=r["s"] * math.cos(lat * DEG)))(tissot_indicatrix(projection, lat, lon)),
     phrases=("Tissot indicatrix", "Tissot's indicatrix", "Projection distortion"),
     examples=[_ex({"projection": "mercator", "lat": 60, "lon": 0}), _ex({"projection": "sinusoidal", "lat": 40, "lon": 30}),
               _ex({"projection": "mollweide", "lat": 50, "lon": 100})],
     level="expert")


# ================================================================================================ GRID REFERENCES AND UNITS
def dms_to_decimal(degrees, minutes, seconds, hemisphere):
    _need(isinstance(degrees, int) and degrees >= 0, "degrees must be a whole number >= 0 (the hemisphere gives the sign)")
    _need(isinstance(minutes, int) and 0 <= minutes <= 59, "minutes must be a whole number from 0 to 59")
    _fin("seconds", seconds)
    _need(0 <= seconds < 60, "seconds must lie in [0, 60)")
    _need(hemisphere in ("N", "S", "E", "W"), "the hemisphere must be N, S, E or W")
    v = Fraction(degrees) + Fraction(minutes, 60) + Fraction(repr(float(seconds))) / 3600
    _need(v <= (90 if hemisphere in "NS" else 180), "a latitude is at most 90 and a longitude at most 180 degrees")
    if hemisphere in "SW":
        v = -v
    return {"decimal": float(v) + 0.0, "radians": _r(float(v) * DEG)}


def _ref_dms(degrees, minutes, seconds, hemisphere):
    total = (degrees * 3600 + minutes * 60) + seconds
    d = total / 3600.0 * (-1 if hemisphere in ("S", "W") else 1)
    return {"decimal": d, "radians": math.radians(d)}


_add("geo_dms_to_decimal",
     name="Degrees, minutes and seconds to decimal degrees", topic="coordinate notation",
     purpose="A latitude or longitude written in degrees, minutes and seconds with a hemisphere letter, converted to signed decimal degrees and radians.",
     inputs=[_intin("degrees", "whole degrees (>= 0)", 0, 180), _intin("minutes", "whole minutes (0-59)", 0, 59), _numin("seconds", "seconds (0 <= s < 60)"),
             _choice("hemisphere", ["N", "S", "E", "W"], "hemisphere letter (S and W are negative)")],
     outputs=[_o("decimal", "number", "signed decimal degrees"), _o("radians", "number", "signed angle in radians")],
     primary="decimal", conditions=["0 <= minutes <= 59", "0 <= seconds < 60", "at most 90 (N/S) or 180 (E/W) degrees"],
     method="decimal = sign (degrees + minutes/60 + seconds/3600), exact rational arithmetic",
     latex=r"\varphi = \pm\left(d + \frac{m}{60} + \frac{s}{3600}\right)",
     run=dms_to_decimal, reference=_ref_dms,
     rand=lambda rng: (lambda h: {"degrees": rng.randint(0, 89 if h in "NS" else 179), "minutes": rng.randint(0, 59), "seconds": round(rng.uniform(0, 59.999), 3),
                                  "hemisphere": h})(rng.choice("NSEW")),
     wrong=lambda degrees, minutes, seconds, hemisphere: (lambda d: {"decimal": d, "radians": math.radians(d)})(
         (degrees + minutes / 100 + seconds / 10000) * (-1 if hemisphere in "SW" else 1)),
     phrases=("DMS to decimal degrees", "Degrees minutes seconds to decimal"),
     examples=[_ex({"degrees": 51, "minutes": 28, "seconds": 38.0, "hemisphere": "N"}), _ex({"degrees": 0, "minutes": 7, "seconds": 39.0, "hemisphere": "W"})],
     extra_q=[{"q": "DMS to decimal degrees: degrees = 51, minutes = 75, seconds = 38, hemisphere = N", "outcome": "INVALID_INPUT"}],
     level="beginner", weak="the same sum computed in floating point (a pure definition)")


def decimal_to_dms(decimal, axis):
    _fin("the angle", decimal)
    _need(axis in ("latitude", "longitude"), "the axis must be latitude or longitude")
    _need(abs(decimal) <= (90 if axis == "latitude" else 180), "a latitude is at most 90 and a longitude at most 180 degrees in size")
    v = abs(Fraction(repr(float(decimal))))
    d = int(v)
    m = int((v - d) * 60)
    s = (v - d - Fraction(m, 60)) * 3600
    hemi = ("N" if decimal >= 0 else "S") if axis == "latitude" else ("E" if decimal >= 0 else "W")
    return {"degrees": d, "minutes": m, "seconds": _r(float(s)), "hemisphere": hemi}


def _ref_decimal_to_dms(decimal, axis):
    from decimal import Decimal
    v = abs(Decimal(repr(float(decimal))))
    total = v * 3600
    d = int(total // 3600)
    m = int((total - d * 3600) // 60)
    s = total - d * 3600 - m * 60
    hemi = ("N" if decimal >= 0 else "S") if axis == "latitude" else ("E" if decimal >= 0 else "W")
    return {"degrees": d, "minutes": m, "seconds": float(s), "hemisphere": hemi}


_add("geo_decimal_to_dms",
     name="Decimal degrees to degrees, minutes and seconds", topic="coordinate notation",
     purpose="A signed latitude or longitude in decimal degrees written as whole degrees, whole minutes, seconds and a hemisphere letter.",
     inputs=[_degin("decimal", "signed angle"), _choice("axis", ["latitude", "longitude"], "whether the angle is a latitude (N/S) or a longitude (E/W)")],
     outputs=[_o("degrees", "number", "whole degrees"), _o("minutes", "number", "whole minutes"), _o("seconds", "number", "seconds"), _o("hemisphere", "text", "N, S, E or W")],
     primary="seconds", conditions=["|latitude| <= 90, |longitude| <= 180"],
     method="exact decimal arithmetic: degrees = floor(|x|), minutes = floor(60 frac), seconds = the remainder times 3600",
     latex=r"d = \lfloor|x|\rfloor,\quad m = \lfloor60(|x| - d)\rfloor,\quad s = 3600\left(|x| - d - \tfrac{m}{60}\right)",
     run=decimal_to_dms, reference=_ref_decimal_to_dms,
     rand=lambda rng: (lambda ax: {"decimal": round(rng.uniform(-89.99, 89.99) if ax == "latitude" else rng.uniform(-179.99, 179.99), 6), "axis": ax})(rng.choice(["latitude", "longitude"])),
     wrong=lambda decimal, axis: (lambda v: {"degrees": int(v), "minutes": int((v - int(v)) * 100), "seconds": _r(((v - int(v)) * 100 - int((v - int(v)) * 100)) * 100),
                                             "hemisphere": "N"})(abs(decimal)),
     phrases=("Decimal degrees to DMS", "Decimal to degrees minutes seconds"),
     examples=[_ex({"decimal": 51.477222, "axis": "latitude"}), _ex({"decimal": -0.1275, "axis": "longitude"})],
     level="beginner", weak="the same conversion in Python's decimal arithmetic (a pure definition)")


def map_scale_ground_distance(map_distance, scale):
    _pos("the map distance", map_distance)
    _pos("the scale denominator", scale)
    return {"ground_distance": float(Fraction(repr(map_distance)) * Fraction(repr(float(scale)))), "ground_km": float(Fraction(repr(map_distance)) * Fraction(repr(float(scale))) / 1000)}


_add("geo_map_scale_ground_distance",
     name="Ground distance from a map distance and a representative fraction", topic="cartography",
     purpose="The real-world distance represented by a distance measured on a map of scale 1 : scale.",
     inputs=[_phys("map_distance", "length", "distance measured on the map"), _numin("scale", "scale denominator (1 : scale)")],
     outputs=[_o("ground_distance", "length", "ground distance"), _o("ground_km", "number", "ground distance in kilometres")],
     primary="ground_distance", conditions=["positive map distance and scale"],
     method="ground distance = map distance x scale denominator",
     latex=r"D = d \times S",
     run=map_scale_ground_distance, reference=lambda map_distance, scale: {"ground_distance": map_distance * scale, "ground_km": map_distance * scale / 1000},
     rand=lambda rng: {"map_distance": _sig(_lu(rng, 1e-3, 1.0), 4), "scale": float(rng.choice([1000, 2500, 10000, 24000, 25000, 50000, 63360, 100000, 250000, 1000000]))},
     wrong=lambda map_distance, scale: {"ground_distance": map_distance * scale / 100, "ground_km": map_distance * scale / 100000},
     phrases=("Map scale ground distance", "Ground distance from map scale"),
     examples=[_ex({"map_distance": "4.2 cm", "scale": 50000}), _ex({"map_distance": "12 mm", "scale": 25000})],
     level="beginner", weak="the same product in floating point (a pure definition)")


def map_scale_from_distances(map_distance, ground_distance):
    _pos("the map distance", map_distance)
    _pos("the ground distance", ground_distance)
    _need(ground_distance > map_distance, "the ground distance must exceed the map distance")
    s = Fraction(repr(ground_distance)) / Fraction(repr(map_distance))
    return {"scale": _r(float(s)), "rf": _r(float(1 / s))}


_add("geo_map_scale_from_distances",
     name="Map scale (representative fraction) from a map and a ground distance", topic="cartography",
     purpose="The scale denominator and representative fraction of a map on which a known ground distance measures a given map distance.",
     inputs=[_phys("map_distance", "length", "distance on the map"), _phys("ground_distance", "length", "corresponding ground distance")],
     outputs=[_o("scale", "number", "scale denominator S (scale 1 : S)"), _o("rf", "number", "representative fraction 1/S")],
     primary="scale", conditions=["ground distance > map distance > 0"],
     method="S = ground distance / map distance in the same unit",
     latex=r"S = \frac{D}{d},\quad RF = \frac{1}{S}",
     run=map_scale_from_distances, reference=lambda map_distance, ground_distance: {"scale": ground_distance / map_distance, "rf": map_distance / ground_distance},
     rand=lambda rng: {"map_distance": _sig(_lu(rng, 1e-3, 1.0), 4), "ground_distance": _sig(_lu(rng, 10, 1e5), 5)},
     wrong=lambda map_distance, ground_distance: {"scale": ground_distance / map_distance / 100, "rf": map_distance / ground_distance * 100},
     phrases=("Map scale from distances", "Representative fraction from distances"),
     examples=[_ex({"map_distance": "5 cm", "ground_distance": "2.5 km"}), _ex({"map_distance": "3.2 cm", "ground_distance": "800 m"})],
     level="beginner", weak="the same quotient in floating point (a pure definition)")


_GH = "0123456789bcdefghjkmnpqrstuvwxyz"


def _gh_bounds(code):
    lat, lon = [-90.0, 90.0], [-180.0, 180.0]
    even = True
    for ch in code:
        _need(ch in _GH, "a geohash uses the characters 0-9 and b-z without a, i, l, o")
        v = _GH.index(ch)
        for b in range(4, -1, -1):
            bit = (v >> b) & 1
            rng_ = lon if even else lat
            mid = (rng_[0] + rng_[1]) / 2
            if bit:
                rng_[0] = mid
            else:
                rng_[1] = mid
            even = not even
    return lat, lon


def geohash_encode(lat, lon, precision):
    _lat("lat", lat, True)
    _lon("lon", lon)
    _need(isinstance(precision, int) and 1 <= precision <= 12, "the precision must be a whole number of characters from 1 to 12")
    lon = _nlon(lon) if lon != 180 else 180.0
    la, lo = [-90.0, 90.0], [-180.0, 180.0]
    code, bits, v, even = [], 0, 0, True
    while len(code) < precision:
        r, x = (lo, lon) if even else (la, lat)
        mid = (r[0] + r[1]) / 2
        if x >= mid:
            v = v * 2 + 1
            r[0] = mid
        else:
            v = v * 2
            r[1] = mid
        even = not even
        bits += 1
        if bits == 5:
            code.append(_GH[v])
            bits, v = 0, 0
    return {"geohash": "".join(code), "lat_center": (la[0] + la[1]) / 2, "lon_center": (lo[0] + lo[1]) / 2,
            "lat_error": (la[1] - la[0]) / 2, "lon_error": (lo[1] - lo[0]) / 2}


def _ref_geohash_encode(lat, lon, precision):
    import pygeohash as pgh
    code = pgh.encode(lat, lon, precision=precision)
    la, lo, le, loe = pgh.decode_exactly(code)
    return {"geohash": code, "lat_center": la, "lon_center": lo, "lat_error": le, "lon_error": loe}


def _rand_gh(rng):
    for _ in range(100):
        x = {"lat": _sig(rng.uniform(-89.9, 89.9), 9), "lon": _sig(rng.uniform(-179.9, 179.9), 9), "precision": rng.randint(1, 12)}
        o = geohash_encode(**x)
        if abs(x["lat"] - o["lat_center"]) < 0.999 * o["lat_error"] and abs(x["lon"] - o["lon_center"]) < 0.999 * o["lon_error"]:
            return x
    return x


_add("geo_geohash_encode",
     name="Geohash of a point and its cell", topic="coordinate notation",
     purpose="The geohash of a latitude and longitude at a given number of characters, with the centre and half-sizes of the geohash cell.",
     inputs=[_degin("lat", "latitude"), _degin("lon", "longitude"), _intin("precision", "number of geohash characters (1-12)", 1, 12)],
     outputs=[_o("geohash", "text", "geohash"), _o("lat_center", "number", "latitude of the cell centre"), _o("lon_center", "number", "longitude of the cell centre"),
              _o("lat_error", "number", "half the cell height (degrees)"), _o("lon_error", "number", "half the cell width (degrees)")],
     primary="lat_center", conditions=["1 to 12 characters"],
     method="interleaved binary bisection of longitude and latitude (longitude first), five bits per base-32 character",
     latex=r"\text{bits} = \lambda_1\varphi_1\lambda_2\varphi_2\ldots,\quad \text{character} = \text{base32}(5\ \text{bits})",
     run=geohash_encode, reference=_ref_geohash_encode, rand=_rand_gh,
     wrong=lambda lat, lon, precision: geohash_encode(lon / 2, lat * 2 if abs(lat) <= 90 else lat, precision),
     phrases=("Geohash encode", "Geohash of a point", "Encode geohash"),
     examples=[_ex({"lat": 57.64911, "lon": 10.40744, "precision": 11}), _ex({"lat": -33.8688, "lon": 151.2093, "precision": 6})],
     level="beginner", weak="pygeohash implements the same bisection definition")


def geohash_decode(geohash):
    _need(isinstance(geohash, str) and 1 <= len(geohash) <= 12, "a geohash has 1 to 12 characters")
    la, lo = _gh_bounds(geohash.lower())
    return {"lat_center": (la[0] + la[1]) / 2, "lon_center": (lo[0] + lo[1]) / 2, "lat_error": (la[1] - la[0]) / 2, "lon_error": (lo[1] - lo[0]) / 2}


def _ref_geohash_decode(geohash):
    import pygeohash as pgh
    la, lo, le, loe = pgh.decode_exactly(geohash.lower())
    return {"lat_center": la, "lon_center": lo, "lat_error": le, "lon_error": loe}


_add("geo_geohash_decode",
     name="Decode a geohash: cell centre and size", topic="coordinate notation",
     purpose="The centre latitude and longitude of a geohash cell and its half-height and half-width in degrees.",
     inputs=[_textin("geohash", "geohash string (characters 0-9, b-z without a, i, l, o)", True, 1, 12)],
     outputs=[_o("lat_center", "number", "latitude of the cell centre"), _o("lon_center", "number", "longitude of the cell centre"),
              _o("lat_error", "number", "half the cell height (degrees)"), _o("lon_error", "number", "half the cell width (degrees)")],
     primary="lat_center", conditions=["1 to 12 valid characters"],
     method="each character gives five bits that halve the longitude and latitude intervals alternately (longitude first)",
     latex=r"[\varphi_{min}, \varphi_{max}] \times [\lambda_{min}, \lambda_{max}] \ \text{by alternate bisection}",
     run=geohash_decode, reference=_ref_geohash_decode,
     rand=lambda rng: {"geohash": "".join(rng.choice(_GH) for _ in range(rng.randint(1, 12)))},
     wrong=lambda geohash: (lambda r: dict(r, lat_center=r["lon_center"] / 2, lon_center=r["lat_center"] * 2))(geohash_decode(geohash)),
     phrases=("Geohash decode", "Decode geohash"),
     examples=[_ex({"geohash": "u4pruydqqvj"}), _ex({"geohash": "9q8yy"})],
     extra_q=[{"q": "Geohash decode: geohash = u4pruaqqvj", "outcome": "INVALID_INPUT"}],
     level="beginner", weak="pygeohash implements the same bisection definition")


_MGRS_COLS = ["ABCDEFGH", "JKLMNPQR", "STUVWXYZ"]
_MGRS_ROWS = "ABCDEFGHJKLMNPQRSTUV"


def mgrs_encode(lat, lon, ellipsoid="WGS84"):
    E = _ellipsoid(ellipsoid)
    z = utm_zone(lat, lon)
    zone = z["zone"]
    X, Y, g, k = _tm_fwd(E, lat * DEG, _nlon(lon - z["central_meridian"]) * DEG)
    e = 500000.0 + 0.9996 * Y
    n = (0.0 if lat >= 0 else 1e7) + 0.9996 * X
    col = int(math.floor(e / 100000.0))
    _need(1 <= col <= 8, "the point lies outside its zone's grid squares")
    letter_e = _MGRS_COLS[(zone - 1) % 3][col - 1]
    row = int(math.floor(n / 100000.0)) % 20
    if zone % 2 == 0:
        row = (row + 5) % 20
    ee = int(math.floor(e)) % 100000
    nn = int(math.floor(n)) % 100000
    return {"mgrs": f"{zone:02d}{z['band']}{letter_e}{_MGRS_ROWS[row]}{ee:05d}{nn:05d}"}


def _ref_mgrs(lat, lon, ellipsoid="WGS84"):
    import mgrs
    return {"mgrs": mgrs.MGRS().toMGRS(lat, lon, MGRSPrecision=5).replace(" ", "")}


def _rand_mgrs(rng):
    for _ in range(200):
        x = {"lat": _sig(rng.uniform(-79.5, 83.5), 9), "lon": _sig(rng.uniform(-179.9, 179.9), 9)}
        E = _ellipsoid("WGS84")
        z = utm_zone(x["lat"], x["lon"])
        X, Y, g, k = _tm_fwd(E, x["lat"] * DEG, _nlon(x["lon"] - z["central_meridian"]) * DEG)
        e, n = 500000.0 + 0.9996 * Y, 0.9996 * X
        if min(e % 1, 1 - e % 1, n % 1, 1 - n % 1) > 0.01 and abs(x["lat"]) > 0.01:
            return x
    return x


_add("geo_mgrs_encode",
     name="MGRS grid reference of a point (1 m)", topic="coordinate notation",
     purpose="The Military Grid Reference System (MGRS / USNG) reference of a WGS84 latitude and longitude to 1 m: zone, latitude band, 100 km square letters and 5-digit easting and northing.",
     inputs=[_degin("lat", "latitude (WGS84)"), _degin("lon", "longitude (WGS84)")],
     outputs=[_o("mgrs", "text", "MGRS reference (no spaces)")],
     primary="mgrs", conditions=["80 S <= latitude <= 84 N (polar regions use UPS)"],
     method="UTM coordinates (Krueger series), 100 km column letter from the zone's set (A-H, J-R, S-Z), row letter cycling A-V (offset by 5 in even zones), digits truncated to 1 m; the zone is written with two digits",
     latex=r"\text{MGRS} = z\,B\,C_{100k}R_{100k}\,\lfloor E\rfloor \bmod 10^5\,\lfloor N\rfloor \bmod 10^5",
     run=mgrs_encode, reference=_ref_mgrs, rand=_rand_mgrs,
     wrong=lambda lat, lon, ellipsoid="WGS84": (lambda r: {"mgrs": r["mgrs"][:-10] + r["mgrs"][-5:] + r["mgrs"][-10:-5]})(mgrs_encode(lat, lon)),
     phrases=("MGRS grid reference", "MGRS encode", "Military grid reference"),
     examples=[_ex({"lat": 50.0, "lon": 10.0}), _ex({"lat": 38.8977, "lon": -77.0365}), _ex({"lat": -33.8568, "lon": 151.2153})],
     level="intermediate")


# ================================================================================================ PLANAR ANALYSIS
_PLAN = "planar vector analysis"


def _seg_dist(px, py, ax, ay, bx, by):
    dx, dy = bx - ax, by - ay
    L2 = dx * dx + dy * dy
    if L2 == 0:
        return math.hypot(px - ax, py - ay)
    t = max(0.0, min(1.0, ((px - ax) * dx + (py - ay) * dy) / L2))
    return math.hypot(px - (ax + t * dx), py - (ay + t * dy))


def douglas_peucker(x, y, tolerance):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(n >= 3, "a line needs at least 3 vertices to simplify")
    _pos("the tolerance", tolerance)
    _need(len(set(zip(x, y))) == n, "the vertices must be distinct")
    keep = [False] * n
    keep[0] = keep[-1] = True
    stack = [(0, n - 1)]
    while stack:
        i, j = stack.pop()
        best, idx = -1.0, -1
        for k in range(i + 1, j):
            d = _seg_dist(x[k], y[k], x[i], y[i], x[j], y[j])
            if d > best:
                best, idx = d, k
        if idx >= 0 and best > tolerance:
            keep[idx] = True
            stack.append((i, idx))
            stack.append((idx, j))
    kept = [float(i) for i in range(n) if keep[i]]
    return {"kept": kept, "n_kept": len(kept)}


def _ref_douglas_peucker(x, y, tolerance):
    import shapely
    line = shapely.LineString(list(zip(x, y)))
    s = shapely.simplify(line, tolerance, preserve_topology=False)
    pos = {(a, b): i for i, (a, b) in enumerate(zip(x, y))}
    kept = [float(pos[tuple(c)]) for c in s.coords]
    return {"kept": kept, "n_kept": len(kept)}


def _rand_dp(rng):
    n = rng.randint(6, 40)
    x, y, t = [], [], 0.0
    for i in range(n):
        t += rng.uniform(0.5, 2.0)
        x.append(round(t, 3))
        y.append(round(rng.gauss(0, 1) + 0.3 * math.sin(t), 3))
    d = [abs(v) for v in y]
    return {"x": x, "y": y, "tolerance": round(rng.uniform(0.2, 1.2), 3)}


_add("geo_douglas_peucker_simplification",
     name="Douglas-Peucker line simplification", topic=_PLAN,
     purpose="The vertices of a polyline kept by the Ramer-Douglas-Peucker simplification at a distance tolerance (indices from 0).",
     inputs=[_vecin("x", "x coordinates of the vertices in order", 3, 500), _vecin("y", "y coordinates of the vertices", 3, 500),
             _numin("tolerance", "distance tolerance (same unit as the coordinates)")],
     outputs=[_o("kept", "number", "indices of the kept vertices (from 0, in order)"), _o("n_kept", "number", "number of kept vertices")],
     primary="n_kept", conditions=["distinct vertices", "tolerance > 0"],
     method="recursive: keep the vertex farthest from the segment joining the ends of a run if its distance exceeds the tolerance, then split there",
     latex=r"d_{max} = \max_k \operatorname{dist}(P_k, \overline{P_iP_j}) > \varepsilon \Rightarrow \text{keep } P_k",
     run=douglas_peucker, reference=_ref_douglas_peucker, rand=_rand_dp,
     wrong=lambda x, y, tolerance: douglas_peucker(x, y, tolerance * 2),
     phrases=("Douglas-Peucker simplification", "Line simplification", "Ramer-Douglas-Peucker"),
     examples=[_ex({"x": [0, 1, 2, 3, 4, 5, 6, 7], "y": [0, 0.1, -0.1, 5, 6, 7, 8.1, 9], "tolerance": 1.0}),
               _ex({"x": [0, 2, 4, 6, 8, 10], "y": [0, 1.5, 0.2, 2.4, 0.1, 0], "tolerance": 0.5})],
     level="intermediate")


def frechet_distance(x1, y1, x2, y2):
    _need(len(x1) == len(y1) and len(x2) == len(y2), "give one y per x")
    _need(len(x1) >= 2 and len(x2) >= 2, "each line needs at least 2 vertices")
    n, m = len(x1), len(x2)
    c = [[0.0] * m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            d = math.hypot(x1[i] - x2[j], y1[i] - y2[j])
            if i == 0 and j == 0:
                c[i][j] = d
            elif i == 0:
                c[i][j] = max(c[i][j - 1], d)
            elif j == 0:
                c[i][j] = max(c[i - 1][j], d)
            else:
                c[i][j] = max(min(c[i - 1][j], c[i - 1][j - 1], c[i][j - 1]), d)
    return {"distance": c[n - 1][m - 1]}


def _ref_frechet(x1, y1, x2, y2):
    """Decision version: the smallest pairwise vertex distance eps for which a monotone coupling exists (reachability
    over the free-space grid), found by binary search over the sorted candidate distances (scipy cdist). GEOS 3.14's
    DiscreteFrechetDistance returned a larger, non-optimal value on 2 of 2000 random cases, so it is not used."""
    import numpy as np
    from scipy.spatial.distance import cdist
    D = cdist(np.c_[x1, y1], np.c_[x2, y2])
    cand = np.unique(D)
    n, m = D.shape

    def ok(eps):
        F = D <= eps
        R = np.zeros_like(F)
        R[0, 0] = F[0, 0]
        for i in range(n):
            for j in range(m):
                if (i or j) and F[i, j]:
                    R[i, j] = (i > 0 and R[i - 1, j]) or (j > 0 and R[i, j - 1]) or (i > 0 and j > 0 and R[i - 1, j - 1])
        return bool(R[-1, -1])
    lo, hi = 0, len(cand) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if ok(cand[mid]):
            hi = mid
        else:
            lo = mid + 1
    return {"distance": float(cand[lo])}


def _rand_frechet(rng):
    n, m = rng.randint(2, 25), rng.randint(2, 25)
    x1 = sorted(round(rng.uniform(0, 100), 2) for _ in range(n))
    x2 = sorted(round(rng.uniform(0, 100), 2) for _ in range(m))
    return {"x1": x1, "y1": [round(rng.uniform(0, 20), 2) for _ in range(n)], "x2": x2, "y2": [round(rng.uniform(0, 20), 2) for _ in range(m)]}


_add("geo_discrete_frechet_distance",
     name="Discrete Frechet distance between two polylines", topic=_PLAN,
     purpose="The discrete Frechet ('dog-walking') distance between two polylines (GPS tracks, routes) measured over their vertices.",
     inputs=[_vecin("x1", "x coordinates of the first line", 2, 300), _vecin("y1", "y coordinates of the first line", 2, 300),
             _vecin("x2", "x coordinates of the second line", 2, 300), _vecin("y2", "y coordinates of the second line", 2, 300)],
     outputs=[_o("distance", "number", "discrete Frechet distance (coordinate units)")],
     primary="distance", conditions=["at least 2 vertices per line"],
     method="dynamic programme (Eiter-Mannila): c(i,j) = max(d(i,j), min(c(i-1,j), c(i-1,j-1), c(i,j-1)))",
     latex=r"c_{i,j} = \max\left(d(P_i, Q_j),\ \min(c_{i-1,j},\ c_{i-1,j-1},\ c_{i,j-1})\right)",
     run=frechet_distance, reference=_ref_frechet, rand=_rand_frechet,
     wrong=lambda x1, y1, x2, y2: {"distance": max(math.hypot(x1[i] - x2[round(i * (len(x2) - 1) / (len(x1) - 1))], y1[i] - y2[round(i * (len(x2) - 1) / (len(x1) - 1))])
                                                   for i in range(len(x1)))},
     phrases=("Frechet distance", "Discrete Frechet distance"),
     examples=[_ex({"x1": [0, 1, 2], "y1": [0, 0, 0], "x2": [0, 2], "y2": [1, 1]}), _ex({"x1": [0, 2, 4, 6], "y1": [0, 2, 0, 2], "x2": [0, 3, 6], "y2": [1, 1, 1]})],
     level="advanced")


def _polygon_simple(x, y):
    n = len(x)

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    P = list(zip(x, y))
    for i in range(n):
        a, b = P[i], P[(i + 1) % n]
        for j in range(i + 1, n):
            if j == i or (j + 1) % n == i or j == (i + 1) % n:
                continue
            c, d = P[j], P[(j + 1) % n]
            d1, d2, d3, d4 = cross(a, b, c), cross(a, b, d), cross(c, d, a), cross(c, d, b)
            if ((d1 > 0) != (d2 > 0)) and ((d3 > 0) != (d4 > 0)) and d1 and d2 and d3 and d4:
                return False
    return True


def _hull(P):
    P = sorted(set(P))
    if len(P) <= 2:
        return P
    cross = lambda o, a, b: (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo, up = [], []
    for p in P:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], p) <= 0:
            lo.pop()
        lo.append(p)
    for p in reversed(P):
        while len(up) >= 2 and cross(up[-2], up[-1], p) <= 0:
            up.pop()
        up.append(p)
    return lo[:-1] + up[:-1]


def _shoelace(P):
    return 0.5 * math.fsum(P[i][0] * P[(i + 1) % len(P)][1] - P[(i + 1) % len(P)][0] * P[i][1] for i in range(len(P)))


def _mec(P):
    """Minimum enclosing circle (incremental Welzl, in the given order)."""
    def c2(a, b):
        cx, cy = (a[0] + b[0]) / 2, (a[1] + b[1]) / 2
        return cx, cy, math.hypot(a[0] - cx, a[1] - cy)

    def c3(a, b, c):
        d = 2 * (a[0] * (b[1] - c[1]) + b[0] * (c[1] - a[1]) + c[0] * (a[1] - b[1]))
        if d == 0:
            cands = [c2(a, b), c2(a, c), c2(b, c)]
            return max(cands, key=lambda t: t[2])
        ux = ((a[0] ** 2 + a[1] ** 2) * (b[1] - c[1]) + (b[0] ** 2 + b[1] ** 2) * (c[1] - a[1]) + (c[0] ** 2 + c[1] ** 2) * (a[1] - b[1])) / d
        uy = ((a[0] ** 2 + a[1] ** 2) * (c[0] - b[0]) + (b[0] ** 2 + b[1] ** 2) * (a[0] - c[0]) + (c[0] ** 2 + c[1] ** 2) * (b[0] - a[0])) / d
        return ux, uy, math.hypot(a[0] - ux, a[1] - uy)

    inside = lambda c, p: math.hypot(p[0] - c[0], p[1] - c[1]) <= c[2] * (1 + 1e-12)
    c = None
    for i, p in enumerate(P):
        if c is None or not inside(c, p):
            c = (p[0], p[1], 0.0)
            for j in range(i):
                q = P[j]
                if not inside(c, q):
                    c = c2(p, q)
                    for k in range(j):
                        if not inside(c, P[k]):
                            c = c3(p, q, P[k])
    return c


def compactness(x, y):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(3 <= n <= 200, "a polygon needs 3 to 200 vertices")
    P = list(zip(x, y))
    _need(len(set(P)) == n, "the vertices must be distinct (do not repeat the first vertex)")
    _need(_polygon_simple(x, y), "the polygon must be simple (edges must not cross)")
    A = abs(_shoelace(P))
    _need(A > 0, "the polygon has no area")
    L = math.fsum(math.hypot(P[(i + 1) % n][0] - P[i][0], P[(i + 1) % n][1] - P[i][1]) for i in range(n))
    H = abs(_shoelace(_hull(P)))
    r = _mec(P)[2]
    pp = 4 * math.pi * A / (L * L)
    return _out(area=A, perimeter=L, polsby_popper=pp, schwartzberg=1 / math.sqrt(pp), convex_hull_ratio=A / H, reock=A / (math.pi * r * r))


def _ref_compactness(x, y):
    import shapely
    g = shapely.Polygon(list(zip(x, y)))
    A, L = g.area, g.length
    r = shapely.minimum_bounding_radius(g)
    return {"area": A, "perimeter": L, "polsby_popper": 4 * math.pi * A / L ** 2, "schwartzberg": L / (2 * math.sqrt(math.pi * A)),
            "convex_hull_ratio": A / g.convex_hull.area, "reock": A / (math.pi * r * r)}


def _rand_star(rng, n=None):
    n = n or rng.randint(3, 25)
    angs = sorted(rng.uniform(0, 2 * math.pi) for _ in range(n))
    if max(b - a for a, b in zip(angs, angs[1:] + [angs[0] + 2 * math.pi])) > 2.5:
        angs = sorted(2 * math.pi * k / n + rng.uniform(-0.2, 0.2) for k in range(n))
    cx, cy = rng.uniform(-1e4, 1e4), rng.uniform(-1e4, 1e4)
    R = _lu(rng, 10, 5000)
    x = [round(cx + R * rng.uniform(0.3, 1.0) * math.cos(a), 3) for a in angs]
    y = [round(cy + R * rng.uniform(0.3, 1.0) * math.sin(a), 3) for a in angs]
    return x, y


_add("geo_polygon_compactness",
     name="Shape compactness indices of a polygon (Polsby-Popper, Schwartzberg, convex hull, Reock)", topic=_PLAN,
     purpose="Area, perimeter and the standard compactness measures of a polygon in projected coordinates (used for districts and land parcels): Polsby-Popper, Schwartzberg, convex-hull ratio and Reock.",
     inputs=[_vecin("x", "x coordinates of the vertices in order (not repeating the first)", 3, 200), _vecin("y", "y coordinates of the vertices", 3, 200)],
     outputs=[_o("area", "number", "area (square coordinate units)"), _o("perimeter", "number", "perimeter (coordinate units)"),
              _o("polsby_popper", "number", "4 pi A / P^2"), _o("schwartzberg", "number", "P / (2 sqrt(pi A))"),
              _o("convex_hull_ratio", "number", "area / convex hull area"), _o("reock", "number", "area / area of the minimum enclosing circle")],
     primary="polsby_popper", conditions=["a simple polygon with distinct vertices"],
     method="shoelace area, perimeter, monotone-chain convex hull, Welzl minimum enclosing circle",
     latex=r"PP = \frac{4\pi A}{P^2},\quad S = \frac{P}{2\sqrt{\pi A}},\quad C_H = \frac{A}{A_{hull}},\quad R = \frac{A}{\pi r_{mec}^2}",
     run=compactness, reference=_ref_compactness, rand=lambda rng: dict(zip(("x", "y"), _rand_star(rng))),
     wrong=lambda x, y: (lambda r: dict(r, polsby_popper=math.pi * r["area"] / r["perimeter"] ** 2, reock=r["reock"] * 2))(compactness(x, y)),
     phrases=("Polygon compactness", "Compactness indices", "Shape compactness"),
     examples=[_ex({"x": [0, 4, 4, 0], "y": [0, 0, 3, 3]}), _ex({"x": [0, 10, 10, 5, 0], "y": [0, 0, 10, 2, 10]})],
     extra_q=[{"q": "Polygon compactness: x = [0, 4, 0, 4], y = [0, 0, 3, 3]", "outcome": "INVALID_INPUT"}],
     level="intermediate", weak="shapely computes the area, perimeter, convex hull and minimum bounding circle; the indices are those quantities combined by their definitions")


# ================================================================================================ SPATIAL STATISTICS
_SPS = "spatial statistics"
_PTS = "point pattern analysis"
_GST = "geostatistics"


def _weights(W, n, transform="none", binary=False, symmetric=False, islands_ok=False):
    _need(len(W) == n and all(len(r) == n for r in W), f"the weights matrix must be {n} x {n} (one row and column per observation)")
    for i in range(n):
        _need(W[i][i] == 0, "the weights matrix must have a zero diagonal")
        for j in range(n):
            _fin("each weight", W[i][j])
            _need(W[i][j] >= 0, "weights must be zero or positive")
            if binary:
                _need(W[i][j] in (0, 1), "the weights must be binary (0 or 1)")
            if symmetric:
                _need(W[i][j] == W[j][i], "the weights matrix must be symmetric")
    rows = [math.fsum(r) for r in W]
    if not islands_ok:
        _need(all(s > 0 for s in rows), "every observation needs at least one neighbour (no islands)")
    if transform == "row":
        return [[w / rows[i] for w in W[i]] for i in range(n)]
    return [[float(w) for w in r] for r in W]


def _s012(W):
    n = len(W)
    s0 = math.fsum(math.fsum(r) for r in W)
    s1 = 0.5 * math.fsum((W[i][j] + W[j][i]) ** 2 for i in range(n) for j in range(n))
    s2 = math.fsum((math.fsum(W[i]) + math.fsum(W[j][i] for j in range(n))) ** 2 for i in range(n))
    return s0, s1, s2


def _pysal():
    """Import esda, libpysal and pointpats for a reference. numba (used by them) imports a module named `coverage` when
    it can; the factory's tools folder, first on sys.path while certifying, holds an unrelated coverage.py, so it is
    hidden while importing."""
    import os
    import sys
    mod = sys.modules.get("coverage")
    if mod is not None and not hasattr(mod, "types"):
        del sys.modules["coverage"]
    saved = list(sys.path)
    sys.path[:] = [p for p in sys.path if not os.path.exists(os.path.join(p or ".", "coverage.py"))]
    try:
        import esda
        import libpysal  # noqa: F401
        import pointpats  # noqa: F401
    finally:
        sys.path[:] = saved
    return esda


def _wpysal(W, transform):
    import numpy as np
    _pysal()
    from libpysal.weights import full2W
    w = full2W(np.array(W, dtype=float))
    w.transform = transform
    return w


def _y_ok(y, lo=4):
    _need(len(y) >= lo, f"give at least {lo} observations")
    for v in y:
        _fin("each value", v)
    m = math.fsum(y) / len(y)
    _need(math.fsum((v - m) ** 2 for v in y) > 0, "the values must not all be equal")


def _rand_W(rng, n, binary=False, symmetric=True):
    pts = [(rng.uniform(0, 10), rng.uniform(0, 10)) for _ in range(n)]
    k = rng.randint(2, min(5, n - 1))
    W = [[0.0] * n for _ in range(n)]
    for i in range(n):
        d = sorted(range(n), key=lambda j: math.hypot(pts[i][0] - pts[j][0], pts[i][1] - pts[j][1]))
        for j in d[1:k + 1]:
            w = 1.0 if binary else rng.choice([1.0, 0.5, 2.0])
            W[i][j] = w
            if symmetric:
                W[j][i] = w
    return W, pts


def _rand_y(rng, n, pts=None, positive=False):
    if pts is None:
        return [round(rng.uniform(1 if positive else -5, 20), 3) for _ in range(n)]
    return [round((0.6 * p[0] + rng.gauss(0, 2)) + (12 if positive else 0), 3) for p in pts]


# ---------------------------------------------------------------- Moran's I
def morans_i(y, W, transform):
    n = len(y)
    _y_ok(y)
    _need(transform in ("row", "none"), "the transform must be row or none")
    Wt = _weights(W, n, transform)
    m = math.fsum(y) / n
    z = [v - m for v in y]
    s0, s1, s2 = _s012(Wt)
    m2 = math.fsum(v * v for v in z)
    I = n / s0 * math.fsum(Wt[i][j] * z[i] * z[j] for i in range(n) for j in range(n)) / m2
    EI = -1.0 / (n - 1)
    VIn = (n * n * s1 - n * s2 + 3 * s0 * s0) / ((n * n - 1) * s0 * s0) - EI * EI
    k = (math.fsum(v ** 4 for v in z) / n) / (m2 / n) ** 2
    VIr = (n * ((n * n - 3 * n + 3) * s1 - n * s2 + 3 * s0 * s0) - k * (n * (n - 1) * s1 - 2 * n * s2 + 6 * s0 * s0)) / ((n - 1) * (n - 2) * (n - 3) * s0 * s0) - EI * EI
    zn, zr = (I - EI) / math.sqrt(VIn), (I - EI) / math.sqrt(VIr)
    return _out(I=I, EI=EI, VI_norm=VIn, z_norm=zn, p_norm=_p_two(zn), VI_rand=VIr, z_rand=zr, p_rand=_p_two(zr))


def _ref_morans_i(y, W, transform):
    esda = _pysal()
    import numpy as np
    m = esda.Moran(np.array(y, dtype=float), _wpysal(W, "r" if transform == "row" else "o"), transformation="r" if transform == "row" else "o",
                   permutations=0, two_tailed=True)
    return {"I": m.I, "EI": m.EI, "VI_norm": m.VI_norm, "z_norm": m.z_norm, "p_norm": m.p_norm, "VI_rand": m.VI_rand, "z_rand": m.z_rand, "p_rand": m.p_rand}


def _rand_moran(rng, binary=False):
    n = rng.randint(6, 25)
    W, pts = _rand_W(rng, n, binary=binary, symmetric=rng.random() < 0.7)
    return {"y": _rand_y(rng, n, pts if rng.random() < 0.6 else None), "W": W}


_WIN = _matin("W", "spatial weights matrix (n x n, zero diagonal, row i lists the weights of i's neighbours)")
_TRIN = _choice("transform", ["row", "none"], "row-standardize the weights (row) or use them as given (none)")
_add("geo_morans_i",
     name="Global Moran's I spatial autocorrelation with normal and randomization inference", topic=_SPS,
     purpose="Global Moran's I of a variable observed at n locations with a given spatial weights matrix, its expectation and variance under normality and under randomization, z-scores and two-sided p-values.",
     inputs=[_vecin("y", "values at the n locations", 4, 400), _WIN, _TRIN],
     outputs=[_o("I", "number", "Moran's I"), _o("EI", "number", "expected I = -1/(n-1)"), _o("VI_norm", "number", "variance under normality"),
              _o("z_norm", "number", "z-score under normality"), _o("p_norm", "number", "two-sided p-value under normality"),
              _o("VI_rand", "number", "variance under randomization"), _o("z_rand", "number", "z-score under randomization"),
              _o("p_rand", "number", "two-sided p-value under randomization")],
     primary="I", conditions=["n >= 4, values not all equal", "no observation without neighbours"],
     method="I = (n/S0) sum w_ij z_i z_j / sum z_i^2 with z = y - mean; Cliff-Ord moments with S0, S1, S2 and the sample kurtosis",
     latex=r"I = \frac{n}{S_0}\frac{\sum_i\sum_j w_{ij}z_iz_j}{\sum_i z_i^2},\quad E[I] = -\frac{1}{n-1},\quad V_N[I] = \frac{n^2S_1 - nS_2 + 3S_0^2}{(n^2-1)S_0^2} - E[I]^2",
     run=morans_i, reference=_ref_morans_i, rand=lambda rng: dict(_rand_moran(rng), transform=rng.choice(["row", "none"])),
     wrong=lambda y, W, transform: (lambda r: dict(r, I=r["I"] * (len(y) - 1) / len(y), z_norm=(r["I"] * (len(y) - 1) / len(y) - r["EI"]) / math.sqrt(r["VI_norm"])))(morans_i(y, W, transform)),
     phrases=("Moran's I", "Global Moran's I", "Morans I"),
     examples=[_ex({"y": [8, 6, 6, 3, 2, 1], "W": [[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]], "transform": "row"}),
               _ex({"y": [1, 2, 1, 2, 1, 2], "W": [[0, 1, 0, 0, 0, 1], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [1, 0, 0, 0, 1, 0]], "transform": "none"})],
     level="advanced")


def gearys_c(y, W, transform):
    n = len(y)
    _y_ok(y)
    _need(transform in ("row", "none"), "the transform must be row or none")
    Wt = _weights(W, n, transform)
    m = math.fsum(y) / n
    z = [v - m for v in y]
    s0, s1, s2 = _s012(Wt)
    m2 = math.fsum(v * v for v in z)
    C = (n - 1) * math.fsum(Wt[i][j] * (y[i] - y[j]) ** 2 for i in range(n) for j in range(n)) / (2 * s0 * m2)
    VCn = ((2 * s1 + s2) * (n - 1) - 4 * s0 * s0) / (2 * (n + 1) * s0 * s0)
    k = (math.fsum(v ** 4 for v in z) / n) / (m2 / n) ** 2
    VCr = ((n - 1) * s1 * (n * n - 3 * n + 3 - (n - 1) * k) - 0.25 * (n - 1) * s2 * (n * n + 3 * n - 6 - (n * n - n + 2) * k)
           + s0 * s0 * (n * n - 3 - (n - 1) ** 2 * k)) / (n * (n - 2) * (n - 3) * s0 * s0)
    return _out(C=C, VC_norm=VCn, z_norm=(C - 1) / math.sqrt(VCn), VC_rand=VCr, z_rand=(C - 1) / math.sqrt(VCr))


def _ref_gearys_c(y, W, transform):
    esda = _pysal()
    import numpy as np
    g = esda.Geary(np.array(y, dtype=float), _wpysal(W, "r" if transform == "row" else "o"), transformation="r" if transform == "row" else "o", permutations=0)
    return {"C": g.C, "VC_norm": g.VC_norm, "z_norm": g.z_norm, "VC_rand": g.VC_rand, "z_rand": g.z_rand}


_add("geo_gearys_c",
     name="Geary's C spatial autocorrelation with normal and randomization inference", topic=_SPS,
     purpose="Geary's contiguity ratio C of a variable with a given spatial weights matrix, its variance under normality and randomization, and the z-scores (C < 1 positive, C > 1 negative autocorrelation).",
     inputs=[_vecin("y", "values at the n locations", 4, 400), _WIN, _TRIN],
     outputs=[_o("C", "number", "Geary's C"), _o("VC_norm", "number", "variance under normality"), _o("z_norm", "number", "z-score under normality"),
              _o("VC_rand", "number", "variance under randomization"), _o("z_rand", "number", "z-score under randomization")],
     primary="C", conditions=["n >= 4, values not all equal", "no observation without neighbours"],
     method="C = (n-1) sum w_ij (y_i - y_j)^2 / (2 S0 sum z^2); Cliff-Ord moments (E[C] = 1)",
     latex=r"C = \frac{(n-1)\sum_i\sum_j w_{ij}(y_i - y_j)^2}{2S_0\sum_i (y_i - \bar y)^2},\quad V_N[C] = \frac{(2S_1 + S_2)(n-1) - 4S_0^2}{2(n+1)S_0^2}",
     run=gearys_c, reference=_ref_gearys_c, rand=lambda rng: dict(_rand_moran(rng), transform=rng.choice(["row", "none"])),
     wrong=lambda y, W, transform: (lambda r: dict(r, C=r["C"] * len(y) / (len(y) - 1), z_norm=(r["C"] * len(y) / (len(y) - 1) - 1) / math.sqrt(r["VC_norm"])))(gearys_c(y, W, transform)),
     phrases=("Geary's C", "Gearys C", "Geary contiguity ratio"),
     examples=[_ex({"y": [8, 6, 6, 3, 2, 1], "W": [[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]], "transform": "row"}),
               _ex({"y": [1, 2, 1, 2, 1, 2], "W": [[0, 1, 0, 0, 0, 1], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [1, 0, 0, 0, 1, 0]], "transform": "none"})],
     level="advanced")


def general_g(y, W):
    n = len(y)
    _y_ok(y)
    _need(all(v > 0 for v in y), "the values must be positive")
    Wt = _weights(W, n, binary=True)
    s0, s1, s2 = _s012(Wt)
    num = math.fsum(Wt[i][j] * y[i] * y[j] for i in range(n) for j in range(n))
    m1, m2, m3, m4 = (math.fsum(v ** p for v in y) for p in (1, 2, 3, 4))
    G = num / (m1 * m1 - m2)
    EG = s0 / (n * (n - 1))
    b0 = (n * n - 3 * n + 3) * s1 - n * s2 + 3 * s0 * s0
    b1 = -((n * n - n) * s1 - 2 * n * s2 + 6 * s0 * s0)
    b2 = -(2 * n * s1 - (n + 3) * s2 + 6 * s0 * s0)
    b3 = 4 * (n - 1) * s1 - 2 * (n + 1) * s2 + 8 * s0 * s0
    b4 = s1 - s2 + s0 * s0
    EG2 = (b0 * m2 * m2 + b1 * m4 + b2 * m1 * m1 * m2 + b3 * m1 * m3 + b4 * m1 ** 4) / ((m1 * m1 - m2) ** 2 * n * (n - 1) * (n - 2) * (n - 3))
    VG = EG2 - EG * EG
    _need(s0 < n * (n - 1) and VG > 1e-12 * EG * EG, "with every pair of locations neighbours G is constant (variance zero, z undefined)")
    return _out(G=G, EG=EG, VG=VG, z_norm=(G - EG) / math.sqrt(VG))


def _ref_general_g(y, W):
    esda = _pysal()
    import numpy as np
    g = esda.G(np.array(y, dtype=float), _wpysal(W, "b"), permutations=0)
    return {"G": g.G, "EG": g.EG, "VG": g.VG, "z_norm": g.z_norm}


_add("geo_getis_ord_general_g",
     name="Getis-Ord general G (global high/low clustering)", topic=_SPS,
     purpose="The Getis-Ord general G statistic of a positive variable with binary spatial weights, its expectation and variance under randomization and the z-score (G above its expectation: high values cluster).",
     inputs=[_vecin("y", "positive values at the n locations", 4, 400), _matin("W", "binary spatial weights matrix (0/1, zero diagonal)")],
     outputs=[_o("G", "number", "general G"), _o("EG", "number", "expected G = S0/(n(n-1))"), _o("VG", "number", "variance of G under randomization"),
              _o("z_norm", "number", "z-score")],
     primary="G", conditions=["positive values, n >= 4", "binary weights, no observation without neighbours"],
     method="G = sum_{i != j} w_ij y_i y_j / sum_{i != j} y_i y_j; Getis-Ord (1992) moments",
     latex=r"G = \frac{\sum_i\sum_{j\neq i} w_{ij}y_iy_j}{\sum_i\sum_{j\neq i}y_iy_j},\quad E[G] = \frac{S_0}{n(n-1)}",
     run=general_g, reference=_ref_general_g,
     rand=lambda rng: (lambda W, pts: {"y": _rand_y(rng, len(W), pts, positive=True), "W": W})(*_rand_W(rng, rng.randint(6, 25), binary=True)),
     wrong=lambda y, W: (lambda r: dict(r, G=r["G"] * (1 - 1 / len(y)), z_norm=(r["G"] * (1 - 1 / len(y)) - r["EG"]) / math.sqrt(r["VG"])))(general_g(y, W)),
     phrases=("Getis-Ord general G", "General G statistic", "Getis-Ord G"),
     examples=[_ex({"y": [10, 12, 11, 2, 3, 2], "W": [[0, 1, 1, 0, 0, 0], [1, 0, 1, 0, 0, 0], [1, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 1], [0, 0, 0, 1, 0, 1], [0, 0, 0, 1, 1, 0]]}),
               _ex({"y": [4, 8, 2, 6, 5, 7, 3], "W": [[0, 1, 0, 0, 0, 0, 1], [1, 0, 1, 0, 0, 0, 0], [0, 1, 0, 1, 0, 0, 0], [0, 0, 1, 0, 1, 0, 0], [0, 0, 0, 1, 0, 1, 0], [0, 0, 0, 0, 1, 0, 1], [1, 0, 0, 0, 0, 1, 0]]})],
     level="advanced")


def local_g(y, W, statistic):
    n = len(y)
    _y_ok(y)
    _need(all(v > 0 for v in y), "the values must be positive")
    _need(statistic in ("gi_star", "gi"), "the statistic must be gi_star (location included) or gi")
    Wt = _weights(W, n, binary=True)
    star = statistic == "gi_star"
    if star:
        Wt = [[1.0 if i == j else Wt[i][j] for j in range(n)] for i in range(n)]
    sy, sy2 = math.fsum(y), math.fsum(v * v for v in y)
    N = n if star else n - 1
    Gs, Zs = [], []
    for i in range(n):
        tot = sy if star else sy - y[i]
        g = math.fsum(Wt[i][j] * y[j] for j in range(n)) / tot
        mean = tot / N
        var = (sy2 if star else sy2 - y[i] * y[i]) / N - mean * mean
        card = math.fsum(Wt[i])
        eg = card / N
        vg = card * (N - card) / (N - 1) / N ** 2 * var / mean ** 2
        _need(vg > 0, "a location whose neighbourhood covers every observation has no variance (z undefined)")
        Gs.append(g)
        Zs.append((g - eg) / math.sqrt(vg))
    return {"Gs": _r(Gs), "Zs": _r(Zs)}


def _ref_local_g(y, W, statistic):
    esda = _pysal()
    import numpy as np
    g = esda.G_Local(np.array(y, dtype=float), _wpysal(W, "b"), transform="B", permutations=0, star=statistic == "gi_star")
    return {"Gs": [float(v) for v in g.Gs], "Zs": [float(v) for v in g.Zs]}


_add("geo_local_getis_ord_gi",
     name="Local Getis-Ord Gi and Gi* hot-spot statistics", topic=_SPS,
     purpose="The local Getis-Ord statistic of every location (Gi* includes the location itself, Gi excludes it) with binary weights, and its z-score for hot-spot (positive) and cold-spot (negative) mapping.",
     inputs=[_vecin("y", "positive values at the n locations", 4, 400), _matin("W", "binary spatial weights matrix (0/1, zero diagonal)"),
             _choice("statistic", ["gi_star", "gi"], "Gi* (the location counts as its own neighbour) or Gi")],
     outputs=[_o("Gs", "number", "local G of each location"), _o("Zs", "number", "z-score of each location")],
     primary="Gs", conditions=["positive values, n >= 4", "binary weights, no observation without neighbours"],
     method="G_i = sum_j w_ij y_j / sum_j y_j (j != i for Gi); E = W_i/N, V = W_i (N - W_i)/(N-1)/N^2 * s^2/ybar^2 with the values other than i for Gi",
     latex=r"G_i^* = \frac{\sum_j w_{ij}y_j}{\sum_j y_j},\quad Z_i = \frac{G_i - W_i/N}{\sqrt{\frac{W_i(N - W_i)}{(N-1)N^2}\frac{s^2}{\bar y^2}}}",
     run=local_g, reference=_ref_local_g,
     rand=lambda rng: (lambda W, pts: {"y": _rand_y(rng, len(W), pts, positive=True), "W": W, "statistic": rng.choice(["gi_star", "gi"])})(*_rand_W(rng, rng.randint(6, 25), binary=True)),
     wrong=lambda y, W, statistic: local_g(y, W, "gi" if statistic == "gi_star" else "gi_star"),
     phrases=("Local Getis-Ord Gi", "Getis-Ord Gi star", "Hot spot analysis Gi"),
     examples=[_ex({"y": [10, 12, 11, 2, 3, 2], "W": [[0, 1, 1, 0, 0, 0], [1, 0, 1, 0, 0, 0], [1, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 1], [0, 0, 0, 1, 0, 1], [0, 0, 0, 1, 1, 0]], "statistic": "gi_star"}),
               _ex({"y": [10, 12, 11, 2, 3, 2], "W": [[0, 1, 1, 0, 0, 0], [1, 0, 1, 0, 0, 0], [1, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 1], [0, 0, 0, 1, 0, 1], [0, 0, 0, 1, 1, 0]], "statistic": "gi"})],
     level="expert")


def local_morans_i(y, W):
    n = len(y)
    _y_ok(y)
    Wt = _weights(W, n, "row")
    m = math.fsum(y) / n
    z = [v - m for v in y]
    den = math.fsum(v * v for v in z)
    lag = [math.fsum(Wt[i][j] * z[j] for j in range(n)) for i in range(n)]
    Is = [(n - 1) * z[i] * lag[i] / den for i in range(n)]
    q = [1.0 if z[i] > 0 and lag[i] > 0 else 2.0 if z[i] < 0 and lag[i] > 0 else 3.0 if z[i] < 0 and lag[i] < 0 else 4.0 if z[i] > 0 and lag[i] < 0 else 0.0 for i in range(n)]
    return {"Is": _r(Is), "quadrant": q}


def _ref_local_morans_i(y, W):
    esda = _pysal()
    import numpy as np
    m = esda.Moran_Local(np.array(y, dtype=float), _wpysal(W, "r"), transformation="r", permutations=0)
    return {"Is": [float(v) for v in m.Is], "quadrant": [float(v) for v in m.q]}


def _rand_lisa(rng):
    for _ in range(50):
        x = _rand_moran(rng)
        n = len(x["y"])
        m = sum(x["y"]) / n
        if all(abs(v - m) > 1e-6 for v in x["y"]):
            o = local_morans_i(**x)
            if all(abs(v) > 1e-9 for v in o["Is"]):
                return x
    return x


_add("geo_local_morans_i",
     name="Local Moran's I (LISA) and Moran scatterplot quadrants", topic=_SPS,
     purpose="The local Moran statistic of every location with row-standardized weights, and its Moran scatterplot quadrant: 1 high-high, 2 low-high, 3 low-low, 4 high-low.",
     inputs=[_vecin("y", "values at the n locations", 4, 400), _matin("W", "spatial weights matrix (row-standardized internally)")],
     outputs=[_o("Is", "number", "local Moran's I of each location"), _o("quadrant", "number", "quadrant of each location (1 HH, 2 LH, 3 LL, 4 HL)")],
     primary="Is", conditions=["n >= 4, values not all equal", "no observation without neighbours"],
     method="I_i = (n - 1) z_i sum_j w_ij z_j / sum_k z_k^2 with z = y - mean and row-standardized w (as esda)",
     latex=r"I_i = (n-1)\frac{z_i\sum_j w_{ij}z_j}{\sum_k z_k^2}",
     run=local_morans_i, reference=_ref_local_morans_i, rand=_rand_lisa,
     wrong=lambda y, W: (lambda r: dict(r, Is=[v * len(y) / (len(y) - 1) for v in r["Is"]]))(local_morans_i(y, W)),
     phrases=("Local Moran's I", "LISA", "Local indicators of spatial association"),
     examples=[_ex({"y": [8, 6, 6, 3, 2, 1], "W": [[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]]}),
               _ex({"y": [10, 12, 11, 2, 3, 1], "W": [[0, 1, 1, 0, 0, 0], [1, 0, 1, 0, 0, 0], [1, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 1], [0, 0, 0, 1, 0, 1], [0, 0, 0, 1, 1, 0]]})],
     level="expert")


def bivariate_morans_i(x, y, W):
    n = len(y)
    _need(len(x) == n, "give one x per y")
    _y_ok(y)
    _y_ok(x)
    Wt = _weights(W, n, "row")
    mx, my = math.fsum(x) / n, math.fsum(y) / n
    sx = math.sqrt(math.fsum((v - mx) ** 2 for v in x) / (n - 1))
    sy = math.sqrt(math.fsum((v - my) ** 2 for v in y) / (n - 1))
    zx = [(v - mx) / sx for v in x]
    zy = [(v - my) / sy for v in y]
    I = math.fsum(zx[i] * Wt[i][j] * zy[j] for i in range(n) for j in range(n)) / (n - 1)
    return _out(I_xy=I)


def _ref_bivariate_morans_i(x, y, W):
    esda = _pysal()
    import numpy as np
    m = esda.Moran_BV(np.array(x, dtype=float), np.array(y, dtype=float), _wpysal(W, "r"), transformation="r", permutations=0)
    return {"I_xy": m.I}


_add("geo_bivariate_morans_i",
     name="Bivariate Moran's I (spatial cross-correlation)", topic=_SPS,
     purpose="The bivariate Moran's I between a variable x and the spatial lag of a second variable y with row-standardized weights (how x at a location relates to y around it).",
     inputs=[_vecin("x", "first variable at the n locations", 4, 400), _vecin("y", "second variable at the n locations", 4, 400),
             _matin("W", "spatial weights matrix (row-standardized internally)")],
     outputs=[_o("I_xy", "number", "bivariate Moran's I")],
     primary="I_xy", conditions=["n >= 4, neither variable constant", "no observation without neighbours"],
     method="I_xy = zx' W zy / (n - 1) with both variables standardized (sample standard deviation) and row-standardized W",
     latex=r"I_{xy} = \frac{\sum_i z_{x,i}\sum_j w_{ij}z_{y,j}}{n-1}",
     run=bivariate_morans_i, reference=_ref_bivariate_morans_i,
     rand=lambda rng: (lambda W, pts: {"x": _rand_y(rng, len(W), pts), "y": _rand_y(rng, len(W), pts), "W": W})(*_rand_W(rng, rng.randint(6, 25))),
     wrong=lambda x, y, W: {"I_xy": bivariate_morans_i(x, y, W)["I_xy"] * (len(y) - 1) / len(y) ** 0.5},
     phrases=("Bivariate Moran's I", "Bivariate Moran"),
     examples=[_ex({"x": [8, 6, 6, 3, 2, 1], "y": [7, 7, 5, 4, 1, 2], "W": [[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]]}),
               _ex({"x": [1, 2, 3, 4, 5, 6], "y": [6, 5, 4, 3, 2, 1], "W": [[0, 1, 0, 0, 0, 1], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [1, 0, 0, 0, 1, 0]]})],
     level="expert")


def join_counts(y, W):
    n = len(y)
    _need(n >= 3, "give at least 3 observations")
    _need(all(v in (0, 1) for v in y), "the variable must be binary (1 black, 0 white)")
    Wt = _weights(W, n, binary=True, symmetric=True, islands_ok=True)
    bb = bw = ww = 0
    for i in range(n):
        for j in range(i + 1, n):
            if Wt[i][j]:
                if y[i] == 1 and y[j] == 1:
                    bb += 1
                elif y[i] == 0 and y[j] == 0:
                    ww += 1
                else:
                    bw += 1
    return {"BB": float(bb), "BW": float(bw), "WW": float(ww), "J": float(bb + bw + ww)}


def _ref_join_counts(y, W):
    esda = _pysal()
    import numpy as np
    w = _wpysal(W, "b")
    try:
        j = esda.Join_Counts(np.array(y, dtype=float), w, permutations=0)
        return {"BB": float(j.bb), "BW": float(j.bw), "WW": float(j.ww), "J": float(j.J)}
    except ValueError:              # esda's chi-square table fails when a join type is absent: count with the sparse W
        M = w.sparse.toarray()
        v = np.array(y, dtype=float)
        bb = v @ M @ v / 2
        ww = (1 - v) @ M @ (1 - v) / 2
        J = M.sum() / 2
        return {"BB": float(bb), "BW": float(J - bb - ww), "WW": float(ww), "J": float(J)}


_add("geo_join_counts",
     name="Join count statistics of a binary map (BB, BW, WW)", topic=_SPS,
     purpose="The numbers of black-black, black-white and white-white joins between neighbouring areas of a binary (presence/absence) map, and the total number of joins.",
     inputs=[_vecin("y", "binary variable at the n areas (1 black, 0 white)", 3, 400), _matin("W", "binary symmetric contiguity matrix (0/1, zero diagonal)")],
     outputs=[_o("BB", "number", "black-black joins"), _o("BW", "number", "black-white joins"), _o("WW", "number", "white-white joins"), _o("J", "number", "total joins")],
     primary="BB", conditions=["binary symmetric weights"],
     method="count each neighbouring pair once by the colours of its two areas",
     latex=r"BB = \tfrac12\sum_{i,j} w_{ij}y_iy_j,\ BW = \tfrac12\sum_{i,j} w_{ij}(y_i - y_j)^2,\ WW = \tfrac12\sum_{i,j}w_{ij}(1-y_i)(1-y_j)",
     run=join_counts, reference=_ref_join_counts,
     rand=lambda rng: (lambda W: {"y": [float(rng.random() < 0.45) for _ in W], "W": W})(_rand_W(rng, rng.randint(5, 25), binary=True)[0]),
     wrong=lambda y, W: (lambda r: dict(r, BB=2 * r["BB"], J=r["J"] + r["BB"]))(join_counts(y, W)),
     phrases=("Join count statistics", "Join counts"),
     examples=[_ex({"y": [1, 1, 0, 0, 1, 0], "W": [[0, 1, 1, 0, 0, 0], [1, 0, 1, 1, 0, 0], [1, 1, 0, 0, 1, 0], [0, 1, 0, 0, 1, 1], [0, 0, 1, 1, 0, 1], [0, 0, 0, 1, 1, 0]]}),
               _ex({"y": [1, 0, 1, 0], "W": [[0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0]]})],
     level="beginner", weak="esda's Join_Counts counts the same joins (a counting definition)")


def spatial_lag(y, W, transform):
    n = len(y)
    _need(n >= 2, "give at least 2 observations")
    for v in y:
        _fin("each value", v)
    _need(transform in ("row", "none"), "the transform must be row or none")
    Wt = _weights(W, n, transform)
    return {"lag": _r([math.fsum(Wt[i][j] * y[j] for j in range(n)) for i in range(n)])}


def _ref_spatial_lag(y, W, transform):
    import numpy as np
    _pysal()
    from libpysal.weights import lag_spatial
    return {"lag": [float(v) for v in lag_spatial(_wpysal(W, "r" if transform == "row" else "o"), np.array(y, dtype=float))]}


_add("geo_spatial_lag",
     name="Spatial lag of a variable", topic=_SPS,
     purpose="The spatial lag W y of a variable: for row-standardized weights, the average of each location's neighbours; otherwise the weighted sum.",
     inputs=[_vecin("y", "values at the n locations", 2, 400), _WIN, _TRIN],
     outputs=[_o("lag", "number", "spatial lag at each location")],
     primary="lag", conditions=["no observation without neighbours"],
     method="lag_i = sum_j w_ij y_j (weights row-standardized first if asked)",
     latex=r"[Wy]_i = \sum_j w_{ij}y_j",
     run=spatial_lag, reference=_ref_spatial_lag, rand=lambda rng: dict(_rand_moran(rng), transform=rng.choice(["row", "none"])),
     wrong=lambda y, W, transform: {"lag": _r([math.fsum(W[j][i] * y[j] for j in range(len(y))) / (math.fsum(W[i]) if transform == "row" else 1) + 0.5 for i in range(len(y))])},
     phrases=("Spatial lag", "Spatially lagged variable"),
     examples=[_ex({"y": [8, 6, 6, 3], "W": [[0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0]], "transform": "row"}),
               _ex({"y": [1, 2, 3], "W": [[0, 1, 1], [1, 0, 0], [1, 0, 0]], "transform": "none"})],
     level="beginner", weak="libpysal's lag_spatial computes the same weighted sum")


def knn_weights(x, y, k):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(isinstance(k, int) and 1 <= k < n, "k must be a whole number from 1 to n - 1")
    _need(len(set(zip(x, y))) == n, "the points must be distinct")
    W = [[0.0] * n for _ in range(n)]
    for i in range(n):
        d = sorted((math.hypot(x[i] - x[j], y[i] - y[j]), j) for j in range(n) if j != i)
        if k < n - 1:
            _need(d[k - 1][0] < d[k][0], "the k-th nearest neighbour is tied: the weights are not unique")
        for _, j in d[:k]:
            W[i][j] = 1.0
    return {"W": W, "s0": float(n * k), "symmetric": all(W[i][j] == W[j][i] for i in range(n) for j in range(n))}


def _ref_knn_weights(x, y, k):
    import numpy as np
    _pysal()
    from libpysal.weights import KNN
    w = KNN.from_array(np.c_[x, y], k=k)
    M = w.full()[0]
    return {"W": [[float(v) for v in r] for r in M], "s0": float(w.s0), "symmetric": bool((M == M.T).all())}


def _rand_pts(rng, lo=5, hi=30, scale=None):
    n = rng.randint(lo, hi)
    s = scale or _lu(rng, 1, 1e4)
    return [round(rng.uniform(0, s), 3) for _ in range(n)], [round(rng.uniform(0, s), 3) for _ in range(n)]


_add("geo_knn_weights",
     name="k-nearest-neighbour spatial weights", topic=_SPS,
     purpose="The binary k-nearest-neighbour weights matrix of a set of points (row i marks the k nearest points to point i), its total weight and whether it is symmetric.",
     inputs=[_vecin("x", "x coordinates", 2, 200), _vecin("y", "y coordinates", 2, 200), _intin("k", "number of neighbours", 1, 199)],
     outputs=[_o("W", "number", "weights matrix (n x n, 0/1)"), _o("s0", "number", "sum of all weights (n k)"), _o("symmetric", "boolean", "whether W is symmetric")],
     primary="s0", conditions=["distinct points", "1 <= k < n, no tie at the k-th neighbour"],
     method="sort each point's distances to the others and mark the k smallest",
     latex=r"w_{ij} = 1 \iff j \in N_k(i)",
     run=knn_weights, reference=_ref_knn_weights, rand=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1], "k": rng.randint(1, min(6, len(xy[0]) - 1))})(_rand_pts(rng)),
     wrong=lambda x, y, k: knn_weights(x, y, k + 1) if k + 1 < len(x) else {"W": [], "s0": 0.0, "symmetric": False},
     phrases=("k nearest neighbour weights", "KNN weights", "K-nearest neighbor spatial weights"),
     examples=[_ex({"x": [0, 1, 3.5, 6, 10], "y": [0, 0, 0, 0, 0.5], "k": 2}), _ex({"x": [0, 2, 0, 2, 5], "y": [0, 0, 2.2, 2.5, 1], "k": 1})],
     level="intermediate")


def distance_band_weights(x, y, threshold):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(n >= 2, "give at least 2 points")
    _pos("the threshold", threshold)
    W = [[1.0 if i != j and math.hypot(x[i] - x[j], y[i] - y[j]) <= threshold else 0.0 for j in range(n)] for i in range(n)]
    card = [math.fsum(r) for r in W]
    return {"W": W, "neighbours": card, "s0": math.fsum(card), "islands": float(sum(1 for c in card if c == 0))}


def _ref_distance_band(x, y, threshold):
    import numpy as np
    import warnings
    _pysal()
    from libpysal.weights import DistanceBand
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        w = DistanceBand(np.c_[x, y], threshold=threshold, binary=True, silence_warnings=True)
    M = w.full()[0]
    return {"W": [[float(v) for v in r] for r in M], "neighbours": [float(v) for v in M.sum(axis=1)], "s0": float(M.sum()), "islands": float(len(w.islands))}


def _rand_dband(rng):
    for _ in range(50):
        xs, ys = _rand_pts(rng, scale=100.0)
        t = round(rng.uniform(5, 40), 3)
        d = [math.hypot(xs[i] - xs[j], ys[i] - ys[j]) for i in range(len(xs)) for j in range(i)]
        if min(abs(v - t) for v in d) > 1e-6:
            return {"x": xs, "y": ys, "threshold": t}
    return {"x": xs, "y": ys, "threshold": t}


_add("geo_distance_band_weights",
     name="Distance-band spatial weights", topic=_SPS,
     purpose="The binary distance-band weights matrix of a set of points (neighbours within a threshold distance), the number of neighbours of each point, the total weight and the number of islands.",
     inputs=[_vecin("x", "x coordinates", 2, 200), _vecin("y", "y coordinates", 2, 200), _numin("threshold", "distance threshold (coordinate units)")],
     outputs=[_o("W", "number", "weights matrix (n x n, 0/1)"), _o("neighbours", "number", "number of neighbours of each point"),
              _o("s0", "number", "sum of all weights"), _o("islands", "number", "number of points without neighbours")],
     primary="s0", conditions=["threshold > 0"],
     method="w_ij = 1 when 0 < d_ij <= threshold",
     latex=r"w_{ij} = \begin{cases}1 & d_{ij} \le \delta,\ i \ne j\\ 0 & \text{otherwise}\end{cases}",
     run=distance_band_weights, reference=_ref_distance_band, rand=_rand_dband,
     wrong=lambda x, y, threshold: distance_band_weights(x, y, threshold / 2),
     phrases=("Distance band weights", "Distance-band spatial weights", "Fixed distance weights"),
     examples=[_ex({"x": [0, 1, 3, 6, 10], "y": [0, 0, 0, 0, 0], "threshold": 2.5}), _ex({"x": [0, 2, 0, 2, 5], "y": [0, 0, 2, 2, 1], "threshold": 2.0001})],
     level="intermediate")


# ---------------------------------------------------------------- point patterns
def nearest_neighbour_index(x, y, area):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(n >= 3, "give at least 3 points")
    _pos("the study area", area)
    _need(len(set(zip(x, y))) == n, "the points must be distinct")
    d = [min(math.hypot(x[i] - x[j], y[i] - y[j]) for j in range(n) if j != i) for i in range(n)]
    dobs = math.fsum(d) / n
    dexp = 0.5 / math.sqrt(n / area)
    se = 0.26136 / math.sqrt(n * n / area)
    z = (dobs - dexp) / se
    return _out(d_obs=dobs, d_exp=dexp, R=dobs / dexp, z=z, p_value=_p_two(z))


def _ref_nni(x, y, area):
    import numpy as np
    from scipy.spatial import cKDTree
    from scipy.stats import norm
    P = np.c_[x, y]
    d, _ = cKDTree(P).query(P, k=2)
    n = len(x)
    dobs = float(d[:, 1].mean())
    dexp = 0.5 / math.sqrt(n / area)
    z = (dobs - dexp) / (0.26136 / math.sqrt(n * n / area))
    return {"d_obs": dobs, "d_exp": dexp, "R": dobs / dexp, "z": z, "p_value": float(2 * norm.sf(abs(z)))}


_add("geo_nearest_neighbour_index",
     name="Nearest-neighbour index (Clark-Evans ratio) of a point pattern", topic=_PTS,
     purpose="The mean observed nearest-neighbour distance of a point pattern, the mean expected under complete spatial randomness in a study area, their ratio R (R < 1 clustered, R > 1 dispersed), the z-score and two-sided p-value.",
     inputs=[_vecin("x", "x coordinates of the points", 3, 2000), _vecin("y", "y coordinates of the points", 3, 2000), _numin("area", "area of the study region (square coordinate units)")],
     outputs=[_o("d_obs", "number", "mean observed nearest-neighbour distance"), _o("d_exp", "number", "expected mean distance 0.5/sqrt(n/A)"),
              _o("R", "number", "nearest-neighbour index d_obs/d_exp"), _o("z", "number", "z-score"), _o("p_value", "number", "two-sided p-value")],
     primary="R", conditions=["distinct points", "no edge correction"],
     method="mean of each point's distance to its nearest neighbour; Clark-Evans expectation 0.5/sqrt(density) and standard error 0.26136/sqrt(n^2/A)",
     latex=r"R = \frac{\bar d_{obs}}{0.5/\sqrt{n/A}},\quad z = \frac{\bar d_{obs} - \bar d_{exp}}{0.26136/\sqrt{n^2/A}}",
     run=nearest_neighbour_index, reference=_ref_nni,
     rand=lambda rng: (lambda s: (lambda xy: {"x": xy[0], "y": xy[1], "area": round(s * s * rng.uniform(1.0, 1.3), 3)})(_rand_pts(rng, 5, 60, s)))(_lu(rng, 10, 1e4)),
     wrong=lambda x, y, area: (lambda r: dict(r, d_exp=r["d_exp"] * 2, R=r["R"] / 2))(nearest_neighbour_index(x, y, area)),
     phrases=("Nearest neighbour index", "Clark-Evans ratio", "Average nearest neighbour"),
     examples=[_ex({"x": [1, 2, 8, 9, 5], "y": [1, 2, 8, 9, 5], "area": 100}), _ex({"x": [0, 10, 0, 10, 5], "y": [0, 0, 10, 10, 5], "area": 100})],
     level="intermediate", weak="scipy's k-d tree gives the nearest-neighbour distances; the index is then the Clark-Evans definition")


def _chi2_sf(x, df):
    import mpmath
    return float(mpmath.gammainc(df / 2.0, x / 2.0, mpmath.inf, regularized=True))


def quadrat_test(x, y, nx, ny):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(n >= 2, "give at least 2 points")
    _need(isinstance(nx, int) and isinstance(ny, int) and nx >= 1 and ny >= 1 and nx * ny >= 2, "the grid needs at least 2 cells")
    x0, x1, y0, y1 = min(x), max(x), min(y), max(y)
    _need(x1 > x0 and y1 > y0, "the points must span a rectangle")
    w, h = (x1 - x0) / nx, (y1 - y0) / ny
    counts = [0] * (nx * ny)
    for a, b in zip(x, y):
        ix, iy = int((a - x0) // w), int((b - y0) // h)
        ix, iy = min(ix, nx - 1), min(iy, ny - 1)
        counts[iy * nx + ix] += 1
    k = nx * ny
    e = n / k
    chi2 = math.fsum((c - e) ** 2 for c in counts) / e
    return {"chi2": _r(chi2), "df": float(k - 1), "p_value": _r(_chi2_sf(chi2, k - 1)), "vmr": _r(chi2 / (k - 1)), "counts": [float(c) for c in counts]}


def _ref_quadrat(x, y, nx, ny):
    import numpy as np
    _pysal()
    from pointpats import PointPattern, QStatistic
    import warnings
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        q = QStatistic(PointPattern(np.c_[x, y]), shape="rectangle", nx=nx, ny=ny)
    counts = q.mr.point_location_sta()
    return {"chi2": float(q.chi2), "df": float(q.df), "p_value": float(q.chi2_pvalue), "vmr": float(q.chi2) / q.df,
            "counts": [float(counts[i]) for i in range(nx * ny)]}


def _rand_quadrat(rng):
    nx, ny = rng.randint(1, 5), rng.randint(1, 5)
    if nx * ny < 2:
        nx = 2
    xs, ys = _rand_pts(rng, 10, 150, 100.0)
    return {"x": xs, "y": ys, "nx": nx, "ny": ny}


_add("geo_quadrat_test",
     name="Quadrat analysis: chi-square test of complete spatial randomness", topic=_PTS,
     purpose="Counts of points in an nx x ny grid of quadrats over the bounding rectangle of a point pattern, the chi-square statistic against complete spatial randomness, its p-value and the variance-to-mean ratio.",
     inputs=[_vecin("x", "x coordinates of the points", 2, 5000), _vecin("y", "y coordinates of the points", 2, 5000),
             _intin("nx", "number of quadrat columns", 1, 50), _intin("ny", "number of quadrat rows", 1, 50)],
     outputs=[_o("chi2", "number", "chi-square statistic"), _o("df", "number", "degrees of freedom (cells - 1)"), _o("p_value", "number", "upper-tail p-value"),
              _o("vmr", "number", "variance-to-mean ratio of the counts (chi2/df)"), _o("counts", "number", "count of each quadrat, row by row from the south-west")],
     primary="chi2", conditions=["the window is the bounding rectangle of the points", "cells include their west and south edges; the last column and row include the east and north edges"],
     method="equal-size cells over the bounding box; chi2 = sum (O - E)^2 / E with E = n/cells; p from the chi-square upper tail (regularized incomplete gamma)",
     latex=r"\chi^2 = \sum_{c}\frac{(O_c - E)^2}{E},\quad E = \frac{n}{n_xn_y},\quad VMR = \frac{\chi^2}{n_xn_y - 1}",
     run=quadrat_test, reference=_ref_quadrat, rand=_rand_quadrat,
     wrong=lambda x, y, nx, ny: (lambda r: dict(r, chi2=r["chi2"] * len(x) / (nx * ny), p_value=_chi2_sf(r["chi2"] * len(x) / (nx * ny), nx * ny - 1)))(quadrat_test(x, y, nx, ny)),
     phrases=("Quadrat test", "Quadrat analysis", "Quadrat count test"),
     examples=[_ex({"x": [1, 2, 3, 8, 9, 2, 1, 7, 8, 9], "y": [1, 1, 2, 8, 9, 8, 9, 1, 2, 1], "nx": 2, "ny": 2}),
               _ex({"x": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], "y": [0, 5, 1, 6, 2, 7, 3, 8, 4, 9], "nx": 3, "ny": 1})],
     level="advanced")


def ripley_k(x, y, d):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(n >= 3, "give at least 3 points")
    _need(len(d) >= 1, "give at least one distance")
    for v in d:
        _pos("each distance", v)
    A = (max(x) - min(x)) * (max(y) - min(y))
    _need(A > 0, "the points must span a rectangle")
    pd = [math.hypot(x[i] - x[j], y[i] - y[j]) for i in range(n) for j in range(i + 1, n)]
    K = [A * 2 * sum(1 for p in pd if p < t) / (n * n) for t in d]
    return {"K": _r(K), "L": _r([math.sqrt(k / math.pi) for k in K]), "area": _r(A)}


def _ref_ripley_k(x, y, d):
    import numpy as np
    _pysal()
    from pointpats import k
    sup, K = k(np.c_[x, y], support=np.array(d, dtype=float))
    A = (max(x) - min(x)) * (max(y) - min(y))
    return {"K": [float(v) for v in K], "L": [math.sqrt(float(v) / math.pi) for v in K], "area": A}


def _rand_ripley(rng):
    for _ in range(50):
        xs, ys = _rand_pts(rng, 5, 60, 100.0)
        d = sorted(round(rng.uniform(1, 50), 3) for _ in range(rng.randint(1, 8)))
        pd = [math.hypot(xs[i] - xs[j], ys[i] - ys[j]) for i in range(len(xs)) for j in range(i)]
        if min(abs(p - t) for p in pd for t in d) > 1e-6:
            return {"x": xs, "y": ys, "d": d}
    return {"x": xs, "y": ys, "d": d}


_add("geo_ripley_k",
     name="Ripley's K and L functions of a point pattern", topic=_PTS,
     purpose="Ripley's K function and its variance-stabilized L function at given distances for a point pattern, without edge correction, in the bounding rectangle of the points.",
     inputs=[_vecin("x", "x coordinates of the points", 3, 2000), _vecin("y", "y coordinates of the points", 3, 2000), _vecin("d", "distances at which to evaluate K", 1, 50)],
     outputs=[_o("K", "number", "K(d) at each distance"), _o("L", "number", "L(d) = sqrt(K/pi) at each distance"), _o("area", "number", "area of the bounding rectangle")],
     primary="K", conditions=["window = bounding rectangle of the points", "pairs closer than d (strictly) are counted; no edge correction"],
     method="K(d) = A * (number of ordered pairs closer than d) / n^2 with A the bounding-rectangle area (as pointpats)",
     latex=r"\hat K(d) = \frac{A}{n^2}\sum_{i\ne j}\mathbf{1}(d_{ij} < d),\quad L(d) = \sqrt{K(d)/\pi}",
     run=ripley_k, reference=_ref_ripley_k, rand=_rand_ripley,
     wrong=lambda x, y, d: (lambda r: dict(r, K=[v / 2 for v in r["K"]], L=[math.sqrt(v / 2 / math.pi) for v in r["K"]]))(ripley_k(x, y, d)),
     phrases=("Ripley's K", "Ripley K function", "K function"),
     examples=[_ex({"x": [0, 1, 2, 8, 9, 10], "y": [0, 1, 0, 9, 10, 9], "d": [1.5, 3, 12]}), _ex({"x": [0, 10, 0, 10, 5], "y": [0, 0, 10, 10, 5], "d": [5, 8]})],
     level="advanced")


def mean_center(x, y):
    n = len(x)
    _need(n == len(y) and n >= 1, "give one y per x")
    mx, my = math.fsum(x) / n, math.fsum(y) / n
    sd = math.sqrt(math.fsum((a - mx) ** 2 + (b - my) ** 2 for a, b in zip(x, y)) / n)
    return _out(mean_x=mx, mean_y=my, std_distance=sd)


def _ref_mean_center(x, y):
    import numpy as np
    _pysal()
    from pointpats import centrography as c
    P = np.c_[x, y]
    m = c.mean_center(P)
    return {"mean_x": float(m[0]), "mean_y": float(m[1]), "std_distance": float(c.std_distance(P))}


_add("geo_mean_center_standard_distance",
     name="Mean centre and standard distance of a point pattern", topic=_PTS,
     purpose="The mean centre of a set of points and the standard distance (root-mean-square distance from the mean centre).",
     inputs=[_vecin("x", "x coordinates", 1, 5000), _vecin("y", "y coordinates", 1, 5000)],
     outputs=[_o("mean_x", "number", "x of the mean centre"), _o("mean_y", "number", "y of the mean centre"), _o("std_distance", "number", "standard distance")],
     primary="std_distance", conditions=["population form (divide by n)"],
     method="mean of the coordinates; standard distance sqrt(sum ((x - xbar)^2 + (y - ybar)^2)/n)",
     latex=r"SD = \sqrt{\frac{\sum_i (x_i - \bar x)^2 + (y_i - \bar y)^2}{n}}",
     run=mean_center, reference=_ref_mean_center, rand=lambda rng: dict(zip(("x", "y"), _rand_pts(rng, 2, 80))),
     wrong=lambda x, y: (lambda r: dict(r, std_distance=r["std_distance"] * math.sqrt(len(x) / max(1, len(x) - 1)) if len(x) > 1 else 1.0))(mean_center(x, y)),
     phrases=("Mean center and standard distance", "Standard distance", "Mean centre"),
     examples=[_ex({"x": [0, 4, 4, 0], "y": [0, 0, 3, 3]}), _ex({"x": [66.22, 22.52, 31.01, 9.47, 30.78, 75.21, 79.26, 8.23, 98.73, 89.78, 65.19],
                                                               "y": [32.54, 22.39, 81.21, 31.02, 60.10, 58.93, 7.68, 39.93, 77.17, 42.53, 92.08]})],
     level="beginner")


def weighted_mean_center(x, y, w):
    n = len(x)
    _need(n == len(y) == len(w) and n >= 1, "give one y and one weight per x")
    for v in w:
        _nonneg("each weight", v)
    W = math.fsum(w)
    _need(W > 0, "the weights must not all be zero")
    mx, my = math.fsum(a * b for a, b in zip(w, x)) / W, math.fsum(a * b for a, b in zip(w, y)) / W
    sd = math.sqrt(math.fsum(c * ((a - mx) ** 2 + (b - my) ** 2) for a, b, c in zip(x, y, w)) / W)
    return _out(wmean_x=mx, wmean_y=my, weighted_std_distance=sd)


def _ref_wmc(x, y, w):
    import numpy as np
    _pysal()
    from pointpats import centrography as c
    m = c.weighted_mean_center(np.c_[x, y], np.array(w, dtype=float))
    d2 = (np.array(x) - m[0]) ** 2 + (np.array(y) - m[1]) ** 2
    return {"wmean_x": float(m[0]), "wmean_y": float(m[1]), "weighted_std_distance": float(np.sqrt(np.average(d2, weights=w)))}


_add("geo_weighted_mean_center",
     name="Weighted mean centre and weighted standard distance", topic=_PTS,
     purpose="The weighted mean centre of a set of points (e.g. population-weighted centroid) and the weighted standard distance about it.",
     inputs=[_vecin("x", "x coordinates", 1, 5000), _vecin("y", "y coordinates", 1, 5000), _vecin("w", "non-negative weights", 1, 5000)],
     outputs=[_o("wmean_x", "number", "x of the weighted mean centre"), _o("wmean_y", "number", "y of the weighted mean centre"),
              _o("weighted_std_distance", "number", "weighted standard distance")],
     primary="wmean_x", conditions=["non-negative weights, not all zero"],
     method="weighted means of the coordinates; sqrt(sum w d^2 / sum w)",
     latex=r"\bar x_w = \frac{\sum w_ix_i}{\sum w_i},\quad SD_w = \sqrt{\frac{\sum w_i\left[(x_i - \bar x_w)^2 + (y_i - \bar y_w)^2\right]}{\sum w_i}}",
     run=weighted_mean_center, reference=_ref_wmc,
     rand=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1], "w": [round(rng.uniform(0.1, 100), 2) for _ in xy[0]]})(_rand_pts(rng, 2, 80)),
     wrong=lambda x, y, w: mean_center(x, y) and _out(wmean_x=math.fsum(x) / len(x), wmean_y=math.fsum(y) / len(y), weighted_std_distance=mean_center(x, y)["std_distance"]),
     phrases=("Weighted mean center", "Weighted mean centre", "Population-weighted centroid"),
     examples=[_ex({"x": [0, 10, 20], "y": [0, 0, 10], "w": [100, 50, 50]}), _ex({"x": [1, 2, 3, 4], "y": [4, 3, 2, 1], "w": [1, 2, 3, 4]})],
     level="beginner", weak="pointpats computes the weighted centre; the weighted standard distance is numpy's weighted average of the same squared distances")


def std_ellipse(x, y):
    n = len(x)
    _need(n == len(y), "give one y per x")
    _need(n >= 3, "give at least 3 points")
    mx, my = math.fsum(x) / n, math.fsum(y) / n
    sxx = math.fsum((a - mx) ** 2 for a in x) / n
    syy = math.fsum((b - my) ** 2 for b in y) / n
    sxy = math.fsum((a - mx) * (b - my) for a, b in zip(x, y)) / n
    tr, det = sxx + syy, sxx * syy - sxy * sxy
    disc = math.sqrt(max(0.0, (sxx - syy) ** 2 / 4 + sxy * sxy))
    l1, l2 = tr / 2 + disc, tr / 2 - disc
    _need(l2 > 1e-12 * l1 and disc > 1e-9 * tr, "the points must not be collinear or circularly symmetric (the axes are undefined)")
    corr = 2 * n / (n - 2)
    ang = math.atan2(l1 - sxx, sxy) if sxy != 0 else (0.0 if syy > sxx else math.pi / 2)
    bearing = math.fmod(math.pi / 2 - ang + 4 * math.pi, math.pi)
    return _out(sx=math.sqrt(l1 * corr), sy=math.sqrt(l2 * corr), orientation=bearing / DEG)


def _ref_std_ellipse(x, y):
    import numpy as np
    _pysal()
    from pointpats import centrography as c
    a, b, th = c.ellipse(np.c_[x, y])
    P = np.c_[x, y] - np.mean(np.c_[x, y], axis=0)
    vals, vecs = np.linalg.eigh(P.T @ P / len(x))
    v = vecs[:, 1]
    bearing = math.degrees(math.atan2(v[0], v[1])) % 180.0
    return {"sx": float(max(a, b)), "sy": float(min(a, b)), "orientation": bearing}


def _rand_ellipse(rng):
    n = rng.randint(5, 60)
    th = rng.uniform(0, math.pi)
    a, b = _lu(rng, 1, 100), _lu(rng, 1, 100)
    pts = [(rng.gauss(0, a), rng.gauss(0, b)) for _ in range(n)]
    xs = [round(p[0] * math.cos(th) - p[1] * math.sin(th) + 500, 3) for p in pts]
    ys = [round(p[0] * math.sin(th) + p[1] * math.cos(th) + 500, 3) for p in pts]
    return {"x": xs, "y": ys}


_add("geo_standard_deviational_ellipse",
     name="Standard deviational ellipse of a point pattern", topic=_PTS,
     purpose="The semi-axes of the standard deviational ellipse of a point pattern (CrimeStat correction sqrt(2) sqrt(n/(n-2))) and the orientation of its major axis.",
     inputs=[_vecin("x", "x coordinates", 3, 5000), _vecin("y", "y coordinates", 3, 5000)],
     outputs=[_o("sx", "number", "semi-major axis"), _o("sy", "number", "semi-minor axis"), _o("orientation", "number", "bearing of the major axis, degrees clockwise from north (y axis), in [0, 180)")],
     primary="sx", conditions=["n >= 3, points neither collinear nor circularly symmetric"],
     method="eigenvalues of the (1/n) covariance of the coordinates give the variances along the principal axes; axes = sqrt(2 n/(n-2) lambda) (CrimeStat); orientation from the major eigenvector",
     latex=r"s_{x,y} = \sqrt{\frac{2n}{n-2}\lambda_{1,2}},\quad \lambda_{1,2} = \frac{s_{xx}+s_{yy}}{2} \pm \sqrt{\frac{(s_{xx}-s_{yy})^2}{4} + s_{xy}^2}",
     run=std_ellipse, reference=_ref_std_ellipse, rand=_rand_ellipse,
     wrong=lambda x, y: (lambda r: dict(r, sx=r["sx"] / math.sqrt(2), sy=r["sy"] / math.sqrt(2), orientation=math.fmod(r["orientation"] + 90, 180)))(std_ellipse(x, y)),
     phrases=("Standard deviational ellipse", "Directional distribution ellipse"),
     examples=[_ex({"x": [66.22, 22.52, 31.01, 9.47, 30.78, 75.21, 79.26, 8.23, 98.73, 89.78, 65.19],
                    "y": [32.54, 22.39, 81.21, 31.02, 60.10, 58.93, 7.68, 39.93, 77.17, 42.53, 92.08]}),
               _ex({"x": [0, 1, 2, 3, 4, 5], "y": [0, 1.5, 1.8, 3.2, 3.9, 5.5]})],
     level="advanced")


def kernel_density(x, y, px, py, bandwidth, kernel):
    n = len(x)
    _need(n == len(y) and n >= 1, "give one y per x")
    _fin("the query point", px, py)
    _pos("the bandwidth", bandwidth)
    _need(kernel in ("gaussian", "epanechnikov", "uniform"), "the kernel must be gaussian, epanechnikov or uniform")
    h = bandwidth
    terms = []
    for a, b in zip(x, y):
        u2 = ((a - px) ** 2 + (b - py) ** 2) / (h * h)
        if kernel == "gaussian":
            terms.append(math.exp(-u2 / 2) / (2 * math.pi * h * h))
        elif kernel == "epanechnikov":
            terms.append(2 / (math.pi * h * h) * (1 - u2) if u2 < 1 else 0.0)
        else:
            terms.append(1 / (math.pi * h * h) if u2 < 1 else 0.0)
    s = math.fsum(terms)
    return _out(density=s / n, intensity=s)


def _ref_kde(x, y, px, py, bandwidth, kernel):
    import numpy as np
    from sklearn.neighbors import KernelDensity
    kd = KernelDensity(bandwidth=bandwidth, kernel={"uniform": "tophat"}.get(kernel, kernel), rtol=0, atol=0, algorithm="kd_tree").fit(np.c_[x, y])
    d = float(np.exp(kd.score_samples(np.array([[px, py]]))[0]))
    return {"density": d, "intensity": d * len(x)}


def _rand_kde(rng):
    for _ in range(50):
        xs, ys = _rand_pts(rng, 3, 80, 100.0)
        x = {"x": xs, "y": ys, "px": round(rng.uniform(0, 100), 3), "py": round(rng.uniform(0, 100), 3), "bandwidth": round(rng.uniform(5, 40), 3),
             "kernel": rng.choice(["gaussian", "epanechnikov", "uniform"])}
        u = [math.hypot(a - x["px"], b - x["py"]) / x["bandwidth"] for a, b in zip(xs, ys)]
        if any(v < 0.95 for v in u) and all(abs(v - 1) > 1e-6 for v in u):
            return x
    return x


_add("geo_kernel_density",
     name="Kernel density estimate of a point pattern at a location", topic=_PTS,
     purpose="The two-dimensional kernel density (per unit area, integrating to 1) and the intensity (events per unit area) of a point pattern at a location, with a stated kernel and bandwidth.",
     inputs=[_vecin("x", "x coordinates of the events", 1, 5000), _vecin("y", "y coordinates of the events", 1, 5000), _numin("px", "x of the location"),
             _numin("py", "y of the location"), _numin("bandwidth", "bandwidth h (coordinate units)"),
             _choice("kernel", ["gaussian", "epanechnikov", "uniform"], "kernel (gaussian with sd h; epanechnikov and uniform with radius h)")],
     outputs=[_o("density", "number", "density (per square coordinate unit, integrates to 1)"), _o("intensity", "number", "intensity = n x density (events per square unit)")],
     primary="density", conditions=["bandwidth > 0"],
     method="f(p) = (1/n) sum K_h(|p - x_i|) with 2-D normalized kernels: gaussian exp(-u^2/2)/(2 pi h^2), epanechnikov 2(1 - u^2)/(pi h^2), uniform 1/(pi h^2) for u = d/h < 1",
     latex=r"\hat f(\mathbf p) = \frac1n\sum_i K_h(\lVert\mathbf p - \mathbf x_i\rVert),\quad K^{gauss}_h = \frac{e^{-u^2/2}}{2\pi h^2},\ K^{epa}_h = \frac{2(1-u^2)}{\pi h^2},\ K^{unif}_h = \frac{1}{\pi h^2}",
     run=kernel_density, reference=_ref_kde, rand=_rand_kde,
     wrong=lambda x, y, px, py, bandwidth, kernel: (lambda r: _out(density=r["density"] * bandwidth, intensity=r["intensity"] * bandwidth))(kernel_density(x, y, px, py, bandwidth, kernel)),
     phrases=("Kernel density", "Kernel density estimate", "KDE at a point"),
     examples=[_ex({"x": [0, 1, 2, 5], "y": [0, 1, 0, 5], "px": 1, "py": 0.5, "bandwidth": 2, "kernel": "gaussian"}),
               _ex({"x": [0, 1, 2, 5], "y": [0, 1, 0, 5], "px": 1, "py": 0.5, "bandwidth": 2, "kernel": "epanechnikov"})],
     level="intermediate")


# ---------------------------------------------------------------- interpolation and geostatistics
def idw(x, y, z, px, py, power):
    n = len(x)
    _need(n == len(y) == len(z) and n >= 1, "give one y and one value per x")
    _fin("the query point", px, py)
    _pos("the power", power)
    _need(power <= 10, "the power must be at most 10")
    d = [math.hypot(a - px, b - py) for a, b in zip(x, y)]
    for i, v in enumerate(d):
        if v == 0:
            return _out(estimate=z[i])
    w = [v ** -power for v in d]
    return _out(estimate=math.fsum(a * b for a, b in zip(w, z)) / math.fsum(w))


def _ref_idw(x, y, z, px, py, power):
    import numpy as np
    from sklearn.neighbors import KNeighborsRegressor
    m = KNeighborsRegressor(n_neighbors=len(x), weights=(lambda d: 1.0 / d ** power) if power != 1 else "distance", algorithm="brute").fit(np.c_[x, y], np.array(z, dtype=float))
    return {"estimate": float(m.predict(np.array([[px, py]]))[0])}


_add("geo_idw_interpolation",
     name="Inverse distance weighted (IDW) interpolation at a point", topic=_GST,
     purpose="The inverse-distance-weighted (Shepard) estimate of a variable at a location from sample points, with a stated power (2 is the usual choice).",
     inputs=[_vecin("x", "x coordinates of the samples", 1, 5000), _vecin("y", "y coordinates of the samples", 1, 5000), _vecin("z", "sample values", 1, 5000),
             _numin("px", "x of the location"), _numin("py", "y of the location"), _numin("power", "distance power p (> 0)")],
     outputs=[_o("estimate", "number", "interpolated value")],
     primary="estimate", conditions=["power > 0 (at most 10)", "a location on a sample returns that sample's value"],
     method="z = sum d_i^-p z_i / sum d_i^-p over all samples",
     latex=r"\hat z(\mathbf p) = \frac{\sum_i d_i^{-p}z_i}{\sum_i d_i^{-p}}",
     run=idw, reference=_ref_idw,
     rand=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1], "z": [round(rng.uniform(0, 100), 2) for _ in xy[0]], "px": round(rng.uniform(0, 100), 3),
                                   "py": round(rng.uniform(0, 100), 3), "power": rng.choice([1.0, 2.0, 3.0, round(rng.uniform(0.5, 4), 2)])})(_rand_pts(rng, 2, 60, 100.0)),
     wrong=lambda x, y, z, px, py, power: idw(x, y, z, px, py, power + 1),
     phrases=("IDW interpolation", "Inverse distance weighting", "Inverse distance weighted interpolation"),
     examples=[_ex({"x": [0, 10, 0, 10], "y": [0, 0, 10, 10], "z": [1, 2, 3, 4], "px": 2, "py": 3, "power": 2}),
               _ex({"x": [0, 4, 8], "y": [0, 3, 0], "z": [10, 20, 15], "px": 4, "py": 1, "power": 1})],
     level="intermediate")


def empirical_semivariogram(x, y, z, bin_edges):
    n = len(x)
    _need(n == len(y) == len(z) and n >= 3, "give one y and one value per x (at least 3 samples)")
    e = list(bin_edges)
    _need(len(e) >= 2 and all(b > a for a, b in zip(e, e[1:])) and e[0] >= 0, "the bin edges must increase from a non-negative start")
    k = len(e) - 1
    sums, counts = [[] for _ in range(k)], [0] * k
    for i in range(n):
        for j in range(i + 1, n):
            d = math.hypot(x[i] - x[j], y[i] - y[j])
            for b in range(k):
                if e[b] <= d < e[b + 1]:
                    sums[b].append((z[i] - z[j]) ** 2)
                    counts[b] += 1
                    break
    _need(all(c > 0 for c in counts), "every lag bin must contain at least one pair")
    return {"gamma": _r([math.fsum(s) / (2 * c) for s, c in zip(sums, counts)]), "counts": [float(c) for c in counts]}


def _ref_semivariogram(x, y, z, bin_edges):
    import numpy as np
    import gstools as gs
    c, g, cnt = gs.vario_estimate((np.array(x, dtype=float), np.array(y, dtype=float)), np.array(z, dtype=float), np.array(bin_edges, dtype=float), return_counts=True)
    return {"gamma": [float(v) for v in g], "counts": [float(v) for v in cnt]}


def _rand_svg(rng):
    for _ in range(50):
        xs, ys = _rand_pts(rng, 8, 60, 100.0)
        z = [round(0.05 * a + rng.gauss(0, 1), 3) for a in xs]
        k = rng.randint(1, 6)
        e = sorted(set(round(v, 2) for v in [0.0] + [rng.uniform(5, 70) for _ in range(k)]))
        try:
            empirical_semivariogram(xs, ys, z, e)
        except ValueError:
            continue
        d = [math.hypot(xs[i] - xs[j], ys[i] - ys[j]) for i in range(len(xs)) for j in range(i)]
        if min(abs(p - t) for p in d for t in e) > 1e-6:
            return {"x": xs, "y": ys, "z": z, "bin_edges": e}
    return {"x": xs, "y": ys, "z": z, "bin_edges": e}


_add("geo_empirical_semivariogram",
     name="Empirical (Matheron) semivariogram in distance bins", topic=_GST,
     purpose="The classical (Matheron) empirical semivariogram of sampled values: half the mean squared difference of all pairs whose separation falls in each distance bin, and the pair counts.",
     inputs=[_vecin("x", "x coordinates of the samples", 3, 2000), _vecin("y", "y coordinates of the samples", 3, 2000), _vecin("z", "sample values", 3, 2000),
             _vecin("bin_edges", "increasing distance bin edges (a bin includes its lower edge)", 2, 50)],
     outputs=[_o("gamma", "number", "semivariance of each bin"), _o("counts", "number", "number of pairs in each bin")],
     primary="gamma", conditions=["every bin contains at least one pair", "bins [e_k, e_k+1)"],
     method="gamma(h_k) = 1/(2 N_k) sum over pairs in bin k of (z_i - z_j)^2",
     latex=r"\hat\gamma(h_k) = \frac{1}{2N_k}\sum_{(i,j)\in k}(z_i - z_j)^2",
     run=empirical_semivariogram, reference=_ref_semivariogram, rand=_rand_svg,
     wrong=lambda x, y, z, bin_edges: (lambda r: dict(r, gamma=[2 * v for v in r["gamma"]]))(empirical_semivariogram(x, y, z, bin_edges)),
     phrases=("Empirical semivariogram", "Experimental semivariogram", "Semivariogram"),
     examples=[_ex({"x": [0, 1, 2, 3, 4], "y": [0, 0, 0, 0, 0], "z": [1, 3, 2, 5, 4], "bin_edges": [0, 1.5, 2.5, 4.5]}),
               _ex({"x": [0, 10, 0, 10, 5], "y": [0, 0, 10, 10, 5], "z": [1, 2, 3, 4, 2.5], "bin_edges": [0, 8, 12, 15]})],
     level="advanced")


def _vmodel(model, h, a):
    if model == "exponential":
        return 1 - math.exp(-3 * h / a)
    if model == "gaussian":
        return 1 - math.exp(-(h / (a * 4.0 / 7.0)) ** 2)
    return 1.5 * h / a - 0.5 * (h / a) ** 3 if h <= a else 1.0


def _dvmodel(model, h, a):
    """d/da of the model shape."""
    if model == "exponential":
        return -math.exp(-3 * h / a) * 3 * h / (a * a)
    c = 4.0 / 7.0
    return -math.exp(-(h / (a * c)) ** 2) * 2 * h * h / (c * c * a ** 3)


def variogram_fit(h, gamma, model):
    n = len(h)
    _need(n == len(gamma) and n >= 4, "give one semivariance per lag (at least 4)")
    _need(model in ("exponential", "gaussian"), "the model must be exponential or gaussian")
    for v in h:
        _pos("each lag", v)
    for v in gamma:
        _fin("each semivariance", v)

    def linfit(a):
        f = [_vmodel(model, t, a) for t in h]
        c0, c, _ = _linfit(f, gamma)
        res = [g - c0 - c * fv for g, fv in zip(gamma, f)]
        return c0, c, res

    def sse(a):
        f = [_vmodel(model, t, a) for t in h]
        if max(f) - min(f) < 1e-9:
            return math.inf
        return math.fsum(r * r for r in linfit(a)[2])

    def grad(a):
        c0, c, res = linfit(a)
        return -2 * math.fsum(r * c * _dvmodel(model, t, a) for r, t in zip(res, h))
    lo, hi = min(h) / 20, max(h) * 20
    grid = [lo * (hi / lo) ** (i / 400) for i in range(401)]
    vals = [sse(a) for a in grid]
    i = min(range(len(grid)), key=lambda k: vals[k])
    _need(0 < i < len(grid) - 1, "the fit does not determine the range (no interior minimum)")
    a0, a1 = grid[i - 1], grid[i + 1]
    g0, g1 = grad(a0), grad(a1)
    _need(g0 < 0 < g1, "the fit does not determine the range")
    for _ in range(200):
        m = 0.5 * (a0 + a1)
        if m == a0 or m == a1:
            break
        gm = grad(m)
        if gm < 0:
            a0 = m
        else:
            a1 = m
    # polish the root of d(SSE)/da in 30-digit arithmetic (the double-precision gradient is limited by cancellation)
    import mpmath
    with mpmath.workdps(30):
        H = [mpmath.mpf(repr(float(t))) for t in h]
        G = [mpmath.mpf(repr(float(v))) for v in gamma]
        q = mpmath.mpf(4) / 7

        def shape(t, a):
            return 1 - mpmath.exp(-3 * t / a) if model == "exponential" else 1 - mpmath.exp(-(t / (a * q)) ** 2)

        def dshape(t, a):
            return -mpmath.exp(-3 * t / a) * 3 * t / a ** 2 if model == "exponential" else -mpmath.exp(-(t / (a * q)) ** 2) * 2 * t * t / (q * q * a ** 3)

        def lin(a):
            F = [shape(t, a) for t in H]
            mf, mg = mpmath.fsum(F) / n, mpmath.fsum(G) / n
            cc = mpmath.fsum((fv - mf) * (g - mg) for fv, g in zip(F, G)) / mpmath.fsum((fv - mf) ** 2 for fv in F)
            return mg - cc * mf, cc, F

        def gmp(a):
            c0m, cm, F = lin(a)
            return -2 * mpmath.fsum((g - c0m - cm * fv) * cm * dshape(t, a) for g, fv, t in zip(G, F, H))
        lo_, hi_ = mpmath.mpf(repr(a0)), mpmath.mpf(repr(a1))
        if gmp(lo_) < 0 < gmp(hi_):
            for _ in range(80):
                mid = (lo_ + hi_) / 2
                if gmp(mid) < 0:
                    lo_ = mid
                else:
                    hi_ = mid
        am = (lo_ + hi_) / 2
        c0m, cm, _ = lin(am)
        return _out(nugget=float(c0m), psill=float(cm), range=float(am), sill=float(c0m + cm))


def _ref_variogram_fit(h, gamma, model):
    import numpy as np
    from scipy.optimize import curve_fit
    if model == "exponential":
        f = lambda t, c0, c, a: c0 + c * (1 - np.exp(-3 * t / a))
        jac = lambda t, c0, c, a: np.c_[np.ones_like(t), 1 - np.exp(-3 * t / a), -c * np.exp(-3 * t / a) * 3 * t / a ** 2]
    else:
        q = 4.0 / 7.0
        f = lambda t, c0, c, a: c0 + c * (1 - np.exp(-(t / (a * q)) ** 2))
        jac = lambda t, c0, c, a: np.c_[np.ones_like(t), 1 - np.exp(-(t / (a * q)) ** 2), -c * np.exp(-(t / (a * q)) ** 2) * 2 * t * t / (q * q * a ** 3)]
    r = variogram_fit(h, gamma, model)
    p, _ = curve_fit(f, np.array(h, dtype=float), np.array(gamma, dtype=float), p0=[r["nugget"] * 1.05 + 0.01, r["psill"] * 0.95, r["range"] * 1.1],
                     jac=jac, method="lm", xtol=1e-15, ftol=1e-15, gtol=1e-15, maxfev=100000)
    # Gauss-Newton polish in numpy's extended precision (LM in doubles stalls near 1e-9 on flat problems)
    L = np.longdouble
    t, y, p = np.array(h, dtype=L), np.array(gamma, dtype=L), np.array(p, dtype=L)
    for _ in range(8):
        J = jac(t, *p)
        res = y - f(t, *p)
        p = p + _gn_step(J, res)
    p = [float(v) for v in p]
    return {"nugget": p[0], "psill": p[1], "range": p[2], "sill": p[0] + p[1]}


def _gn_step(J, res):
    """Gauss-Newton step (J'J) d = J'res solved by Gaussian elimination in the arrays' own (extended) precision."""
    import numpy as np
    A = J.T @ J
    b = J.T @ res
    n = len(b)
    M = np.concatenate([A, b.reshape(-1, 1)], axis=1)
    for c in range(n):
        piv = c + int(np.argmax(np.abs(M[c:, c])))
        M[[c, piv]] = M[[piv, c]]
        for r in range(c + 1, n):
            M[r] -= M[r, c] / M[c, c] * M[c]
    d = np.zeros(n, dtype=M.dtype)
    for r in range(n - 1, -1, -1):
        d[r] = (M[r, n] - M[r, r + 1:n] @ d[r + 1:]) / M[r, r]
    return d


def _rand_vfit(rng):
    model = rng.choice(["exponential", "gaussian"])
    a, c0, c = _lu(rng, 5, 50), rng.uniform(0.1, 2), rng.uniform(1, 10)
    k = rng.randint(10, 20)
    h = [round(a * 2.5 * (i + 0.5) / k * rng.uniform(0.9, 1.1), 3) for i in range(k)]
    g = [round(c0 + c * _vmodel(model, t, a) + rng.gauss(0, 0.01 * c), 5) for t in h]
    return {"h": h, "gamma": g, "model": model}


_add("geo_variogram_model_fit",
     name="Fit an exponential or Gaussian variogram model (nugget, partial sill, range)", topic=_GST,
     purpose="The nugget, partial sill, range and sill of an exponential or Gaussian variogram model fitted by ordinary least squares to an empirical semivariogram (lags and semivariances).",
     inputs=[_vecin("h", "lag distances", 4, 200), _vecin("gamma", "semivariances at the lags", 4, 200),
             _choice("model", ["exponential", "gaussian"], "variogram model (pykrige parameterization, practical range)")],
     outputs=[_o("nugget", "number", "nugget c0"), _o("psill", "number", "partial sill c"), _o("range", "number", "practical range a"), _o("sill", "number", "sill c0 + c")],
     primary="range", conditions=["the least-squares range has an interior minimum (between min(h)/20 and 20 max(h))"],
     method="variable projection: for each range the nugget and partial sill are linear least squares; the range solves d(SSE)/da = 0 (grid search, then bisection, polished in 30-digit arithmetic with mpmath)",
     latex=r"\gamma(h) = c_0 + c\left(1 - e^{-3h/a}\right)\ \text{or}\ c_0 + c\left(1 - e^{-(7h/4a)^2}\right),\quad \min_{c_0, c, a}\sum_k\left(\hat\gamma_k - \gamma(h_k)\right)^2",
     run=variogram_fit, reference=_ref_variogram_fit, rand=_rand_vfit,
     wrong=lambda h, gamma, model: variogram_fit(h, gamma, "gaussian" if model == "exponential" else "exponential"),
     phrases=("Variogram model fit", "Fit a variogram model", "Semivariogram model fitting"),
     examples=[_ex({"h": [1, 2, 4, 6, 8, 12, 16, 20], "gamma": [1.6, 2.9, 4.6, 5.6, 6.3, 6.8, 7.0, 7.05], "model": "exponential"}),
               _ex({"h": [1, 2, 4, 6, 8, 12, 16, 20], "gamma": [0.6, 1.1, 2.8, 4.6, 5.8, 6.9, 7.0, 7.0], "model": "gaussian"})],
     level="expert", weak="scipy curve_fit (Levenberg-Marquardt) re-solves the same least-squares problem from a nearby start")


def ordinary_kriging(x, y, z, px, py, model, nugget, sill, range):
    n = len(x)
    _need(n == len(y) == len(z) and n >= 3, "give one y and one value per x (at least 3 samples)")
    _need(model in ("exponential", "gaussian", "spherical"), "the model must be exponential, gaussian or spherical")
    _nonneg("the nugget", nugget)
    _pos("the range", range)
    _need(sill > nugget, "the sill must exceed the nugget")
    _fin("the location", px, py)
    _need(len(set(zip(x, y))) == n, "the sample locations must be distinct")
    for i in builtins_range(n):
        if x[i] == px and y[i] == py:
            return _out(estimate=z[i], variance=0.0)
    ps = sill - nugget
    g = lambda d: 0.0 if d == 0 else nugget + ps * _vmodel(model, d, range)
    A = [[g(math.hypot(x[i] - x[j], y[i] - y[j])) for j in builtins_range(n)] + [1.0] for i in builtins_range(n)] + [[1.0] * n + [0.0]]
    b = [g(math.hypot(x[i] - px, y[i] - py)) for i in builtins_range(n)] + [1.0]
    sol = _solve_lin(A, b)
    lam, mu = sol[:n], sol[n]
    return _out(estimate=math.fsum(l * v for l, v in zip(lam, z)), variance=math.fsum(l * v for l, v in zip(lam, b[:n])) + mu)


builtins_range = range


def _ref_kriging(x, y, z, px, py, model, nugget, sill, range):
    import numpy as np
    from pykrige.ok import OrdinaryKriging
    ok = OrdinaryKriging(np.array(x, dtype=float), np.array(y, dtype=float), np.array(z, dtype=float), variogram_model=model,
                         variogram_parameters={"sill": sill, "range": range, "nugget": nugget}, exact_values=True, pseudo_inv=False)
    est, ss = ok.execute("points", np.array([float(px)]), np.array([float(py)]), backend="vectorized")
    return {"estimate": float(est[0]), "variance": float(ss[0])}


def _rand_krig(rng):
    xs, ys = _rand_pts(rng, 4, 30, 100.0)
    return {"x": xs, "y": ys, "z": [round(0.1 * a + rng.gauss(0, 2), 3) for a in xs], "px": round(rng.uniform(0, 100), 3), "py": round(rng.uniform(0, 100), 3),
            "model": rng.choice(["exponential", "gaussian", "spherical"]), "nugget": round(rng.uniform(0, 1), 3), "sill": round(rng.uniform(2, 10), 3),
            "range": round(rng.uniform(20, 120), 3)}


_add("geo_ordinary_kriging",
     name="Ordinary kriging estimate and kriging variance at a point", topic=_GST,
     purpose="The ordinary kriging estimate of a variable at a location from samples, with a given variogram model (nugget, sill, range), and the kriging variance.",
     inputs=[_vecin("x", "x coordinates of the samples", 3, 500), _vecin("y", "y coordinates of the samples", 3, 500), _vecin("z", "sample values", 3, 500),
             _numin("px", "x of the location"), _numin("py", "y of the location"),
             _choice("model", ["exponential", "gaussian", "spherical"], "variogram model (pykrige parameterization)"),
             _numin("nugget", "nugget"), _numin("sill", "sill (nugget + partial sill)"), _numin("range", "range (practical range for exponential and gaussian)")],
     outputs=[_o("estimate", "number", "kriging estimate"), _o("variance", "number", "kriging variance")],
     primary="estimate", conditions=["distinct sample locations", "sill > nugget >= 0, range > 0", "the variogram is 0 at zero distance (exact interpolation)"],
     method="solve the ordinary kriging system [Gamma 1; 1' 0][lambda; mu] = [gamma0; 1]; estimate = lambda'z, variance = lambda'gamma0 + mu",
     latex=r"\begin{bmatrix}\Gamma & \mathbf 1\\ \mathbf 1^T & 0\end{bmatrix}\begin{bmatrix}\boldsymbol\lambda\\ \mu\end{bmatrix} = \begin{bmatrix}\boldsymbol\gamma_0\\ 1\end{bmatrix},\quad \hat z = \boldsymbol\lambda^T\mathbf z,\quad \sigma^2_{OK} = \boldsymbol\lambda^T\boldsymbol\gamma_0 + \mu",
     run=ordinary_kriging, reference=_ref_kriging, rand=_rand_krig,
     wrong=lambda x, y, z, px, py, model, nugget, sill, range: ordinary_kriging(x, y, z, px, py, model, 0.0, sill - nugget + 1, range / 3),
     phrases=("Ordinary kriging", "Kriging estimate", "Kriging interpolation"),
     examples=[_ex({"x": [0, 10, 0, 10], "y": [0, 0, 10, 10], "z": [1, 2, 3, 4], "px": 4, "py": 6, "model": "spherical", "nugget": 0, "sill": 2, "range": 20}),
               _ex({"x": [0, 50, 20, 80, 60], "y": [0, 10, 70, 40, 90], "z": [12, 15, 9, 20, 14], "px": 40, "py": 40, "model": "exponential", "nugget": 0.5, "sill": 10, "range": 100})],
     level="expert", weak="pykrige solves the same kriging system (a library implementation of the defining equations)")


def trend_surface(x, y, z, order):
    n = len(x)
    _need(n == len(y) == len(z), "give one y and one value per x")
    _need(isinstance(order, int) and order in (1, 2), "the order must be 1 or 2")
    p = 3 if order == 1 else 6
    _need(n > p, f"give more than {p} samples for an order-{order} surface")
    cols = [[1.0] * n, list(map(float, x)), list(map(float, y))]
    if order == 2:
        cols += [[a * a for a in x], [a * b for a, b in zip(x, y)], [b * b for b in y]]
    beta = _ols(cols, z)
    fit = [math.fsum(beta[k] * cols[k][i] for k in range(p)) for i in range(n)]
    mz = math.fsum(z) / n
    sst = math.fsum((v - mz) ** 2 for v in z)
    _need(sst > 0, "the values must not all be equal")
    sse = math.fsum((a - b) ** 2 for a, b in zip(z, fit))
    return {"coefficients": _r(beta), "r_squared": _r(1 - sse / sst)}


def _ref_trend(x, y, z, order):
    import numpy as np
    x, y, z = (np.array(v, dtype=float) for v in (x, y, z))
    X = np.c_[np.ones_like(x), x, y] if order == 1 else np.c_[np.ones_like(x), x, y, x * x, x * y, y * y]
    b, *_ = np.linalg.lstsq(X, z, rcond=None)
    r = z - X @ b
    return {"coefficients": [float(v) for v in b], "r_squared": float(1 - (r @ r) / ((z - z.mean()) @ (z - z.mean())))}


def _rand_trend(rng):
    o = rng.choice([1, 2])
    xs, ys = _rand_pts(rng, 8, 60, 10.0)
    c = [rng.uniform(-5, 5) for _ in range(6)]
    z = [round(c[0] + c[1] * a + c[2] * b + (c[3] * a * a + c[4] * a * b + c[5] * b * b if o == 2 else 0) + rng.gauss(0, 1), 4) for a, b in zip(xs, ys)]
    return {"x": xs, "y": ys, "z": z, "order": o}


_add("geo_trend_surface",
     name="Polynomial trend surface (first or second order) by least squares", topic=_GST,
     purpose="The coefficients and R^2 of a first-order (plane) or second-order (quadratic) trend surface fitted to sampled values by least squares.",
     inputs=[_vecin("x", "x coordinates", 4, 5000), _vecin("y", "y coordinates", 4, 5000), _vecin("z", "values", 4, 5000), _intin("order", "order of the surface (1 or 2)", 1, 2)],
     outputs=[_o("coefficients", "number", "b0, b1 (x), b2 (y)[, b3 (x^2), b4 (xy), b5 (y^2)]"), _o("r_squared", "number", "coefficient of determination")],
     primary="r_squared", conditions=["more samples than coefficients", "values not all equal"],
     method="ordinary least squares by the normal equations",
     latex=r"z = b_0 + b_1x + b_2y\ \left[+\, b_3x^2 + b_4xy + b_5y^2\right]",
     run=trend_surface, reference=_ref_trend, rand=_rand_trend,
     wrong=lambda x, y, z, order: (lambda r: dict(r, coefficients=[r["coefficients"][0]] + r["coefficients"][2:3] + r["coefficients"][1:2] + r["coefficients"][3:]))(trend_surface(x, y, z, order)),
     phrases=("Trend surface", "Trend surface analysis", "Polynomial trend surface"),
     examples=[_ex({"x": [0, 1, 2, 0, 1, 2], "y": [0, 0, 0, 1, 1, 1], "z": [1, 3, 5.2, 2, 4.1, 6], "order": 1}),
               _ex({"x": [0, 1, 2, 0, 1, 2, 0, 1, 2], "y": [0, 0, 0, 1, 1, 1, 2, 2, 2], "z": [0.3, 1.1, 3.9, 1, 2.4, 5.2, 4.1, 5, 8.6], "order": 2})],
     level="advanced")


def rank_size(populations):
    n = len(populations)
    _need(n >= 3, "give at least 3 settlement populations")
    for v in populations:
        _pos("each population", v)
    p = sorted(populations, reverse=True)
    lx = [math.log(i + 1) for i in range(n)]
    ly = [math.log(v) for v in p]
    c0, c1, r2 = _linfit(lx, ly)
    return _out(q=-c1, P1_fit=math.exp(c0), r_squared=r2, primacy=p[0] / p[1])


def _ref_rank_size(populations):
    import numpy as np
    from scipy.stats import linregress
    p = np.sort(np.array(populations, dtype=float))[::-1]
    r = linregress(np.log(np.arange(1, len(p) + 1)), np.log(p))
    return {"q": -r.slope, "P1_fit": float(np.exp(r.intercept)), "r_squared": r.rvalue ** 2, "primacy": float(p[0] / p[1])}


_add("geo_rank_size_rule",
     name="Rank-size rule (Zipf) fit of a settlement system", topic="urban geography",
     purpose="The Zipf exponent q and fitted size of the largest city from a log-log regression of population on rank (P_r = P_1 r^-q), the R^2 of the fit and the two-city primacy ratio.",
     inputs=[_vecin("populations", "populations of the settlements (any order)", 3, 10000)],
     outputs=[_o("q", "number", "rank-size exponent (1 is Zipf's law)"), _o("P1_fit", "number", "fitted population of rank 1"),
              _o("r_squared", "number", "R^2 of the log-log fit"), _o("primacy", "number", "largest / second largest population")],
     primary="q", conditions=["positive populations"],
     method="sort descending; least squares of ln P on ln rank",
     latex=r"\ln P_r = \ln P_1 - q\ln r",
     run=rank_size, reference=_ref_rank_size,
     rand=lambda rng: (lambda q, n: {"populations": [round(1e6 * (i + 1) ** -q * math.exp(rng.gauss(0, 0.2))) + 1.0 for i in range(n)]})(rng.uniform(0.6, 1.4), rng.randint(5, 60)),
     wrong=lambda populations: (lambda r: dict(r, q=r["q"] * math.log(10)))(rank_size(populations)),
     phrases=("Rank-size rule", "Zipf rank size", "Rank size fit"),
     examples=[_ex({"populations": [8400000, 3900000, 2700000, 2300000, 1600000, 1500000, 1400000, 1300000]}), _ex({"populations": [1000, 480, 350, 260, 190, 170]})],
     level="intermediate")


def density_gradient(r, D):
    n = len(r)
    _need(n == len(D) and n >= 3, "give one density per distance (at least 3)")
    for v in r:
        _nonneg("each distance", v)
    for v in D:
        _pos("each density", v)
    c0, c1, r2 = _linfit(r, [math.log(v) for v in D])
    return _out(D0=math.exp(c0), b=-c1, r_squared=r2, half_distance=math.log(2) / -c1 if c1 < 0 else None)


def _ref_density_gradient(r, D):
    import numpy as np
    from scipy.stats import linregress
    q = linregress(np.array(r, dtype=float), np.log(np.array(D, dtype=float)))
    return {"D0": float(np.exp(q.intercept)), "b": -q.slope, "r_squared": q.rvalue ** 2, "half_distance": math.log(2) / -q.slope if q.slope < 0 else None}


_add("geo_density_gradient",
     name="Urban population density gradient (Clark's negative exponential model)", topic="urban geography",
     purpose="The central density D0, density gradient b and half-distance of Clark's model D(r) = D0 exp(-b r) fitted by log-linear least squares to densities at distances from the centre, with the R^2.",
     inputs=[_vecin("r", "distances from the centre (any unit)", 3, 5000), _vecin("D", "population densities at those distances", 3, 5000)],
     outputs=[_o("D0", "number", "fitted central density"), _o("b", "number", "density gradient (per distance unit)"), _o("r_squared", "number", "R^2 of the log-linear fit"),
              _o("half_distance", "number", "distance over which density halves (ln 2 / b), when b > 0")],
     primary="b", conditions=["positive densities", "distances not all equal"],
     method="least squares of ln D on r",
     latex=r"D(r) = D_0e^{-br},\quad \ln D = \ln D_0 - br",
     run=density_gradient, reference=_ref_density_gradient,
     rand=lambda rng: (lambda b, n: (lambda rs: {"r": rs, "D": [round(1e4 * math.exp(-b * v + rng.gauss(0, 0.1)), 2) for v in rs]})(sorted(round(rng.uniform(0, 30), 2) for _ in range(n))))(rng.uniform(0.05, 0.5), rng.randint(5, 40)),
     wrong=lambda r, D: (lambda q: dict(q, b=q["b"] * math.log10(math.e)))(density_gradient(r, D)),
     phrases=("Density gradient", "Clark density gradient", "Population density gradient"),
     examples=[_ex({"r": [1, 2, 4, 6, 8, 10], "D": [9000, 7600, 5300, 3900, 2800, 2000]}), _ex({"r": [0.5, 1.5, 2.5, 3.5], "D": [12000, 8500, 6100, 4300]})],
     level="intermediate")


def gravity_poisson(T, O, D, d):
    n = len(T)
    _need(n == len(O) == len(D) == len(d) and n >= 6, "give the flow, origin mass, destination mass and distance of each pair (at least 6 pairs)")
    for v in T:
        _nonneg("each flow", v)
    for v in list(O) + list(D) + list(d):
        _pos("masses and distances", v)
    X = [[1.0] * n, [math.log(v) for v in O], [math.log(v) for v in D], [math.log(v) for v in d]]
    for c in X[1:]:
        _need(max(c) - min(c) > 0, "masses and distances must vary across the pairs")
    mT = math.fsum(T) / n
    _need(mT > 0, "the flows must not all be zero")
    beta = [math.log(mT), 0.0, 0.0, 0.0]
    delta = math.inf
    for it in range(100):
        eta = [math.fsum(beta[k] * X[k][i] for k in range(4)) for i in range(n)]
        mu = [math.exp(e) for e in eta]
        zw = [eta[i] + (T[i] - mu[i]) / mu[i] for i in range(n)]
        A = [[math.fsum(mu[i] * X[a][i] * X[b][i] for i in range(n)) for b in range(4)] for a in range(4)]
        rhs = [math.fsum(mu[i] * X[a][i] * zw[i] for i in range(n)) for a in range(4)]
        new = _solve_lin(A, rhs)
        delta = max(abs(a - b) for a, b in zip(new, beta))
        beta = new
        if delta < 1e-14 * max(1.0, max(abs(v) for v in beta)):
            break
    _need(delta < 1e-9 * max(1.0, max(abs(v) for v in beta)), "the Poisson fit did not converge")
    return _out(k=math.exp(beta[0]), alpha=beta[1], beta=beta[2], gamma=-beta[3])


def _ref_gravity(T, O, D, d):
    import numpy as np
    import statsmodels.api as sm
    X = np.c_[np.ones(len(T)), np.log(O), np.log(D), np.log(d)]
    r = sm.GLM(np.array(T, dtype=float), X, family=sm.families.Poisson()).fit(tol=1e-15, maxiter=500)
    b = r.params
    return {"k": float(np.exp(b[0])), "alpha": float(b[1]), "beta": float(b[2]), "gamma": float(-b[3])}


def _rand_gravity(rng):
    n = rng.randint(8, 40)
    O = [round(_lu(rng, 1e3, 1e6)) for _ in range(n)]
    D = [round(_lu(rng, 1e3, 1e6)) for _ in range(n)]
    d = [round(_lu(rng, 5, 500), 1) for _ in range(n)]
    a, b, g = rng.uniform(0.5, 1.2), rng.uniform(0.5, 1.2), rng.uniform(0.8, 2.5)
    raw = [o ** a * dd ** b / di ** g for o, dd, di in zip(O, D, d)]
    k = 200.0 / (sum(raw) / n)
    T = [float(max(0, round(k * r * math.exp(rng.gauss(0, 0.2))))) for r in raw]
    return {"T": T, "O": [float(v) for v in O], "D": [float(v) for v in D], "d": d}


_add("geo_gravity_model_poisson_fit",
     name="Spatial interaction (gravity) model fitted by Poisson regression", topic="urban geography",
     purpose="The constant k, the origin and destination mass exponents and the distance-decay exponent of the unconstrained gravity model T = k O^alpha D^beta d^-gamma, fitted to observed flows by Poisson maximum likelihood.",
     inputs=[_vecin("T", "observed flows between the pairs", 6, 5000), _vecin("O", "origin masses (e.g. populations)", 6, 5000),
             _vecin("D", "destination masses", 6, 5000), _vecin("d", "distances between the pairs", 6, 5000)],
     outputs=[_o("k", "number", "constant"), _o("alpha", "number", "origin mass exponent"), _o("beta", "number", "destination mass exponent"),
              _o("gamma", "number", "distance-decay exponent")],
     primary="gamma", conditions=["non-negative flows, positive masses and distances that vary"],
     method="Poisson GLM with log link, ln mu = ln k + alpha ln O + beta ln D - gamma ln d, by iteratively reweighted least squares",
     latex=r"T_{ij} \sim \mathrm{Poisson}(\mu_{ij}),\quad \ln\mu_{ij} = \ln k + \alpha\ln O_i + \beta\ln D_j - \gamma\ln d_{ij}",
     run=gravity_poisson, reference=_ref_gravity, rand=_rand_gravity,
     wrong=lambda T, O, D, d: (lambda X, y: (lambda c: _out(k=math.exp(c[0]), alpha=c[1], beta=c[2], gamma=-c[3]))(_ols(X, y)))(
         [[1.0] * len(T), [math.log(v) for v in O], [math.log(v) for v in D], [math.log(v) for v in d]], [math.log(v + 1) for v in T]),
     phrases=("Gravity model fit", "Poisson gravity model", "Spatial interaction model fit"),
     examples=[_ex({"T": [120, 45, 300, 80, 15, 60, 200, 33], "O": [5e5, 2e5, 9e5, 3e5, 1e5, 4e5, 8e5, 2.5e5], "D": [3e5, 4e5, 6e5, 2e5, 3e5, 1e5, 5e5, 2e5],
                    "d": [50, 120, 40, 90, 200, 70, 60, 110]}),
               _ex({"T": [10, 25, 5, 40, 18, 7, 30], "O": [1e4, 3e4, 1e4, 5e4, 2e4, 1e4, 4e4], "D": [2e4, 2e4, 1e4, 3e4, 3e4, 2e4, 2e4], "d": [10, 15, 30, 12, 20, 40, 14]})],
     level="expert")


# ================================================================================================ REMOTE SENSING
_RS = "remote sensing"


def _refl(name, *vals):
    for v in vals:
        _fin(name, v)
        _need(0 <= v <= 1, f"{name} must be a reflectance between 0 and 1")


def _nd(a, b):
    _need(a + b != 0, "a normalized difference needs a non-zero sum of the bands")
    return (a - b) / (a + b)


def vegetation_indices(red, nir, blue, L):
    _refl("each band", red, nir, blue)
    _fin("L", L)
    _need(0 <= L <= 1, "the soil adjustment factor L must lie between 0 and 1")
    evi_den = nir + 6 * red - 7.5 * blue + 1
    _need(evi_den > 0, "the EVI denominator must be positive")
    _need(nir + red + L > 0 and nir + red + 0.16 > 0, "the band sums must be positive")
    msavi_disc = (2 * nir + 1) ** 2 - 8 * (nir - red)
    return _out(NDVI=_nd(nir, red), EVI=2.5 * (nir - red) / evi_den, SAVI=(1 + L) * (nir - red) / (nir + red + L),
                OSAVI=(nir - red) / (nir + red + 0.16), MSAVI=0.5 * (2 * nir + 1 - math.sqrt(msavi_disc)), EVI2=2.5 * (nir - red) / (nir + 2.4 * red + 1))


def _ref_veg(red, nir, blue, L):
    import spyndex
    p = {"N": nir, "R": red, "B": blue, "L": L, "g": 2.5, "C1": 6.0, "C2": 7.5}
    out = {k: float(spyndex.computeIndex(k, params=p)) for k in ("NDVI", "SAVI", "OSAVI", "MSAVI")}
    out["EVI"] = float(spyndex.computeIndex("EVI", params=dict(p, L=1.0)))
    out["EVI2"] = float(spyndex.computeIndex("EVI2", params=dict(p, L=1.0)))
    return out


_add("geo_vegetation_indices",
     name="Vegetation indices: NDVI, EVI, SAVI, OSAVI, MSAVI and EVI2 from band reflectances", topic=_RS,
     purpose="The common spectral vegetation indices of a pixel from its red, near-infrared and blue surface reflectances, with a stated soil adjustment factor L for SAVI (EVI uses its standard coefficients G = 2.5, C1 = 6, C2 = 7.5, L = 1).",
     inputs=[_numin("red", "red reflectance (0-1)"), _numin("nir", "near-infrared reflectance (0-1)"), _numin("blue", "blue reflectance (0-1)"),
             _numin("L", "SAVI soil adjustment factor (0.5 is usual)")],
     outputs=[_o("NDVI", "number", "(NIR - R)/(NIR + R)"), _o("EVI", "number", "2.5 (NIR - R)/(NIR + 6R - 7.5B + 1)"),
              _o("SAVI", "number", "(1 + L)(NIR - R)/(NIR + R + L)"), _o("OSAVI", "number", "(NIR - R)/(NIR + R + 0.16)"),
              _o("MSAVI", "number", "(2NIR + 1 - sqrt((2NIR + 1)^2 - 8(NIR - R)))/2"), _o("EVI2", "number", "2.5 (NIR - R)/(NIR + 2.4R + 1)")],
     primary="NDVI", conditions=["reflectances between 0 and 1", "positive denominators"],
     method="the published formulas (Rouse 1974; Huete 1988, 2002; Rondeaux 1996; Qi 1994; Jiang 2008)",
     latex=r"NDVI = \frac{\rho_{N} - \rho_R}{\rho_N + \rho_R},\ EVI = 2.5\frac{\rho_N - \rho_R}{\rho_N + 6\rho_R - 7.5\rho_B + 1},\ SAVI = (1+L)\frac{\rho_N - \rho_R}{\rho_N + \rho_R + L}",
     run=vegetation_indices, reference=_ref_veg,
     rand=lambda rng: {"red": round(rng.uniform(0.01, 0.3), 4), "nir": round(rng.uniform(0.1, 0.6), 4), "blue": round(rng.uniform(0.01, 0.15), 4), "L": rng.choice([0.5, 0.25, 1.0, round(rng.uniform(0.1, 0.9), 2)])},
     wrong=lambda red, nir, blue, L: (lambda r: dict(r, NDVI=(nir - red) / nir, EVI=2.5 * (nir - red) / (nir + 6 * red - 7.5 * blue + 1) * 0.4))(vegetation_indices(red, nir, blue, L)),
     phrases=("Vegetation indices", "NDVI and vegetation indices", "Spectral vegetation indices"),
     examples=[_ex({"red": 0.08, "nir": 0.45, "blue": 0.04, "L": 0.5}), _ex({"red": 0.2, "nir": 0.25, "blue": 0.1, "L": 0.5})],
     level="beginner", weak="spyndex evaluates the same published formulas from its catalogue (it checks the formulas and coefficients, not an independent computation)")


def water_indices(green, nir, swir1, swir2):
    _refl("each band", green, nir, swir1, swir2)
    return _out(NDWI=_nd(green, nir), MNDWI=_nd(green, swir1), NDMI=_nd(nir, swir1), NBR=_nd(nir, swir2), NDBI=_nd(swir1, nir))


def _ref_water(green, nir, swir1, swir2):
    import spyndex
    p = {"G": green, "N": nir, "S1": swir1, "S2": swir2}
    return {k: float(spyndex.computeIndex(k, params=p)) for k in ("NDWI", "MNDWI", "NDMI", "NBR", "NDBI")}


_add("geo_water_burn_built_indices",
     name="Water, moisture, burn and built-up indices: NDWI, MNDWI, NDMI, NBR and NDBI", topic=_RS,
     purpose="Normalized-difference indices for open water (NDWI, MNDWI), vegetation moisture (NDMI), burn severity (NBR) and built-up land (NDBI) from green, near-infrared and two shortwave-infrared reflectances.",
     inputs=[_numin("green", "green reflectance (0-1)"), _numin("nir", "near-infrared reflectance (0-1)"), _numin("swir1", "shortwave infrared 1 (about 1.6 um) reflectance"),
             _numin("swir2", "shortwave infrared 2 (about 2.2 um) reflectance")],
     outputs=[_o("NDWI", "number", "(G - NIR)/(G + NIR)"), _o("MNDWI", "number", "(G - SWIR1)/(G + SWIR1)"), _o("NDMI", "number", "(NIR - SWIR1)/(NIR + SWIR1)"),
              _o("NBR", "number", "(NIR - SWIR2)/(NIR + SWIR2)"), _o("NDBI", "number", "(SWIR1 - NIR)/(SWIR1 + NIR)")],
     primary="NDWI", conditions=["reflectances between 0 and 1, band sums non-zero"],
     method="normalized differences of the band pairs (McFeeters 1996; Xu 2006; Gao 1996; Key and Benson 2006; Zha 2003)",
     latex=r"ND(a, b) = \frac{\rho_a - \rho_b}{\rho_a + \rho_b}",
     run=water_indices, reference=_ref_water,
     rand=lambda rng: {k: round(rng.uniform(0.01, 0.6), 4) for k in ("green", "nir", "swir1", "swir2")},
     wrong=lambda green, nir, swir1, swir2: (lambda r: dict(r, NDWI=_nd(nir, green), NBR=_nd(nir, swir1)))(water_indices(green, nir, swir1, swir2)),
     phrases=("Water indices", "NDWI and water indices", "Burn and water indices"),
     examples=[_ex({"green": 0.08, "nir": 0.03, "swir1": 0.02, "swir2": 0.01}), _ex({"green": 0.07, "nir": 0.35, "swir1": 0.2, "swir2": 0.1})],
     level="beginner", weak="spyndex evaluates the same published formulas from its catalogue")


def _spencer(doy):
    g = 2 * math.pi * (doy - 1) / 365.0
    return 1.00011 + 0.034221 * math.cos(g) + 0.00128 * math.sin(g) + 0.000719 * math.cos(2 * g) + 0.000077 * math.sin(2 * g)


def toa_reflectance(radiance, esun, doy, sun_elevation):
    _nonneg("the radiance", radiance)
    _pos("ESUN", esun)
    _need(isinstance(doy, int) and 1 <= doy <= 366, "the day of year must be a whole number from 1 to 366")
    _fin("the sun elevation", sun_elevation)
    _need(0 < sun_elevation <= 90, "the sun must be above the horizon (elevation 0-90 degrees)")
    d2 = 1.0 / _spencer(doy)
    rho = math.pi * radiance * d2 / (esun * math.sin(sun_elevation * DEG))
    return _out(earth_sun_distance=math.sqrt(d2), reflectance=rho)


def _ref_toa(radiance, esun, doy, sun_elevation):
    import pvlib
    f = float(pvlib.irradiance.get_extra_radiation(doy, solar_constant=1.0, method="spencer"))
    zen = 90.0 - sun_elevation
    return {"earth_sun_distance": 1 / math.sqrt(f), "reflectance": math.pi * radiance / (f * esun * math.cos(math.radians(zen)))}


_add("geo_toa_reflectance",
     name="Top-of-atmosphere reflectance from at-sensor radiance", topic=_RS,
     purpose="The top-of-atmosphere (planetary) reflectance of a pixel from its at-sensor spectral radiance, the band's mean exoatmospheric solar irradiance ESUN, the day of year and the sun elevation, with the Earth-Sun distance.",
     inputs=[_numin("radiance", "at-sensor spectral radiance (W/(m^2 sr um))"), _numin("esun", "band mean exoatmospheric solar irradiance ESUN (W/(m^2 um))"),
             _intin("doy", "day of year (1-366)", 1, 366), _degin("sun_elevation", "sun elevation angle")],
     outputs=[_o("earth_sun_distance", "number", "Earth-Sun distance (astronomical units)"), _o("reflectance", "number", "top-of-atmosphere reflectance")],
     primary="reflectance", conditions=["sun above the horizon"],
     method="rho = pi L d^2 / (ESUN sin(sun elevation)); d^2 from Spencer's (1971) Fourier series for (r0/r)^2",
     latex=r"\rho_{TOA} = \frac{\pi L_\lambda d^2}{ESUN_\lambda\cos\theta_s},\quad \frac{1}{d^2} = 1.00011 + 0.034221\cos\Gamma + 0.00128\sin\Gamma + 0.000719\cos2\Gamma + 0.000077\sin2\Gamma",
     run=toa_reflectance, reference=_ref_toa,
     rand=lambda rng: {"radiance": round(rng.uniform(5, 200), 3), "esun": round(rng.uniform(200, 2000), 2), "doy": rng.randint(1, 365), "sun_elevation": round(rng.uniform(15, 75), 3)},
     wrong=lambda radiance, esun, doy, sun_elevation: (lambda r: dict(r, reflectance=math.pi * radiance * _spencer(doy) / (esun * math.cos(sun_elevation * DEG))))(toa_reflectance(radiance, esun, doy, sun_elevation)),
     phrases=("TOA reflectance", "Top of atmosphere reflectance", "Planetary reflectance"),
     examples=[_ex({"radiance": 85.0, "esun": 1536, "doy": 172, "sun_elevation": 60}), _ex({"radiance": 40.5, "esun": 1031, "doy": 15, "sun_elevation": 25})],
     level="intermediate", weak="pvlib's Spencer series gives the Earth-Sun distance factor; the reflectance is then the standard formula")


def brightness_temperature(radiance, K1, K2):
    _pos("the radiance", radiance)
    _pos("K1 and K2", K1, K2)
    return _out(T=K2 / math.log(K1 / radiance + 1))


def _ref_bt(radiance, K1, K2):
    from scipy.optimize import brentq
    T = brentq(lambda t: K1 / math.expm1(K2 / t) - radiance, 30.0, 5000.0, xtol=1e-14, rtol=8.9e-16)
    return {"T": T}


_add("geo_brightness_temperature",
     name="At-sensor brightness temperature from thermal radiance", topic=_RS,
     purpose="The at-sensor brightness temperature of a thermal-infrared band from its spectral radiance and the band's calibration constants K1 and K2 (as in Landsat metadata).",
     inputs=[_numin("radiance", "spectral radiance (W/(m^2 sr um))"), _numin("K1", "band constant K1 (W/(m^2 sr um))"), _numin("K2", "band constant K2 (kelvin)")],
     outputs=[_o("T", "temperature", "brightness temperature")],
     primary="T", conditions=["positive radiance and constants"],
     method="inverted Planck relation T = K2 / ln(K1/L + 1)",
     latex=r"T_B = \frac{K_2}{\ln\left(\frac{K_1}{L_\lambda} + 1\right)}",
     run=brightness_temperature, reference=_ref_bt,
     rand=lambda rng: {"radiance": round(rng.uniform(3, 15), 4), "K1": rng.choice([774.8853, 480.8883, 607.76, 666.09]), "K2": rng.choice([1321.0789, 1201.1442, 1260.56, 1282.71])},
     wrong=lambda radiance, K1, K2: _out(T=K2 / math.log(K1 / radiance)),
     phrases=("Brightness temperature", "At-sensor brightness temperature"),
     examples=[_ex({"radiance": 10.5, "K1": 774.8853, "K2": 1321.0789}), _ex({"radiance": 8.2, "K1": 480.8883, "K2": 1201.1442})],
     level="intermediate", weak="scipy's brentq re-solves the defining radiance relation for T")


def ground_sample_distance(pixel_pitch, focal_length, altitude, pixels):
    _pos("the pixel pitch, focal length and altitude", pixel_pitch, focal_length, altitude)
    _need(isinstance(pixels, int) and pixels >= 1, "the number of pixels across must be a whole number >= 1")
    g = pixel_pitch * altitude / focal_length
    return _out(gsd=g, swath=g * pixels)


def _ref_gsd(pixel_pitch, focal_length, altitude, pixels):
    ifov = 2 * math.atan(pixel_pitch / (2 * focal_length))
    g = 2 * altitude * math.tan(ifov / 2)
    return {"gsd": g, "swath": 2 * altitude * math.tan(math.atan(pixels * pixel_pitch / (2 * focal_length)))}


_add("geo_ground_sample_distance",
     name="Ground sample distance and swath of a nadir-looking camera", topic=_RS,
     purpose="The ground sample distance (pixel footprint) and swath width of a nadir-looking frame camera or scanner from its pixel pitch, focal length, flying height above ground and number of pixels across track.",
     inputs=[_phys("pixel_pitch", "length", "detector pixel pitch"), _phys("focal_length", "length", "focal length"), _phys("altitude", "length", "height above the ground"),
             _intin("pixels", "number of pixels across track", 1, 10 ** 6)],
     outputs=[_o("gsd", "length", "ground sample distance"), _o("swath", "length", "swath width")],
     primary="gsd", conditions=["flat ground, nadir view"],
     method="similar triangles: GSD = pixel pitch x height / focal length; swath = GSD x pixels",
     latex=r"GSD = \frac{p\,H}{f},\quad W = GSD \times N",
     run=ground_sample_distance, reference=_ref_gsd,
     rand=lambda rng: {"pixel_pitch": round(rng.uniform(2, 15), 2) * 1e-6, "focal_length": round(rng.uniform(0.008, 1.5), 4), "altitude": round(rng.uniform(50, 7e5), 1), "pixels": rng.randint(500, 20000)},
     wrong=lambda pixel_pitch, focal_length, altitude, pixels: _out(gsd=pixel_pitch * focal_length / altitude, swath=pixel_pitch * focal_length / altitude * pixels),
     phrases=("Ground sample distance", "GSD and swath"),
     examples=[_ex({"pixel_pitch": "0.0024 mm", "focal_length": "8.8 mm", "altitude": "120 m", "pixels": 5472}),
               _ex({"pixel_pitch": "0.012 mm", "focal_length": "1200 mm", "altitude": "700 km", "pixels": 12000})],
     level="beginner", weak="the same similar-triangle relation through the instantaneous field of view (a geometric identity)")


def classification_accuracy(matrix):
    k = len(matrix)
    _need(k >= 2 and all(len(r) == k for r in matrix), "the confusion matrix must be square with at least 2 classes")
    for r in matrix:
        for v in r:
            _need(v >= 0 and float(v) == int(v), "the matrix entries must be whole counts")
    N = math.fsum(math.fsum(r) for r in matrix)
    _need(N > 0, "the matrix must contain samples")
    rows = [math.fsum(r) for r in matrix]
    cols = [math.fsum(matrix[i][j] for i in range(k)) for j in range(k)]
    _need(all(c > 0 for c in cols) and all(r > 0 for r in rows), "every class needs reference and map samples")
    po = math.fsum(matrix[i][i] for i in range(k)) / N
    pe = math.fsum(rows[i] * cols[i] for i in range(k)) / (N * N)
    _need(pe < 1, "kappa is undefined when chance agreement is 1")
    return {"overall": _r(po), "kappa": _r((po - pe) / (1 - pe)), "producers": _r([matrix[i][i] / cols[i] for i in range(k)]),
            "users": _r([matrix[i][i] / rows[i] for i in range(k)])}


def _ref_accuracy(matrix):
    import numpy as np
    from sklearn.metrics import accuracy_score, cohen_kappa_score, precision_score, recall_score
    yt, yp = [], []
    for i, r in enumerate(matrix):
        for j, c in enumerate(r):
            yp += [i] * int(c)
            yt += [j] * int(c)
    lab = list(range(len(matrix)))
    return {"overall": accuracy_score(yt, yp), "kappa": cohen_kappa_score(yt, yp, labels=lab),
            "producers": [float(v) for v in recall_score(yt, yp, labels=lab, average=None)], "users": [float(v) for v in precision_score(yt, yp, labels=lab, average=None)]}


def _rand_cm(rng):
    k = rng.randint(2, 6)
    M = [[float(rng.randint(20, 120) if i == j else rng.randint(0, 25)) for j in range(k)] for i in range(k)]
    return {"matrix": M}


_add("geo_classification_accuracy",
     name="Thematic map accuracy: overall accuracy, kappa, producer's and user's accuracy", topic=_RS,
     purpose="The accuracy assessment of a classified map from its error (confusion) matrix: overall accuracy, Cohen's kappa, and the producer's and user's accuracy of each class.",
     inputs=[_matin("matrix", "confusion matrix of sample counts: rows = map (classified) classes, columns = reference classes")],
     outputs=[_o("overall", "number", "overall accuracy"), _o("kappa", "number", "Cohen's kappa"),
              _o("producers", "number", "producer's accuracy of each class (1 - omission error)"), _o("users", "number", "user's accuracy of each class (1 - commission error)")],
     primary="kappa", conditions=["square matrix of whole counts", "every class sampled in the map and the reference"],
     method="overall = trace/N; kappa = (po - pe)/(1 - pe) with pe from the row and column totals; producer's = diagonal/column total, user's = diagonal/row total",
     latex=r"OA = \frac{\sum_i n_{ii}}{N},\quad \kappa = \frac{p_o - p_e}{1 - p_e},\quad PA_j = \frac{n_{jj}}{n_{+j}},\quad UA_i = \frac{n_{ii}}{n_{i+}}",
     run=classification_accuracy, reference=_ref_accuracy, rand=_rand_cm,
     wrong=lambda matrix: (lambda r: dict(r, producers=r["users"], users=r["producers"], kappa=r["overall"]))(classification_accuracy(matrix)),
     phrases=("Classification accuracy", "Accuracy assessment", "Confusion matrix accuracy"),
     examples=[_ex({"matrix": [[65, 4, 22, 24], [6, 81, 5, 8], [0, 11, 85, 19], [4, 7, 3, 90]]}), _ex({"matrix": [[45, 5], [10, 40]]})],
     level="intermediate")


# ================================================================================================ TERRAIN
_TER = "terrain analysis"
_KU["temperature"] = "K"


def _window(w):
    _need(len(w) == 3 and all(len(r) == 3 for r in w), "the elevation window must be 3 x 3 (row 0 to the north)")
    for r in w:
        for v in r:
            _fin("each elevation", v)


def _horn(w, L):
    dzdx = ((w[0][2] + 2 * w[1][2] + w[2][2]) - (w[0][0] + 2 * w[1][0] + w[2][0])) / (8 * L)
    dzdy = ((w[0][0] + 2 * w[0][1] + w[0][2]) - (w[2][0] + 2 * w[2][1] + w[2][2])) / (8 * L)
    return dzdx, dzdy


def slope_aspect(window, cell_size):
    _window(window)
    _pos("the cell size", cell_size)
    p, q = _horn(window, cell_size)
    g = math.hypot(p, q)
    aspect = None if g == 0 else _az(math.atan2(-p, -q) / DEG)
    return _out(slope_deg=math.atan(g) / DEG, slope_percent=100 * g, aspect_deg=aspect)


def _lsq_grad(window, L):
    """Gradient at the centre of the least-squares quadratic surface through the 9 cells (x east, y north)."""
    import numpy as np
    X, Z = [], []
    for r in range(3):
        for c in range(3):
            x, y = (c - 1) * L, (1 - r) * L
            X.append([1, x, y, x * x, x * y, y * y])
            Z.append(window[r][c])
    b, *_ = np.linalg.lstsq(np.array(X, dtype=float), np.array(Z, dtype=float), rcond=None)
    return float(b[1]), float(b[2])


def _ref_slope_aspect(window, cell_size):
    p, q = _lsq_grad(window, cell_size)
    g = math.hypot(p, q)
    return {"slope_deg": math.degrees(math.atan(g)), "slope_percent": 100 * g, "aspect_deg": None if g == 0 else (math.degrees(math.atan2(-p, -q)) % 360.0)}


def _rand_quadratic_window(rng):
    for _ in range(100):
        L = rng.choice([1.0, 5.0, 10.0, 30.0, round(rng.uniform(1, 100), 2)])
        c = [rng.uniform(100, 2000), rng.uniform(-1, 1), rng.uniform(-1, 1), rng.uniform(-0.01, 0.01) / L, rng.uniform(-0.01, 0.01) / L, rng.uniform(-0.01, 0.01) / L]
        w = [[c[0] + c[1] * x + c[2] * y + c[3] * x * x + c[4] * x * y + c[5] * y * y for x in (-L, 0.0, L)] for y in (L, 0.0, -L)]
        a = slope_aspect(w, L)["aspect_deg"]
        if a is not None and _not_near_north(a):
            return {"window": w, "cell_size": L}
    return {"window": w, "cell_size": L}


_add("geo_slope_aspect_horn",
     name="Slope and aspect of a 3x3 elevation window (Horn's method)", topic=_TER,
     purpose="The slope (degrees and per cent) and aspect (downslope direction, degrees clockwise from north) at the centre cell of a 3 x 3 window of a digital elevation model, by Horn's third-order finite differences.",
     inputs=[_matin("window", "3 x 3 elevations in metres, row 0 to the north, column 0 to the west"), _phys("cell_size", "length", "cell size (square cells)")],
     outputs=[_o("slope_deg", "number", "slope (degrees)"), _o("slope_percent", "number", "slope (per cent)"),
              _o("aspect_deg", "number", "aspect: downslope direction clockwise from north (none for a flat window)")],
     primary="slope_deg", conditions=["square cells", "row 0 is the northern row"],
     method="Horn (1981): dz/dx = ((c + 2f + i) - (a + 2d + g))/(8 dx), dz/dy = ((a + 2b + c) - (g + 2h + i))/(8 dy) with y north; slope = atan(|grad|), aspect = atan2(-dz/dx, -dz/dy)",
     latex=r"\frac{\partial z}{\partial x} = \frac{(z_3 + 2z_6 + z_9) - (z_1 + 2z_4 + z_7)}{8\Delta},\ \frac{\partial z}{\partial y} = \frac{(z_1 + 2z_2 + z_3) - (z_7 + 2z_8 + z_9)}{8\Delta},\ S = \arctan\lVert\nabla z\rVert",
     run=slope_aspect, reference=_ref_slope_aspect, rand=_rand_quadratic_window,
     wrong=lambda window, cell_size: (lambda p, q: _out(slope_deg=math.atan(math.hypot(p, q) * 2) / DEG, slope_percent=200 * math.hypot(p, q), aspect_deg=_az(math.atan2(p, -q) / DEG)))(*_horn(window, cell_size)),
     phrases=("Slope and aspect", "Horn slope and aspect", "Terrain slope and aspect"),
     examples=[_ex({"window": [[37.2, 42, 47.2], [45.2, 50, 55.2], [53.2, 58, 63.2]], "cell_size": "10 m"}), _ex({"window": [[100, 101, 102], [100, 101, 102], [100, 101, 102]], "cell_size": "10 m"})],
     level="intermediate", weak="the gradient of a least-squares quadratic surface (numpy) agrees with Horn's differences on quadratic terrain only, so random windows are drawn from quadratic surfaces")


def hillshade(window, cell_size, sun_azimuth, sun_altitude):
    _window(window)
    _pos("the cell size", cell_size)
    _fin("the sun azimuth and altitude", sun_azimuth, sun_altitude)
    _need(0 <= sun_altitude <= 90, "the sun altitude must lie between 0 and 90 degrees")
    p, q = _horn(window, cell_size)
    s = math.atan(math.hypot(p, q))
    zen = (90 - sun_altitude) * DEG
    if p == 0 and q == 0:
        ci = math.cos(zen)
    else:
        asp = math.atan2(-p, -q)
        ci = math.cos(zen) * math.cos(s) + math.sin(zen) * math.sin(s) * math.cos(sun_azimuth * DEG - asp)
    return _out(illumination=ci, hillshade=255 * max(0.0, ci))


def _ref_hillshade(window, cell_size, sun_azimuth, sun_altitude):
    import numpy as np
    p, q = _lsq_grad(window, cell_size)
    n = np.array([-p, -q, 1.0]) / math.sqrt(p * p + q * q + 1)
    a, e = math.radians(sun_azimuth), math.radians(sun_altitude)
    sv = np.array([math.sin(a) * math.cos(e), math.cos(a) * math.cos(e), math.sin(e)])
    ci = float(n @ sv)
    return {"illumination": ci, "hillshade": 255 * max(0.0, ci)}


def _rand_hill(rng):
    for _ in range(100):
        x = dict(_rand_quadratic_window(rng), sun_azimuth=round(rng.uniform(0, 360), 2), sun_altitude=round(rng.uniform(10, 80), 2))
        if hillshade(**x)["illumination"] > 0.01:
            return x
    return x


_add("geo_hillshade",
     name="Hillshade (analytical relief shading) of a 3x3 elevation window", topic=_TER,
     purpose="The illumination (cosine of the solar incidence angle on the terrain) and the 0-255 hillshade value at the centre of a 3 x 3 elevation window for a sun azimuth and altitude.",
     inputs=[_matin("window", "3 x 3 elevations in metres, row 0 to the north"), _phys("cell_size", "length", "cell size"),
             _degin("sun_azimuth", "sun azimuth, clockwise from north"), _degin("sun_altitude", "sun altitude above the horizon")],
     outputs=[_o("illumination", "number", "cos(incidence angle) (negative in self-shadow)"), _o("hillshade", "number", "255 x max(0, illumination)")],
     primary="hillshade", conditions=["row 0 is the northern row", "no cast shadows"],
     method="Horn slope and aspect, then cos i = cos(Zenith) cos(Slope) + sin(Zenith) sin(Slope) cos(Azimuth - Aspect)",
     latex=r"\cos i = \cos Z\cos S + \sin Z\sin S\cos(A - \alpha),\quad H = 255\max(0, \cos i)",
     run=hillshade, reference=_ref_hillshade, rand=_rand_hill,
     wrong=lambda window, cell_size, sun_azimuth, sun_altitude: hillshade(window, cell_size, sun_azimuth, 90 - sun_altitude),
     phrases=("Hillshade", "Hill shading", "Analytical hillshade"),
     examples=[_ex({"window": [[2452.25, 2463, 2473.75], [2450, 2461, 2472], [2447.75, 2459, 2470.25]], "cell_size": "5 m", "sun_azimuth": 315, "sun_altitude": 45}),
               _ex({"window": [[100, 101, 102], [100, 101, 102], [100, 101, 102]], "cell_size": "10 m", "sun_azimuth": 135, "sun_altitude": 30})],
     level="intermediate", weak="vector geometry (surface normal from a least-squares quadratic surface dotted with the sun vector) on quadratic terrain")


def curvature(window, cell_size):
    _window(window)
    _pos("the cell size", cell_size)
    z = [v for r in window for v in r]
    L = cell_size
    D = ((z[3] + z[5]) / 2 - z[4]) / L ** 2
    E = ((z[1] + z[7]) / 2 - z[4]) / L ** 2
    F = (-z[0] + z[2] + z[6] - z[8]) / (4 * L ** 2)
    G = (-z[3] + z[5]) / (2 * L)
    H = (z[1] - z[7]) / (2 * L)
    gg = G * G + H * H
    _need(gg > 0, "profile and plan curvature are undefined on a flat centre (zero gradient)")
    return _out(curvature=-2 * (D + E), profile=-2 * (D * G * G + E * H * H + F * G * H) / gg, plan=2 * (D * H * H + E * G * G - F * G * H) / gg)


def _ref_curvature(window, cell_size):
    """Exact biquadratic surface through the 9 cells (numpy solve), differentiated at the centre."""
    import numpy as np
    L = cell_size
    X, Z = [], []
    for r in range(3):
        for c in range(3):
            x, y = (c - 1) * L, (1 - r) * L
            X.append([x * x * y * y, x * x * y, x * y * y, x * x, y * y, x * y, x, y, 1])
            Z.append(window[r][c])
    A, B, C, D, E, F, G, H, I = np.linalg.solve(np.array(X, dtype=float), np.array(Z, dtype=float))
    zxx, zyy, zxy, zx, zy = 2 * D, 2 * E, F, G, H
    gg = zx * zx + zy * zy
    return {"curvature": -(zxx + zyy), "profile": -(zxx * zx * zx + zyy * zy * zy + 2 * zxy * zx * zy) / gg,
            "plan": (zxx * zy * zy + zyy * zx * zx - 2 * zxy * zx * zy) / gg}


def _rand_window(rng):
    L = rng.choice([1.0, 5.0, 10.0, 30.0])
    base = rng.uniform(100, 2000)
    tx, ty = rng.uniform(-0.5, 0.5), rng.uniform(-0.5, 0.5)
    return {"window": [[round(base + tx * x + ty * y + rng.gauss(0, 0.3 * L), 2) for x in (-L, 0, L)] for y in (L, 0, -L)], "cell_size": L}


_add("geo_terrain_curvature",
     name="Terrain curvature: total, profile and plan curvature (Zevenbergen-Thorne)", topic=_TER,
     purpose="The total, profile (along the slope) and plan (across the slope) curvature at the centre of a 3 x 3 elevation window from the Zevenbergen-Thorne partial quartic surface.",
     inputs=[_matin("window", "3 x 3 elevations in metres, row 0 to the north"), _phys("cell_size", "length", "cell size")],
     outputs=[_o("curvature", "number", "total curvature -2(D + E) (1/m; positive upwardly convex)"), _o("profile", "number", "profile curvature (1/m)"),
              _o("plan", "number", "plan curvature (1/m)")],
     primary="curvature", conditions=["non-zero gradient at the centre for profile and plan curvature"],
     method="Zevenbergen and Thorne (1987): D, E, F, G, H from the window; profile = -2(DG^2 + EH^2 + FGH)/(G^2 + H^2), plan = 2(DH^2 + EG^2 - FGH)/(G^2 + H^2) (sign convention of Moore et al. 1991 and ArcGIS, without the factor 100)",
     latex=r"K = -2(D+E),\quad K_{prof} = \frac{-2(DG^2 + EH^2 + FGH)}{G^2 + H^2},\quad K_{plan} = \frac{2(DH^2 + EG^2 - FGH)}{G^2 + H^2}",
     run=curvature, reference=_ref_curvature, rand=_rand_window,
     wrong=lambda window, cell_size: (lambda r: dict(r, profile=r["plan"], plan=r["profile"], curvature=-r["curvature"]))(curvature(window, cell_size)),
     phrases=("Terrain curvature", "Profile and plan curvature", "Surface curvature of a DEM"),
     examples=[_ex({"window": [[10, 12, 14], [9, 10, 13], [7, 9, 11]], "cell_size": "1 m"}), _ex({"window": [[105, 100, 98], [104, 101, 97], [106, 103, 99]], "cell_size": "10 m"})],
     level="advanced", weak="the exact biquadratic interpolation solved by numpy is the same surface the formulas encode")


def terrain_ruggedness(window):
    _window(window)
    c = window[1][1]
    nb = [window[r][k] for r in range(3) for k in range(3) if (r, k) != (1, 1)]
    return _out(TRI=math.sqrt(math.fsum((v - c) ** 2 for v in nb)), TPI=c - math.fsum(nb) / 8, relief=max(nb + [c]) - min(nb + [c]))


def _ref_tri(window):
    import numpy as np
    w = np.array(window, dtype=float)
    d = w - w[1, 1]
    mask = np.ones((3, 3), bool)
    mask[1, 1] = False
    return {"TRI": float(np.sqrt((d[mask] ** 2).sum())), "TPI": float(w[1, 1] - w[mask].mean()), "relief": float(w.max() - w.min())}


_add("geo_terrain_ruggedness",
     name="Terrain ruggedness index (Riley), topographic position index and local relief", topic=_TER,
     purpose="Riley's terrain ruggedness index (root of the summed squared elevation differences to the 8 neighbours), the topographic position index (centre minus neighbour mean) and the local relief of a 3 x 3 window.",
     inputs=[_matin("window", "3 x 3 elevations in metres")],
     outputs=[_o("TRI", "number", "terrain ruggedness index (m)"), _o("TPI", "number", "topographic position index (m)"), _o("relief", "number", "maximum minus minimum elevation (m)")],
     primary="TRI", conditions=["Riley et al. (1999) form of the TRI"],
     method="TRI = sqrt(sum (z_i - z_0)^2); TPI = z_0 - mean(z_i); relief = max - min",
     latex=r"TRI = \sqrt{\sum_{i=1}^{8}(z_i - z_0)^2},\quad TPI = z_0 - \frac18\sum_{i=1}^{8}z_i",
     run=terrain_ruggedness, reference=_ref_tri, rand=lambda rng: {"window": _rand_window(rng)["window"]},
     wrong=lambda window: (lambda r: dict(r, TRI=r["TRI"] ** 2 / 8, TPI=-r["TPI"]))(terrain_ruggedness(window)),
     phrases=("Terrain ruggedness index", "TRI and TPI", "Topographic position index"),
     examples=[_ex({"window": [[10, 12, 14], [9, 10, 13], [7, 9, 11]]}), _ex({"window": [[105, 100, 98], [104, 101, 97], [106, 103, 99]]})],
     level="beginner", weak="numpy evaluates the same definitions")


_D8 = [(0, 1, 1), (1, 1, 2), (1, 0, 4), (1, -1, 8), (0, -1, 16), (-1, -1, 32), (-1, 0, 64), (-1, 1, 128)]


def _dem(dem):
    r = len(dem)
    _need(r >= 2 and all(len(row) == len(dem[0]) for row in dem) and len(dem[0]) >= 2, "the elevation raster must be a rectangular matrix of at least 2 x 2")
    _need(r * len(dem[0]) <= 900, "the raster may have at most 900 cells")
    for row in dem:
        for v in row:
            _fin("each elevation", v)
    return r, len(dem[0])


def _d8(dem, cell_size):
    R, C = _dem(dem)
    _pos("the cell size", cell_size)
    out = [[0.0] * C for _ in range(R)]
    for i in range(R):
        for j in range(C):
            best, code = 0.0, 0
            for di, dj, cd in _D8:
                a, b = i + di, j + dj
                if 0 <= a < R and 0 <= b < C:
                    drop = (dem[i][j] - dem[a][b]) / (cell_size * (math.sqrt(2) if di and dj else 1.0))
                    if drop > best:
                        best, code = drop, cd
            out[i][j] = float(code)
    return out


def d8_flow_direction(dem, cell_size):
    return {"direction": _d8(dem, cell_size)}


def _ref_d8(dem, cell_size):
    import numpy as np
    Z = np.array(dem, dtype=float)
    R, C = Z.shape
    P = np.pad(Z, 1, constant_values=np.inf)
    drops = []
    for di, dj, cd in _D8:
        nb = P[1 + di:1 + di + R, 1 + dj:1 + dj + C]
        drops.append((Z - nb) / (cell_size * (np.sqrt(2) if di and dj else 1.0)))
    D = np.stack(drops)
    k = np.argmax(D, axis=0)
    codes = np.array([c for _, _, c in _D8], dtype=float)[k]
    codes[D.max(axis=0) <= 0] = 0
    return {"direction": codes.tolist()}


def _rand_dem(rng):
    R, C = rng.randint(3, 10), rng.randint(3, 10)
    sx, sy = rng.uniform(-2, 2), rng.uniform(-2, 2)
    cx, cy, a = rng.uniform(0, C), rng.uniform(0, R), rng.uniform(-0.5, 0.5)
    dem = [[round(100 + sx * j + sy * i + a * ((i - cy) ** 2 + (j - cx) ** 2) + rng.uniform(-1, 1), 3) for j in range(C)] for i in range(R)]
    return {"dem": dem, "cell_size": rng.choice([1.0, 10.0, 30.0])}


_add("geo_d8_flow_direction",
     name="D8 flow direction of every cell of a small DEM", topic=_TER,
     purpose="The D8 (steepest-descent) flow direction of every cell of a digital elevation model, as ESRI codes (1 E, 2 SE, 4 S, 8 SW, 16 W, 32 NW, 64 N, 128 NE; 0 where no neighbour inside the raster is lower).",
     inputs=[_matin("dem", "elevations (row 0 to the north), at most 900 cells"), _phys("cell_size", "length", "cell size")],
     outputs=[_o("direction", "number", "D8 code of each cell (matrix)")],
     primary="direction", conditions=["square cells; diagonal distance sqrt(2) cells", "ties go to the first direction in the order E, SE, S, SW, W, NW, N, NE"],
     method="for each cell the neighbour with the largest drop per unit distance (positive drops only)",
     latex=r"d^* = \arg\max_{k}\frac{z_0 - z_k}{\ell_k},\quad \ell_k \in \{\Delta, \sqrt2\Delta\}",
     run=d8_flow_direction, reference=_ref_d8, rand=_rand_dem,
     wrong=lambda dem, cell_size: {"direction": _d8_nodist(dem)},
     phrases=("D8 flow direction", "Flow direction D8"),
     examples=[_ex({"dem": [[78, 72, 69, 71], [74, 67, 56, 49], [69, 53, 44, 37], [64, 58, 55, 22]], "cell_size": "10 m"}),
               _ex({"dem": [[10, 9, 8], [9, 7, 5], [8, 5, 1]], "cell_size": "1 m"})],
     level="advanced", weak="a vectorized numpy evaluation of the same steepest-drop rule")


def _d8_nodist(dem):
    R, C = len(dem), len(dem[0])
    out = [[0.0] * C for _ in range(R)]
    for i in range(R):
        for j in range(C):
            best, code = 0.0, 0
            for di, dj, cd in _D8:
                a, b = i + di, j + dj
                if 0 <= a < R and 0 <= b < C and dem[i][j] - dem[a][b] > best:
                    best, code = dem[i][j] - dem[a][b], cd
            out[i][j] = float(code)
    return out


def _receivers(dirs):
    R, C = len(dirs), len(dirs[0])
    step = {cd: (di, dj) for di, dj, cd in _D8}
    rec = {}
    for i in range(R):
        for j in range(C):
            cd = int(dirs[i][j])
            if cd:
                di, dj = step[cd]
                rec[(i, j)] = (i + di, j + dj)
    return rec


def flow_accumulation(dem, cell_size):
    dirs = _d8(dem, cell_size)
    R, C = len(dem), len(dem[0])
    rec = _receivers(dirs)
    indeg = {(i, j): 0 for i in range(R) for j in range(C)}
    for v in rec.values():
        indeg[v] += 1
    acc = {k: 1 for k in indeg}
    stack = [k for k, d in indeg.items() if d == 0]
    while stack:
        k = stack.pop()
        if k in rec:
            t = rec[k]
            acc[t] += acc[k]
            indeg[t] -= 1
            if indeg[t] == 0:
                stack.append(t)
    return {"accumulation": [[float(acc[(i, j)]) for j in range(C)] for i in range(R)], "max_accumulation": float(max(acc.values()))}


def _ref_flow_acc(dem, cell_size):
    import networkx as nx
    dirs = _ref_d8(dem, cell_size)["direction"]
    G = nx.DiGraph()
    R, C = len(dem), len(dem[0])
    G.add_nodes_from((i, j) for i in range(R) for j in range(C))
    G.add_edges_from(_receivers(dirs).items())
    acc = [[float(len(nx.ancestors(G, (i, j))) + 1) for j in range(C)] for i in range(R)]
    return {"accumulation": acc, "max_accumulation": max(max(r) for r in acc)}


_add("geo_d8_flow_accumulation",
     name="D8 flow accumulation of a small DEM", topic=_TER,
     purpose="The number of cells draining through each cell (the cell itself included) under D8 flow directions, and the largest accumulation (the main outlet).",
     inputs=[_matin("dem", "elevations (row 0 to the north), at most 900 cells"), _phys("cell_size", "length", "cell size")],
     outputs=[_o("accumulation", "number", "contributing cells of each cell, itself included (matrix)"), _o("max_accumulation", "number", "largest accumulation")],
     primary="max_accumulation", conditions=["D8 directions as in geo_d8_flow_direction"],
     method="D8 directions, then accumulation in topological order of the flow graph (Kahn's algorithm)",
     latex=r"A(c) = 1 + \sum_{u\to c}A(u)",
     run=flow_accumulation, reference=_ref_flow_acc, rand=_rand_dem,
     wrong=lambda dem, cell_size: (lambda r: {"accumulation": [[v - 1 for v in row] for row in r["accumulation"]], "max_accumulation": r["max_accumulation"] - 1})(flow_accumulation(dem, cell_size)),
     phrases=("Flow accumulation", "D8 flow accumulation"),
     examples=[_ex({"dem": [[78, 72, 69, 71], [74, 67, 56, 49], [69, 53, 44, 37], [64, 58, 55, 22]], "cell_size": "10 m"}),
               _ex({"dem": [[10, 9, 8], [9, 7, 5], [8, 5, 1]], "cell_size": "1 m"})],
     level="advanced")


def watershed(dem, cell_size, row, col):
    dirs = _d8(dem, cell_size)
    R, C = len(dem), len(dem[0])
    _need(isinstance(row, int) and isinstance(col, int) and 0 <= row < R and 0 <= col < C, "the outlet must be a cell of the raster (row and column from 0)")
    rec = _receivers(dirs)
    up = {}
    for k, v in rec.items():
        up.setdefault(v, []).append(k)
    seen, stack = {(row, col)}, [(row, col)]
    while stack:
        k = stack.pop()
        for u in up.get(k, []):
            if u not in seen:
                seen.add(u)
                stack.append(u)
    mask = [[1.0 if (i, j) in seen else 0.0 for j in range(C)] for i in range(R)]
    return {"cells": float(len(seen)), "area": _r(len(seen) * cell_size * cell_size), "mask": mask}


def _ref_watershed(dem, cell_size, row, col):
    import networkx as nx
    dirs = _ref_d8(dem, cell_size)["direction"]
    G = nx.DiGraph()
    R, C = len(dem), len(dem[0])
    G.add_nodes_from((i, j) for i in range(R) for j in range(C))
    G.add_edges_from(_receivers(dirs).items())
    S = nx.ancestors(G, (row, col)) | {(row, col)}
    return {"cells": float(len(S)), "area": len(S) * cell_size ** 2, "mask": [[1.0 if (i, j) in S else 0.0 for j in range(C)] for i in range(R)]}


def _rand_ws(rng):
    x = _rand_dem(rng)
    acc = flow_accumulation(**x)["accumulation"]
    cells = sorted(((acc[i][j], i, j) for i in range(len(acc)) for j in range(len(acc[0]))), reverse=True)
    _, i, j = rng.choice(cells[:max(1, len(cells) // 3)])
    return dict(x, row=i, col=j)


_add("geo_watershed_delineation",
     name="Watershed (catchment) of an outlet cell on a small DEM", topic=_TER,
     purpose="The cells that drain to a chosen outlet cell under D8 flow directions: their number, the catchment area and the catchment mask.",
     inputs=[_matin("dem", "elevations (row 0 to the north), at most 900 cells"), _phys("cell_size", "length", "cell size"),
             _intin("row", "outlet row (from 0, north)", 0, 899), _intin("col", "outlet column (from 0, west)", 0, 899)],
     outputs=[_o("cells", "number", "number of cells in the catchment (outlet included)"), _o("area", "area", "catchment area"),
              _o("mask", "number", "1 for cells in the catchment, 0 elsewhere (matrix)")],
     primary="cells", conditions=["D8 directions as in geo_d8_flow_direction"],
     method="D8 directions, then a search upstream from the outlet through the cells that flow into it",
     latex=r"W(o) = \{c : c \to \cdots \to o\}",
     run=watershed, reference=_ref_watershed, rand=_rand_ws,
     wrong=lambda dem, cell_size, row, col: (lambda r: dict(r, cells=r["cells"] - 1, area=(r["cells"] - 1) * cell_size ** 2))(watershed(dem, cell_size, row, col)),
     phrases=("Watershed delineation", "Catchment of an outlet", "Watershed of a cell"),
     examples=[_ex({"dem": [[78, 72, 69, 71], [74, 67, 56, 49], [69, 53, 44, 37], [64, 58, 55, 22]], "cell_size": "10 m", "row": 3, "col": 3}),
               _ex({"dem": [[10, 9, 8], [9, 7, 5], [8, 5, 1]], "cell_size": "1 m", "row": 2, "col": 2})],
     level="advanced")


R_EARTH_LOS = 6371000.0
K_REFRACTION = 0.13


def line_of_sight(d, z, observer_height, target_height, curvature):
    n = len(d)
    _need(n == len(z) and n >= 2, "give one elevation per distance (at least 2 points)")
    _need(d[0] == 0 and all(b > a for a, b in zip(d, d[1:])), "distances must start at 0 (the observer) and increase")
    for v in z:
        _fin("each elevation", v)
    _pos("the observer and target heights", observer_height, target_height)
    _need(curvature in ("none", "earth_refraction"), "curvature must be none or earth_refraction")
    zc = [v - (t * t * (1 - K_REFRACTION) / (2 * R_EARTH_LOS) if curvature == "earth_refraction" else 0.0) for v, t in zip(z, d)]
    eye = zc[0] + observer_height
    vis = [1.0]
    for i in range(1, n):
        tip = zc[i] + target_height
        ok = True
        for j in range(1, i):
            if zc[j] >= eye + (tip - eye) * d[j] / d[i]:
                ok = False
                break
        vis.append(1.0 if ok else 0.0)
    return {"visible": vis, "n_visible": float(sum(vis) - 1)}


def _ref_los(d, z, observer_height, target_height, curvature):
    import shapely
    zc = [v - (t * t * (1 - K_REFRACTION) / (2 * R_EARTH_LOS) if curvature == "earth_refraction" else 0.0) for v, t in zip(z, d)]
    base = min(zc) - 1000.0
    terrain = shapely.Polygon([(d[0], base)] + list(zip(d, zc)) + [(d[-1], base)])
    vis = [1.0]
    for i in range(1, len(d)):
        line = shapely.LineString([(d[0], zc[0] + observer_height), (d[i], zc[i] + target_height)])
        vis.append(0.0 if line.intersects(terrain) else 1.0)
    return {"visible": vis, "n_visible": float(sum(vis) - 1)}


def _rand_los(rng):
    for _ in range(50):
        n = rng.randint(5, 40)
        step = _lu(rng, 10, 2000)
        d = [round(step * i, 2) for i in range(n)]
        z, h = [], rng.uniform(100, 1000)
        for i in range(n):
            h += rng.gauss(0, 0.05 * step)
            z.append(round(h, 2))
        x = {"d": d, "z": z, "observer_height": round(rng.uniform(1, 30), 2), "target_height": round(rng.uniform(1, 30), 2), "curvature": rng.choice(["none", "earth_refraction"])}
        zc = [v - (t * t * (1 - K_REFRACTION) / (2 * R_EARTH_LOS) if x["curvature"] == "earth_refraction" else 0.0) for v, t in zip(z, d)]
        eye = zc[0] + x["observer_height"]
        margin = min(abs(zc[j] - (eye + (zc[i] + x["target_height"] - eye) * d[j] / d[i])) for i in range(2, n) for j in range(1, i))
        if margin > 1e-6:
            return x
    return x


_add("geo_line_of_sight_profile",
     name="Line-of-sight visibility along a terrain profile (with Earth curvature and refraction)", topic=_TER,
     purpose="Which points along a terrain profile are visible from an observer at its start, for given observer and target heights above the ground, optionally with the Earth-curvature and refraction correction.",
     inputs=[_vecin("d", "distances along the profile in metres, starting at 0 (the observer)", 2, 5000), _vecin("z", "ground elevations in metres", 2, 5000),
             _phys("observer_height", "length", "eye height above the ground"), _phys("target_height", "length", "target height above the ground"),
             _choice("curvature", ["none", "earth_refraction"], "none, or the drop d^2 (1 - k)/(2R) with R = 6371 km and k = 0.13")],
     outputs=[_o("visible", "number", "1 if the target over each point is visible, else 0 (the observer's own point is 1)"), _o("n_visible", "number", "number of visible points beyond the observer")],
     primary="n_visible", conditions=["distances increase from 0", "positive observer and target heights"],
     method="a target is visible when every intermediate ground point lies strictly below the sight line from the eye to the target",
     latex=r"\text{visible}(i) \iff z_j < z_{eye} + (z_i + h_t - z_{eye})\frac{d_j}{d_i}\ \forall\, 0 < j < i,\quad \Delta z_{curv} = \frac{d^2(1-k)}{2R}",
     run=line_of_sight, reference=_ref_los, rand=_rand_los,
     wrong=lambda d, z, observer_height, target_height, curvature: line_of_sight(d, z, observer_height, target_height, "earth_refraction" if curvature == "none" else "none") if False else
         (lambda r: {"visible": [1.0] * len(d), "n_visible": float(len(d) - 1)} if r["n_visible"] < len(d) - 1 else {"visible": [1.0] + [0.0] * (len(d) - 1), "n_visible": 0.0})(line_of_sight(d, z, observer_height, target_height, curvature)),
     phrases=("Line of sight", "Line-of-sight profile", "Viewshed along a profile"),
     examples=[_ex({"d": [0, 100, 200, 300, 400, 500], "z": [100, 104, 112, 103, 101, 109], "observer_height": "2 m", "target_height": "2 m", "curvature": "none"}),
               _ex({"d": [0, 5000, 10000, 15000, 20000], "z": [50, 52, 49, 51, 55], "observer_height": "10 m", "target_height": "10 m", "curvature": "earth_refraction"})],
     level="advanced")


# ================================================================================================ NETWORK AND URBAN ANALYSIS
_NET = "network accessibility"
_LOC = "facility location"
_URB = "urban geography"


def _node(v, name="a node"):
    s = str(v).strip()
    _need(re.fullmatch(r"\d+", s) is not None, f"{name} must be a whole number (nodes are numbered 0 .. n-1)")
    return int(s)


def _net(edges, n=None, weighted=True):
    """Adjacency of an undirected network with nodes 0 .. n-1 (n from the edges when not given)."""
    _need(len(edges) >= 1, "give at least one edge")
    E = []
    for e in edges:
        u, v = _node(e[0]), _node(e[1])
        if weighted:
            _need(len(e) == 3, "every edge needs a length ('0-1 4.5')")
            w = float(e[2])
            _need(math.isfinite(w) and w > 0, "edge lengths must be positive")
        else:
            w = float(e[2]) if len(e) == 3 else 1.0
        _need(u != v, "an edge must join two different nodes")
        E.append((u, v, w))
    m = max(max(u, v) for u, v, _ in E) + 1
    if n is None:
        n = m
    _need(m <= n, f"the edges use node {m - 1}, but only nodes 0 .. {n - 1} exist")
    _need(n <= 500, "the network may have at most 500 nodes")
    adj = [[] for _ in range(n)]
    for u, v, w in E:
        adj[u].append((v, w))
        adj[v].append((u, w))
    return adj


def _dijkstra(adj, s):
    dist = [math.inf] * len(adj)
    dist[s] = 0.0
    pq = [(0.0, s)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue
        for v, w in adj[u]:
            nd = d + w
            if nd < dist[v]:
                dist[v] = nd
                heapq.heappush(pq, (nd, v))
    return dist


def _nx_graph(edges, n=None):
    import networkx as nx
    G = nx.Graph()
    if n is not None:
        G.add_nodes_from(range(n))
    for e in edges:
        u, v = int(e[0]), int(e[1])
        w = float(e[2]) if len(e) == 3 else 1.0
        if G.has_edge(u, v):
            w = min(w, G[u][v]["weight"])
        G.add_edge(u, v, weight=w)
    return G


def _rand_network(rng, n=None, connected=True):
    n = n or rng.randint(5, 14)
    pts = [(rng.uniform(0, 100), rng.uniform(0, 100)) for _ in range(n)]
    edges = set()
    order = list(range(n))
    rng.shuffle(order)
    if connected:
        for k in range(1, n):
            edges.add(tuple(sorted((order[k], order[rng.randrange(k)]))))
    for _ in range(rng.randint(0 if connected else n - 1, n)):
        a, b = rng.sample(range(n), 2)
        edges.add(tuple(sorted((a, b))))
    return [(str(a), str(b), round(math.hypot(pts[a][0] - pts[b][0], pts[a][1] - pts[b][1]) * rng.uniform(1.0, 1.5) + 0.1, 3)) for a, b in sorted(edges)], pts


def gravity_accessibility(edges, opportunities, beta, impedance):
    n = len(opportunities)
    adj = _net(edges, n)
    for v in opportunities:
        _nonneg("each opportunity", v)
    _pos("beta", beta)
    _need(impedance in ("power", "exponential"), "the impedance function must be power or exponential")
    acc = []
    for i in range(n):
        d = _dijkstra(adj, i)
        acc.append(math.fsum(opportunities[j] * (d[j] ** -beta if impedance == "power" else math.exp(-beta * d[j]))
                             for j in range(n) if j != i and math.isfinite(d[j])))
    return {"accessibility": _r(acc)}


def _ref_gravity_acc(edges, opportunities, beta, impedance):
    import networkx as nx
    G = _nx_graph(edges, len(opportunities))
    D = dict(nx.all_pairs_dijkstra_path_length(G))
    f = (lambda x: x ** -beta) if impedance == "power" else (lambda x: math.exp(-beta * x))
    return {"accessibility": [sum(opportunities[j] * f(D[i][j]) for j in D[i] if j != i) for i in range(len(opportunities))]}


def _rand_acc(rng):
    edges, pts = _rand_network(rng)
    imp = rng.choice(["power", "exponential"])
    return {"edges": edges, "opportunities": [float(rng.randint(0, 500)) for _ in pts], "beta": round(rng.uniform(0.5, 2.5), 3) if imp == "power" else round(rng.uniform(0.005, 0.05), 4),
            "impedance": imp}


_add("geo_gravity_accessibility",
     name="Gravity-based (Hansen) accessibility on a road network", topic=_NET,
     purpose="The Hansen gravity accessibility of every node of a network: the sum of the opportunities (jobs, services) at the other nodes discounted by a power or negative-exponential function of the shortest-path distance.",
     inputs=[_edgein(), _vecin("opportunities", "opportunities at nodes 0 .. n-1", 2, 500), _numin("beta", "distance-decay parameter"),
             _choice("impedance", ["power", "exponential"], "impedance function d^-beta or exp(-beta d)")],
     outputs=[_o("accessibility", "number", "accessibility of each node")],
     primary="accessibility", conditions=["positive edge lengths", "a node's own opportunities are excluded; unreachable nodes contribute 0"],
     method="Dijkstra shortest paths from every node; A_i = sum_{j != i} O_j f(d_ij)",
     latex=r"A_i = \sum_{j\ne i} O_j\,f(d_{ij}),\quad f(d) = d^{-\beta}\ \text{or}\ e^{-\beta d}",
     run=gravity_accessibility, reference=_ref_gravity_acc, rand=_rand_acc,
     wrong=lambda edges, opportunities, beta, impedance: gravity_accessibility(edges, opportunities, beta * 2, impedance),
     phrases=("Gravity accessibility", "Hansen accessibility", "Accessibility index"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1", "opportunities": [100, 50, 80, 20], "beta": 1, "impedance": "power"}),
               _ex({"edges": "0-1 5; 1-2 5; 2-3 5; 3-0 5", "opportunities": [10, 20, 30, 40], "beta": 0.1, "impedance": "exponential"})],
     level="intermediate", weak="networkx gives the network distances; the accessibility sum is the Hansen definition")


def huff_model(edges, origin, stores, attractiveness, alpha, beta):
    adj = _net(edges)
    n = len(adj)
    o = int(origin)
    _need(0 <= o < n and float(origin) == o, "the origin must be a node of the network")
    S = [int(s) for s in stores]
    _need(len(S) >= 1 and all(float(a) == b and 0 <= b < n for a, b in zip(stores, S)), "the stores must be nodes of the network")
    _need(len(set(S)) == len(S), "each store node may be listed once")
    _need(len(attractiveness) == len(S), "give one attractiveness per store")
    for v in attractiveness:
        _pos("each attractiveness", v)
    _fin("alpha and beta", alpha, beta)
    _need(beta > 0, "beta must be positive")
    d = _dijkstra(adj, o)
    _need(all(math.isfinite(d[s]) for s in S), "every store must be reachable from the origin")
    _need(all(d[s] > 0 for s in S), "a store at the origin node has zero distance (the model is undefined)")
    u = [attractiveness[k] ** alpha * d[s] ** -beta for k, s in enumerate(S)]
    t = math.fsum(u)
    return {"probabilities": _r([v / t for v in u]), "distances": _r([d[s] for s in S])}


def _ref_huff(edges, origin, stores, attractiveness, alpha, beta):
    import networkx as nx
    G = _nx_graph(edges)
    D = nx.single_source_dijkstra_path_length(G, int(origin))
    u = [a ** alpha / D[int(s)] ** beta for a, s in zip(attractiveness, stores)]
    return {"probabilities": [v / sum(u) for v in u], "distances": [D[int(s)] for s in stores]}


def _rand_huff(rng):
    edges, pts = _rand_network(rng)
    n = len(pts)
    o = rng.randrange(n)
    S = rng.sample([k for k in range(n) if k != o], rng.randint(2, min(5, n - 1)))
    return {"edges": edges, "origin": float(o), "stores": [float(s) for s in S], "attractiveness": [float(rng.randint(500, 20000)) for _ in S],
            "alpha": round(rng.uniform(0.5, 1.5), 3), "beta": round(rng.uniform(1, 3), 3)}


_add("geo_huff_model",
     name="Huff model: probabilities of shopping at each store from an origin (network distances)", topic=_URB,
     purpose="The Huff probabilities that a consumer at an origin node patronizes each store, from store attractiveness (floor space) and shortest-path distances on a road network.",
     inputs=[_edgein(), _numin("origin", "origin node"), _vecin("stores", "store nodes", 1, 100), _vecin("attractiveness", "attractiveness of each store (e.g. floor area)", 1, 100),
             _numin("alpha", "attractiveness exponent"), _numin("beta", "distance-decay exponent")],
     outputs=[_o("probabilities", "number", "probability of each store"), _o("distances", "number", "shortest-path distance to each store")],
     primary="probabilities", conditions=["every store reachable and not at the origin"],
     method="Dijkstra distances from the origin; P_j = S_j^alpha d_j^-beta / sum_k S_k^alpha d_k^-beta",
     latex=r"P_{ij} = \frac{S_j^{\alpha}d_{ij}^{-\beta}}{\sum_k S_k^{\alpha}d_{ik}^{-\beta}}",
     run=huff_model, reference=_ref_huff, rand=_rand_huff,
     wrong=lambda edges, origin, stores, attractiveness, alpha, beta: huff_model(edges, origin, stores, attractiveness, beta, alpha),
     phrases=("Huff model", "Huff probabilities", "Huff retail model"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1; 1-3 5", "origin": 0, "stores": [1, 2, 3], "attractiveness": [5000, 12000, 8000], "alpha": 1, "beta": 2}),
               _ex({"edges": "0-1 1; 1-2 1; 2-3 1; 3-4 1", "origin": 2, "stores": [0, 4], "attractiveness": [100, 300], "alpha": 1, "beta": 1.5})],
     level="intermediate", weak="networkx gives the network distances; the probabilities are the Huff definition")


def two_step_fca(edges, population, supply_nodes, capacity, threshold):
    n = len(population)
    adj = _net(edges, n)
    for v in population:
        _nonneg("each population", v)
    S = [int(s) for s in supply_nodes]
    _need(len(S) >= 1 and all(float(a) == b and 0 <= b < n for a, b in zip(supply_nodes, S)), "the supply sites must be nodes of the network")
    _need(len(set(S)) == len(S), "each supply node may be listed once")
    _need(len(capacity) == len(S), "give one capacity per supply site")
    for v in capacity:
        _nonneg("each capacity", v)
    _pos("the catchment threshold", threshold)
    ratios = []
    dsup = [_dijkstra(adj, s) for s in S]
    for k, s in enumerate(S):
        pop = math.fsum(population[i] for i in range(n) if dsup[k][i] <= threshold)
        ratios.append(capacity[k] / pop if pop > 0 else 0.0)
    acc = [math.fsum(ratios[k] for k in range(len(S)) if dsup[k][i] <= threshold) for i in range(n)]
    return {"accessibility": _r(acc), "ratios": _r(ratios)}


def _ref_2sfca(edges, population, supply_nodes, capacity, threshold):
    import networkx as nx
    G = _nx_graph(edges, len(population))
    R = []
    for s, c in zip(supply_nodes, capacity):
        D = nx.single_source_dijkstra_path_length(G, int(s), cutoff=threshold)
        p = sum(population[i] for i in D)
        R.append(c / p if p > 0 else 0.0)
    A = []
    for i in range(len(population)):
        D = nx.single_source_dijkstra_path_length(G, i, cutoff=threshold)
        A.append(sum(r for s, r in zip(supply_nodes, R) if int(s) in D))
    return {"accessibility": A, "ratios": R}


def _rand_2sfca(rng):
    edges, pts = _rand_network(rng)
    n = len(pts)
    S = rng.sample(range(n), rng.randint(1, min(4, n)))
    x = {"edges": edges, "population": [float(rng.randint(100, 5000)) for _ in range(n)], "supply_nodes": [float(s) for s in S],
         "capacity": [float(rng.randint(1, 20)) for _ in S], "threshold": round(rng.uniform(30, 120), 2)}
    adj = _net(edges, n)
    ds = [d for s in S for d in _dijkstra(adj, s)]
    if all(abs(d - x["threshold"]) > 1e-6 for d in ds if math.isfinite(d)):
        return x
    x["threshold"] += 0.0013
    return x


_add("geo_two_step_floating_catchment",
     name="Two-step floating catchment area (2SFCA) accessibility", topic=_NET,
     purpose="The 2SFCA spatial accessibility of every node to services (doctors, schools): supply-to-population ratios within each site's catchment, summed over the sites within reach of each node.",
     inputs=[_edgein(), _vecin("population", "population at nodes 0 .. n-1", 2, 500), _vecin("supply_nodes", "nodes with a service site", 1, 100),
             _vecin("capacity", "capacity of each site (e.g. physicians)", 1, 100), _numin("threshold", "catchment threshold (network distance)")],
     outputs=[_o("accessibility", "number", "accessibility of each node (supply per person)"), _o("ratios", "number", "supply-to-population ratio of each site")],
     primary="accessibility", conditions=["a catchment includes nodes at network distance <= threshold", "a site with no population in reach has ratio 0"],
     method="step 1: R_j = S_j / sum of population within d0 of j; step 2: A_i = sum of R_j over sites within d0 of i (Luo and Wang 2003)",
     latex=r"R_j = \frac{S_j}{\sum_{k: d_{kj}\le d_0}P_k},\quad A_i = \sum_{j: d_{ij}\le d_0}R_j",
     run=two_step_fca, reference=_ref_2sfca, rand=_rand_2sfca,
     wrong=lambda edges, population, supply_nodes, capacity, threshold: two_step_fca(edges, population, supply_nodes, capacity, threshold / 2),
     phrases=("Two-step floating catchment", "2SFCA accessibility", "Floating catchment area"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1", "population": [1000, 500, 800, 200], "supply_nodes": [1, 3], "capacity": [5, 2], "threshold": 3}),
               _ex({"edges": "0-1 5; 1-2 5; 2-3 5; 3-4 5", "population": [100, 200, 300, 400, 500], "supply_nodes": [0, 4], "capacity": [3, 6], "threshold": 10})],
     level="advanced", weak="networkx gives the catchments (distance cut-offs); the ratios and sums are the 2SFCA definition")


def service_area(edges, source, cutoff):
    adj = _net(edges)
    s = int(source)
    _need(float(source) == s and 0 <= s < len(adj), "the source must be a node of the network")
    _pos("the cutoff", cutoff)
    d = _dijkstra(adj, s)
    R = [i for i in range(len(adj)) if d[i] <= cutoff]
    return {"reachable": [float(i) for i in R], "count": float(len(R)), "farthest": _r(max(d[i] for i in R))}


def _ref_service_area(edges, source, cutoff):
    import networkx as nx
    D = nx.single_source_dijkstra_path_length(_nx_graph(edges), int(source), cutoff=cutoff)
    return {"reachable": [float(i) for i in sorted(D)], "count": float(len(D)), "farthest": max(D.values())}


def _rand_sa(rng):
    edges, pts = _rand_network(rng)
    s = rng.randrange(len(pts))
    d = _dijkstra(_net(edges), s)
    c = round(rng.uniform(20, 150), 2)
    if any(abs(v - c) < 1e-6 for v in d if math.isfinite(v)):
        c += 0.0017
    return {"edges": edges, "source": float(s), "cutoff": c}


_add("geo_service_area",
     name="Network service area (isochrone) of a facility", topic=_NET,
     purpose="The nodes of a road network within a travel-cost cutoff of a facility (the service area or isochrone), their number and the largest distance reached.",
     inputs=[_edgein(), _numin("source", "facility node"), _numin("cutoff", "maximum network distance or travel time")],
     outputs=[_o("reachable", "number", "nodes within the cutoff (ascending)"), _o("count", "number", "number of nodes reached (the facility included)"),
              _o("farthest", "number", "largest network distance within the cutoff")],
     primary="count", conditions=["positive edge costs", "distance <= cutoff counts as reached"],
     method="Dijkstra from the facility, keeping the nodes with distance <= cutoff",
     latex=r"SA(s, c) = \{v : d(s, v) \le c\}",
     run=service_area, reference=_ref_service_area, rand=_rand_sa,
     wrong=lambda edges, source, cutoff: service_area(edges, source, cutoff * 0.6),
     phrases=("Service area", "Network service area", "Isochrone"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1; 3-4 4", "source": 0, "cutoff": 6}), _ex({"edges": "0-1 1; 1-2 1; 2-3 1; 3-4 1", "source": 2, "cutoff": 1.5})],
     level="beginner")


def closest_facility(edges, facilities):
    adj = _net(edges)
    n = len(adj)
    F = [int(f) for f in facilities]
    _need(len(F) >= 1 and all(float(a) == b and 0 <= b < n for a, b in zip(facilities, F)), "the facilities must be nodes of the network")
    _need(len(set(F)) == len(F), "each facility may be listed once")
    D = [_dijkstra(adj, f) for f in F]
    near, dist = [], []
    for i in range(n):
        best = min(range(len(F)), key=lambda k: (D[k][i], F[k]))
        _need(math.isfinite(D[best][i]), "every node must be able to reach a facility")
        others = sorted(D[k][i] for k in range(len(F)))
        _need(len(F) == 1 or others[1] > others[0] or D[best][i] == 0, "two facilities are equally near a node: the nearest is not unique")
        near.append(float(F[best]))
        dist.append(D[best][i])
    return {"nearest": near, "distance": _r(dist), "mean_distance": _r(math.fsum(dist) / n)}


def _ref_closest(edges, facilities):
    import networkx as nx
    G = _nx_graph(edges)
    dist, paths = nx.multi_source_dijkstra(G, {int(f) for f in facilities})
    nodes = sorted(G.nodes)
    d = [dist[i] for i in nodes]
    return {"nearest": [float(paths[i][0]) for i in nodes], "distance": d, "mean_distance": sum(d) / len(d)}


_add("geo_closest_facility",
     name="Closest facility for every node of a network", topic=_LOC,
     purpose="For every node of a road network, the nearest facility by shortest-path distance and that distance, with the mean distance to the nearest facility.",
     inputs=[_edgein(), _vecin("facilities", "facility nodes", 1, 100)],
     outputs=[_o("nearest", "number", "nearest facility of each node"), _o("distance", "number", "distance to it"), _o("mean_distance", "number", "mean distance to the nearest facility")],
     primary="mean_distance", conditions=["every node reaches a facility", "no node equally near two facilities"],
     method="Dijkstra from every facility; each node takes the minimum",
     latex=r"f^*(i) = \arg\min_{f\in F} d(i, f)",
     run=closest_facility, reference=_ref_closest,
     rand=lambda rng: (lambda e, p: {"edges": e, "facilities": [float(f) for f in rng.sample(range(len(p)), rng.randint(1, min(4, len(p))))]})(*_rand_network(rng)),
     wrong=lambda edges, facilities: (lambda r: dict(r, mean_distance=r["mean_distance"] * len(r["distance"]) / max(1, len(r["distance"]) - len(facilities))))(closest_facility(edges, facilities)),
     phrases=("Closest facility", "Nearest facility on a network"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1; 3-4 4", "facilities": [0, 3]}), _ex({"edges": "0-1 1; 1-2 1.5; 2-3 1; 3-4 1", "facilities": [4]})],
     level="intermediate")


def _all_dist(adj):
    return [_dijkstra(adj, i) for i in range(len(adj))]


def p_median(edges, demand, candidates, p):
    n = len(demand)
    adj = _net(edges, n)
    for v in demand:
        _nonneg("each demand", v)
    C = [int(c) for c in candidates]
    _need(len(C) >= 1 and all(float(a) == b and 0 <= b < n for a, b in zip(candidates, C)), "the candidates must be nodes of the network")
    _need(len(set(C)) == len(C), "each candidate may be listed once")
    _need(isinstance(p, int) and 1 <= p <= len(C), "p must be a whole number from 1 to the number of candidates")
    _need(math.comb(len(C), p) <= 20000, "too many candidate combinations (at most 20000)")
    D = _all_dist(adj)
    for i in range(n):
        _need(all(math.isfinite(D[c][i]) for c in C), "the network must be connected")
    best = []
    for S in itertools.combinations(sorted(C), p):
        obj = math.fsum(demand[i] * min(D[s][i] for s in S) for i in range(n))
        best.append((obj, S))
    best.sort()
    _need(len(best) == 1 or best[1][0] - best[0][0] > 1e-9 * max(1.0, best[0][0]), "two facility sets are equally good: the p-median is not unique")
    obj, S = best[0]
    return {"objective": _r(obj), "facilities": [float(s) for s in S], "mean_distance": _r(obj / math.fsum(demand)) if math.fsum(demand) > 0 else 0.0}


def _ref_p_median(edges, demand, candidates, p):
    import networkx as nx
    import numpy as np
    from scipy.optimize import milp, LinearConstraint, Bounds
    n, C = len(demand), [int(c) for c in candidates]
    D = dict(nx.all_pairs_dijkstra_path_length(_nx_graph(edges, n)))
    m = len(C)
    nv = m + n * m
    c = np.zeros(nv)
    for i in range(n):
        for j, f in enumerate(C):
            c[m + i * m + j] = demand[i] * D[i][f]
    A, lo, hi = [], [], []
    for i in range(n):
        row = np.zeros(nv)
        row[m + i * m:m + i * m + m] = 1
        A.append(row); lo.append(1); hi.append(1)
        for j in range(m):
            row = np.zeros(nv)
            row[m + i * m + j] = 1
            row[j] = -1
            A.append(row); lo.append(-np.inf); hi.append(0)
    row = np.zeros(nv)
    row[:m] = 1
    A.append(row); lo.append(p); hi.append(p)
    res = milp(c, constraints=LinearConstraint(np.array(A), lo, hi), integrality=np.r_[np.ones(m), np.zeros(n * m)], bounds=Bounds(0, 1),
               options={"mip_rel_gap": 0.0})
    S = sorted(C[j] for j in range(m) if res.x[j] > 0.5)
    obj = sum(demand[i] * min(D[i][s] for s in S) for i in range(n))
    return {"objective": obj, "facilities": [float(s) for s in S], "mean_distance": obj / sum(demand)}


def _rand_pmed(rng):
    edges, pts = _rand_network(rng, rng.randint(6, 14))
    n = len(pts)
    C = rng.sample(range(n), rng.randint(2, min(8, n)))
    return {"edges": edges, "demand": [round(rng.uniform(1, 100), 2) for _ in range(n)], "candidates": [float(c) for c in C], "p": rng.randint(1, min(3, len(C)))}


_add("geo_p_median",
     name="p-median facility location on a network", topic=_LOC,
     purpose="The p facility sites, chosen from candidate nodes, that minimize the total demand-weighted shortest-path distance from every node to its nearest open facility.",
     inputs=[_edgein(), _vecin("demand", "demand at nodes 0 .. n-1", 2, 200), _vecin("candidates", "candidate facility nodes", 1, 60),
             _intin("p", "number of facilities to open", 1, 60)],
     outputs=[_o("objective", "number", "minimum total weighted distance"), _o("facilities", "number", "chosen facility nodes (ascending)"),
              _o("mean_distance", "number", "demand-weighted mean distance")],
     primary="objective", conditions=["connected network", "at most 20000 candidate combinations", "a unique optimal set"],
     method="exact enumeration of the candidate combinations with all-pairs Dijkstra distances",
     latex=r"\min_{S\subseteq C, |S| = p}\ \sum_i w_i\min_{j\in S} d_{ij}",
     run=p_median, reference=_ref_p_median, rand=_rand_pmed,
     wrong=lambda edges, demand, candidates, p: (lambda r: dict(r, objective=r["objective"] * 1.1))(p_median(edges, [1.0] * len(demand), candidates, p) if len(set(demand)) > 1 else p_median(edges, demand, candidates, p)),
     phrases=("p-median", "p-median location", "P-median facility location"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1; 3-4 4; 4-5 2; 1-5 7", "demand": [10, 20, 15, 30, 5, 25], "candidates": [0, 1, 2, 3, 4, 5], "p": 2}),
               _ex({"edges": "0-1 1; 1-2 1; 2-3 1; 3-4 1; 4-5 1", "demand": [5, 1, 1, 1, 1, 7], "candidates": [0, 2, 3, 5], "p": 1})],
     level="expert")


def max_covering(edges, demand, candidates, p, coverage):
    n = len(demand)
    adj = _net(edges, n)
    for v in demand:
        _nonneg("each demand", v)
    C = [int(c) for c in candidates]
    _need(len(C) >= 1 and all(float(a) == b and 0 <= b < n for a, b in zip(candidates, C)), "the candidates must be nodes of the network")
    _need(len(set(C)) == len(C), "each candidate may be listed once")
    _need(isinstance(p, int) and 1 <= p <= len(C), "p must be a whole number from 1 to the number of candidates")
    _pos("the coverage distance", coverage)
    _need(math.comb(len(C), p) <= 20000, "too many candidate combinations (at most 20000)")
    D = {c: _dijkstra(adj, c) for c in C}
    best = max(math.fsum(demand[i] for i in range(n) if any(D[s][i] <= coverage for s in S)) for S in itertools.combinations(C, p))
    tot = math.fsum(demand)
    return {"covered": _r(best), "fraction": _r(best / tot) if tot > 0 else 0.0}


def _ref_mclp(edges, demand, candidates, p, coverage):
    import networkx as nx
    import numpy as np
    from scipy.optimize import milp, LinearConstraint, Bounds
    n, C = len(demand), [int(c) for c in candidates]
    G = _nx_graph(edges, n)
    m = len(C)
    cov = {j: nx.single_source_dijkstra_path_length(G, f, cutoff=coverage) for j, f in enumerate(C)}
    c = np.r_[np.zeros(m), -np.array(demand, dtype=float)]
    A, lo, hi = [], [], []
    for i in range(n):
        row = np.zeros(m + n)
        row[m + i] = 1
        for j in range(m):
            if i in cov[j]:
                row[j] = -1
        A.append(row); lo.append(-np.inf); hi.append(0)
    row = np.zeros(m + n)
    row[:m] = 1
    A.append(row); lo.append(p); hi.append(p)
    res = milp(c, constraints=LinearConstraint(np.array(A), lo, hi), integrality=np.r_[np.ones(m), np.zeros(n)], bounds=Bounds(0, 1), options={"mip_rel_gap": 0.0})
    S = [j for j in range(m) if res.x[j] > 0.5]
    cv = sum(demand[i] for i in range(n) if any(i in cov[j] for j in S))
    return {"covered": cv, "fraction": cv / sum(demand)}


def _rand_mclp(rng):
    x = _rand_pmed(rng)
    adj = _net(x["edges"], len(x["demand"]))
    s = round(rng.uniform(15, 70), 2)
    ds = [d for c in x["candidates"] for d in _dijkstra(adj, int(c))]
    if any(abs(d - s) < 1e-6 for d in ds):
        s += 0.0019
    return dict(x, coverage=s)


_add("geo_maximal_covering_location",
     name="Maximal covering location problem (MCLP) on a network", topic=_LOC,
     purpose="The largest demand that p facilities, chosen from candidate nodes, can cover within a coverage distance on a road network, and the covered share.",
     inputs=[_edgein(), _vecin("demand", "demand at nodes 0 .. n-1", 2, 200), _vecin("candidates", "candidate facility nodes", 1, 60),
             _intin("p", "number of facilities", 1, 60), _numin("coverage", "coverage distance (network distance <= coverage counts)")],
     outputs=[_o("covered", "number", "maximum covered demand"), _o("fraction", "number", "covered share of the total demand")],
     primary="covered", conditions=["at most 20000 candidate combinations"],
     method="exact enumeration of the candidate combinations (Church and ReVelle 1974 objective)",
     latex=r"\max_{S\subseteq C,|S| = p}\ \sum_i w_i\,\mathbf 1\left[\min_{j\in S}d_{ij}\le s\right]",
     run=max_covering, reference=_ref_mclp, rand=_rand_mclp,
     wrong=lambda edges, demand, candidates, p, coverage: max_covering(edges, demand, candidates, max(1, p - 1), coverage / 2),
     phrases=("Maximal covering location", "MCLP", "Maximum coverage location"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1; 3-4 4; 4-5 2; 1-5 7", "demand": [10, 20, 15, 30, 5, 25], "candidates": [0, 1, 2, 3, 4, 5], "p": 2, "coverage": 3}),
               _ex({"edges": "0-1 1; 1-2 1; 2-3 1; 3-4 1; 4-5 1", "demand": [5, 1, 1, 1, 1, 7], "candidates": [0, 2, 3, 5], "p": 1, "coverage": 2})],
     level="expert")


def detour_index(coords, edges, origin, destination):
    n = len(coords)
    _need(n >= 2 and all(len(r) == 2 for r in coords), "give the x and y of every node (an n x 2 matrix)")
    E = [(e[0], e[1], math.hypot(coords[_node(e[0])][0] - coords[_node(e[1])][0], coords[_node(e[0])][1] - coords[_node(e[1])][1])) for e in edges
         if _node(e[0]) < n and _node(e[1]) < n] if all(_node(e[0]) < n and _node(e[1]) < n for e in edges) else None
    _need(E is not None, "every edge must join nodes that have coordinates")
    adj = _net(E, n)
    o, t = int(origin), int(destination)
    _need(float(origin) == o and float(destination) == t and 0 <= o < n and 0 <= t < n and o != t, "the origin and destination must be two different nodes")
    d = _dijkstra(adj, o)[t]
    _need(math.isfinite(d), "the destination is not reachable from the origin")
    e = math.hypot(coords[o][0] - coords[t][0], coords[o][1] - coords[t][1])
    _need(e > 0, "the origin and destination coincide in space")
    return _out(network_distance=d, euclidean_distance=e, detour_index=d / e)


def _ref_detour(coords, edges, origin, destination):
    import networkx as nx
    import numpy as np
    P = np.array(coords, dtype=float)
    G = nx.Graph()
    for e in edges:
        u, v = int(e[0]), int(e[1])
        G.add_edge(u, v, weight=float(np.linalg.norm(P[u] - P[v])))
    d = nx.dijkstra_path_length(G, int(origin), int(destination))
    e = float(np.linalg.norm(P[int(origin)] - P[int(destination)]))
    return {"network_distance": d, "euclidean_distance": e, "detour_index": d / e}


def _rand_detour(rng):
    edges, pts = _rand_network(rng)
    o, t = rng.sample(range(len(pts)), 2)
    return {"coords": [[round(a, 3), round(b, 3)] for a, b in pts], "edges": [(a, b) for a, b, _ in edges], "origin": float(o), "destination": float(t)}


_add("geo_detour_index",
     name="Network detour index (circuity) between two nodes", topic=_NET,
     purpose="The shortest network distance between two nodes of a street network (edge lengths from the node coordinates), the straight-line distance and their ratio, the detour index or circuity.",
     inputs=[_matin("coords", "x and y of nodes 0 .. n-1 (n x 2, projected coordinates)"),
             _edgein(meaning="street segments between nodes (lengths are computed from the coordinates)"), _numin("origin", "origin node"), _numin("destination", "destination node")],
     outputs=[_o("network_distance", "number", "shortest network distance"), _o("euclidean_distance", "number", "straight-line distance"),
              _o("detour_index", "number", "network / straight-line distance (>= 1)")],
     primary="detour_index", conditions=["straight edges between the node coordinates", "destination reachable"],
     method="Euclidean edge lengths, Dijkstra, ratio to the straight-line distance",
     latex=r"DI = \frac{d_{net}(o, t)}{\lVert\mathbf x_o - \mathbf x_t\rVert}",
     run=detour_index, reference=_ref_detour, rand=_rand_detour,
     wrong=lambda coords, edges, origin, destination: (lambda r: dict(r, detour_index=r["euclidean_distance"] / r["network_distance"]))(detour_index(coords, edges, origin, destination)),
     phrases=("Detour index", "Network circuity", "Route directness"),
     examples=[_ex({"coords": [[0, 0], [3, 0], [3, 4], [0, 4]], "edges": "0-1; 1-2; 2-3", "origin": 0, "destination": 2}),
               _ex({"coords": [[0, 0], [1, 1], [2, 0], [3, 1], [4, 0]], "edges": "0-1; 1-2; 2-3; 3-4; 0-2", "origin": 0, "destination": 4})],
     level="beginner")


def network_indices(edges):
    nodes, E = set(), set()
    for e in edges:
        u, v = _node(e[0]), _node(e[1])
        _need(u != v, "an edge must join two different nodes")
        nodes |= {u, v}
        E.add((min(u, v), max(u, v)))
    vN, eN = len(nodes), len(E)
    _need(vN >= 3, "the network needs at least 3 nodes")
    parent = {x: x for x in nodes}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x
    for u, v in E:
        parent[find(u)] = find(v)
    p = len({find(x) for x in nodes})
    mu = eN - vN + p
    return {"vertices": float(vN), "edges": float(eN), "components": float(p), "cyclomatic": float(mu), "alpha": _r(mu / (2 * vN - 5)),
            "beta": _r(eN / vN), "gamma": _r(eN / (3 * (vN - 2)))}


def _ref_net_indices(edges):
    import networkx as nx
    G = nx.Graph()
    G.add_edges_from((int(e[0]), int(e[1])) for e in edges)
    v, e, p = G.number_of_nodes(), G.number_of_edges(), nx.number_connected_components(G)
    return {"vertices": float(v), "edges": float(e), "components": float(p), "cyclomatic": float(len(nx.cycle_basis(G))), "alpha": (e - v + p) / (2 * v - 5),
            "beta": e / v, "gamma": e / (3 * (v - 2))}


_add("geo_network_connectivity_indices",
     name="Network connectivity indices: cyclomatic number, alpha, beta and gamma", topic=_NET,
     purpose="The graph-theoretic connectivity measures of a (planar) transport network: vertices, edges, components, cyclomatic number and Kansky's alpha, beta and gamma indices.",
     inputs=[_edgein(meaning="network edges between nodes numbered from 0 (lengths, if given, are ignored)")],
     outputs=[_o("vertices", "number", "number of vertices v"), _o("edges", "number", "number of distinct edges e"), _o("components", "number", "connected components p"),
              _o("cyclomatic", "number", "cyclomatic number e - v + p"), _o("alpha", "number", "(e - v + p)/(2v - 5)"), _o("beta", "number", "e / v"), _o("gamma", "number", "e / (3(v - 2))")],
     primary="alpha", conditions=["at least 3 vertices", "planar formulas for alpha and gamma"],
     method="count vertices and distinct edges; components by union-find",
     latex=r"\mu = e - v + p,\quad \alpha = \frac{e - v + p}{2v - 5},\quad \beta = \frac{e}{v},\quad \gamma = \frac{e}{3(v-2)}",
     run=network_indices, reference=_ref_net_indices,
     rand=lambda rng: {"edges": [(a, b) for a, b, _ in _rand_network(rng, connected=rng.random() < 0.7)[0]]},
     wrong=lambda edges: (lambda r: dict(r, alpha=r["cyclomatic"] / (2 * r["vertices"] - 3), gamma=r["edges"] / (r["vertices"] * (r["vertices"] - 1) / 2)))(network_indices(edges)),
     phrases=("Network connectivity indices", "Alpha beta gamma indices", "Graph connectivity indices"),
     examples=[_ex({"edges": "0-1; 1-2; 2-3; 3-0; 0-2"}), _ex({"edges": "0-1; 1-2; 3-4; 4-5; 5-3"})],
     level="beginner", weak="networkx counts the components and the cycle basis; alpha, beta and gamma are then Kansky's definitions")


def shimbel_index(edges):
    adj = _net(edges)
    n = len(adj)
    D = _all_dist(adj)
    _need(all(math.isfinite(v) for r in D for v in r), "the network must be connected (every node numbered 0 .. n-1 must be reachable)")
    S = [math.fsum(r) for r in D]
    return {"shimbel": _r(S), "total": _r(math.fsum(S)), "most_accessible": float(min(range(n), key=lambda i: (S[i], i)))}


def _ref_shimbel(edges):
    import networkx as nx
    G = _nx_graph(edges)
    D = dict(nx.all_pairs_dijkstra_path_length(G))
    nodes = sorted(G.nodes)
    S = [sum(D[i].values()) for i in nodes]
    return {"shimbel": S, "total": sum(S), "most_accessible": float(nodes[int(min(range(len(S)), key=lambda k: (S[k], k)))])}


_add("geo_shimbel_index",
     name="Shimbel accessibility index of every node", topic=_NET,
     purpose="The Shimbel (total shortest-path distance) index of every node of a connected network, the network total (dispersion) and the most accessible node (smallest index).",
     inputs=[_edgein()],
     outputs=[_o("shimbel", "number", "sum of shortest-path distances from each node"), _o("total", "number", "network dispersion (sum over all nodes)"),
              _o("most_accessible", "number", "node with the smallest Shimbel index (lowest number on ties)")],
     primary="total", conditions=["connected network with nodes 0 .. n-1"],
     method="all-pairs Dijkstra; row sums of the distance matrix",
     latex=r"A_i = \sum_j d_{ij},\quad D(G) = \sum_i A_i",
     run=shimbel_index, reference=_ref_shimbel, rand=lambda rng: {"edges": _rand_network(rng)[0]},
     wrong=lambda edges: (lambda r: dict(r, total=r["total"] / 2, shimbel=[v / 2 for v in r["shimbel"]]))(shimbel_index(edges)),
     phrases=("Shimbel index", "Shimbel accessibility"),
     examples=[_ex({"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1"}), _ex({"edges": "0-1 1; 1-2 1; 2-3 1; 3-4 1"})],
     level="intermediate")


# ---------------------------------------------------------------- segregation
def _seg_in(x, t):
    _need(len(x) == len(t) and len(x) >= 2, "give the group count and the total of every areal unit (at least 2 units)")
    for a, b in zip(x, t):
        _nonneg("each count", a)
        _pos("each unit total", b)
        _need(a <= b, "a group count cannot exceed its unit's total")
    X, T = math.fsum(x), math.fsum(t)
    P = X / T
    _need(0 < P < 1, "the group must be present but not the whole population")
    return X, T, P


def segregation_evenness(x, t):
    X, T, P = _seg_in(x, t)
    pi = [a / b for a, b in zip(x, t)]
    D = math.fsum(b * abs(p - P) for b, p in zip(t, pi)) / (2 * T * P * (1 - P))
    G = math.fsum(t[i] * t[j] * abs(pi[i] - pi[j]) for i in range(len(t)) for j in range(len(t))) / (2 * T * T * P * (1 - P))
    ent = lambda p: 0.0 if p in (0.0, 1.0) else -(p * math.log(p) + (1 - p) * math.log(1 - p))
    E = ent(P)
    H = math.fsum(b * (E - ent(p)) for b, p in zip(t, pi)) / (E * T)
    return _out(dissimilarity=D, gini=G, theil_h=H)


def _seg_df(x, t):
    import pandas as pd
    return pd.DataFrame({"x": list(map(float, x)), "t": list(map(float, t))})


def _segmod():
    """Import segregation (it uses numba, which imports a module named `coverage`; the factory's tools/coverage.py is
    hidden while importing, as in _pysal)."""
    import os
    import sys
    mod = sys.modules.get("coverage")
    if mod is not None and not hasattr(mod, "types"):
        del sys.modules["coverage"]
    saved = list(sys.path)
    sys.path[:] = [p for p in sys.path if not os.path.exists(os.path.join(p or ".", "coverage.py"))]
    try:
        import segregation.singlegroup as sg
    finally:
        sys.path[:] = saved
    return sg


def _ref_seg_even(x, t):
    sg = _segmod()
    Dissim, Entropy, GiniSeg = sg.Dissim, sg.Entropy, sg.Gini
    df = _seg_df(x, t)
    return {"dissimilarity": float(Dissim(df, "x", "t").statistic), "gini": float(GiniSeg(df, "x", "t").statistic), "theil_h": float(Entropy(df, "x", "t").statistic)}


def _rand_seg(rng):
    n = rng.randint(3, 30)
    t = [float(rng.randint(200, 5000)) for _ in range(n)]
    x = [float(rng.randint(0, int(b * rng.choice([0.05, 0.3, 0.7])))) for b in t]
    if sum(x) == 0:
        x[0] = 10.0
    return {"x": x, "t": t}


_add("geo_segregation_evenness",
     name="Residential segregation (evenness): dissimilarity, Gini and Theil entropy indices", topic=_URB,
     purpose="The evenness dimension of residential segregation of a group across areal units (census tracts): Duncan's index of dissimilarity, the segregation Gini index and Theil's entropy index H.",
     inputs=[_vecin("x", "group count in each areal unit", 2, 10000), _vecin("t", "total population of each areal unit", 2, 10000)],
     outputs=[_o("dissimilarity", "number", "index of dissimilarity D"), _o("gini", "number", "Gini segregation index"), _o("theil_h", "number", "Theil's information theory index H")],
     primary="dissimilarity", conditions=["0 <= x <= t in each unit", "the group present but not the whole population"],
     method="Massey and Denton (1988): D = sum t_i |p_i - P| / (2TP(1-P)); G = sum_ij t_i t_j |p_i - p_j| / (2T^2P(1-P)); H = sum t_i (E - E_i)/(ET)",
     latex=r"D = \frac{\sum_i t_i|p_i - P|}{2TP(1-P)},\quad G = \frac{\sum_i\sum_j t_it_j|p_i - p_j|}{2T^2P(1-P)},\quad H = \sum_i\frac{t_i(E - E_i)}{ET}",
     run=segregation_evenness, reference=_ref_seg_even, rand=_rand_seg,
     wrong=lambda x, t: (lambda r: dict(r, dissimilarity=r["dissimilarity"] * 2 * (1 - math.fsum(x) / math.fsum(t)), gini=r["gini"] * 2))(segregation_evenness(x, t)),
     phrases=("Segregation evenness", "Dissimilarity index", "Index of dissimilarity"),
     examples=[_ex({"x": [100, 200, 50, 800], "t": [1000, 1000, 1000, 1000]}), _ex({"x": [0, 10, 990], "t": [500, 500, 1000]})],
     level="intermediate", weak="the segregation library (pysal) evaluates the same Massey-Denton definitions")


def segregation_exposure(x, t):
    X, T, P = _seg_in(x, t)
    xPx = math.fsum((a / X) * (a / b) for a, b in zip(x, t))
    xPy = math.fsum((a / X) * ((b - a) / b) for a, b in zip(x, t))
    return _out(isolation=xPx, exposure=xPy, correlation_ratio=(xPx - P) / (1 - P))


def _ref_seg_exp(x, t):
    sg = _segmod()
    CorrelationR, Interaction, Isolation = sg.CorrelationR, sg.Interaction, sg.Isolation
    df = _seg_df(x, t)
    return {"isolation": float(Isolation(df, "x", "t").statistic), "exposure": float(Interaction(df, "x", "t").statistic),
            "correlation_ratio": float(CorrelationR(df, "x", "t").statistic)}


_add("geo_segregation_exposure",
     name="Residential segregation (exposure): isolation, interaction and correlation ratio", topic=_URB,
     purpose="The exposure dimension of residential segregation: the isolation index xPx (probability that a group member meets a member of the same group in their unit), the interaction (exposure) index xPy and the correlation ratio eta^2.",
     inputs=[_vecin("x", "group count in each areal unit", 2, 10000), _vecin("t", "total population of each areal unit", 2, 10000)],
     outputs=[_o("isolation", "number", "isolation index xPx"), _o("exposure", "number", "interaction index xPy"), _o("correlation_ratio", "number", "correlation ratio (xPx - P)/(1 - P)")],
     primary="isolation", conditions=["0 <= x <= t in each unit", "the group present but not the whole population"],
     method="xPx = sum (x_i/X)(x_i/t_i); xPy = sum (x_i/X)(y_i/t_i) with y = t - x; eta^2 = (xPx - P)/(1 - P)",
     latex=r"{}_xP^*_x = \sum_i\frac{x_i}{X}\frac{x_i}{t_i},\quad {}_xP^*_y = \sum_i\frac{x_i}{X}\frac{y_i}{t_i},\quad \eta^2 = \frac{{}_xP^*_x - P}{1-P}",
     run=segregation_exposure, reference=_ref_seg_exp, rand=_rand_seg,
     wrong=lambda x, t: (lambda r: dict(r, isolation=r["isolation"] * 0.5 + 0.1, exposure=1 - r["isolation"] * 0.5))(segregation_exposure(x, t)),
     phrases=("Segregation exposure", "Isolation index", "Exposure and isolation indices"),
     examples=[_ex({"x": [100, 200, 50, 800], "t": [1000, 1000, 1000, 1000]}), _ex({"x": [0, 10, 990], "t": [500, 500, 1000]})],
     level="intermediate", weak="the segregation library (pysal) evaluates the same Massey-Denton definitions")


# ================================================================================================ worked-example outputs
# Each example's outputs, computed by `run` and re-checked against the reference by a build-time script on the box
# (all 203 examples agreed with their references when the block was written, 2026-10-23).
# @@EXV-START@@
_EXV = {'geo_albers_equal_area_forward': [{'h': 1.00911370664, 'k': 0.990968602864, 'x': -338390.587551, 'y': 1894100.14004},
                                   {'h': 1.01155863249, 'k': 0.988573442887, 'x': 299362.220846, 'y': -2706085.97953}],
 'geo_albers_equal_area_inverse': [{'lat': 40.5185245587, 'lon': -100.033951495}, {'lat': -24.9455667384, 'lon': 135.00495475}],
 'geo_auxiliary_latitudes': [{'authalic': 44.8717028734,
                              'conformal': 44.8076840561,
                              'geocentric': 44.807576784,
                              'rectifying': 44.8556819889,
                              'reduced': 44.9037878494},
                             {'authalic': -29.8889970339,
                              'conformal': -29.8336820417,
                              'geocentric': -29.833635809,
                              'rectifying': -29.8751479354,
                              'reduced': -29.9167477128}],
 'geo_azimuthal_equidistant_forward': [{'azimuth': 288.373133069, 'distance': 5585806.31834, 'x': -5301063.81044, 'y': 1760668.82282},
                                       {'azimuth': 54.1174104432, 'distance': 713809.545779, 'x': 578342.615822, 'y': 418382.464223}],
 'geo_bivariate_morans_i': [{'I_xy': 0.891760344576}, {'I_xy': -0.142857142857}],
 'geo_brightness_temperature': [{'T': 306.173902537}, {'T': 293.792580898}],
 'geo_classification_accuracy': [{'kappa': 0.653516270789,
                                  'overall': 0.739631336406,
                                  'producers': [0.866666666667, 0.78640776699, 0.739130434783, 0.63829787234],
                                  'users': [0.565217391304, 0.81, 0.739130434783, 0.865384615385]},
                                 {'kappa': 0.7, 'overall': 0.85, 'producers': [0.818181818182, 0.888888888889], 'users': [0.9, 0.8]}],
 'geo_closest_facility': [{'distance': [0.0, 2.0, 1.0, 0.0, 4.0], 'mean_distance': 1.4, 'nearest': [0.0, 0.0, 3.0, 3.0, 3.0]},
                          {'distance': [4.5, 3.5, 2.0, 1.0, 0.0], 'mean_distance': 2.2, 'nearest': [4.0, 4.0, 4.0, 4.0, 4.0]}],
 'geo_cylindrical_equal_area_forward': [{'x': 786266.866639, 'y': 6371000.0}, {'x': 1.81379936423, 'y': -0.57735026919}],
 'geo_d8_flow_accumulation': [{'accumulation': [[1.0, 1.0, 1.0, 1.0], [1.0, 2.0, 2.0, 3.0], [1.0, 4.0, 8.0, 6.0], [1.0, 1.0, 1.0, 16.0]],
                               'max_accumulation': 16.0},
                              {'accumulation': [[1.0, 1.0, 1.0], [1.0, 2.0, 3.0], [1.0, 3.0, 9.0]], 'max_accumulation': 9.0}],
 'geo_d8_flow_direction': [{'direction': [[2.0, 2.0, 2.0, 4.0], [2.0, 2.0, 2.0, 4.0], [1.0, 1.0, 2.0, 4.0], [128.0, 128.0, 1.0, 0.0]]},
                           {'direction': [[2.0, 2.0, 4.0], [2.0, 2.0, 4.0], [1.0, 1.0, 0.0]]}],
 'geo_decimal_to_dms': [{'degrees': 51, 'hemisphere': 'N', 'minutes': 28, 'seconds': 37.9992},
                        {'degrees': 0, 'hemisphere': 'W', 'minutes': 7, 'seconds': 39.0}],
 'geo_density_gradient': [{'D0': 10546.3650606, 'b': 0.166382733649, 'half_distance': 4.16598023941, 'r_squared': 0.999624012299},
                          {'D0': 14226.4030311, 'b': 0.341065227358, 'half_distance': 2.0323009353, 'r_squared': 0.999907570534}],
 'geo_detour_index': [{'detour_index': 1.4, 'euclidean_distance': 5.0, 'network_distance': 7.0},
                      {'detour_index': 1.20710678119, 'euclidean_distance': 4.0, 'network_distance': 4.82842712475}],
 'geo_discrete_frechet_distance': [{'distance': 1.4142135623730951}, {'distance': 1.4142135623730951}],
 'geo_distance_band_weights': [{'W': [[0.0, 1.0, 0.0, 0.0, 0.0],
                                      [1.0, 0.0, 1.0, 0.0, 0.0],
                                      [0.0, 1.0, 0.0, 0.0, 0.0],
                                      [0.0, 0.0, 0.0, 0.0, 0.0],
                                      [0.0, 0.0, 0.0, 0.0, 0.0]],
                                'islands': 2.0,
                                'neighbours': [1.0, 2.0, 1.0, 0.0, 0.0],
                                's0': 4.0},
                               {'W': [[0.0, 1.0, 1.0, 0.0, 0.0],
                                      [1.0, 0.0, 0.0, 1.0, 0.0],
                                      [1.0, 0.0, 0.0, 1.0, 0.0],
                                      [0.0, 1.0, 1.0, 0.0, 0.0],
                                      [0.0, 0.0, 0.0, 0.0, 0.0]],
                                'islands': 1.0,
                                'neighbours': [2.0, 2.0, 2.0, 2.0, 0.0],
                                's0': 8.0}],
 'geo_dms_to_decimal': [{'decimal': 51.477222222222224, 'radians': 0.898447017559}, {'decimal': -0.1275, 'radians': -0.00222529479629}],
 'geo_douglas_peucker_simplification': [{'kept': [0.0, 2.0, 3.0, 7.0], 'n_kept': 4}, {'kept': [0.0, 1.0, 2.0, 3.0, 4.0, 5.0], 'n_kept': 6}],
 'geo_ecef_to_geodetic': [{'h': 49468.6669929, 'lat': 44.5599648566, 'lon': 10.0000370024},
                          {'h': 278.965934223, 'lat': -33.8652469349, 'lon': 151.214498353}],
 'geo_empirical_semivariogram': [{'counts': [4.0, 3.0, 3.0], 'gamma': [1.875, 1.5, 4.33333333333]},
                                 {'counts': [4.0, 4.0, 2.0], 'gamma': [0.625, 1.25, 2.5]}],
 'geo_enu_to_geodetic': [{'h': 4526.79728174, 'lat': 45.975991297, 'lon': 7.65815026928},
                         {'h': 10.394076664, 'lat': 0.0180873601614, 'lon': 0.00898313868332}],
 'geo_equirectangular_forward': [{'x': 1111949.26645, 'y': 5003771.69901}, {'x': 962977.370511, 'y': -2223900.97635}],
 'geo_gearys_c': [{'C': 0.1953125, 'VC_norm': 0.111111111111, 'VC_rand': 0.132231212798, 'z_norm': -2.4140625, 'z_rand': -2.21289223291},
                  {'C': 1.66666666667, 'VC_norm': 0.0714285714286, 'VC_rand': 0.111111111111, 'z_norm': 2.49443825785, 'z_rand': 2.0}],
 'geo_geodesic_destination': [{'azimuth2': 45.832536554, 'lat2': 52.8308364668, 'lon2': 1.16914595053},
                              {'azimuth2': 252.759395954, 'lat2': -41.1854319442, 'lon2': 130.450314423}],
 'geo_geodesic_midpoint': [{'distance': 5585806.31834, 'lat_mid': 52.3849552356, 'lon_mid': -41.2785904708},
                           {'distance': 7792095.15319, 'lat_mid': 0.912597287725, 'lon_mid': 145.513149737}],
 'geo_geodesic_polygon_area': [{'area': 12308778361.5, 'perimeter': 443770.917248}, {'area': 33669734803.1, 'perimeter': 850957.62133}],
 'geo_geodetic_to_ecef': [{'X': 4449028.15885, 'Y': 784483.702337, 'Z': 4487419.11954},
                          {'X': -4647005.02841, 'Y': 2553096.91367, 'Z': -3533294.98335}],
 'geo_geodetic_to_enu': [{'e': -7134.75719598, 'n': -4556.32151385, 'slant_range': 8933.13814478, 'u': 2852.39042395},
                         {'e': 55665.6600083, 'n': 55295.1768134, 'slant_range': 78463.2931553, 'u': 515.838910843}],
 'geo_geohash_decode': [{'lat_center': 57.64911063015461,
                         'lat_error': 6.705522537231445e-07,
                         'lon_center': 10.407439693808556,
                         'lon_error': 6.705522537231445e-07},
                        {'lat_center': 37.77099609375, 'lat_error': 0.02197265625, 'lon_center': -122.40966796875, 'lon_error': 0.02197265625}],
 'geo_geohash_encode': [{'geohash': 'u4pruydqqvj',
                         'lat_center': 57.64911063015461,
                         'lat_error': 6.705522537231445e-07,
                         'lon_center': 10.407439693808556,
                         'lon_error': 6.705522537231445e-07},
                        {'geohash': 'r3gx2f',
                         'lat_center': -33.86810302734375,
                         'lat_error': 0.00274658203125,
                         'lon_center': 151.2103271484375,
                         'lon_error': 0.0054931640625}],
 'geo_getis_ord_general_g': [{'EG': 0.466666666667, 'G': 0.656814449918, 'VG': 0.00892117237964, 'z_norm': 2.01316931036},
                             {'EG': 0.333333333333, 'G': 0.309197651663, 'VG': 0.00034194449657, 'z_norm': -1.30521410986}],
 'geo_gnomonic_forward': [{'x': 857807.91927, 'y': -508965.779873}, {'x': -0.288675134595, 'y': -0.5}],
 'geo_graticule_cell_area': [{'area': 12308463894.0, 'fraction_of_ellipsoid': 2.41311379747e-05},
                             {'area': 7892218858.04, 'fraction_of_ellipsoid': 1.54729480325e-05}],
 'geo_gravity_accessibility': [{'accessibility': [44.3333333333, 81.6666666667, 56.6666666667, 109.166666667]},
                               {'accessibility': [47.4282228179, 38.9764040354, 40.0706339945, 31.6188152119]}],
 'geo_gravity_model_poisson_fit': [{'alpha': 0.802892254661, 'beta': 0.334750182682, 'gamma': 0.509078509818, 'k': 0.00037095702369},
                                   {'alpha': 0.830605425899, 'beta': 0.345863474825, 'gamma': 0.261762898436, 'k': 0.000287153292619}],
 'geo_ground_sample_distance': [{'gsd': 0.0327272727273, 'swath': 179.083636364}, {'gsd': 7.0, 'swath': 84000.0}],
 'geo_hillshade': [{'hillshade': 167.305141983, 'illumination': 0.656098596012}, {'hillshade': 111.329241383, 'illumination': 0.436585260327}],
 'geo_huff_model': [{'distances': [2.0, 5.0, 6.0], 'probabilities': [0.640295959021, 0.245873648264, 0.113830392715]},
                    {'distances': [2.0, 2.0], 'probabilities': [0.25, 0.75]}],
 'geo_idw_interpolation': [{'estimate': 1.65897994017}, {'estimate': 16.3068312315}],
 'geo_join_counts': [{'BB': 1.0, 'BW': 6.0, 'J': 8.0, 'WW': 1.0}, {'BB': 0.0, 'BW': 4.0, 'J': 4.0, 'WW': 0.0}],
 'geo_kernel_density': [{'density': 0.0267647992836, 'intensity': 0.107059197135}, {'density': 0.092011451475, 'intensity': 0.3680458059}],
 'geo_knn_weights': [{'W': [[0.0, 1.0, 1.0, 0.0, 0.0],
                            [1.0, 0.0, 1.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0, 1.0, 0.0]],
                      's0': 10.0,
                      'symmetric': False},
                     {'W': [[0.0, 1.0, 0.0, 0.0, 0.0],
                            [1.0, 0.0, 0.0, 0.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0, 0.0]],
                      's0': 5.0,
                      'symmetric': False}],
 'geo_lambert_azimuthal_equal_area_forward': [{'x': 3962799.45096, 'y': 2999718.85316}, {'x': -85464.2834458, 'y': -554752.191216}],
 'geo_lambert_conformal_conic_forward': [{'gamma': -2.52198500556, 'k': 0.994666043649, 'x': -339643.77848, 'y': 1904595.04641},
                                         {'gamma': -0.471645047285, 'k': 0.999062989821, 'x': 650426.484139, 'y': 6633521.72023}],
 'geo_lambert_conformal_conic_inverse': [{'lat': 40.2965807494, 'lon': -100.015890022}, {'lat': 48.856248141, 'lon': 2.34581195868}],
 'geo_line_of_sight_profile': [{'n_visible': 2.0, 'visible': [1.0, 1.0, 1.0, 0.0, 0.0, 0.0]},
                               {'n_visible': 4.0, 'visible': [1.0, 1.0, 1.0, 1.0, 1.0]}],
 'geo_local_getis_ord_gi': [{'Gs': [0.825, 0.825, 0.875, 0.45, 0.175, 0.175],
                             'Zs': [2.21006630298, 2.21006630298, 1.50264699011, -1.56275286971, -2.21006630298, -2.21006630298]},
                            {'Gs': [0.766666666667, 0.75, 0.827586206897, 0.421052631579, 0.108108108108, 0.131578947368],
                             'Zs': [1.98852919667, 1.98558020126, 1.25086177191, -1.31451594415, -1.97973605279, -1.97177391622]}],
 'geo_local_morans_i': [{'Is': [0.818452380952, 0.595238095238, 0.0372023809524, 0.0595238095238, 0.729166666667, 1.04166666667],
                         'quadrant': [1.0, 1.0, 1.0, 3.0, 3.0, 3.0]},
                        {'Is': [0.697211155378, 0.876494023904, 0.268924302789, 0.268924302789, 0.697211155378, 0.876494023904],
                         'quadrant': [1.0, 1.0, 1.0, 3.0, 3.0, 3.0]}],
 'geo_look_angles': [{'azimuth': 172.243227782, 'elevation': 43.4643492959, 'slant_range': 37513689.2073},
                     {'azimuth': 237.437325357, 'elevation': 18.298258795, 'slant_range': 8916.28979808}],
 'geo_map_scale_from_distances': [{'rf': 2e-05, 'scale': 50000.0}, {'rf': 4e-05, 'scale': 25000.0}],
 'geo_map_scale_ground_distance': [{'ground_distance': 2100.0, 'ground_km': 2.1}, {'ground_distance': 300.0, 'ground_km': 0.3}],
 'geo_maximal_covering_location': [{'covered': 95.0, 'fraction': 0.904761904762}, {'covered': 11.0, 'fraction': 0.6875}],
 'geo_mean_center_standard_distance': [{'mean_x': 2.0, 'mean_y': 1.5, 'std_distance': 2.5},
                                       {'mean_x': 52.4, 'mean_y': 49.5981818182, 'std_distance': 40.2157598728}],
 'geo_mercator_forward': [{'k': 1.60309133889, 'x': -14471.5338031, 'y': 6676757.75408},
                          {'k': 1.02774470374, 'x': 1075504.88717, 'y': -2181959.02129}],
 'geo_mercator_inverse': [{'lat': 51.5223892048, 'lon': -0.129999696341}, {'lat': -18.3926859019, 'lon': 119.297958679}],
 'geo_meridian_arc': [{'distance': 10001965.7293, 'm1': 0.0, 'm2': 10001965.7293},
                      {'distance': 111044.260878, 'm1': 4429529.03024, 'm2': 4540573.29111}],
 'geo_mgrs_encode': [{'mgrs': '32UNA7166639109'}, {'mgrs': '18SUJ2339407395'}, {'mgrs': '56HLH3490052288'}],
 'geo_mollweide_forward': [{'x': 7261188.13511, 'y': 5334269.29077}, {'x': -1.00619678407, 'y': -0.571303746545}],
 'geo_morans_i': [{'EI': -0.2,
                   'I': 0.65625,
                   'VI_norm': 0.140952380952,
                   'VI_rand': 0.181792623299,
                   'p_norm': 0.0225673825813,
                   'p_rand': 0.0446193448302,
                   'z_norm': 2.28068011671,
                   'z_rand': 2.00822540212},
                  {'EI': -0.2,
                   'I': -1.0,
                   'VI_norm': 0.102857142857,
                   'VI_rand': 0.16,
                   'p_norm': 0.0126156672218,
                   'p_rand': 0.0455002638964,
                   'z_norm': -2.49443825785,
                   'z_rand': -2.0}],
 'geo_nearest_neighbour_index': [{'R': 0.885437744847, 'd_exp': 2.2360679775, 'd_obs': 1.97989898732, 'p_value': 0.62408490773, 'z': -0.490069234346},
                                 {'R': 3.16227766017,
                                  'd_exp': 2.2360679775,
                                  'd_obs': 7.07106781187,
                                  'p_value': 2.25137130498e-20,
                                  'z': 9.24969359191}],
 'geo_network_connectivity_indices': [{'alpha': 0.666666666667,
                                       'beta': 1.25,
                                       'components': 1.0,
                                       'cyclomatic': 2.0,
                                       'edges': 5.0,
                                       'gamma': 0.833333333333,
                                       'vertices': 4.0},
                                      {'alpha': 0.142857142857,
                                       'beta': 0.833333333333,
                                       'components': 2.0,
                                       'cyclomatic': 1.0,
                                       'edges': 5.0,
                                       'gamma': 0.416666666667,
                                       'vertices': 6.0}],
 'geo_ordinary_kriging': [{'estimate': 2.6048, 'variance': 0.877114762629}, {'estimate': 13.877929793, 'variance': 7.57644401216}],
 'geo_orthographic_forward': [{'x': 847484.573556, 'y': -502840.597785}, {'x': 0.336824088833, 'y': -0.173648177667}],
 'geo_p_median': [{'facilities': [2.0, 5.0], 'mean_distance': 1.42857142857, 'objective': 150.0},
                  {'facilities': [3.0], 'mean_distance': 2.0625, 'objective': 33.0}],
 'geo_parallel_arc': [{'distance': 55800.0015724, 'radius': 3197104.58692}, {'distance': 2042820.70095, 'radius': 5852250.22333}],
 'geo_polar_stereographic_forward': [{'k': 0.986664282035, 'x': 142401.981162, 'y': -1627662.0927},
                                     {'k': 0.983785800023, 'x': 305467.162732, 'y': -1289201.53771}],
 'geo_polygon_compactness': [{'area': 12.0,
                              'convex_hull_ratio': 1.0,
                              'perimeter': 14.0,
                              'polsby_popper': 0.769369629451,
                              'reock': 0.611154981473,
                              'schwartzberg': 1.14007252777},
                             {'area': 60.0,
                              'convex_hull_ratio': 0.6,
                              'perimeter': 48.8679622641,
                              'polsby_popper': 0.315727677346,
                              'reock': 0.381971863421,
                              'schwartzberg': 1.7796871826}],
 'geo_quadrat_test': [{'chi2': 0.4, 'counts': [3.0, 3.0, 2.0, 2.0], 'df': 3.0, 'p_value': 0.940242494839, 'vmr': 0.133333333333},
                      {'chi2': 0.2, 'counts': [3.0, 3.0, 4.0], 'df': 2.0, 'p_value': 0.904837418036, 'vmr': 0.1}],
 'geo_radii_of_curvature': [{'M': 6367381.81562, 'N': 6388838.29012, 'R_gauss': 6378101.0302},
                            {'M': 6335439.32708, 'N': 6378137.0, 'R_gauss': 6356752.31414}],
 'geo_rank_size_rule': [{'P1_fit': 7735648.98926, 'primacy': 2.15384615385, 'q': 0.903102243624, 'r_squared': 0.985306902371},
                        {'P1_fit': 994.941762864, 'primacy': 2.08333333333, 'q': 0.992989376123, 'r_squared': 0.996107541089}],
 'geo_rhumb_line_destination': [{'lat2': 53.8916814711, 'lon2': -1.3612136334}, {'lat2': -20.190773968, 'lon2': 96.1764363876}],
 'geo_rhumb_line_inverse': [{'bearing': 30.4064403372, 'distance': 1032440.86879}, {'bearing': 88.536494154, 'distance': 9118857.16619}],
 'geo_ripley_k': [{'K': [22.2222222222, 33.3333333333, 44.4444444444], 'L': [2.65961520268, 3.25735007935, 3.76126389032], 'area': 100.0},
                  {'K': [0.0, 32.0], 'L': [0.0, 3.19153824321], 'area': 100.0}],
 'geo_segregation_evenness': [{'dissimilarity': 0.625476735317, 'gini': 0.717009916095, 'theil_h': 0.364724080233},
                              {'dissimilarity': 0.98, 'gini': 0.985, 'theil_h': 0.924243296417}],
 'geo_segregation_exposure': [{'correlation_ratio': 0.441647597254, 'exposure': 0.397826086957, 'isolation': 0.602173913043},
                              {'correlation_ratio': 0.9606, 'exposure': 0.0197, 'isolation': 0.9803}],
 'geo_service_area': [{'count': 4.0, 'farthest': 6.0, 'reachable': [0.0, 1.0, 2.0, 3.0]},
                      {'count': 3.0, 'farthest': 1.0, 'reachable': [1.0, 2.0, 3.0]}],
 'geo_shimbel_index': [{'most_accessible': 1.0, 'shimbel': [13.0, 9.0, 9.0, 11.0], 'total': 42.0},
                       {'most_accessible': 2.0, 'shimbel': [10.0, 7.0, 6.0, 7.0, 10.0], 'total': 40.0}],
 'geo_sinusoidal_forward': [{'x': 788468.35094, 'y': 4984944.37798}, {'x': -1086978.19837, 'y': -1382396.61348}],
 'geo_slope_aspect_horn': [{'aspect_deg': 327.994616792, 'slope_deg': 43.3317197502, 'slope_percent': 94.3398113206},
                           {'aspect_deg': 270.0, 'slope_deg': 5.7105931375, 'slope_percent': 10.0}],
 'geo_spatial_lag': [{'lag': [4.5, 7.0, 4.5, 7.0]}, {'lag': [5.0, 1.0, 1.0]}],
 'geo_standard_deviational_ellipse': [{'orientation': 70.1436772398, 'sx': 50.130294591, 'sy': 37.9522267027},
                                      {'orientation': 43.7667604566, 'sx': 4.26287682001, 'sy': 0.324624733188}],
 'geo_terrain_curvature': [{'curvature': -3.0, 'plan': 1.36, 'profile': -1.64},
                           {'curvature': 0.0, 'plan': 0.00689655172414, 'profile': 0.00689655172414}],
 'geo_terrain_ruggedness': [{'TPI': -0.625, 'TRI': 6.40312423743, 'relief': 7.0}, {'TPI': -0.5, 'TRI': 9.16515138991, 'relief': 9.0}],
 'geo_tissot_indicatrix': [{'a': 2.0, 'b': 2.0, 'h': 2.0, 'k': 2.0, 'omega': 0.0, 's': 4.0, 'theta_prime': 90.0},
                           {'a': 1.18234186958,
                            'b': 0.845779064182,
                            'h': 1.05511825024,
                            'k': 1.0,
                            'omega': 19.104631309,
                            's': 1.0,
                            'theta_prime': 71.3986812236},
                           {'a': 1.5501868645,
                            'b': 0.645083520509,
                            'h': 1.29970223711,
                            'k': 1.0630080706,
                            'omega': 48.6984184462,
                            's': 1.0,
                            'theta_prime': 46.3692507281}],
 'geo_toa_reflectance': [{'earth_sun_distance': 1.01668719635, 'reflectance': 0.207501700248},
                         {'earth_sun_distance': 0.983269293999, 'reflectance': 0.282320832026}],
 'geo_transverse_mercator_forward': [{'gamma': -1.26948854384, 'k': 0.999744793458, 'x': 391390.731339, 'y': 5817855.24069},
                                     {'gamma': 1.4636802137, 'k': 0.999808117046, 'x': 529796.825275, 'y': 179631.668971}],
 'geo_trend_surface': [{'coefficients': [1.01666666667, 2.05, 0.966666666667], 'r_squared': 0.998720409469},
                       {'coefficients': [0.280555555556, -0.208333333333, -0.0916666666667, 1.01666666667, 0.225, 0.966666666667],
                        'r_squared': 0.997304297001}],
 'geo_two_step_floating_catchment': [{'accessibility': [0.00217391304348, 0.00217391304348, 0.00417391304348, 0.002],
                                      'ratios': [0.00217391304348, 0.002]},
                                     {'accessibility': [0.005, 0.005, 0.01, 0.005, 0.005], 'ratios': [0.005, 0.005]}],
 'geo_utm_forward': [{'easting': 699195.875854, 'gamma': 2.24681916539, 'hemisphere': 'N', 'k': 1.00008718886, 'northing': 5709335.01926, 'zone': 30},
                     {'easting': 259583.22166, 'gamma': 1.45083291153, 'hemisphere': 'S', 'k': 1.00031259368, 'northing': 6245888.04544, 'zone': 34},
                     {'easting': 296191.546952, 'gamma': -3.21822745324, 'hemisphere': 'N', 'k': 1.00010902953, 'northing': 6701684.475, 'zone': 32}],
 'geo_utm_inverse': [{'lat': 51.5074022552, 'lon': -0.127803235224}, {'lat': -33.9248663405, 'lon': 18.4240620959}],
 'geo_utm_zone': [{'band': 'U', 'central_meridian': -3.0, 'hemisphere': 'N', 'zone': 30},
                  {'band': 'V', 'central_meridian': 9.0, 'hemisphere': 'N', 'zone': 32},
                  {'band': 'X', 'central_meridian': 15.0, 'hemisphere': 'N', 'zone': 33},
                  {'band': 'H', 'central_meridian': 21.0, 'hemisphere': 'S', 'zone': 34}],
 'geo_variogram_model_fit': [{'nugget': -0.0628821025575, 'psill': 7.16434833449, 'range': 11.3285006727, 'sill': 7.10146623193},
                             {'nugget': 0.428936895438, 'psill': 6.57148309768, 'range': 10.5441569706, 'sill': 7.00041999312}],
 'geo_vegetation_indices': [{'EVI': 0.567484662577,
                             'EVI2': 0.563337393423,
                             'MSAVI': 0.546887112585,
                             'NDVI': 0.698113207547,
                             'OSAVI': 0.536231884058,
                             'SAVI': 0.538834951456},
                            {'EVI': 0.0735294117647,
                             'EVI2': 0.0722543352601,
                             'MSAVI': 0.0699264745632,
                             'NDVI': 0.111111111111,
                             'OSAVI': 0.0819672131148,
                             'SAVI': 0.0789473684211}],
 'geo_water_burn_built_indices': [{'MNDWI': 0.6, 'NBR': 0.5, 'NDBI': -0.2, 'NDMI': 0.2, 'NDWI': 0.454545454545},
                                  {'MNDWI': -0.481481481481,
                                   'NBR': 0.555555555556,
                                   'NDBI': -0.272727272727,
                                   'NDMI': 0.272727272727,
                                   'NDWI': -0.666666666667}],
 'geo_watershed_delineation': [{'area': 1600.0,
                                'cells': 16.0,
                                'mask': [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]]},
                               {'area': 9.0, 'cells': 9.0, 'mask': [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]}],
 'geo_web_map_tile': [{'quadkey': '031313131130', 'tile_x': 2046, 'tile_y': 1362}, {'quadkey': '31123013', 'tile_x': 235, 'tile_y': 153}],
 'geo_web_mercator_forward': [{'x': -14471.5338031, 'y': 6710219.08322}, {'x': 16832620.2029, 'y': -4011359.53105}],
 'geo_web_mercator_inverse': [{'lat': 51.5000000938, 'lon': -0.129999696341}, {'lat': -33.8673183058, 'lon': 151.209423256}],
 'geo_weighted_mean_center': [{'weighted_std_distance': 9.35414346693, 'wmean_x': 7.5, 'wmean_y': 2.5},
                              {'weighted_std_distance': 1.41421356237, 'wmean_x': 3.0, 'wmean_y': 2.0}]}
# @@EXV-END@@

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)
