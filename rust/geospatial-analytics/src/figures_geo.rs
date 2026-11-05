//! Geodesy: geodesic and rhumb paths, geodesic circles, polygons moved toward the pole, the sky seen from a station,
//! Earth-centred and local frames, degree scales and two-input value maps. Every point is a run of the model.

use crate::figures::{agrees, dg, fl, given, n4, n6, o, pass};
use crate::figures_plot::Plot;
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

fn ell(s: &Solved) -> Vec<(&'static str, Input)> {
    given(s, "ellipsoid").map(|e| vec![("ellipsoid", e)]).unwrap_or_default()
}

/// A geodesic destination (lat, lon) from a start along an azimuth for a distance (m).
fn geo_dest(s: &Solved, from: (f64, f64), az: f64, d: f64) -> Option<(f64, f64)> {
    let mut v = vec![("lat1", fl(from.0)), ("lon1", fl(from.1)), ("azimuth", fl(az)), ("distance", fl(d))];
    v.extend(ell(s));
    let a = s.solve_other("geo_geodesic_destination", &v)?;
    Some((o(&a, "lat2")?, o(&a, "lon2")?))
}

/// A rhumb-line destination.
fn rhumb_dest(s: &Solved, from: (f64, f64), bearing: f64, d: f64) -> Option<(f64, f64)> {
    let mut v = vec![("lat1", fl(from.0)), ("lon1", fl(from.1)), ("bearing", fl(bearing)), ("distance", fl(d))];
    v.extend(ell(s));
    let a = s.solve_other("geo_rhumb_line_destination", &v)?;
    Some((o(&a, "lat2")?, o(&a, "lon2")?))
}

/// The geodesic from a point to another: (azimuth, distance) by the azimuthal equidistant projection centred on it.
fn geo_inverse(s: &Solved, from: (f64, f64), to: (f64, f64)) -> Option<(f64, f64)> {
    let mut v = vec![("lat", fl(to.0)), ("lon", fl(to.1)), ("lat0", fl(from.0)), ("lon0", fl(from.1))];
    v.extend(ell(s));
    let a = s.solve_other("geo_azimuthal_equidistant_forward", &v)?;
    Some((o(&a, "azimuth")?, o(&a, "distance")?))
}

/// The rhumb line between two points: (bearing, distance).
fn rhumb_inverse(s: &Solved, from: (f64, f64), to: (f64, f64)) -> Option<(f64, f64)> {
    let mut v = vec![("lat1", fl(from.0)), ("lon1", fl(from.1)), ("lat2", fl(to.0)), ("lon2", fl(to.1))];
    v.extend(ell(s));
    let a = s.solve_other("geo_rhumb_line_inverse", &v)?;
    Some((o(&a, "bearing")?, o(&a, "distance")?))
}

/// Mercator (x, y) in km of a point (central meridian `lon0`).
fn merc(s: &Solved, p: (f64, f64), lon0: f64) -> Option<(f64, f64)> {
    let mut v = vec![("lat", fl(p.0)), ("lon", fl(p.1)), ("lon0", fl(lon0)), ("lat_ts", fl(0.0))];
    v.extend(ell(s));
    let a = s.solve_other("geo_mercator_forward", &v)?;
    Some((o(&a, "x")? / 1000.0, o(&a, "y")? / 1000.0))
}

/// A longitude brought next to a reference (no jump of 360° along a path).
fn unwrap(lon: f64, near: f64) -> f64 {
    let mut l = lon;
    while l - near > 180.0 {
        l -= 360.0;
    }
    while near - l > 180.0 {
        l += 360.0;
    }
    l
}

fn path_ll(pts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = vec![];
    for p in pts {
        let lon = out.last().map(|l: &(f64, f64)| unwrap(p.1, l.0)).unwrap_or(p.1);
        out.push((lon, p.0));
    }
    out
}

pub fn destination_paths(s: &Solved) -> Option<Figure> {
    let (la, lo, az, d) = (s.inp("lat1")?, s.inp("lon1")?, s.inp("azimuth")?, s.inp("distance")?);
    let end = (s.out("lat2")?, s.out("lon2")?);
    if !(d > 0.0) {
        return None;
    }
    let (rb, rd) = rhumb_inverse(s, (la, lo), end)?;
    let n = 14;
    let lon0 = lo.round();
    let mut geo = vec![merc(s, (la, lo), lon0)?];
    let mut rh = geo.clone();
    let mut last = (la, lo);
    for i in 1..=n {
        let f = i as f64 / n as f64;
        if let Some(q) = geo_dest(s, (la, lo), az, d * f) {
            let q = (q.0, unwrap(q.1, last.1));
            last = q;
            geo.push(merc(s, q, lon0).unwrap_or((f64::NAN, f64::NAN)));
        }
        if let Some(q) = rhumb_dest(s, (la, lo), rb, rd * f) {
            rh.push(merc(s, (q.0, unwrap(q.1, lo)), lon0).unwrap_or((f64::NAN, f64::NAN)));
        }
    }
    if geo.len() < 4 || rh.len() < 4 {
        return None;
    }
    let pts: Vec<(f64, f64)> = geo.iter().chain(rh.iter()).copied().filter(|p| p.0.is_finite()).collect();
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let mut p = Plot::equal(&xs, &ys)?;
    p.line(&rh, 1, &format!("rhumb line, {} km", n4(rd / 1000.0)));
    p.line(&geo, 0, &format!("geodesic, {} km", n4(d / 1000.0)));
    p.tag(geo[0].0, geo[0].1, "start");
    let e = *geo.last()?;
    p.mark(e.0, e.1, &format!("{}, {}", dg(end.0), dg(end.1)));
    let body = p.finish("Mercator x [km]", "Mercator y [km]", "");
    Some(s.chart("map", "Geodesic against rhumb line to the destination",
                 &format!("From latitude {}, longitude {} the geodesic leaves at azimuth {} for {} km and arrives at {}, {} \
                           (arriving azimuth {}); the rhumb line to the same point keeps bearing {} and is {} km. Each path \
                           is {} destination points computed by the model and projected by its Mercator projection, on which \
                           the rhumb line is straight.", dg(la), dg(lo), dg(az), n6(d / 1000.0), dg(end.0), dg(end.1),
                          dg(s.out("azimuth2")?), dg(rb), n6(rd / 1000.0), n), &body))
}

pub fn destination_circles(s: &Solved) -> Option<Figure> {
    let (la, lo, az, d) = (s.inp("lat1")?, s.inp("lon1")?, s.inp("azimuth")?, s.inp("distance")?);
    let end = (s.out("lat2")?, s.out("lon2")?);
    if !(d > 0.0) {
        return None;
    }
    let mut series = vec![];
    let mut runs = 0;
    for (i, f) in [1.0, 2.0, 4.0].iter().enumerate() {
        let mut pts = vec![];
        for k_ in 0..=36 {
            let a = 10.0 * k_ as f64;
            match geo_dest(s, (la, lo), a, d * f) {
                Some(q) => {
                    runs += 1;
                    pts.push((unwrap(q.1, lo), q.0));
                }
                None => pts.push((f64::NAN, f64::NAN)),
            }
        }
        if pts.iter().filter(|p| p.0.is_finite()).count() >= 6 {
            series.push((i, *f, pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let xs: Vec<f64> = series.iter().flat_map(|s_| s_.2.iter().map(|p| p.0)).chain([lo]).collect();
    let ys: Vec<f64> = series.iter().flat_map(|s_| s_.2.iter().map(|p| p.1)).chain([la]).collect();
    let mut p = Plot::new(&xs, &ys, false, false)?;
    for (i, f, pts) in &series {
        p.line(pts, [3, 2, 1][*i], &format!("{} km in every direction", n4(d * f / 1000.0)));
    }
    p.line(&[(lo, la), (unwrap(end.1, lo), end.0)], 0, "");
    p.dots(&[(lo, la)], 0, "");
    p.mark(unwrap(end.1, lo), end.0, &format!("{}, {}", dg(end.0), dg(end.1)));
    let body = p.finish("longitude [°]", "latitude [°]", "");
    Some(s.chart("map-2", "Geodesic circles round the starting point",
                 &format!("Destinations from latitude {}, longitude {} at every 10° of azimuth for distances {} km, {} km \
                           and {} km, each point a run of the model ({} runs), drawn in longitude and latitude (a circle \
                           widens east to west where a degree of longitude is short). Marked: azimuth {} for {} km → {}, {}, \
                           arriving at azimuth {}.", dg(la), dg(lo), n6(d / 1000.0), n6(2.0 * d / 1000.0),
                          n6(4.0 * d / 1000.0), runs, dg(az), n6(d / 1000.0), dg(end.0), dg(end.1), dg(s.out("azimuth2")?)),
                 &body))
}

pub fn midpoint_path(s: &Solved) -> Option<Figure> {
    let (a1, w1, a2, w2) = (s.inp("lat1")?, s.inp("lon1")?, s.inp("lat2")?, s.inp("lon2")?);
    let (ml, mw, dist) = (s.out("lat_mid")?, s.out("lon_mid")?, s.out("distance")?);
    let (az, d) = geo_inverse(s, (a1, w1), (a2, w2))?;
    if !agrees(d, dist) || !(d > 0.0) {
        return None;
    }
    let n = 20;
    let mut pts = vec![(a1, w1)];
    for i in 1..=n {
        pts.push(geo_dest(s, (a1, w1), az, d * i as f64 / n as f64)?);
    }
    let path = path_ll(&pts);
    let xs: Vec<f64> = path.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = path.iter().map(|p| p.1).collect();
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line(&path, 0, &format!("geodesic, {} km", n4(d / 1000.0)));
    p.dots(&[path[0], path[n]], 0, "");
    p.tag(path[0].0, path[0].1, &format!("{}, {}", dg(a1), dg(w1)));
    p.tag(path[n].0, path[n].1, &format!("{}, {}", dg(a2), dg(w2)));
    p.mark(unwrap(mw, path[n / 2].0), ml, &format!("midpoint {}, {}", dg(ml), dg(mw)));
    let body = p.finish("longitude [°]", "latitude [°]", "");
    Some(s.chart("map", "The geodesic and its midpoint",
                 &format!("The geodesic from {}, {} to {}, {} drawn in longitude and latitude: {} destination points along \
                           the initial azimuth {} that the model's azimuthal projection returns for the pair (its distance \
                           {} km agrees with the returned {} km). Marked: the midpoint the model computes, {}, {}; on these \
                           axes the geodesic bows toward the pole.", dg(a1), dg(w1), dg(a2), dg(w2), n, dg(az),
                          n6(d / 1000.0), n6(dist / 1000.0), dg(ml), dg(mw)), &body))
}

pub fn polygon_shifts(s: &Solved) -> Option<Figure> {
    let (lats, lons) = (s.vec("lats")?, s.vec("lons")?);
    if lats.len() != lons.len() || lats.len() < 3 {
        return None;
    }
    let mean = lats.iter().sum::<f64>() / lats.len() as f64;
    let sign = if mean >= 0.0 { 1.0 } else { -1.0 };
    let mut shapes = vec![];
    for sh in [0.0, 30.0, 60.0] {
        let l2: Vec<f64> = lats.iter().map(|x| x + sign * sh).collect();
        if l2.iter().any(|x| x.abs() > 89.0) {
            continue;
        }
        let a = if sh == 0.0 {
            (s.out("area")?, s.out("perimeter")?)
        } else {
            let v = pass(s, &["ellipsoid"], vec![("lats", crate::figures::jl(&l2)), ("lons", crate::figures::jl(&lons))]);
            let b = s.solve_other(s.key, &v)?;
            (o(&b, "area")?, o(&b, "perimeter")?)
        };
        shapes.push((sh, l2, a));
    }
    // one panel per position, each polygon in degrees at the same scale; the bar under it is its area
    let mut c = Canvas::new(300.0);
    let (xl, xh) = (lons.iter().cloned().fold(f64::MAX, f64::min), lons.iter().cloned().fold(f64::MIN, f64::max));
    let (yl, yh) = (lats.iter().cloned().fold(f64::MAX, f64::min), lats.iter().cloned().fold(f64::MIN, f64::max));
    let sc = 120.0 / (xh - xl).max(yh - yl).max(1e-12);
    let amax = shapes.iter().map(|x| (x.2).0).fold(0.0, f64::max);
    let cw = 520.0 / shapes.len() as f64;
    for (i, (sh, l2, (ar, pe))) in shapes.iter().enumerate() {
        let cx = cw * (i as f64 + 0.5);
        let (ox, oy) = (cx - sc * (xh - xl) / 2.0, 70.0 + sc * (yh - yl) / 2.0 + 60.0);
        let mut d = String::new();
        for (j, (u, v)) in lons.iter().zip(l2).enumerate() {
            d.push_str(&format!("{}{:.1},{:.1} ", if j == 0 { "M" } else { "L" }, ox + sc * (u - xl),
                                oy - sc * (v - sign * sh - yl)));
        }
        let cl = [("axp-s", "axp-c"), ("axp-s2", "axp-c2"), ("axp-s3", "axp-c3")][i % 3];
        c.raw(&format!("<path class=\"{}\" d=\"{d}Z\"/><path class=\"{}\" d=\"{d}Z\"/>", cl.0, cl.1));
        let lo = l2.iter().cloned().fold(f64::MAX, f64::min);
        let hi = l2.iter().cloned().fold(f64::MIN, f64::max);
        c.bold(cx, 40.0, "middle", &format!("latitudes {}° to {}°", n4(lo), n4(hi)));
        let bw = if amax > 0.0 { (cw - 30.0) * ar / amax } else { 0.0 };
        c.raw(&format!("<rect class=\"{}\" x=\"{:.1}\" y=\"236\" width=\"{bw:.1}\" height=\"12\"/>",
                       ["axp-b", "axp-b2", "axp-b3"][i % 3], cx - (cw - 30.0) / 2.0));
        c.value(cx, 226.0, "middle", &format!("{} km²", n4(ar / 1e6)));
        c.name(cx, 268.0, "middle", &format!("perimeter {} km", n4(pe / 1000.0)));
    }
    c.name(504.0, 292.0, "end", "the same shape in degrees; bars: the area");
    let list: Vec<String> = shapes.iter()
        .map(|(sh, _, (ar, pe))| format!("moved {}°: {} km², perimeter {} km", n6(*sh), n6(ar / 1e6), n6(pe / 1000.0)))
        .collect();
    Some(s.figure("polygons", "Geodesic area of one shape at three latitudes",
                 &format!("The stated polygon (latitudes {}, longitudes {}) and the same shape moved 30° and 60° toward the \
                           pole, each with the geodesic area and perimeter the model returns: {}. The same span of degrees \
                           encloses less ground toward the pole.", crate::figures::lst(&lats), crate::figures::lst(&lons),
                          list.join("; ")), c.body(), c.h))
}

pub fn rhumb_rays(s: &Solved) -> Option<Figure> {
    let (la, lo, br, d) = (s.inp("lat1")?, s.inp("lon1")?, s.inp("bearing")?, s.inp("distance")?);
    let end = (s.out("lat2")?, s.out("lon2")?);
    if !(d > 0.0) {
        return None;
    }
    let lon0 = lo.round();
    let start = merc(s, (la, lo), lon0)?;
    let reach = 3.0 * d;
    let mut rays = vec![];
    for k_ in 0..12 {
        let b = 30.0 * k_ as f64;
        let mut pts = vec![start];
        for i in 1..=5 {
            match rhumb_dest(s, (la, lo), b, reach * i as f64 / 5.0).and_then(|q| merc(s, (q.0, unwrap(q.1, lo)), lon0)) {
                Some(q) => pts.push(q),
                None => break,
            }
        }
        if pts.len() >= 2 {
            rays.push((b, pts));
        }
    }
    let mine = merc(s, (end.0, unwrap(end.1, lo)), lon0)?;
    if rays.len() < 4 {
        return None;
    }
    let xs: Vec<f64> = rays.iter().flat_map(|r| r.1.iter().map(|p| p.0)).chain([mine.0]).collect();
    let ys: Vec<f64> = rays.iter().flat_map(|r| r.1.iter().map(|p| p.1)).chain([mine.1]).collect();
    let mut p = Plot::equal(&xs, &ys)?;
    for (b, pts) in &rays {
        p.line(pts, 3, &format!("bearings every 30°, {} km", n4(reach / 1000.0)));
        let e = *pts.last().unwrap();
        let (sx, sy) = (b.to_radians().sin(), b.to_radians().cos());
        p.label(e.0, e.1, 10.0 * sx, -10.0 * sy + 4.0, "middle", "axp-lt", &format!("{}°", n6(*b)));
    }
    p.line(&[start, mine], 0, &format!("bearing {}, {} km", n4(br), n4(d / 1000.0)));
    p.mark(mine.0, mine.1, &format!("{}, {}", dg(end.0), dg(end.1)));
    let body = p.finish("Mercator x [km]", "Mercator y [km]", "");
    let n: usize = rays.iter().map(|r| r.1.len() - 1).sum();
    Some(s.chart("map", "Rhumb lines are straight on a Mercator map",
                 &format!("Rhumb lines from latitude {}, longitude {} at bearings 0° to 330° for {} km, every point a \
                           destination returned by the model and projected by its Mercator projection ({} points): each \
                           constant-bearing path is a straight ray on this map. Marked: bearing {} for {} km → {}, {}.",
                          dg(la), dg(lo), n6(reach / 1000.0), n, dg(br), n6(d / 1000.0), dg(end.0), dg(end.1)), &body))
}

pub fn rhumb_vs_geodesic(s: &Solved) -> Option<Figure> {
    let (a1, w1, a2, w2) = (s.inp("lat1")?, s.inp("lon1")?, s.inp("lat2")?, s.inp("lon2")?);
    let (d, br) = (s.out("distance")?, s.out("bearing")?);
    if !(d > 0.0) {
        return None;
    }
    let n = 16;
    let mut rh = vec![(a1, w1)];
    for i in 1..=n {
        rh.push(rhumb_dest(s, (a1, w1), br, d * i as f64 / n as f64)?);
    }
    // the path drawn must end at the stated target
    let last = rh[n];
    if (last.0 - a2).abs() > 1e-5 || (unwrap(last.1, w2) - w2).abs() > 1e-5 {
        return None;
    }
    let (gaz, gd) = geo_inverse(s, (a1, w1), (a2, w2))?;
    let mut ge = vec![(a1, w1)];
    for i in 1..=n {
        if let Some(q) = geo_dest(s, (a1, w1), gaz, gd * i as f64 / n as f64) {
            ge.push(q);
        }
    }
    let (rp, gp) = (path_ll(&rh), path_ll(&ge));
    let xs: Vec<f64> = rp.iter().chain(gp.iter()).map(|p| p.0).collect();
    let ys: Vec<f64> = rp.iter().chain(gp.iter()).map(|p| p.1).collect();
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line(&gp, 2, &format!("geodesic, {} km", n4(gd / 1000.0)));
    p.line(&rp, 0, &format!("rhumb line, {} km", n4(d / 1000.0)));
    p.dots(&[rp[0], rp[n]], 0, "");
    p.tag(rp[0].0, rp[0].1, &format!("{}, {}", dg(a1), dg(w1)));
    p.tag(rp[n].0, rp[n].1, &format!("{}, {}", dg(a2), dg(w2)));
    let body = p.finish("longitude [°]", "latitude [°]", "");
    Some(s.chart("map", "Rhumb line against geodesic between two points",
                 &format!("From {}, {} to {}, {}: the model returns the rhumb line's constant bearing {} and length {} km; \
                           the path is drawn from {} rhumb-line destinations along that bearing and ends at the target. For \
                           comparison, the geodesic ({} km at initial azimuth {}, from the model's geodesic runs) is {}.",
                          dg(a1), dg(w1), dg(a2), dg(w2), dg(br), n6(d / 1000.0), n, n6(gd / 1000.0), dg(gaz),
                          if gd < d { "shorter" } else { "no longer" }), &body))
}

/// Targets at the stated latitude and height every 5° of longitude, as seen from the station (zenith at the centre).
pub fn sky(s: &Solved) -> Option<Figure> {
    let (lat, lon, h) = (s.inp("lat")?, s.inp("lon")?, s.inp("h")?);
    let (az0, el0, sr) = (s.out("azimuth")?, s.out("elevation")?, s.out("slant_range")?);
    let mut pts = vec![];
    for k_ in -18..=18 {
        let l = lon + 5.0 * k_ as f64;
        if let Some(a) = s.with(&[("lon", l)]) {
            if let (Some(az), Some(el)) = (o(&a, "azimuth"), o(&a, "elevation")) {
                if el >= 0.0 {
                    pts.push((l, az, el));
                }
            }
        }
    }
    let mut c = Canvas::new(440.0);
    let (cx, cy, rr) = (250.0, 232.0, 175.0);
    for (e, lab) in [(60.0, "elev. 60°"), (30.0, "elev. 30°"), (0.0, "elev. 0°")] {
        let r = rr * (90.0 - e) / 90.0;
        c.raw(&format!("<circle class=\"axp-c4\" cx=\"{cx}\" cy=\"{cy}\" r=\"{r:.1}\" stroke-dasharray=\"4 4\"/>"));
        let a = 40f64.to_radians();
        c.name(cx + r * a.sin() + 4.0, cy - r * a.cos() - 4.0, "start", lab);
    }
    for (i, lab) in ["N", "30°", "60°", "E", "120°", "150°", "S", "210°", "240°", "W", "300°", "330°"].iter().enumerate() {
        let a = (i as f64 * 30.0).to_radians();
        c.name(cx + (rr + 16.0) * a.sin(), cy - (rr + 16.0) * a.cos() + 5.0, "middle", lab);
    }
    let pos = |az: f64, el: f64| {
        let r = rr * (90.0 - el) / 90.0;
        (cx + r * az.to_radians().sin(), cy - r * az.to_radians().cos())
    };
    let mut d = String::new();
    for (i, (_, az, el)) in pts.iter().enumerate() {
        let q = pos(*az, *el);
        d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, q.0, q.1));
    }
    if pts.len() >= 2 {
        c.raw(&format!("<path class=\"axp-c3\" d=\"{}\"/>", d.trim_end()));
    }
    for (l, az, el) in &pts {
        let q = pos(*az, *el);
        c.raw(&format!("<circle class=\"axp-d3\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.6\"/>", q.0, q.1));
        let rel = ((l - lon) / 5.0).round() as i64;
        if rel % 4 == 0 && rel != 0 {
            c.name(q.0, q.1 + 16.0, "middle", &dg(*l));
        }
    }
    let q = pos(az0, el0.max(0.0));
    if el0 >= 0.0 {
        c.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", q.0, q.1));
    }
    c.value(16.0, 40.0, "start", &format!("az {}, el {}", dg(az0), dg(el0)));
    c.bold(16.0, 22.0, "start", &format!("station {}, {}", dg(s.inp("lat0")?), dg(s.inp("lon0")?)));
    c.name(504.0, 22.0, "end", &format!("targets at {}, {} km", dg(lat), n6(h / 1000.0)));
    Some(s.figure("sky", "Targets along the latitude in the station's sky",
                  &format!("A sky plot seen from {}, {}, {} m (zenith at the centre, horizon at the edge, azimuth clockwise \
                            from north): the look angles the model returns for targets at latitude {}, height {} km and \
                            longitudes every 5° from {} to {} (the {} above the horizon). Marked: longitude {}, azimuth {}, \
                            elevation {}, slant range {} km.", dg(s.inp("lat0")?), dg(s.inp("lon0")?), n6(s.inp("h0")?),
                           dg(lat), n6(h / 1000.0), dg(lon - 90.0), dg(lon + 90.0), pts.len(), dg(lon), dg(az0), dg(el0),
                           n6(sr / 1000.0)), c.body(), c.h))
}

/// The Earth-centred coordinates of the point and the frames they live in: the meridian section (traced by the model
/// at every 5° of latitude) and the equatorial plane (every 10° of longitude).
pub fn ecef_frames(s: &Solved) -> Option<Figure> {
    let (lat, lon) = (s.inp("lat")?, s.inp("lon")?);
    let (x, y, z) = (s.out("X")?, s.out("Y")?, s.out("Z")?);
    let mut sec = vec![];
    for (side, l) in [(1.0, lon), (-1.0, lon + 180.0)] {
        for k_ in -18..=18 {
            let la = if side > 0.0 { 5.0 * k_ as f64 } else { -5.0 * k_ as f64 };
            let a = s.with(&[("lat", la), ("lon", l), ("h", 0.0)])?;
            let (ax, ay, az) = (o(&a, "X")?, o(&a, "Y")?, o(&a, "Z")?);
            sec.push((side * (ax * ax + ay * ay).sqrt() / 1000.0, az / 1000.0));
        }
    }
    sec.push(sec[0]);
    let mut eq = vec![];
    for k_ in 0..=36 {
        let a = s.with(&[("lat", 0.0), ("lon", -180.0 + 10.0 * k_ as f64), ("h", 0.0)])?;
        eq.push((o(&a, "X")? / 1000.0, o(&a, "Y")? / 1000.0));
    }
    let pr = (x * x + y * y).sqrt() / 1000.0;
    let (xs, ys): (Vec<f64>, Vec<f64>) = sec.iter().copied().chain([(pr, z / 1000.0)]).unzip();
    let mut p1 = Plot::equal(&xs, &ys)?;
    p1.line(&sec, 3, &format!("meridian of {}", dg(lon)));
    p1.line(&[(0.0, 0.0), (pr, z / 1000.0)], 0, "to the point");
    p1.mark(pr, z / 1000.0, &format!("Z = {} km", n4(z / 1000.0)));
    let top = p1.finish("distance from the polar axis √(X² + Y²) [km]", "Z [km]", "");
    let (xs, ys): (Vec<f64>, Vec<f64>) = eq.iter().copied().chain([(x / 1000.0, y / 1000.0)]).unzip();
    let mut p2 = Plot::equal(&xs, &ys)?;
    p2.line(&eq, 3, "equator");
    p2.line(&[(0.0, 0.0), (x / 1000.0, y / 1000.0)], 0, "the point on the equator plane");
    p2.mark(x / 1000.0, y / 1000.0, &format!("X = {}, Y = {} km", n4(x / 1000.0), n4(y / 1000.0)));
    let bottom = p2.finish("X [km]", "Y [km]", "");
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("frames", "Geodetic point in Earth-centred coordinates",
                  &format!("The point latitude {}, longitude {}, height {} m in Earth-centred, Earth-fixed coordinates as \
                            returned: X = {} m, Y = {} m, Z = {} m. Top: the meridian section through the point (the surface \
                            traced by the model at every 5° of latitude; across, √(X² + Y²) from the returned X and Y). \
                            Bottom: the equatorial plane, the equator traced every 10° of longitude, and the point's X and Y.",
                           dg(lat), dg(lon), n6(s.inp("h")?), n6(x), n6(y), n6(z)), &body, h))
}

/// The point, its foot on the surface and the normal between them in the meridian plane.
pub fn ecef_section(s: &Solved) -> Option<Figure> {
    let (x, y, z) = (s.inp("X")?, s.inp("Y")?, s.inp("Z")?);
    let (lat, lon, h) = (s.out("lat")?, s.out("lon")?, s.out("h")?);
    let to = |la: f64, hh: f64| -> Option<(f64, f64)> {
        let v = pass(s, &["ellipsoid"], vec![("lat", fl(la)), ("lon", fl(lon)), ("h", fl(hh))]);
        let a = s.solve_other("geo_geodetic_to_ecef", &v)?;
        let (ax, ay, az) = (o(&a, "X")?, o(&a, "Y")?, o(&a, "Z")?);
        Some(((ax * ax + ay * ay).sqrt() / 1000.0, az / 1000.0))
    };
    let half = (1.5f64).max(h.abs() / 111_000.0 * 1.5).min(30.0);
    let mut surf = vec![];
    for i in 0..=12 {
        let la = (lat - half + 2.0 * half * i as f64 / 12.0).clamp(-90.0, 90.0);
        surf.push(to(la, 0.0)?);
    }
    let foot = to(lat, 0.0)?;
    let back = to(lat, h)?;
    let pt = ((x * x + y * y).sqrt() / 1000.0, z / 1000.0);
    // the returned latitude and height must lead back to the stated point
    if ((back.0 - pt.0).powi(2) + (back.1 - pt.1).powi(2)).sqrt() > 1e-3 + 1e-6 * pt.0.abs().max(pt.1.abs()) {
        return None;
    }
    let (xs, ys): (Vec<f64>, Vec<f64>) = surf.iter().copied().chain([pt]).unzip();
    let mut p = Plot::equal(&xs, &ys)?;
    p.line(&surf, 3, "ellipsoid surface");
    p.line(&[foot, pt], 0, &format!("along the normal, h = {} km", n4(h / 1000.0)));
    p.dots(&[foot], 2, "");
    p.label(foot.0, foot.1, 8.0, 16.0, "start", "axp-lt", &format!("foot at latitude {}", dg(lat)));
    p.mark(pt.0, pt.1, "the stated point");
    let body = p.finish("distance from the polar axis √(X² + Y²) [km]", "Z [km]", "");
    Some(s.chart("section", "The point, its foot on the ellipsoid and its height",
                 &format!("The Earth-centred point X = {} m, Y = {} m, Z = {} m converted by the model to latitude {}, \
                           longitude {}, height {} m. Drawn in the meridian plane (across, √(X² + Y²) from the inputs): the \
                           surface traced by the model's forward conversion at ±{} round the returned latitude, the foot of \
                           the normal (returned latitude at h = 0) and the point {} km above it; converting the returned \
                           position forward again reproduces the stated point.", n6(x), n6(y), n6(z), dg(lat), dg(lon),
                          n6(h), dg(half), n6(h / 1000.0)), &body))
}

/// A local east-north-up offset: the plan and the vertical plane through the target.
pub fn enu_frames(s: &Solved, inverse: bool) -> Option<Figure> {
    let (e, n, u) = if inverse { (s.inp("e")?, s.inp("n")?, s.inp("u")?) } else { (s.out("e")?, s.out("n")?, s.out("u")?) };
    let hd = (e * e + n * n).sqrt();
    let mut p1 = Plot::equal(&[0.0, e / 1000.0], &[0.0, n / 1000.0])?;
    p1.arrow((0.0, 0.0), (e / 1000.0, n / 1000.0), 0, 0.0);
    p1.dots(&[(0.0, 0.0)], 3, "");
    p1.label(0.0, 0.0, 8.0, 16.0, "start", "axp-lt", "origin");
    p1.mark(e / 1000.0, n / 1000.0, &format!("e = {} m, n = {} m", n4(e), n4(n)));
    p1.key("horizontal offset", 0);
    let top = p1.finish("east [km]", "north [km]", "");
    let mut p2 = Plot::equal(&[0.0, hd / 1000.0], &[0.0, u / 1000.0])?;
    p2.line(&[(0.0, 0.0), (hd / 1000.0, 0.0)], 3, "");
    p2.arrow((0.0, 0.0), (hd / 1000.0, u / 1000.0), 2, 0.0);
    p2.dashed(&[(hd / 1000.0, 0.0), (hd / 1000.0, u / 1000.0)], 3, "");
    p2.mark(hd / 1000.0, u / 1000.0, &format!("u = {} m", n4(u)));
    let (cap, title, lab);
    if inverse {
        let (la, lo, h) = (s.out("lat")?, s.out("lon")?, s.out("h")?);
        lab = format!("→ {}, {}, {} m", n4(la), n4(lo), n4(h));
        title = "Local offset converted to a geodetic position";
        cap = format!("The local offset e = {} m, n = {} m, u = {} m from the origin {}, {}, {} m (inputs; top: the plan, \
                       bottom: the vertical plane through the target, the horizontal distance √(e² + n²) from the inputs) \
                       and the geodetic position the model returns for it: latitude {}, longitude {}, height {} m.",
                      n6(e), n6(n), n6(u), dg(s.inp("lat0")?), dg(s.inp("lon0")?), n6(s.inp("h0")?), dg(la), dg(lo), n6(h));
    } else {
        let sr = s.out("slant_range")?;
        lab = format!("slant range {} m", n4(sr));
        title = "East-north-up offset of a point";
        cap = format!("Local east-north-up coordinates of the point {}, {}, {} m seen from the origin {}, {}, {} m: \
                       returned e = {} m, n = {} m, u = {} m, slant range {} m. Top: the plan (east, north); bottom: the \
                       vertical plane through the target (horizontal distance √(e² + n²) from the returned e and n).",
                      dg(s.inp("lat")?), dg(s.inp("lon")?), n6(s.inp("h")?), dg(s.inp("lat0")?), dg(s.inp("lon0")?),
                      n6(s.inp("h0")?), n6(e), n6(n), n6(u), n6(sr));
    }
    p2.key(&lab, 2);
    let bottom = p2.finish("horizontal distance [km]", "up [km]", "");
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("frames", title, &cap, &body, h))
}

/// Nested scales: the degree split into minutes, the minute into seconds.
pub fn dms_scale(s: &Solved, from_decimal: bool) -> Option<Figure> {
    let (d, m, sec, dec, hemi) = if from_decimal {
        (s.out("degrees")?, s.out("minutes")?, s.out("seconds")?, s.inp("decimal")?, s.out_text("hemisphere").unwrap_or_default())
    } else {
        (crate::figures::iv(s, "degrees")?, crate::figures::iv(s, "minutes")?, s.inp("seconds")?, s.out("decimal")?,
         s.txt("hemisphere").unwrap_or_default())
    };
    let (d, m) = (d.abs(), m.abs());
    if !(0.0..60.0).contains(&m) || !(0.0..60.5).contains(&sec) {
        return None;
    }
    let mut c = Canvas::new(250.0);
    let (x0, x1) = (40.0, 480.0);
    let sx = |f: f64| x0 + (x1 - x0) * f;
    // the degree
    c.line((x0, 80.0), (x1, 80.0));
    for i in 0..=12 {
        let x = sx(i as f64 / 12.0);
        c.line((x, 80.0), (x, if i % 2 == 0 { 70.0 } else { 74.0 }));
        if i % 2 == 0 {
            c.name(x, 98.0, "middle", &format!("{}′", i * 5));
        }
    }
    let pos = (m + sec / 60.0) / 60.0;
    c.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"80\" r=\"6\"/>", sx(pos)));
    c.bold(x0, 52.0, "start", &format!("{}° to {}° (ticks every 5 minutes)", n6(d), n6(d + 1.0)));
    c.value(x1, 52.0, "end", &format!("{} = {}° {}′ …", dg(dec.abs()), n6(d), n6(m)));
    // the minute, opened below
    let (ma, mb) = (sx(m / 60.0), sx((m + 1.0) / 60.0));
    c.dashed((ma, 84.0), (x0, 170.0));
    c.dashed((mb, 84.0), (x1, 170.0));
    c.line((x0, 170.0), (x1, 170.0));
    for i in 0..=12 {
        let x = sx(i as f64 / 12.0);
        c.line((x, 170.0), (x, if i % 2 == 0 { 160.0 } else { 164.0 }));
        if i % 2 == 0 {
            c.name(x, 188.0, "middle", &format!("{}″", i * 5));
        }
    }
    let spos = sec / 60.0;
    c.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"170\" r=\"6\"/>", sx(spos)));
    c.value(sx(spos), 150.0, "middle", &format!("{}″", n6(sec)));
    c.bold(x0, 216.0, "start", &format!("within minute {}′: seconds 0 to 60", n6(m)));
    c.value(x1, 240.0, "end", &format!("{}° {}′ {}″ {}", n6(d), n6(m), n6(sec), hemi));
    let cap = if from_decimal {
        format!("The angle {} on two nested scales: the whole degree {}° divided into minutes, then minute {}′ divided into \
                 seconds. Returned: {}° {}′ {}″ {}.", dg(dec), n6(d), n6(m), n6(d), n6(m), n6(sec), hemi)
    } else {
        format!("The angle {}° {}′ {}″ {} (inputs) on nested scales: the position within the degree (top), the seconds \
                 within the minute (bottom), and the returned decimal value {} ({} rad).", n6(d), n6(m), n6(sec), hemi,
                dg(dec), n6(s.out("radians")?))
    };
    Some(s.figure("scale", if from_decimal { "Decimal degrees split into degrees, minutes and seconds" } else {
        "Degrees, minutes and seconds as one decimal angle"
    }, &cap, c.body(), c.h))
}

/// The answer over a grid of two inputs (each cell one run of the model), the visitor's point marked.
pub fn two_input_field(s: &Solved, xsym: &str, ysym: &str, out: &str, unit: &str, xr: (f64, f64), yr: (f64, f64))
                       -> Option<Figure> {
    let (xv, yv, ov) = (s.inp(xsym)?, s.inp(ysym)?, s.out(out)?);
    let absolute = s.key == "geo_meridian_arc";
    let rng = |v: f64, r: (f64, f64)| if absolute { (r.0.min(v), r.1.max(v)) } else { (v * r.0, v * r.1) };
    let (xa, xb) = rng(xv, xr);
    let (ya, yb) = rng(yv, yr);
    if !(xb > xa && yb > ya) {
        return None;
    }
    let (nx, ny) = (17, 13);
    let xs = k::span(xa, xb, nx, false);
    let ys = k::span(ya, yb, ny, false);
    let mut vals = vec![];
    let mut runs = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with(&[(xsym, *x), (ysym, *y)]).and_then(|a| o(&a, out));
            runs += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if runs < 6 {
        return None;
    }
    let mean = |sym: &str| -> String {
        let m = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.meaning.clone()))
            .unwrap_or_default();
        let u = if s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.kind == "length"))
            .unwrap_or(false) { " [m]" } else { "" };
        let m = m.split('(').next().unwrap_or("").trim().to_string();
        format!("{sym}{u}: {m}")
    };
    let lab = format!("{} = {}{}", out, n6(ov), if unit.is_empty() { String::new() } else { format!(" {unit}") });
    let body = k::field(&xs, &ys, &vals, (xv, yv), &mean(xsym), &mean(ysym), &lab, (false, false));
    let lo = vals.iter().flatten().flatten().cloned().fold(f64::MAX, f64::min);
    let hi = vals.iter().flatten().flatten().cloned().fold(f64::MIN, f64::max);
    let blank = nx * ny - runs;
    Some(s.chart("field", &format!("{} over {} and {}", out, xsym, ysym),
                 &format!("{} over {} from {} to {} and {} from {} to {}: each of the {} × {} cells is one run of the model, \
                           shaded by its value (light {} to dark {}){}. Marked: {} = {}, {} = {}, {} = {}.", out, xsym,
                          n6(xa), n6(xb), ysym, n6(ya), n6(yb), nx, ny, n6(lo), n6(hi),
                          if blank > 0 { format!("; {blank} blank cells are inputs the capability refuses") } else { String::new() },
                          xsym, n6(xv), ysym, n6(yv), out, n6(ov)), &body))
}
