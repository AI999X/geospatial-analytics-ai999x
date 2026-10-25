"""geospatial-analytics-ai999x — offline geospatial analytics answers with their formula, steps and verification.

    import geospatial_analytics_ai999x as m
    m.ask("Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84").show()
    #   lat2 = 52.8308364668, lon2 = 1.16914595053, azimuth2 = 45.832536554
    m.ask("UTM forward: lat = 51.5, lon = -0.13, ellipsoid = WGS84").show()
    #   zone 30 N, easting = 699195.875854 m, northing = 5709335.01926 m, k = 1.00008718886, gamma = 2.24681916539
    m.solve("geo_morans_i", y=[8, 6, 6, 3, 2, 1], W=[[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0],
            [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]], transform="row")     # I = 0.65625
    m.capabilities()                   # every certified capability

99 certified capabilities in the geospatial view: ellipsoidal geodesy (geodesic destination and midpoint, geodesic
polygon area and perimeter, rhumb-line distance, bearing and destination, meridian and parallel arcs, graticule-cell
areas, radii of curvature), coordinates (geodetic, ECEF and local ENU conversions, look angles, auxiliary latitudes,
UTM zones, MGRS references, geohashes, web-map tiles, degrees-minutes-seconds, map scale), map projections (transverse
Mercator and UTM with scale factor and grid convergence, Mercator, Web Mercator, Lambert conformal conic and Albers
equal-area forward and inverse, polar stereographic, Lambert azimuthal equal-area, azimuthal equidistant, sinusoidal,
Mollweide, orthographic, gnomonic, equirectangular and cylindrical equal-area, Tissot's indicatrix), planar analysis
(Douglas-Peucker simplification, discrete Frechet distance, compactness indices), spatial statistics (Moran's I,
Geary's C, Getis-Ord general G and local Gi/Gi*, local and bivariate Moran, join counts, spatial lag, k-nearest and
distance-band weights, nearest-neighbour index, quadrat test, Ripley's K, centrography and the standard deviational
ellipse, kernel density, IDW, empirical semivariograms, variogram fits, ordinary kriging, trend surfaces, rank-size
rule, density gradients, a Poisson gravity model), remote sensing (vegetation, water, burn and built-up indices,
top-of-atmosphere reflectance, brightness temperature, ground sample distance, classification accuracy), terrain (Horn
slope and aspect, hillshade, curvature, ruggedness, D8 flow direction and accumulation, watersheds, line of sight) and
networks and cities (gravity accessibility, Huff model, two-step floating catchment, service areas, closest facility,
p-median and maximal covering location, detour index, connectivity and Shimbel indices, segregation indices).

Questions use the form "<capability phrase>: name = value, ..." (see templates()); degrees are decimal and may carry a
degree sign, physical values carry their units, lists and matrices are written in brackets, networks as
`edges = [0-1 4; 1-2 2.5]`, and conventions (ellipsoid, hemisphere, weights transform, kernel, variogram model,
impedance function) are stated, never assumed. Built by the AI999X model machine from the same engine as every domain;
a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("geospatial_analytics", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
