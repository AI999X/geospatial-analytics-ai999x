//! Rasters, terrain and indices: flow directions and accumulation on an elevation grid, watersheds, a slope window,
//! hillshade as the sun turns, a line-of-sight profile, accuracy and segregation bars, spectral indices. Every value
//! drawn is an input, an output of the request or of a further run of the model.

use crate::figures::{iv, jm, lst, n4, n6, o, om, pass};
use crate::figures_plot::Plot;
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// A grid of values drawn as shaded cells with the value printed (each panel shaded by its own largest magnitude);
/// row 0 at the top. Returns the cell size used.
fn grid_panel(c: &mut Canvas, x0: f64, y0: f64, cell: f64, m: &[Vec<f64>], title: &str) {
    grid_panel_at(c, x0, y0, cell, m, title, false)
}

/// `grid_panel` with the values in each cell's top-left corner (`corner`), leaving its centre free for an arrow.
fn grid_panel_at(c: &mut Canvas, x0: f64, y0: f64, cell: f64, m: &[Vec<f64>], title: &str, corner: bool) {
    let big = m.iter().flatten().map(|v| v.abs()).fold(0.0, f64::max);
    for (i, row) in m.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let (x, y) = (x0 + cell * j as f64, y0 + cell * i as f64);
            let op = if big > 0.0 { 0.06 + 0.55 * v.abs() / big } else { 0.06 };
            c.raw(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill-opacity=\"{op:.2}\"/>\
                            <rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                           if *v < 0.0 { "axp-hn" } else { "axp-hp" }, cell - 1.0, cell - 1.0, cell - 1.0, cell - 1.0));
            if cell >= 26.0 {
                if corner {
                    c.text(x + 5.0, y + 15.0, "start", "axp-lt", &n4(*v));
                } else {
                    c.text(x + cell / 2.0, y + cell / 2.0 + 5.0, "middle", "axp-bt", &n4(*v));
                }
            }
        }
    }
    if !title.is_empty() {
        c.name(x0 + cell * m[0].len() as f64 / 2.0, y0 - 8.0, "middle", title);
    }
}

fn dem(s: &Solved) -> Option<Vec<Vec<f64>>> {
    let m = s.mat("dem")?;
    let nc = m[0].len();
    (m.len() <= 20 && nc <= 20 && m.iter().all(|r| r.len() == nc)).then_some(m)
}

/// The step (column, row) of a D8 code (row down = south).
fn d8(code: f64) -> Option<(i64, i64)> {
    Some(match code as i64 {
        1 => (1, 0),
        2 => (1, 1),
        4 => (0, 1),
        8 => (-1, 1),
        16 => (-1, 0),
        32 => (-1, -1),
        64 => (0, -1),
        128 => (1, -1),
        _ => return None,
    })
}

fn directions(s: &Solved, m: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let a = s.solve_other("geo_d8_flow_direction", &pass(s, &["cell_size"], vec![("dem", jm(m))]))?;
    let d = om(&a, "direction")?;
    (d.len() == m.len() && d[0].len() == m[0].len()).then_some(d)
}

fn arrow_px(c: &mut Canvas, a: (f64, f64), b: (f64, f64), class: &str) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt().max(1e-9);
    let (ux, uy) = (dx / d, dy / d);
    let t = (b.0 - ux * 7.0, b.1 - uy * 7.0);
    let dot = if class == "axp-c" { "axp-d" } else { "axp-d4" };
    c.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/><path class=\"{dot}\" d=\"M{:.1},{:.1} \
                    L{:.1},{:.1} L{:.1},{:.1} Z\"/>", a.0, a.1, t.0, t.1, b.0, b.1, t.0 - uy * 4.0, t.1 + ux * 4.0,
                   t.0 + uy * 4.0, t.1 - ux * 4.0));
}

pub fn flow_directions(s: &Solved) -> Option<Figure> {
    let m = dem(s)?;
    let d = om(s.a, "direction")?;
    if d.len() != m.len() || d[0].len() != m[0].len() {
        return None;
    }
    let (nr, nc) = (m.len(), m[0].len());
    let cell = (440.0 / nc as f64).min(300.0 / nr as f64).min(80.0);
    let (x0, y0) = ((520.0 - cell * nc as f64) / 2.0, 40.0);
    let h = y0 + cell * nr as f64 + 40.0;
    let mut c = Canvas::new(h);
    grid_panel_at(&mut c, x0, y0, cell, &m, "", true);
    let mut outlets = 0;
    for i in 0..nr {
        for j in 0..nc {
            let ctr = (x0 + cell * (j as f64 + 0.5), y0 + cell * (i as f64 + 0.5));
            match d8(d[i][j]) {
                Some((dc, dr)) => {
                    let a = (ctr.0 - dc as f64 * cell * 0.22, ctr.1 - dr as f64 * cell * 0.22 + cell * 0.08);
                    let b = (ctr.0 + dc as f64 * cell * 0.32, ctr.1 + dr as f64 * cell * 0.32 + cell * 0.08);
                    arrow_px(&mut c, a, b, "axp-c");
                }
                None => {
                    outlets += 1;
                    c.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", ctr.0, ctr.1 + cell * 0.25));
                }
            }
        }
    }
    c.bold(16.0, 22.0, "start", "flow direction (steepest descent), north at the top");
    if outlets > 0 {
        c.value(504.0, h - 12.0, "end", &format!("outlet (code 0): {outlets}"));
    }
    let rows: Vec<String> = d.iter().map(|r| r.iter().map(|v| n6(*v)).collect::<Vec<_>>().join(" ")).collect();
    Some(s.figure("flow", "D8 flow directions on an elevation grid",
                  &format!("The {nr} × {nc} elevation grid (cell size {} m; shading and printed values: elevation, north at \
                            the top) with an arrow in each cell for the D8 direction the model returns (codes 1 = east, \
                            2 = south-east, 4 = south, 8 = south-west, 16 = west, 32 = north-west, 64 = north, \
                            128 = north-east): {}.", n6(s.inp("cell_size")?), rows.join("; ")), c.body(), c.h))
}

pub fn stream_network(s: &Solved) -> Option<Figure> {
    let m = dem(s)?;
    let acc = om(s.a, "accumulation")?;
    let d = directions(s, &m)?;
    let (nr, nc) = (m.len(), m[0].len());
    if acc.len() != nr || acc[0].len() != nc {
        return None;
    }
    let amax = acc.iter().flatten().cloned().fold(1.0, f64::max);
    let cell = (440.0 / nc as f64).min(300.0 / nr as f64).min(80.0);
    let (x0, y0) = ((520.0 - cell * nc as f64) / 2.0, 40.0);
    let h = y0 + cell * nr as f64 + 30.0;
    let mut c = Canvas::new(h);
    for i in 0..nr {
        for j in 0..nc {
            let (x, y) = (x0 + cell * j as f64, y0 + cell * i as f64);
            c.raw(&format!("<rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", cell - 1.0,
                           cell - 1.0));
        }
    }
    for i in 0..nr {
        for j in 0..nc {
            if let Some((dc, dr)) = d8(d[i][j]) {
                let (ti, tj) = (i as i64 + dr, j as i64 + dc);
                if ti < 0 || tj < 0 || ti >= nr as i64 || tj >= nc as i64 {
                    continue;
                }
                let a = (x0 + cell * (j as f64 + 0.5), y0 + cell * (i as f64 + 0.5));
                let b = (x0 + cell * (tj as f64 + 0.5), y0 + cell * (ti as f64 + 0.5));
                let w = 1.5 + 9.0 * (acc[i][j] / amax).sqrt();
                c.raw(&format!("<path class=\"axp-c3\" style=\"stroke-width:{w:.1}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a.0,
                               a.1, b.0, b.1));
            }
        }
    }
    for i in 0..nr {
        for j in 0..nc {
            let ctr = (x0 + cell * (j as f64 + 0.5), y0 + cell * (i as f64 + 0.5));
            c.dot(ctr);
            c.value(x0 + cell * j as f64 + 5.0, y0 + cell * i as f64 + 15.0, "start", &n6(acc[i][j]));
        }
    }
    c.bold(16.0, 22.0, "start", &format!("cells draining through each cell; the outlet collects {}", n6(amax)));
    Some(s.figure("flow", "Flow accumulation as a stream network",
                  &format!("Flow accumulation on the stated elevation grid: each cell shows the number of cells draining \
                            through it (itself included) returned by the model, and a stream line to its downstream \
                            neighbour (direction from the model's D8 capability) whose width grows with that number; the \
                            largest, {}, is the outlet.", n6(amax)), c.body(), c.h))
}

pub fn accumulation_panels(s: &Solved) -> Option<Figure> {
    let m = dem(s)?;
    let acc = om(s.a, "accumulation")?;
    let (nr, nc) = (m.len(), m[0].len());
    if acc.len() != nr || acc[0].len() != nc {
        return None;
    }
    let cell = (220.0 / nc as f64).min(280.0 / nr as f64).min(56.0);
    let w = cell * nc as f64;
    let gap = 520.0 - 2.0 * w - 2.0 * 20.0;
    let (xa, xb) = (20.0, 20.0 + w + gap);
    let h = 60.0 + cell * nr as f64 + 20.0;
    let mut c = Canvas::new(h);
    grid_panel(&mut c, xa, 60.0, cell, &m, "elevation [m]");
    grid_panel(&mut c, xb, 60.0, cell, &acc, "flow accumulation [cells]");
    c.bold(16.0, 22.0, "start", &format!("maximum {} at the outlet", n6(s.out("max_accumulation")?)));
    Some(s.figure("heatmap", "Elevation grid and its flow accumulation",
                  &format!("A {nr} × {nc} elevation grid (left, metres, cell size {} m) and the D8 flow accumulation the model \
                            computes from it (right): each cell shows how many cells drain through it, itself included; the \
                            maximum, {}, is the outlet. Each panel is shaded by its own maximum.", n6(s.inp("cell_size")?),
                           n6(s.out("max_accumulation")?)), c.body(), c.h))
}

pub fn watersheds(s: &Solved) -> Option<Figure> {
    let m = dem(s)?;
    let (nr, nc) = (m.len(), m[0].len());
    let (r0, c0) = (iv(s, "row")? as usize, iv(s, "col")? as usize);
    let d = directions(s, &m)?;
    let acc = s.solve_other("geo_d8_flow_accumulation", &pass(s, &["cell_size"], vec![("dem", jm(&m))]))
        .and_then(|a| om(&a, "accumulation"))?;
    // two more outlets: the cells with the next largest accumulation
    let mut cells: Vec<(usize, usize, f64)> = (0..nr).flat_map(|i| (0..nc).map(move |j| (i, j))).map(|(i, j)| (i, j, acc[i][j]))
        .filter(|c| !(c.0 == r0 && c.1 == c0) && c.2 > 1.0).collect();
    cells.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    let mut outs = vec![(r0, c0, s.out("cells")?, s.out("area")?, om(s.a, "mask")?)];
    for (i, j, _) in cells.iter().take(2) {
        let a = s.with_input(&[("row", Input::Int(*i as i64)), ("col", Input::Int(*j as i64))]);
        if let Some(a) = a {
            outs.push((*i, *j, o(&a, "cells")?, o(&a, "area")?, om(&a, "mask")?));
        }
    }
    let np = outs.len();
    let pw = 500.0 / np as f64;
    let cell = ((pw - 20.0) / nc as f64).min(240.0 / nr as f64).min(56.0);
    let h = 50.0 + cell * nr as f64 + 50.0;
    let mut c = Canvas::new(h);
    for (k_, (ri, ci, n, ar, mask)) in outs.iter().enumerate() {
        if mask.len() != nr || mask[0].len() != nc {
            return None;
        }
        let x0 = 10.0 + pw * k_ as f64 + (pw - cell * nc as f64) / 2.0;
        let y0 = 40.0;
        for i in 0..nr {
            for j in 0..nc {
                let (x, y) = (x0 + cell * j as f64, y0 + cell * i as f64);
                let on = mask[i][j] != 0.0;
                c.raw(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill-opacity=\"{}\"/>\
                                <rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                               if on { "axp-hp" } else { "axp-hn" }, cell - 1.0, cell - 1.0, if on { "0.45" } else { "0.05" },
                               cell - 1.0, cell - 1.0));
                if let Some((dc, dr)) = d8(d[i][j]) {
                    let ctr = (x + cell / 2.0, y + cell / 2.0);
                    arrow_px(&mut c, (ctr.0 - dc as f64 * cell * 0.25, ctr.1 - dr as f64 * cell * 0.25),
                             (ctr.0 + dc as f64 * cell * 0.3, ctr.1 + dr as f64 * cell * 0.3), "axp-c4");
                }
            }
        }
        c.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", x0 + cell * (*ci as f64 + 0.5),
                       y0 + cell * (*ri as f64 + 0.5)));
        let cx = x0 + cell * nc as f64 / 2.0;
        c.bold(cx, y0 + cell * nr as f64 + 20.0, "middle", &format!("row {ri}, col {ci}"));
        c.value(cx, y0 + cell * nr as f64 + 38.0, "middle", &format!("{} cells, {} m²", n6(*n), n4(*ar)));
    }
    c.name(16.0, 22.0, "start", "the outlet (dot) and the cells draining to it, D8 arrows");
    let list: Vec<String> = outs.iter().map(|(r, cc, n, a, _)| format!("row {r}, column {cc}: {} cells, {} m²", n6(*n), n6(*a)))
        .collect();
    Some(s.figure("mask", "Watersheds of three outlets",
                  &format!("Watersheds on the stated elevation grid (cell size {} m) for {} outlets (dot): the cells of the \
                            mask the model returns are shaded, the arrows are the D8 directions from its flow-direction \
                            capability. Rows and columns count from 0 at the top left. {}; the first is the requested \
                            outlet, the others the cells of next largest accumulation.", n6(s.inp("cell_size")?), np,
                           list.join("; ")), c.body(), c.h))
}

pub fn slope_window(s: &Solved) -> Option<Figure> {
    let w = s.mat("window")?;
    if w.len() != 3 || w.iter().any(|r| r.len() != 3) {
        return None;
    }
    let (sd, sp, asp) = (s.out("slope_deg")?, s.out("slope_percent")?, s.out("aspect_deg")?);
    let cell = 80.0;
    let (x0, y0) = (140.0, 50.0);
    let mut c = Canvas::new(330.0);
    grid_panel(&mut c, x0, y0, cell, &w, "");
    let ctr = (x0 + 1.5 * cell, y0 + 1.5 * cell);
    let a = asp.to_radians();
    if sd > 1e-9 {
        arrow_px(&mut c, (ctr.0 - 20.0 * a.sin(), ctr.1 + 20.0 * a.cos()), (ctr.0 + 105.0 * a.sin(), ctr.1 - 105.0 * a.cos()),
                 "axp-c");
    }
    c.bold(16.0, 22.0, "start", &format!("slope {} ({} %)", dg(sd), n4(sp)));
    c.value(504.0, 22.0, "end", &format!("downslope, aspect {} from north", dg(asp)));
    c.name(x0 + 1.5 * cell, y0 + 3.0 * cell + 22.0, "middle", &format!("cell size {} m, north at the top", n4(s.inp("cell_size")?)));
    Some(s.figure("window", "Slope and aspect of an elevation window",
                  &format!("The 3 × 3 elevation window (cell size {} m, north at the top; values as given) and the slope and \
                            aspect the model returns for its centre: slope {} ({} %), aspect {} clockwise from north, drawn \
                            as an arrow pointing downslope.", n6(s.inp("cell_size")?), dg(sd), n6(sp), dg(asp)), c.body(), c.h))
}

fn dg(x: f64) -> String {
    crate::figures::dg(x)
}

pub fn window_matrix(s: &Solved) -> Option<Figure> {
    let w = s.mat("window")?;
    let nc = w[0].len();
    if w.iter().any(|r| r.len() != nc) || w.len() > 12 || nc > 12 {
        return None;
    }
    let outs: Vec<(String, f64)> = s.a.values.iter().filter_map(|(k_, v, _)| v.as_f64().map(|x| (k_.clone(), x))).collect();
    let cell = (300.0 / nc as f64).min(240.0 / w.len() as f64).min(80.0);
    let (x0, y0) = (40.0, 60.0);
    let h = (y0 + cell * w.len() as f64 + 30.0).max(60.0 + 26.0 * outs.len() as f64);
    let mut c = Canvas::new(h);
    grid_panel(&mut c, x0, y0, cell, &w, "window (input)");
    let tx = x0 + cell * nc as f64 + 30.0;
    c.bold(tx, y0 + 4.0, "start", "returned");
    for (i, (k_, v)) in outs.iter().enumerate() {
        c.value(tx, y0 + 30.0 + 24.0 * i as f64, "start", &format!("{k_} = {}", n4(*v)));
    }
    let title = if s.key == "geo_terrain_curvature" { "Terrain curvature of a window" } else { "Terrain ruggedness of a window" };
    Some(s.figure("matrix", title,
                  &format!("The elevation window as a heat map (rows and columns as given; each cell shaded by its value) and \
                            what the model returns for it: {}.{}", outs.iter().map(|(k_, v)| format!("{k_} = {}", n6(*v)))
                               .collect::<Vec<_>>().join(", "),
                           s.inp("cell_size").map(|c| format!(" Cell size {} m.", n6(c))).unwrap_or_default()), c.body(), c.h))
}

pub fn hillshade_compass(s: &Solved) -> Option<Figure> {
    let (az0, alt0, hs0) = (s.inp("sun_azimuth")?, s.inp("sun_altitude")?, s.out("hillshade")?);
    let alts = [alt0, if alt0 >= 32.5 { 20.0 } else { 60.0 }];
    let mut rings = vec![];
    for alt in alts {
        let mut pts = vec![];
        for k_ in 0..=36 {
            let az = 10.0 * k_ as f64;
            if let Some(v) = s.with(&[("sun_azimuth", az % 360.0), ("sun_altitude", alt)]).and_then(|a| o(&a, "hillshade")) {
                pts.push((az, v));
            }
        }
        if pts.len() >= 6 {
            rings.push((alt, pts));
        }
    }
    if rings.is_empty() {
        return None;
    }
    let mut c = Canvas::new(440.0);
    let (cx, cy, rr) = (250.0, 236.0, 170.0);
    for (v, lab) in [(64.0, "64"), (128.0, "128"), (192.0, "192"), (255.0, "255")] {
        let r = rr * v / 255.0;
        c.raw(&format!("<circle class=\"axp-c4\" cx=\"{cx}\" cy=\"{cy}\" r=\"{r:.1}\" stroke-dasharray=\"4 4\"/>"));
        c.name(cx + 4.0, cy - r - 3.0, "start", lab);
    }
    for (i, lab) in ["N", "30°", "60°", "E", "120°", "150°", "S", "210°", "240°", "W", "300°", "330°"].iter().enumerate() {
        let a = (i as f64 * 30.0).to_radians();
        c.name(cx + (rr + 16.0) * a.sin(), cy - (rr + 16.0) * a.cos() + 5.0, "middle", lab);
    }
    let pos = |az: f64, v: f64| (cx + rr * v / 255.0 * az.to_radians().sin(), cy - rr * v / 255.0 * az.to_radians().cos());
    for (i, (alt, pts)) in rings.iter().enumerate() {
        let mut d = String::new();
        for (j, (az, v)) in pts.iter().enumerate() {
            let q = pos(*az, *v);
            d.push_str(&format!("{}{:.1},{:.1} ", if j == 0 { "M" } else { "L" }, q.0, q.1));
        }
        let cl = if i == 0 { "axp-c" } else { "axp-c3" };
        c.raw(&format!("<path class=\"{cl}\" d=\"{}\"/>", d.trim_end()));
        c.raw(&format!("<rect class=\"{}\" x=\"330\" y=\"{}\" width=\"12\" height=\"10\"/>", if i == 0 { "axp-b" } else { "axp-b3" },
                       12.0 + 18.0 * i as f64));
        c.name(348.0, 21.0 + 18.0 * i as f64, "start", &format!("sun {} high", dg(*alt)));
    }
    let q = pos(az0, hs0);
    c.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", q.0, q.1));
    c.value(16.0, 22.0, "start", &format!("sun at {}: hillshade {}", dg(az0), n4(hs0)));
    Some(s.figure("compass", "Hillshade as the sun's azimuth turns",
                  &format!("Hillshade of the stated 3 × 3 window (cell size {} m) as the sun goes round: distance from the \
                            centre is the hillshade value (0 to 255) the model returns for a sun at that azimuth (clockwise \
                            from north, every 10°), at altitudes {} and {}. Marked: the sun at {}, altitude {}, hillshade {} \
                            (illumination {}).", n6(s.inp("cell_size")?), dg(alts[0]), dg(alts[1]), dg(az0), dg(alt0), n6(hs0),
                           n6(s.out("illumination")?)), c.body(), c.h))
}

pub fn sight_profile(s: &Solved) -> Option<Figure> {
    let (d, z) = (s.vec("d")?, s.vec("z")?);
    let vis = s.out_list("visible")?;
    let n = d.len();
    if z.len() != n || vis.len() != n || n < 2 {
        return None;
    }
    let (ho, ht) = (s.inp("observer_height")?, s.inp("target_height")?);
    let eye = (d[0], z[0] + ho);
    let terrain: Vec<(f64, f64)> = d.iter().zip(&z).map(|(a, b)| (*a, *b)).collect();
    let tops: Vec<(f64, f64)> = terrain.iter().map(|p| (p.0, p.1 + ht)).collect();
    let (xs, ys): (Vec<f64>, Vec<f64>) = terrain.iter().chain(tops.iter()).copied().chain([eye]).unzip();
    let mut p = Plot::new(&xs, &ys, false, false)?;
    let ymin = p.bounds().2;
    let mut fill: Vec<(f64, f64)> = terrain.clone();
    fill.push((d[n - 1], ymin));
    fill.push((d[0], ymin));
    p.polygon(&fill, 3, "terrain (input)");
    let mut shown = 0;
    for i in 1..n {
        if vis[i] != 0.0 {
            p.line(&[eye, tops[i]], 1, "visible");
            shown += 1;
        } else {
            p.dashed(&[eye, tops[i]], 0, "hidden");
        }
    }
    p.dots(&tops[1..], 3, "");
    p.mark(eye.0, eye.1, &format!("observer +{} m", n4(ho)));
    let body = p.finish("distance along the profile [m]", "elevation [m]", "");
    Some(s.chart("profile", "Line of sight along a terrain profile",
                 &format!("A terrain profile (elevations {} m at {} m) seen by an observer {} m above the first point, looking \
                           at targets {} m above each point (curvature: {}): sight lines are drawn solid where the model \
                           returns the point as visible and dashed where hidden; {} of the {} other points are visible.",
                          lst(&z), lst(&d), n6(ho), n6(ht), s.txt("curvature").unwrap_or_default(), shown, n - 1), &body))
}

pub fn accuracy_bars(s: &Solved) -> Option<Figure> {
    let (pr, us) = (s.out_list("producers")?, s.out_list("users")?);
    if pr.len() != us.len() || pr.is_empty() || pr.len() > 20 {
        return None;
    }
    let labels: Vec<String> = (1..=pr.len()).map(|i| i.to_string()).collect();
    let g = vec![("producer's".to_string(), pr.iter().map(|v| Some(*v)).collect()),
                 ("user's".to_string(), us.iter().map(|v| Some(*v)).collect())];
    let body = k::bar_groups(&labels, &g, "class", "accuracy", &format!("overall {}, κ = {}", n4(s.out("overall")?),
                                                                       n4(s.out("kappa")?)));
    Some(s.chart("bars", "Producer's and user's accuracy by class",
                 &format!("Per-class accuracies the model returns for the stated {n} × {n} error matrix (rows: classified, \
                           columns: reference): producer's accuracy {}; user's accuracy {}; overall {}, κ = {}.", lst(&pr),
                          lst(&us), n6(s.out("overall")?), n6(s.out("kappa")?), n = pr.len()), &body))
}

pub fn accuracy_matrix(s: &Solved) -> Option<Figure> {
    let m = s.mat("matrix")?;
    let n = m.len();
    let (pr, us) = (s.out_list("producers")?, s.out_list("users")?);
    if n > 12 || m.iter().any(|r| r.len() != n) || pr.len() != n || us.len() != n {
        return None;
    }
    let cell = (300.0 / n as f64).min(250.0 / n as f64).min(60.0);
    let (x0, y0) = (36.0, 60.0);
    let h = y0 + cell * n as f64 + 30.0;
    let mut c = Canvas::new(h);
    grid_panel(&mut c, x0, y0, cell, &m, "error matrix (input)");
    let cols: Vec<Vec<f64>> = (0..n).map(|i| vec![pr[i], us[i]]).collect();
    let x1 = x0 + cell * n as f64 + 40.0;
    let cw = cell.max(52.0);
    grid_panel(&mut c, x1, y0, cw.min(cell * 1.3), &cols, "");
    c.name(x1 + cw.min(cell * 1.3) * 0.5, y0 - 8.0, "middle", "prod.");
    c.name(x1 + cw.min(cell * 1.3) * 1.5, y0 - 8.0, "middle", "user's");
    for i in 0..n {
        c.name(x0 - 8.0, y0 + cell * (i as f64 + 0.5) + 5.0, "end", &format!("{}", i + 1));
        c.name(x0 + cell * (i as f64 + 0.5), y0 + cell * n as f64 + 18.0, "middle", &format!("{}", i + 1));
    }
    c.bold(16.0, 22.0, "start", &format!("overall {}, κ = {}", n4(s.out("overall")?), n4(s.out("kappa")?)));
    Some(s.figure("matrix", "The error matrix and the per-class accuracies",
                  &format!("The stated {n} × {n} error matrix as a heat map (rows: classified, columns: reference; shaded by \
                            its largest count) beside the producer's and user's accuracies the model returns for each class \
                            ({}; {}); overall {}, κ = {}.", lst(&pr), lst(&us), n6(s.out("overall")?), n6(s.out("kappa")?)),
                  c.body(), c.h))
}

pub fn group_bars(s: &Solved) -> Option<Figure> {
    let (x, t) = (s.vec("x")?, s.vec("t")?);
    if x.len() != t.len() || x.is_empty() || x.len() > 24 {
        return None;
    }
    let labels: Vec<String> = (1..=x.len()).map(|i| i.to_string()).collect();
    let g = vec![("total population t".to_string(), t.iter().map(|v| Some(*v)).collect()),
                 ("group count x".to_string(), x.iter().map(|v| Some(*v)).collect())];
    let outs: Vec<(String, f64)> = s.a.values.iter().filter_map(|(k_, v, _)| v.as_f64().map(|x| (k_.clone(), x))).collect();
    let head = outs.first().map(|(k_, v)| format!("{k_} = {}", n4(*v))).unwrap_or_default();
    let body = k::bar_groups(&labels, &g, "areal unit", "population", &head);
    let shares: Vec<String> = x.iter().zip(&t).map(|(a, b)| if *b > 0.0 { format!("{} %", n4(100.0 * a / b)) } else { "-".into() })
        .collect();
    let title = if s.key == "geo_segregation_evenness" { "Group shares behind the evenness indices" } else {
        "Group shares behind isolation and exposure"
    };
    Some(s.chart("bars", title,
                 &format!("Group count x = {} within totals t = {} in each areal unit (shares {}). The model returns {}.",
                          lst(&x), lst(&t), shares.join(", "),
                          outs.iter().map(|(k_, v)| format!("{k_} = {}", n6(*v))).collect::<Vec<_>>().join(", ")), &body))
}

pub fn index_bars(s: &Solved) -> Option<Figure> {
    let outs: Vec<(String, f64)> = s.a.values.iter().filter_map(|(k_, v, _)| v.as_f64().map(|x| (k_.clone(), x))).collect();
    if outs.is_empty() {
        return None;
    }
    let labels: Vec<String> = outs.iter().map(|o_| o_.0.clone()).collect();
    let vals: Vec<f64> = outs.iter().map(|o_| o_.1).collect();
    let hl: Vec<usize> = (0..vals.len()).collect();
    let body = k::bars(&labels, &vals, &hl, "index", "value", "");
    let ins: Vec<String> = s.p.procedure(s.key)?.inputs.iter().filter_map(|i| s.inp(&i.symbol).map(|v| format!("{} = {}", i.symbol, n6(v))))
        .collect();
    let title = if s.key == "geo_vegetation_indices" { "Vegetation indices of one pixel" } else {
        "Water, moisture, burn and built-up indices of one pixel"
    };
    Some(s.chart("bars", title,
                 &format!("All {} indices the model returns for the stated reflectances ({}): {}.", outs.len(), ins.join(", "),
                          outs.iter().map(|(k_, v)| format!("{k_} = {}", n6(*v))).collect::<Vec<_>>().join(", ")), &body))
}

pub fn ndvi_field(s: &Solved) -> Option<Figure> {
    let (red, nir, v) = (s.inp("red")?, s.inp("nir")?, s.out("NDVI")?);
    let xs = k::span(0.0, red.max(0.5), 16, false);
    let ys = k::span(0.0, nir.max(0.6), 13, false);
    let mut vals = vec![];
    let mut runs = 0;
    for a in &xs {
        let mut col = vec![];
        for b in &ys {
            let r = s.with(&[("red", *a), ("nir", *b)]).and_then(|r| o(&r, "NDVI"));
            runs += r.is_some() as usize;
            col.push(r);
        }
        vals.push(col);
    }
    if runs < 20 {
        return None;
    }
    let body = k::field(&xs, &ys, &vals, (red, nir), "red reflectance", "near-infrared reflectance",
                        &format!("NDVI {}", n4(v)), (false, false));
    Some(s.chart("field", "NDVI over the red and near-infrared plane",
                 &format!("NDVI returned by the model over red reflectance 0 to {} and near-infrared 0 to {} ({} runs; other \
                           inputs as stated): vegetation (bright near-infrared, dark red) is high, bare soil and water low. \
                           Marked: red {}, near-infrared {}, NDVI {}.", n6(red.max(0.5)), n6(nir.max(0.6)), runs, n6(red),
                          n6(nir), n6(v)), &body))
}
