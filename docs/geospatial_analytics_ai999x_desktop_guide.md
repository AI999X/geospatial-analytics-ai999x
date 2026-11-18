# Geospatial Analytics: the desktop app, user guide

**Geospatial Analytics**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Geospatial Analytics covers map projections and coordinates, geodesic distance and area, spatial statistics, remote-sensing indices, solar position and terrain, and urban and network analysis. Its desktop app holds all 99 calculations in 14 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-geospatial-analytics-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-geospatial-analytics-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-geospatial-analytics-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-geospatial-analytics.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-geospatial-analytics-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-geospatial-analytics-1.6.5-linux-x86_64.tar.gz
./ai999x-geospatial-analytics-1.6.5-linux-x86_64/ai999x-geospatial-analytics
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Geospatial Analytics app, opened on Latitude and longitude to UTM coordinates (with scale factor and convergence) with its worked example.](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/hero.webp)

*The Geospatial Analytics app, opened on Latitude and longitude to UTM coordinates (with scale factor and convergence) with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Map projections › Latitude and longitude to UTM coordinates (with scale factor and convergence) (or type part of its name in the search box). It opens with a worked example filled in: “UTM forward: lat = 51.5, lon = -0.13, ellipsoid = WGS84” The answer: easting = 699195.8759 m, northing = 5709335.019 m, k = 1.000087189, gamma = 2.246819165.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Latitude and longitude to UTM coordinates (with scale factor and convergence): the live scene of the worked example.](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-1.webp)

*Latitude and longitude to UTM coordinates (with scale factor and convergence): the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Geodesic destination: lat1 = 52.2, lon1 = 0.12, azimuth = 45, distance = 100 km, ellipsoid = WGS84”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Geodesic destination point on the ellipsoid (direct problem)
lat2 = 52.83083647
lon2 = 1.169145951
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Area and perimeter of a geodesic polygon on the ellipsoid](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-2.webp)

*Area and perimeter of a geodesic polygon on the ellipsoid*

![Vegetation indices: NDVI, EVI, SAVI, OSAVI, MSAVI and EVI2 from band reflectances](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-3.webp)

*Vegetation indices: NDVI, EVI, SAVI, OSAVI, MSAVI and EVI2 from band reflectances*

![Global Moran's I spatial autocorrelation with normal and randomization inference](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-4.webp)

*Global Moran's I spatial autocorrelation with normal and randomization inference*

![Map scale (representative fraction) from a map and a ground distance](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-5.webp)

*Map scale (representative fraction) from a map and a ground distance*

![Decimal degrees to degrees, minutes and seconds](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-6.webp)

*Decimal degrees to degrees, minutes and seconds*

![Auxiliary latitudes: geocentric, reduced, conformal, authalic and rectifying](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-7.webp)

*Auxiliary latitudes: geocentric, reduced, conformal, authalic and rectifying*

![Closest facility for every node of a network](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/scene-8.webp)

*Closest facility for every node of a network*

![Latitude and longitude to UTM coordinates (with scale factor and convergence): a chart, enlarged.](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/chart-1.webp)

*Latitude and longitude to UTM coordinates (with scale factor and convergence): a chart, enlarged.*

![Area and perimeter of a geodesic polygon on the ellipsoid: a chart, enlarged.](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/chart-2.webp)

*Area and perimeter of a geodesic polygon on the ellipsoid: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Geospatial Analytics app in the dark theme.](https://www.ai999x.com/?ax_shot=geospatial-analytics-ai999x/hero-dark.webp)

*The Geospatial Analytics app in the dark theme.*

## 7. Every calculation

99 calculations in 14 topics; each has its own entry on the model's page (https://www.ai999x.com/models/geospatial-analytics-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Cartography | Ground distance from a map distance and a representative fraction, Map scale (representative fraction) from a map and a ground distance |
| Coordinate notation | Decimal degrees to degrees, minutes and seconds, Decode a geohash: cell centre and size, Degrees, minutes and seconds to decimal degrees, Geohash of a point and its cell, MGRS grid reference of a point (1 m) |
| Coordinate systems and conversions | Auxiliary latitudes: geocentric, reduced, conformal, authalic and rectifying, Geocentric (ECEF) to geodetic coordinates, Geodetic to geocentric (ECEF) coordinates, Geodetic to local east-north-up (ENU) coordinates, Local east-north-up (ENU) to geodetic coordinates, Look angles: azimuth, elevation and range from an observer to a target, UTM zone, latitude band and central meridian |
| Ellipsoidal geodesy | Arc length along a parallel of latitude, Area and perimeter of a geodesic polygon on the ellipsoid, Area of a latitude-longitude cell on the ellipsoid, Destination along a rhumb line (loxodrome) on the ellipsoid, Geodesic destination point on the ellipsoid (direct problem), Meridian arc length between two latitudes, Midpoint of a geodesic on the ellipsoid, Radii of curvature of the ellipsoid at a latitude, Rhumb-line (loxodrome) distance and bearing on the ellipsoid |
| Facility location | Closest facility for every node of a network, Maximal covering location problem (MCLP) on a network, p-median facility location on a network |
| Geostatistics | Empirical (Matheron) semivariogram in distance bins, Fit an exponential or Gaussian variogram model (nugget, partial sill, range), Inverse distance weighted (IDW) interpolation at a point, Ordinary kriging estimate and kriging variance at a point, Polynomial trend surface (first or second order) by least squares |
| Map projections | Albers equal-area conic projection: coordinates and scale factors, Azimuthal equidistant projection (ellipsoidal, geodesic), Equirectangular (equidistant cylindrical / plate carree) projection, Gnomonic projection (sphere), Inverse Albers equal-area conic projection, Inverse Lambert conformal conic projection (two standard parallels), Inverse Mercator projection (ellipsoidal), Lambert azimuthal equal-area projection (ellipsoidal), Lambert conformal conic projection (two standard parallels): coordinates, scale factor and convergence, Lambert cylindrical equal-area projection (sphere), Latitude and longitude to UTM coordinates (with scale factor and convergence), Latitude and longitude to Web Mercator (EPSG:3857), Mercator projection (ellipsoidal): coordinates and scale factor, Mollweide equal-area projection (sphere), Orthographic projection (sphere), Polar stereographic projection (standard parallel): coordinates and scale factor, Sinusoidal equal-area projection (ellipsoidal), Tissot's indicatrix: scale factors and distortion of a projection at a point, Transverse Mercator projection: coordinates, scale factor and grid convergence, UTM coordinates to latitude and longitude, Web Mercator (EPSG:3857) to latitude and longitude, Web-map tile (slippy map / XYZ) and quadkey containing a point |
| Network accessibility | Gravity-based (Hansen) accessibility on a road network, Network connectivity indices: cyclomatic number, alpha, beta and gamma, Network detour index (circuity) between two nodes, Network service area (isochrone) of a facility, Shimbel accessibility index of every node, Two-step floating catchment area (2SFCA) accessibility |
| Planar vector analysis | Discrete Frechet distance between two polylines, Douglas-Peucker line simplification, Shape compactness indices of a polygon (Polsby-Popper, Schwartzberg, convex hull, Reock) |
| Point pattern analysis | Kernel density estimate of a point pattern at a location, Mean centre and standard distance of a point pattern, Nearest-neighbour index (Clark-Evans ratio) of a point pattern, Quadrat analysis: chi-square test of complete spatial randomness, Ripley's K and L functions of a point pattern, Standard deviational ellipse of a point pattern, Weighted mean centre and weighted standard distance |
| Remote sensing | At-sensor brightness temperature from thermal radiance, Ground sample distance and swath of a nadir-looking camera, Thematic map accuracy: overall accuracy, kappa, producer's and user's accuracy, Top-of-atmosphere reflectance from at-sensor radiance, Vegetation indices: NDVI, EVI, SAVI, OSAVI, MSAVI and EVI2 from band reflectances, Water, moisture, burn and built-up indices: NDWI, MNDWI, NDMI, NBR and NDBI |
| Spatial statistics | Bivariate Moran's I (spatial cross-correlation), Distance-band spatial weights, Geary's C spatial autocorrelation with normal and randomization inference, Getis-Ord general G (global high/low clustering), Global Moran's I spatial autocorrelation with normal and randomization inference, Join count statistics of a binary map (BB, BW, WW), k-nearest-neighbour spatial weights, Local Getis-Ord Gi and Gi* hot-spot statistics, Local Moran's I (LISA) and Moran scatterplot quadrants, Spatial lag of a variable |
| Terrain analysis | D8 flow accumulation of a small DEM, D8 flow direction of every cell of a small DEM, Hillshade (analytical relief shading) of a 3x3 elevation window, Line-of-sight visibility along a terrain profile (with Earth curvature and refraction), Slope and aspect of a 3x3 elevation window (Horn's method), Terrain curvature: total, profile and plan curvature (Zevenbergen-Thorne), Terrain ruggedness index (Riley), topographic position index and local relief, Watershed (catchment) of an outlet cell on a small DEM |
| Urban geography | Huff model: probabilities of shopping at each store from an origin (network distances), Rank-size rule (Zipf) fit of a settlement system, Residential segregation (evenness): dissimilarity, Gini and Theil entropy indices, Residential segregation (exposure): isolation, interaction and correlation ratio, Spatial interaction (gravity) model fitted by Poisson regression, Urban population density gradient (Clark's negative exponential model) |

## 8. Other ways to use it

The same engine runs on the command line (`geospatial-analytics`, its own guide), in Python (`pip install geospatial-analytics-ai999x`, `import geospatial_analytics_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/geospatial-analytics-ai999x/.

## Credits

- Coastlines and lakes: Natural Earth (public domain).

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/geospatial-analytics-ai999x/
