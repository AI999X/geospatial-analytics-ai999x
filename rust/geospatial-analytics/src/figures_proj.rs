//! Projections and grids: graticules traced through the model's own forward projections, the azimuth compass, Tissot
//! ellipses, UTM zones, MGRS squares, web-map tiles and geohash cells. Every point is a run of the model.

use crate::figures::{dg, fl, given, n4, n6, o, ot, pass};
use crate::figures_plot::Plot;
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::collections::BTreeMap;   // ordered maps: a figure is the same on every run

/// Where a graticule's points are kept (a hemisphere, a zone).
enum Keep {
    All,
    /// within this angle (degrees) of a centre (lat, lon)
    Within(f64, f64, f64),
    /// the UTM zone and hemisphere of the request
    Zone(f64, String),
}

struct Cfg {
    fwd: &'static str,
    xo: &'static str,
    yo: &'static str,
    /// the forward projection's other inputs, passed on as given
    params: &'static [&'static str],
    lat: (f64, f64),
    lon: (f64, f64),
    par: f64,
    mer: f64,
    std: Vec<f64>,
    keep: Keep,
    what: String,
    xlab: &'static str,
    ylab: &'static str,
}

fn round_out(lo: f64, hi: f64, step: f64) -> (f64, f64) {
    ((lo / step - 1e-9).floor() * step, (hi / step + 1e-9).ceil() * step)
}

fn ang_dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    // a selection of which points to trace (a visible hemisphere), not a drawn value
    let r = std::f64::consts::PI / 180.0;
    let c = (a.0 * r).sin() * (b.0 * r).sin() + (a.0 * r).cos() * (b.0 * r).cos() * ((a.1 - b.1) * r).cos();
    c.clamp(-1.0, 1.0).acos() / r
}

fn cfg(s: &Solved) -> Option<Cfg> {
    let inp = |k: &str| s.inp(k);
    let (lat, lon) = (inp("lat"), inp("lon"));
    let (rlat, rlon) = (s.out("lat"), s.out("lon"));
    let world = |fwd: &'static str, params: &'static [&'static str], lon0: f64, poles: bool, what: &str| Cfg {
        fwd, xo: "x", yo: "y", params, lat: if poles { (-90.0, 90.0) } else { (-80.0, 80.0) },
        lon: (lon0 - 179.999, lon0 + 179.999), par: 20.0, mer: 30.0, std: vec![], keep: Keep::All, what: what.to_string(),
        xlab: "x [km]", ylab: "y [km]",
    };
    let conic = |fwd: &'static str, c: (f64, f64), what: &str| -> Option<Cfg> {
        let (l1, l2, l0, lon0) = (inp("lat1")?, inp("lat2")?, inp("lat0")?, inp("lon0")?);
        let lo = l1.min(l2).min(l0).min(c.0) - 5.0;
        let hi = l1.max(l2).max(c.0) + 10.0;
        let (lo, hi) = round_out(lo.max(-85.0), hi.min(85.0), 5.0);
        let (wl, wh) = round_out((lon0 - 40.0).min(c.1 - 10.0), (lon0 + 40.0).max(c.1 + 10.0), 10.0);
        Some(Cfg { fwd, xo: "x", yo: "y", params: &["lat1", "lat2", "lat0", "lon0", "ellipsoid", "FE", "FN"], lat: (lo, hi),
                   lon: (wl, wh), par: 5.0, mer: 10.0, std: vec![l1, l2], keep: Keep::All, what: what.into(),
                   xlab: "x [km]", ylab: "y [km]" })
    };
    let near = |fwd: &'static str, params: &'static [&'static str], c: (f64, f64), half: f64, step: f64, what: &str| {
        let (lo, hi) = round_out((c.0 - half).max(-84.0), (c.0 + half).min(84.0), step);
        let (wl, wh) = round_out(c.1 - half, c.1 + half, step);
        Cfg { fwd, xo: "x", yo: "y", params, lat: (lo, hi), lon: (wl, wh), par: step, mer: step, std: vec![],
              keep: Keep::All, what: what.to_string(), xlab: "x [km]", ylab: "y [km]" }
    };
    Some(match s.key {
        "geo_equirectangular_forward" => world("geo_equirectangular_forward", &["lat_ts", "lat0", "lon0", "R"],
                                               inp("lon0")?, true, "Equirectangular graticule"),
        "geo_cylindrical_equal_area_forward" => {
            let ts = inp("lat_ts")?;
            let mut c = world("geo_cylindrical_equal_area_forward", &["lat_ts", "lon0", "R"], inp("lon0")?, true,
                              "Cylindrical equal-area graticule");
            c.std = if ts.abs() > 1e-9 { vec![ts, -ts] } else { vec![0.0] };
            c
        }
        "geo_mercator_forward" => world("geo_mercator_forward", &["lon0", "lat_ts", "ellipsoid"], inp("lon0")?, false,
                                        "Mercator graticule"),
        "geo_web_mercator_forward" => world("geo_web_mercator_forward", &[], 0.0, false, "Web Mercator graticule"),
        "geo_sinusoidal_forward" => world("geo_sinusoidal_forward", &["lon0", "ellipsoid"], inp("lon0")?, true,
                                          "Sinusoidal graticule"),
        "geo_mollweide_forward" => {
            let mut c = world("geo_mollweide_forward", &["lon0", "R"], inp("lon0")?, true,
                              "World graticule in the Mollweide equal-area projection");
            c.par = 15.0;
            c
        }
        "geo_azimuthal_equidistant_forward" => {
            let (l0, w0) = (inp("lat0")?, inp("lon0")?);
            let mut c = world("geo_azimuthal_equidistant_forward", &["lat0", "lon0", "ellipsoid", "FE", "FN"], w0, false,
                              "Azimuthal equidistant graticule");
            c.keep = Keep::Within(178.0, l0, w0);
            c
        }
        "geo_orthographic_forward" => {
            let (l0, w0) = (inp("lat0")?, inp("lon0")?);
            let mut c = world("geo_orthographic_forward", &["lat0", "lon0", "R"], w0, true,
                              "Orthographic view of the hemisphere");
            c.keep = Keep::Within(90.0, l0, w0);
            c
        }
        "geo_gnomonic_forward" => {
            let (l0, w0) = (inp("lat0")?, inp("lon0")?);
            let (lo, hi) = round_out((l0 - 62.0).max(-85.0), (l0 + 62.0).min(85.0), 10.0);
            Cfg { fwd: "geo_gnomonic_forward", xo: "x", yo: "y", params: &["lat0", "lon0", "R"], lat: (lo, hi),
                  lon: (w0 - 90.0, w0 + 90.0), par: 10.0, mer: 15.0, std: vec![], keep: Keep::Within(62.0, l0, w0),
                  what: "Gnomonic graticule".into(), xlab: "x [km]", ylab: "y [km]" }
        }
        "geo_lambert_azimuthal_equal_area_forward" => {
            let (l0, w0) = (inp("lat0")?, inp("lon0")?);
            let (lo, hi) = round_out((l0.min(lat?) - 20.0).max(-90.0), (l0.max(lat?) + 20.0).min(90.0), 10.0);
            let (wl, wh) = round_out((w0 - 30.0).min(lon? - 10.0), (w0 + 30.0).max(lon? + 10.0), 10.0);
            Cfg { fwd: "geo_lambert_azimuthal_equal_area_forward", xo: "x", yo: "y",
                  params: &["lat0", "lon0", "ellipsoid", "FE", "FN"], lat: (lo, hi), lon: (wl, wh), par: 10.0,
                  mer: 10.0, std: vec![], keep: Keep::Within(150.0, l0, w0),
                  what: "Lambert azimuthal equal-area graticule".into(), xlab: "x [km]", ylab: "y [km]" }
        }
        "geo_albers_equal_area_forward" => conic("geo_albers_equal_area_forward", (lat?, lon?),
                                                 "Albers equal-area conic graticule")?,
        "geo_lambert_conformal_conic_forward" => conic("geo_lambert_conformal_conic_forward", (lat?, lon?),
                                                       "Lambert conformal conic graticule")?,
        "geo_albers_equal_area_inverse" => {
            let mut c = conic("geo_albers_equal_area_forward", (rlat?, rlon?), "Albers inverse: the point on its graticule")?;
            let n = near("", &[], (rlat?, rlon?), 10.0, 5.0, "");
            (c.lat, c.lon, c.mer) = (n.lat, n.lon, 5.0);
            c
        }
        "geo_lambert_conformal_conic_inverse" => {
            let mut c = conic("geo_lambert_conformal_conic_forward", (rlat?, rlon?),
                              "Lambert conformal conic inverse: the point on its graticule")?;
            let n = near("", &[], (rlat?, rlon?), 10.0, 5.0, "");
            (c.lat, c.lon, c.mer) = (n.lat, n.lon, 5.0);
            c
        }
        "geo_mercator_inverse" => near("geo_mercator_forward", &["lon0", "lat_ts", "ellipsoid"], (rlat?, rlon?), 1.5, 0.5,
                                       "Mercator inverse: the point on its graticule"),
        "geo_web_mercator_inverse" => near("geo_web_mercator_forward", &[], (rlat?, rlon?), 1.5, 0.5,
                                           "Web Mercator inverse: the point on its graticule"),
        "geo_polar_stereographic_forward" => {
            let north = s.txt("pole")?.to_lowercase().starts_with('n');
            let ts = inp("lat_ts")?;
            let edge = if north { (lat?.min(ts) - 5.0).min(60.0).max(0.0) } else { (lat?.max(ts) + 5.0).max(-60.0).min(0.0) };
            let (lo, hi) = if north { round_out(edge, 90.0, 5.0) } else { round_out(-90.0, edge, 5.0) };
            Cfg { fwd: "geo_polar_stereographic_forward", xo: "x", yo: "y",
                  params: &["pole", "lat_ts", "lon0", "ellipsoid", "FE", "FN"], lat: (lo, hi), lon: (-180.0, 180.0),
                  par: 5.0, mer: 30.0, std: vec![ts], keep: Keep::All,
                  what: format!("{} polar stereographic graticule", if north { "North" } else { "South" }),
                  xlab: "x [km]", ylab: "y [km]" }
        }
        "geo_transverse_mercator_forward" => {
            let w0 = inp("lon0")?;
            let (lo, hi) = if lat? >= 0.0 { (0.0, 80.0) } else { (-80.0, 0.0) };
            let (wl, wh) = round_out((w0 - 18.0).min(lon? - 6.0), (w0 + 18.0).max(lon? + 6.0), 6.0);
            Cfg { fwd: "geo_transverse_mercator_forward", xo: "x", yo: "y",
                  params: &["lat0", "lon0", "k0", "ellipsoid", "FE", "FN"], lat: (lo, hi), lon: (wl, wh), par: 10.0,
                  mer: 6.0, std: vec![], keep: Keep::All, what: format!("Transverse Mercator about the {} meridian", dg(w0)),
                  xlab: "x [km]", ylab: "y [km]" }
        }
        "geo_utm_forward" => {
            let zone = s.out("zone")?;
            let hemi = s.out_text("hemisphere")?;
            let cm = zone * 6.0 - 183.0;
            let (lo, hi) = (lat?.floor() - 2.0, lat?.floor() + 3.0);
            Cfg { fwd: "geo_utm_forward", xo: "easting", yo: "northing", params: &["ellipsoid"],
                  lat: (lo.max(-80.0), hi.min(84.0)), lon: (cm - 3.0, cm + 3.0 - 1e-7), par: 1.0, mer: 1.0, std: vec![],
                  keep: Keep::Zone(zone, hemi), what: format!("UTM zone {} graticule", n6(zone)),
                  xlab: "easting [km]", ylab: "northing [km]" }
        }
        "geo_utm_inverse" => {
            let zone = crate::figures::iv(s, "zone")?;
            let hemi = s.txt("hemisphere")?;
            let mut c = near("geo_utm_forward", &["ellipsoid"], (rlat?, rlon?), 1.0, 0.5,
                             &format!("UTM inverse: the point on the zone {} graticule", n6(zone)));
            (c.xo, c.yo, c.keep, c.xlab, c.ylab) = ("easting", "northing", Keep::Zone(zone, hemi), "easting [km]",
                                                    "northing [km]");
            c
        }
        _ => return None,
    })
}

/// The forward projection of (lat, lon): (x, y) in m and the answer.
fn project(s: &Solved, c: &Cfg, lat: f64, lon: f64) -> Option<(f64, f64)> {
    let a = s.solve_other(c.fwd, &pass(s, c.params, vec![("lat", fl(lat)), ("lon", fl(lon))]))?;
    if let Keep::Zone(z, h) = &c.keep {
        if o(&a, "zone")? != *z || !ot(&a, "hemisphere")?.eq_ignore_ascii_case(h.trim()) {
            return None;
        }
    }
    let (x, y) = (o(&a, c.xo)?, o(&a, c.yo)?);
    (x.is_finite() && y.is_finite()).then_some((x, y))
}

fn kept(c: &Cfg, lat: f64, lon: f64) -> bool {
    match c.keep {
        Keep::Within(r, l0, w0) => ang_dist((lat, lon), (l0, w0)) <= r,
        _ => true,
    }
}

/// Values `anchor + k st` inside [lo, hi], and the two ends themselves (a pole, the edge of a world map).
fn steps(lo: f64, hi: f64, st: f64, anchor: f64) -> Vec<f64> {
    let mut v = vec![lo];
    let mut x = anchor + ((lo - anchor) / st - 1e-9).ceil() * st;
    while x <= hi + 1e-9 && v.len() < 400 {
        if (x - lo).abs() > 1e-9 * st && (x - hi).abs() > 1e-9 * st {
            v.push(x);
        }
        x += st;
    }
    v.push(hi);
    v
}

fn on(v: f64, every: f64) -> bool {
    ((v / every) - (v / every).round()).abs() < 1e-6
}

/// Lines broken where a point is missing or where consecutive points jump across the map (a seam).
fn lines_of(pts: &[Option<(f64, f64)>], jump: f64) -> Vec<(f64, f64)> {
    let mut out = vec![];
    let mut last: Option<(f64, f64)> = None;
    for p in pts {
        match p {
            Some(q) => {
                if let Some(l) = last {
                    if ((q.0 - l.0).powi(2) + (q.1 - l.1).powi(2)).sqrt() > jump {
                        out.push((f64::NAN, f64::NAN));
                    }
                }
                out.push(*q);
                last = Some(*q);
            }
            None => {
                out.push((f64::NAN, f64::NAN));
                last = None;
            }
        }
    }
    out
}

pub fn graticule(s: &Solved) -> Option<Figure> {
    let mut c = cfg(s)?;
    let inverse = s.key.ends_with("_inverse");
    // the fine steps: half the line spacing, coarsened until the points fit the allowance
    let (mut fl_, mut fw) = (c.par / 2.0, c.mer / 2.0);
    let budget = s.left().saturating_sub(40).min(330);
    // meridians of a whole-world map pass through its central meridian; its two edges are meridians too (kept just
    // inside ±180° from the centre, so that neither edge wraps to the other side)
    let world = c.lon.1 - c.lon.0 > 359.0;
    let wref = if world { (c.lon.0 + c.lon.1) / 2.0 } else { 0.0 };
    let (e0, e1) = c.lon;
    let is_mer = |lo: f64, mer: f64| on(lo - wref, mer) || (world && ((lo - e0).abs() < 1e-9 || (lo - e1).abs() < 1e-9));
    loop {
        let lats = steps(c.lat.0, c.lat.1, fl_, 0.0);
        let lons = steps(c.lon.0, c.lon.1, fw, wref);
        let pr = lats.iter().filter(|l| on(**l, c.par)).count();
        let mr = lons.iter().filter(|l| is_mer(**l, c.mer)).count();
        let need = pr * lons.len() + mr * (lats.len() - pr) + c.std.len() * lons.len();
        if need <= budget {
            break;
        }
        if fl_ < c.par && fw < c.mer {
            fl_ *= 2.0;
            fw *= 2.0;
        } else {
            c.par *= 2.0;
            c.mer *= 2.0;
            fl_ = c.par / 2.0;
            fw = c.mer / 2.0;
        }
        if c.par > 90.0 {
            return None;
        }
    }
    let lats = steps(c.lat.0, c.lat.1, fl_, 0.0);
    let lons = steps(c.lon.0, c.lon.1, fw, wref);
    let mut grid: BTreeMap<(usize, usize), Option<(f64, f64)>> = BTreeMap::new();
    let mut n = 0usize;
    for (i, la) in lats.iter().enumerate() {
        for (j, lo) in lons.iter().enumerate() {
            if on(*la, c.par) || is_mer(*lo, c.mer) {
                let p = if kept(&c, *la, *lo) { project(s, &c, *la, *lo) } else { None };
                n += p.is_some() as usize;
                grid.insert((i, j), p.map(|q| (q.0 / 1000.0, q.1 / 1000.0)));
            }
        }
    }
    let all: Vec<(f64, f64)> = grid.values().flatten().copied().collect();
    if all.len() < 8 {
        return None;
    }
    // the marked point: the request's own projected point (forward) or the stated one (inverse)
    let (mx, my, mlab) = if inverse {
        let (x, y) = if s.key == "geo_utm_inverse" { (s.inp("easting")?, s.inp("northing")?) } else { (s.inp("x")?, s.inp("y")?) };
        (x, y, format!("{}, {}", dg(s.out("lat")?), dg(s.out("lon")?)))
    } else {
        (s.out(c.xo)?, s.out(c.yo)?, format!("{}, {}", dg(s.inp("lat")?), dg(s.inp("lon")?)))
    };
    let (mx, my) = (mx / 1000.0, my / 1000.0);
    let mut xs: Vec<f64> = all.iter().map(|p| p.0).collect();
    let mut ys: Vec<f64> = all.iter().map(|p| p.1).collect();
    xs.push(mx);
    ys.push(my);
    // a far outlier (a gnomonic edge) must not squeeze the map: the range of the bulk of the points
    let ext = |v: &[f64]| v.iter().cloned().fold(f64::MIN, f64::max) - v.iter().cloned().fold(f64::MAX, f64::min);
    let span = ext(&xs).max(ext(&ys));
    let mut p = Plot::equal(&xs, &ys)?;
    let jump = span * 0.3;
    let mut parallels = 0;
    let mut meridians = 0;
    for (i, la) in lats.iter().enumerate() {
        if on(*la, c.par) {
            let row: Vec<Option<(f64, f64)>> = (0..lons.len()).map(|j| grid.get(&(i, j)).copied().flatten()).collect();
            let cl = if la.abs() < 1e-9 { 2 } else { 3 };
            p.line(&lines_of(&row, jump), cl, "");
            parallels += 1;
        }
    }
    for (j, lo) in lons.iter().enumerate() {
        if is_mer(*lo, c.mer) {
            let col: Vec<Option<(f64, f64)>> = (0..lats.len()).map(|i| grid.get(&(i, j)).copied().flatten()).collect();
            p.line(&lines_of(&col, jump), 3, "");
            meridians += 1;
        }
    }
    let mut std_n = 0;
    for la in &c.std {
        let row: Vec<Option<(f64, f64)>> = lons.iter()
            .map(|lo| if kept(&c, *la, *lo) { project(s, &c, *la, *lo).map(|q| (q.0 / 1000.0, q.1 / 1000.0)) } else { None })
            .collect();
        n += row.iter().flatten().count();
        if row.iter().flatten().count() >= 2 {
            p.dashed(&lines_of(&row, jump), 1, if c.std.len() > 1 { "standard parallels" } else { "standard parallel" });
            std_n += 1;
        }
    }
    if parallels + meridians < 3 {
        return None;
    }
    p.key(&format!("graticule, {}° × {}°", n6(c.par), n6(c.mer)), 3);
    if lats.iter().any(|l| l.abs() < 1e-9) && c.lat.0 < 0.0 && c.lat.1 > 0.0 {
        p.key("equator", 2);
    }
    p.mark(mx, my, &mlab);
    let body = p.finish(c.xlab, c.ylab, "");
    let params: Vec<String> = c.params.iter().filter_map(|k| given(s, k).map(|v| format!("{k} = {}", in_text(&v)))).collect();
    let extra = match s.key {
        "geo_mercator_forward" | "geo_polar_stereographic_forward" => {
            s.out("k").map(|k_| format!(" Scale factor at the point k = {}.", n6(k_))).unwrap_or_default()
        }
        "geo_albers_equal_area_forward" => format!(" Scale factors at the point: k = {} along the parallel, h = {} along the meridian.",
                                                   n6(s.out("k")?), n6(s.out("h")?)),
        "geo_lambert_conformal_conic_forward" | "geo_transverse_mercator_forward" | "geo_utm_forward" => {
            format!(" Scale factor k = {}, convergence γ = {}.", n6(s.out("k")?), dg(s.out("gamma")?))
        }
        _ => String::new(),
    };
    let hidden = match c.keep {
        Keep::Within(r, ..) if r < 170.0 => format!(" (points more than {}° from the centre left out)", n6(r)),
        Keep::Zone(..) => " (points outside the zone left out)".to_string(),
        _ => String::new(),
    };
    let marked = if inverse {
        format!("Marked: the stated point x = {} m, y = {} m, at the returned latitude {} and longitude {}, where it falls \
                 among the lines.", n6(mx * 1000.0), n6(my * 1000.0), dg(s.out("lat")?), dg(s.out("lon")?))
    } else {
        format!("Marked: latitude {}, longitude {} → {} = {} m, {} = {} m.{extra}", dg(s.inp("lat")?), dg(s.inp("lon")?),
                c.xo, n6(mx * 1000.0), c.yo, n6(my * 1000.0))
    };
    let kind = if s.key == "geo_mollweide_forward" { "projection" } else { "graticule" };
    Some(s.chart(kind, &c.what,
                 &format!("{}: parallels every {}° and meridians every {}°{}, each point projected by the model's forward \
                           projection{} ({} points){hidden}. {marked}", c.what, n6(c.par), n6(c.mer),
                          if std_n > 0 { ", with the standard parallels dashed" } else { "" },
                          if params.is_empty() { String::new() } else { format!(" with {}", params.join(", ")) }, n),
                 &body))
}

/// An input as the caption states it.
pub(crate) fn in_text(v: &Input) -> String {
    match v {
        Input::Text(t) => t.clone(),
        Input::Int(i) => i.to_string(),
        Input::Float(x) => n6(*x),
        Input::Json(j) => j.to_string(),
    }
}

/// Points round the centre: their azimuth and geodesic distance as the model returns them (the stated point and
/// points at its latitude every 30° of longitude).
pub fn azimuth_compass(s: &Solved) -> Option<Figure> {
    let (lat, lon, d0, az0) = (s.inp("lat")?, s.inp("lon")?, s.out("distance")?, s.out("azimuth")?);
    let (l0, w0) = (s.inp("lat0")?, s.inp("lon0")?);
    let mut pts: Vec<(String, f64, f64)> = vec![];
    for k_ in 1..12 {
        let lo = lon + 30.0 * k_ as f64;
        let lo = if lo > 180.0 { lo - 360.0 } else { lo };
        if let Some(a) = s.with(&[("lon", lo)]) {
            if let (Some(d), Some(az)) = (o(&a, "distance"), o(&a, "azimuth")) {
                if d > 1.0 {
                    pts.push((format!("{}°", n4(lo)), d, az));
                }
            }
        }
    }
    let dmax = pts.iter().map(|p| p.1).fold(d0, f64::max);
    if !(dmax > 0.0) {
        return None;
    }
    let mut c = Canvas::new(440.0);
    let (cx, cy, rr) = (230.0, 236.0, 170.0);
    // distance rings: a round step in km
    let km = dmax / 1000.0;
    let step = [500.0, 1000.0, 2000.0, 2500.0, 5000.0, 10000.0].iter().copied().find(|st| km / st <= 4.0).unwrap_or(10000.0);
    let rmax = (km / step).ceil() * step;
    let mut r = step;
    while r <= rmax + 1e-9 {
        let pr = rr * r / rmax;
        c.raw(&format!("<circle class=\"axp-c4\" cx=\"{cx}\" cy=\"{cy}\" r=\"{pr:.1}\" stroke-dasharray=\"4 4\"/>"));
        // the ring's distance written along the south-east, clear of the compass labels
        let a = 140f64.to_radians();
        c.name(cx + pr * a.sin() + 4.0, cy - pr * a.cos() + 14.0, "start", &format!("{} km", n6(r)));
        r += step;
    }
    for (i, lab) in ["N", "30°", "60°", "E", "120°", "150°", "S", "210°", "240°", "W", "300°", "330°"].iter().enumerate() {
        let a = (i as f64 * 30.0).to_radians();
        let (x, y) = (cx + (rr + 16.0) * a.sin(), cy - (rr + 16.0) * a.cos() + 5.0);
        c.name(x, y, "middle", lab);
    }
    for (lab, d, az) in &pts {
        let a = az.to_radians();
        let pr = rr * d / 1000.0 / rmax;
        c.raw(&format!("<path class=\"axp-c3\" d=\"M{cx},{cy} L{:.1},{:.1}\"/>", cx + pr * a.sin(), cy - pr * a.cos()));
        c.raw(&format!("<circle class=\"axp-d3\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.2\"/>", cx + pr * a.sin(), cy - pr * a.cos()));
        let anchor = if a.sin() >= 0.0 { "start" } else { "end" };
        let dx = if a.sin() >= 0.0 { 6.0 } else { -6.0 };
        c.name(cx + pr * a.sin() + dx, cy - pr * a.cos() + 4.0, anchor, lab);
    }
    let a = az0.to_radians();
    let pr = rr * d0 / 1000.0 / rmax;
    let (tx, ty) = (cx + pr * a.sin(), cy - pr * a.cos());
    c.raw(&format!("<path class=\"axp-c\" d=\"M{cx},{cy} L{tx:.1},{ty:.1}\"/><circle class=\"axp-m\" cx=\"{tx:.1}\" \
                    cy=\"{ty:.1}\" r=\"6\"/>"));
    c.dot((cx, cy));
    c.bold(16.0, 22.0, "start", &format!("centre {}, {}", dg(l0), dg(w0)));
    c.value(16.0, 40.0, "start", &format!("{}, {}: {} km at {}", dg(lat), dg(lon), n6(d0 / 1000.0), dg(az0)));
    c.name(504.0, 22.0, "end", "azimuth from north, geodesic distance");
    let list: Vec<String> = pts.iter().map(|(l, d, az)| format!("longitude {l}: {} km at {}", n4(d / 1000.0), dg(*az))).collect();
    Some(s.figure("compass", "Seen from the centre: azimuth and distance",
                  &format!("Points at latitude {} seen from {}, {} as the model computes them: each line points along \
                            the initial azimuth, clockwise from north, its length the geodesic distance (an azimuthal \
                            equidistant view). Marked: longitude {}, {} km at {}. Others: {}.", dg(lat), dg(l0), dg(w0),
                           dg(lon), n6(d0 / 1000.0), dg(az0), list.join("; ")), c.body(), c.h))
}

/// Tissot ellipses (parallel scale k across, meridian scale h up) along the central meridian for every projection the
/// capability offers, the requested one ringed at its latitude.
pub fn tissot(s: &Solved) -> Option<Figure> {
    let proj = s.txt("projection")?;
    let lat = s.inp("lat")?;
    let spec = s.p.procedure(s.key)?.inputs.iter().find(|i| i.symbol == "projection")?.choices.clone();
    let lats = [0.0, 20.0, 40.0, 60.0];
    let mut rows: Vec<(String, Vec<Option<(f64, f64)>>)> = vec![];
    for pj in spec.iter().take(6) {
        let mut v = vec![];
        for la in lats.iter().chain(std::iter::once(&lat)) {
            let a = s.solve_other(s.key, &[("projection", Input::Text(pj.clone())), ("lat", fl(*la)), ("lon", fl(0.0))]);
            v.push(a.and_then(|a| Some((o(&a, "k")?, o(&a, "h")?))).filter(|q| q.0 > 0.0 && q.1 > 0.0));
        }
        rows.push((pj.clone(), v));
    }
    let big = rows.iter().flat_map(|r| r.1.iter().flatten()).map(|q| q.0.max(q.1)).fold(1.0, f64::max);
    let unit = (27.0 / big).clamp(8.0, 27.0);
    let h = 76.0 + 64.0 * rows.len() as f64;
    let mut c = Canvas::new(h);
    let x0 = 160.0;
    let cw = 68.0;
    for (j, la) in lats.iter().enumerate() {
        c.name(x0 + cw * (j as f64 + 0.5), 46.0, "middle", &format!("{}°", n6(*la)));
    }
    c.bold(x0 + cw * 4.5 + 4.0, 46.0, "middle", &dg(lat));
    let mine_row = rows.iter().position(|r| r.0.eq_ignore_ascii_case(&proj));
    for (i, (name, v)) in rows.iter().enumerate() {
        let cy = 90.0 + 64.0 * i as f64;
        c.name(x0 - 10.0, cy + 5.0, "end", &name.replace('_', " "));
        for (j, q) in v.iter().enumerate() {
            let cx = x0 + cw * (j as f64 + 0.5) + if j == 4 { 4.0 } else { 0.0 };
            if j == 4 && Some(i) != mine_row {
                continue;
            }
            c.raw(&format!("<circle class=\"axp-c4\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{unit:.1}\" stroke-dasharray=\"3 3\"/>"));
            if let Some((kk, hh)) = q {
                let cls = if j == 4 { "axp-c" } else { "axp-c3" };
                c.raw(&format!("<ellipse class=\"{cls}\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" rx=\"{:.1}\" ry=\"{:.1}\"/>",
                               kk * unit, hh * unit));
                if j == 4 {
                    c.raw(&format!("<circle class=\"axp-m\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"3\"/>"));
                }
            }
        }
    }
    let (kk, hh) = (s.out("k")?, s.out("h")?);
    c.bold(16.0, 22.0, "start", &format!("{} at {}: h = {}, k = {}, areal scale {}", proj.replace('_', " "), dg(lat), n4(hh),
                                         n4(kk), n4(s.out("s")?)));
    c.name(504.0, h - 8.0, "end", "dashed: scale 1; across k, up h");
    Some(s.figure("ellipses", "Tissot ellipses along the central meridian",
                  &format!("Tissot's indicatrix on the central meridian at latitudes 0°, 20°, 40° and 60° for each projection \
                            the model offers: each ellipse has the returned parallel scale k as its horizontal semi-axis and \
                            the meridian scale h as its vertical one (perpendicular on the central meridian), the dashed \
                            circle is scale 1, all at one scale. Right column, ringed: {} at latitude {} on the central \
                            meridian, h = {}, k = {}. The returned values for the stated longitude are h = {}, k = {}, \
                            angle between them {}.", proj.replace('_', " "), dg(lat),
                           n6(rows.get(mine_row?)?.1[4]?.1), n6(rows.get(mine_row?)?.1[4]?.0), n6(hh), n6(kk),
                           dg(s.out("theta_prime")?)), c.body(), c.h))
}

/// UTM zones and latitude bands round the point: every label is the zone and band the model returns for the cell's
/// centre.
pub fn utm_zones(s: &Solved) -> Option<Figure> {
    let (lat, lon) = (s.inp("lat")?, s.inp("lon")?);
    let (zone, cm) = (s.out("zone")?, s.out("central_meridian")?);
    let band = s.out_text("band")?;
    let z0 = cm - 3.0;
    let b0 = (lat / 8.0).floor() * 8.0;
    let mut cells = vec![];
    for i in -3..=3 {
        for j in -2..=2 {
            let (x, y) = (z0 + 6.0 * i as f64 + 3.0, b0 + 8.0 * j as f64 + 4.0);
            if !(-80.0..84.0).contains(&y) {
                continue;
            }
            let x = if x > 180.0 { x - 360.0 } else if x < -180.0 { x + 360.0 } else { x };
            if let Some(a) = s.with(&[("lat", y), ("lon", x)]) {
                if let (Some(z), Some(b)) = (o(&a, "zone"), ot(&a, "band")) {
                    cells.push((z0 + 6.0 * i as f64 + 3.0, y, format!("{}{}", n6(z), b)));
                }
            }
        }
    }
    if cells.len() < 4 {
        return None;
    }
    let xs: Vec<f64> = cells.iter().flat_map(|c| [c.0 - 3.0, c.0 + 3.0]).collect();
    let ys: Vec<f64> = cells.iter().flat_map(|c| [c.1 - 4.0, c.1 + 4.0]).collect();
    let mut p = Plot::new(&xs, &ys, false, false)?;
    let (xl, xh) = (xs.iter().cloned().fold(f64::MAX, f64::min), xs.iter().cloned().fold(f64::MIN, f64::max));
    let (yl, yh) = (ys.iter().cloned().fold(f64::MAX, f64::min), ys.iter().cloned().fold(f64::MIN, f64::max));
    let mut x = xl;
    while x <= xh + 1e-9 {
        p.line(&[(x, yl), (x, yh)], 3, "");
        x += 6.0;
    }
    let mut y = yl;
    while y <= yh + 1e-9 {
        p.line(&[(xl, y), (xh, y)], 3, "");
        y += 8.0;
    }
    for (x, y, t) in &cells {
        p.label(*x, *y, 0.0, 5.0, "middle", "axp-lt", t);
    }
    p.dashed(&[(cm, yl), (cm, yh)], 1, &format!("central meridian {}", dg(cm)));
    p.mark(lon, lat, "");
    let body = p.finish("longitude [°]", "latitude [°]", &format!("{}, {}: zone {}{}", dg(lat), dg(lon), n6(zone), band));
    Some(s.chart("zones", "UTM zones and latitude bands round the point",
                 &format!("UTM zones (6° of longitude) and latitude bands (8°) round latitude {}, longitude {}: every label \
                           is the zone number and band letter the model returns for the cell's centre ({} runs); the grid \
                           lines are drawn at the 6° and 8° spacing, so a special zone shows where a label differs from its \
                           cell. Marked: zone {}, band {} ({}), central meridian {}.", dg(lat), dg(lon), cells.len(),
                          n6(zone), band, s.out_text("hemisphere")?, dg(cm)), &body))
}

/// The two letters of an MGRS code's 100 km square ("32UNA7166639109" -> "NA") and its zone.
fn mgrs_parts(code: &str) -> Option<(u32, String)> {
    let t = code.trim();
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    let rest: Vec<char> = t[digits.len()..].chars().collect();
    if digits.is_empty() || rest.len() < 3 {
        return None;
    }
    Some((digits.parse().ok()?, rest[1..3].iter().collect()))
}

pub fn mgrs_squares(s: &Solved) -> Option<Figure> {
    let (lat, lon) = (s.inp("lat")?, s.inp("lon")?);
    let code = s.out_text("mgrs")?;
    let (zone, sq) = mgrs_parts(&code)?;
    let w0 = zone as f64 * 6.0 - 186.0;
    let b0 = (lat / 8.0).floor() * 8.0;
    let mut pts = vec![];
    for i in 0..6 {
        for j in 0..8 {
            let (x, y) = (w0 + 0.5 + i as f64, b0 + 0.5 + j as f64);
            if let Some(a) = s.with(&[("lat", y), ("lon", x)]) {
                if let Some((_, l)) = ot(&a, "mgrs").and_then(|t| mgrs_parts(&t)) {
                    pts.push((x, y, l));
                }
            }
        }
    }
    if pts.len() < 8 {
        return None;
    }
    let mut p = Plot::new(&[w0, w0 + 6.0, lon], &[b0, b0 + 8.0, lat], false, false)?;
    for (x, y, l) in &pts {
        let cl = if *l == sq { "axp-vt" } else { "axp-lt" };
        p.label(*x, *y, 0.0, 5.0, "middle", cl, l);
    }
    p.dashed(&[(w0 + 3.0, b0), (w0 + 3.0, b0 + 8.0)], 3, &format!("central meridian of zone {zone}"));
    p.mark(lon, lat, "");
    let body = p.finish("longitude [°]", "latitude [°]", &code);
    Some(s.chart("zones", "MGRS 100 km squares in the grid zone",
                 &format!("Inside the grid zone of the point ({}° to {}° longitude, {}° to {}° latitude): at points every \
                           1° the model's MGRS code, of which the two letters of the 100 km square are printed ({} runs); \
                           the letters change along easting and northing, not along the graticule. Marked: {}, {} → {} \
                           (square {}).", n6(w0), n6(w0 + 6.0), n6(b0), n6(b0 + 8.0), pts.len(), dg(lat), dg(lon), code, sq),
                 &body))
}

pub fn tiles(s: &Solved) -> Option<Figure> {
    let (lat, lon) = (s.inp("lat")?, s.inp("lon")?);
    let zoom = crate::figures::iv(s, "zoom")?;
    let (tx, ty) = (s.out("tile_x")?, s.out("tile_y")?);
    let quad = s.out_text("quadkey").unwrap_or_default();
    let w = 360.0 / 2f64.powf(zoom);
    let h = w * lat.to_radians().cos().max(0.05);
    let n = 18usize;
    let (x0, y0) = (lon - 1.6 * w, (lat - 1.6 * h).max(-85.0));
    let (x1, y1) = (lon + 1.6 * w, (lat + 1.6 * h).min(85.0));
    let (dx, dy) = ((x1 - x0) / n as f64, (y1 - y0) / n as f64);
    let mut p = Plot::new(&[x0, x1], &[y0, y1], false, false)?;
    let mut by_tile: BTreeMap<(i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let zi = fl(zoom);
    let mut runs = 0;
    for i in 0..n {
        for j in 0..n {
            let (cx, cy) = (x0 + dx * (i as f64 + 0.5), y0 + dy * (j as f64 + 0.5));
            let a = s.solve_other(s.key, &[("lat", fl(cy)), ("lon", fl(cx)), ("zoom", zi.clone())]);
            let Some((ax, ay)) = a.and_then(|a| Some((o(&a, "tile_x")?, o(&a, "tile_y")?))) else { continue };
            runs += 1;
            let mine = ax == tx && ay == ty;
            let odd = (ax + ay) as i64 % 2 != 0;
            p.cell(cx - dx / 2.0, cy - dy / 2.0, cx + dx / 2.0, cy + dy / 2.0, if mine { 0.55 } else if odd { 0.3 } else { 0.12 },
                   !mine);
            let e = by_tile.entry((ax as i64, ay as i64)).or_insert((0.0, 0.0, 0));
            e.0 += cx;
            e.1 += cy;
            e.2 += 1;
        }
    }
    if by_tile.len() < 2 {
        return None;
    }
    for ((a, b), (sx, sy, k_)) in &by_tile {
        if *k_ >= 14 {
            p.label(sx / *k_ as f64, sy / *k_ as f64, 0.0, 5.0, "middle", "axp-lt", &format!("({a}, {b})"));
        }
    }
    p.key(&format!("tile ({}, {})", n6(tx), n6(ty)), 0);
    p.key("other tiles (by x + y)", 2);
    p.mark(lon, lat, &format!("quadkey {quad}"));
    let body = p.finish("longitude [°]", "latitude [°]", "");
    Some(s.chart("tiles", &format!("Web-map tiles at zoom {} round the point", n6(zoom)),
                 &format!("Web-map tiles at zoom {} round latitude {}, longitude {}: each small cell is shaded by the tile \
                           the model returns for its centre ({} runs), alternately by the parity of tile x + y so the tile \
                           boundaries show. Marked: tile x = {}, y = {} (quadkey {}).", n6(zoom), dg(lat), dg(lon), runs,
                          n6(tx), n6(ty), quad), &body))
}

pub fn geohash_cells(s: &Solved) -> Option<Figure> {
    let code = s.txt("geohash")?.trim().to_string();
    let n = code.chars().count();
    if n == 0 {
        return None;
    }
    let mut cells: Vec<(String, f64, f64, f64, f64)> = vec![];
    for k_ in 1..=n.min(5) {
        let pre: String = code.chars().take(k_).collect();
        let a = s.solve_other(s.key, &[("geohash", Input::Text(pre.clone()))])?;
        cells.push((pre, o(&a, "lat_center")?, o(&a, "lon_center")?, o(&a, "lat_error")?, o(&a, "lon_error")?));
    }
    let (la, lo) = (s.out("lat_center")?, s.out("lon_center")?);
    let panel = |sel: &[(String, f64, f64, f64, f64)]| -> Option<String> {
        let xs: Vec<f64> = sel.iter().flat_map(|c| [c.2 - c.4, c.2 + c.4]).collect();
        let ys: Vec<f64> = sel.iter().flat_map(|c| [c.1 - c.3, c.1 + c.3]).collect();
        let mut p = Plot::new(&xs, &ys, false, false)?;
        for (i, c) in sel.iter().enumerate() {
            let col = [3, 1, 0][i % 3];
            p.polygon(&[(c.2 - c.4, c.1 - c.3), (c.2 + c.4, c.1 - c.3), (c.2 + c.4, c.1 + c.3), (c.2 - c.4, c.1 + c.3)], col,
                      &format!("'{}'", c.0));
            p.label(c.2 - c.4, c.1 + c.3, 4.0, 14.0, "start", "axp-bt", &format!("'{}'", c.0));
        }
        p.dots(&[(lo, la)], 0, "");
        Some(p.finish("longitude [°]", "latitude [°]", ""))
    };
    let top = panel(&cells[..cells.len().min(3)])?;
    let mut panels = vec![top];
    if cells.len() >= 4 {
        panels.push(panel(&cells[2..])?);
    }
    let (body, h) = k::stack(&panels);
    let desc: Vec<String> = cells.iter().map(|c| format!("'{}': {} ± {}, {} ± {}", c.0, dg(c.1), n6(c.3), dg(c.2), n6(c.4)))
        .collect();
    Some(s.figure("cells", "Nested geohash cells of one code",
                  &format!("The geohash '{}' read character by character: each prefix decoded by the model is a cell \
                            (centre ± returned error), and every added character subdivides the previous cell ({}). The dot \
                            is the full code's centre, {}, {} (error ± {}, ± {}).", code, desc.join("; "), dg(la), dg(lo),
                           n6(s.out("lat_error")?), n6(s.out("lon_error")?)), &body, h))
}
