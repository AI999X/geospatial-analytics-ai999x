//! Point patterns, lines, interpolated surfaces, fitted models, Moran scatters and networks. Every value drawn is an
//! output of the request or of a further run of the model; a fitted curve is drawn only when refitting the model to
//! the curve's own values returns the same parameters.

use crate::figures::{agrees, edges, iv, jl, jm, lst, n4, n6, o, ol, om, ot, same_fit};
use crate::figures_plot::Plot;
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

fn xy(s: &Solved) -> Option<(Vec<f64>, Vec<f64>)> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    (x.len() == y.len() && !x.is_empty() && x.len() <= 200).then_some((x, y))
}

fn pts_of(x: &[f64], y: &[f64]) -> Vec<(f64, f64)> {
    x.iter().zip(y).map(|(a, b)| (*a, *b)).collect()
}

pub fn mean_centre(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (mx, my, sd) = (s.out("mean_x")?, s.out("mean_y")?, s.out("std_distance")?);
    let mut xs = x.clone();
    let mut ys = y.clone();
    xs.extend([mx - sd, mx + sd]);
    ys.extend([my - sd, my + sd]);
    let mut p = Plot::equal(&xs, &ys)?;
    p.circle(mx, my, sd, 0, true, "one standard distance");
    p.dots(&pts_of(&x, &y), 3, &format!("{} points", x.len()));
    p.mark(mx, my, &format!("({}, {})", n4(mx), n4(my)));
    let body = p.finish("x", "y", "");
    Some(s.chart("map", "Mean centre and standard distance of a point pattern",
                 &format!("The {} points x = {}, y = {} with the mean centre and standard distance the model computes: \
                           marked, the centre ({}, {}); the circle has radius {}, the standard distance (root-mean-square \
                           distance of the points from the centre).", x.len(), lst(&x), lst(&y), n6(mx), n6(my), n6(sd)), &body))
}

/// The centre found again with each point left out in turn (each a run of the model).
pub fn mean_centre_leave_one_out(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    if x.len() < 3 || x.len() > 40 {
        return None;
    }
    let (mx, my, sd) = (s.out("mean_x")?, s.out("mean_y")?, s.out("std_distance")?);
    let mut moved = vec![];
    for i in 0..x.len() {
        let x2: Vec<f64> = x.iter().enumerate().filter(|e| e.0 != i).map(|e| *e.1).collect();
        let y2: Vec<f64> = y.iter().enumerate().filter(|e| e.0 != i).map(|e| *e.1).collect();
        let a = s.solve_other(s.key, &[("x", jl(&x2)), ("y", jl(&y2))])?;
        moved.push((o(&a, "mean_x")?, o(&a, "mean_y")?, o(&a, "std_distance")?));
    }
    let mut xs = x.clone();
    let mut ys = y.clone();
    xs.extend(moved.iter().map(|m| m.0).chain([mx - sd, mx + sd]));
    ys.extend(moved.iter().map(|m| m.1).chain([my - sd, my + sd]));
    let mut p = Plot::equal(&xs, &ys)?;
    p.circle(mx, my, sd, 0, true, &format!("standard distance {}", n4(sd)));
    for (i, m) in moved.iter().enumerate() {
        p.dashed(&[(x[i], y[i]), (m.0, m.1)], 3, "");
    }
    p.dots(&moved.iter().map(|m| (m.0, m.1)).collect::<Vec<_>>(), 2, "centre without that point");
    p.dots(&pts_of(&x, &y), 3, "points");
    p.mark(mx, my, &format!("({}, {})", n4(mx), n4(my)));
    let body = p.finish("x", "y", "");
    let list: Vec<String> = moved.iter().enumerate()
        .map(|(i, m)| format!("without point {}: ({}, {}), standard distance {}", i + 1, n4(m.0), n4(m.1), n4(m.2)))
        .collect();
    Some(s.chart("map-2", "Mean centre with each point left out",
                 &format!("The points x = {}, y = {}, their mean centre ({}, {}) and a circle of the returned standard \
                           distance {}; each green dot is the centre the model returns with one point left out, joined to \
                           that point by a dashed line (it moves away from the point left out): {}.", lst(&x), lst(&y), n6(mx),
                          n6(my), n6(sd), list.join("; ")), &body))
}

pub fn weighted_centre(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let w = s.vec("w")?;
    if w.len() != x.len() {
        return None;
    }
    let (wx, wy, wsd) = (s.out("wmean_x")?, s.out("wmean_y")?, s.out("weighted_std_distance")?);
    let plain = s.solve_other("geo_mean_center_standard_distance", &[("x", jl(&x)), ("y", jl(&y))])
        .and_then(|a| Some((o(&a, "mean_x")?, o(&a, "mean_y")?)));
    let mut xs = x.clone();
    let mut ys = y.clone();
    xs.extend([wx - wsd, wx + wsd]);
    ys.extend([wy - wsd, wy + wsd]);
    let mut p = Plot::equal(&xs, &ys)?;
    p.circle(wx, wy, wsd, 0, true, "weighted standard distance");
    let wmax = w.iter().cloned().fold(0.0, f64::max);
    for i in 0..x.len() {
        let (px, py) = p.p(x[i], y[i]);
        let r = if wmax > 0.0 { 3.0 + 11.0 * (w[i].max(0.0) / wmax).sqrt() } else { 4.0 };
        p.raw(&format!("<circle class=\"axp-b3\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{r:.1}\"/>"));
        p.label(x[i], y[i], 0.0, -r - 4.0, "middle", "axp-lt", &format!("w = {}", n4(w[i])));
    }
    if let Some((ux, uy)) = plain {
        p.dots(&[(ux, uy)], 3, "unweighted centre");
    }
    p.mark(wx, wy, &format!("({}, {})", n4(wx), n4(wy)));
    let body = p.finish("x", "y", "");
    Some(s.chart("map", "Weighted mean centre",
                 &format!("Points ({}; {}) with weights {} (dot area grows with weight) and the weighted mean centre the \
                           model returns, ({}, {}), pulled toward the heavy points{}; the circle has the returned weighted \
                           standard distance {} as radius.", lst(&x), lst(&y), lst(&w), n6(wx), n6(wy),
                          plain.map(|(a, b)| format!(" (the unweighted centre, from the model's mean-centre capability, is \
                                                      ({}, {}))", n6(a), n6(b))).unwrap_or_default(), n6(wsd)), &body))
}

pub fn sd_ellipse(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (sx, sy, th) = (s.out("sx")?, s.out("sy")?, s.out("orientation")?);
    let a = s.solve_other("geo_mean_center_standard_distance", &[("x", jl(&x)), ("y", jl(&y))])?;
    let (mx, my) = (o(&a, "mean_x")?, o(&a, "mean_y")?);
    let r = sx.max(sy);
    let mut xs = x.clone();
    let mut ys = y.clone();
    xs.extend([mx - r, mx + r]);
    ys.extend([my - r, my + r]);
    let mut p = Plot::equal(&xs, &ys)?;
    let t = th.to_radians();
    let (ux, uy) = (t.sin(), t.cos());
    p.dashed(&[(mx - ux * sx * 1.15, my - uy * sx * 1.15), (mx + ux * sx * 1.15, my + uy * sx * 1.15)], 3, "");
    p.ellipse(mx, my, sx, sy, 90.0 - th, 0, "standard deviational ellipse");
    p.dots(&pts_of(&x, &y), 3, &format!("{} points", x.len()));
    p.mark(mx, my, &format!("orientation {}", crate::figures::dg(th)));
    let body = p.finish("x", "y", "");
    Some(s.chart("map", "Standard deviational ellipse of a point pattern",
                 &format!("The standard deviational ellipse of {} points, centred on their mean centre ({}, {}) from the \
                           model's mean-centre capability, with the returned semi-axes sx = {} (along the dashed axis, at {} \
                           clockwise from north) and sy = {}.", x.len(), n6(mx), n6(my), n6(sx), crate::figures::dg(th),
                          n6(sy)), &body))
}

pub fn nearest_neighbours(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let n = x.len();
    if n < 2 {
        return None;
    }
    let d_obs = s.out("d_obs")?;
    let mut nn = vec![];
    for i in 0..n {
        let mut best = (usize::MAX, f64::INFINITY);
        for j in 0..n {
            let d = ((x[i] - x[j]).powi(2) + (y[i] - y[j]).powi(2)).sqrt();
            if i != j && d < best.1 {
                best = (j, d);
            }
        }
        nn.push(best);
    }
    // the pairs found must give the model's own mean distance
    let mean = nn.iter().map(|b| b.1).sum::<f64>() / n as f64;
    if !agrees(mean, d_obs) {
        return None;
    }
    let area = s.inp("area").filter(|a| *a > 0.0);
    let (x0, y0) = (x.iter().cloned().fold(f64::MAX, f64::min), y.iter().cloned().fold(f64::MAX, f64::min));
    let side = area.map(|a| a.sqrt()).unwrap_or(0.0);
    let mut xs = x.clone();
    let mut ys = y.clone();
    if side > 0.0 {
        xs.extend([x0, x0 + side]);
        ys.extend([y0, y0 + side]);
    }
    let mut p = Plot::equal(&xs, &ys)?;
    if side > 0.0 {
        p.dashed(&[(x0, y0), (x0 + side, y0), (x0 + side, y0 + side), (x0, y0 + side), (x0, y0)], 3,
                 &format!("study area {} (square)", n4(area?)));
    }
    for (i, (j, _)) in nn.iter().enumerate() {
        p.arrow((x[i], y[i]), (x[*j], y[*j]), 0, 4.0);
    }
    p.key("to the nearest neighbour", 0);
    p.dots(&pts_of(&x, &y), 3, "");
    let body = p.finish("x", "y", "");
    Some(s.chart("map", "Nearest-neighbour distances of a point pattern",
                 &format!("The {} points x = {}, y = {} with an arrow to each one's nearest neighbour (found here from the \
                           inputs; their mean distance equals the returned {}). The model returns the mean nearest-neighbour \
                           distance {} against {} expected for a random pattern{}: R = {} ({}), z = {}, p = {}.", n, lst(&x),
                          lst(&y), n6(d_obs), n6(d_obs), n6(s.out("d_exp")?),
                          area.map(|a| format!(" in area {}", n6(a))).unwrap_or_default(), n6(s.out("R")?),
                          if s.out("R")? < 1.0 { "below 1 leans to clustering" } else { "above 1 leans to dispersion" },
                          n6(s.out("z")?), n6(s.out("p_value")?)), &body))
}

pub fn quadrats(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (nx, ny) = (iv(s, "nx")? as usize, iv(s, "ny")? as usize);
    let counts = s.out_list("counts")?;
    if nx == 0 || ny == 0 || nx * ny != counts.len() || nx * ny > 400 {
        return None;
    }
    let (x0, x1) = (x.iter().cloned().fold(f64::MAX, f64::min), x.iter().cloned().fold(f64::MIN, f64::max));
    let (y0, y1) = (y.iter().cloned().fold(f64::MAX, f64::min), y.iter().cloned().fold(f64::MIN, f64::max));
    if !(x1 > x0 && y1 > y0) {
        return None;
    }
    let (dx, dy) = ((x1 - x0) / nx as f64, (y1 - y0) / ny as f64);
    let mut c = vec![0.0; nx * ny];
    for i in 0..x.len() {
        let cx = (((x[i] - x0) / dx).floor() as usize).min(nx - 1);
        let cy = (((y[i] - y0) / dy).floor() as usize).min(ny - 1);
        c[cy * nx + cx] += 1.0;
    }
    // the grid drawn must give the model's own counts (row by row from the bottom left)
    if c != counts {
        return None;
    }
    let mut p = Plot::equal(&[x0, x1], &[y0, y1])?;
    let cmax = counts.iter().cloned().fold(1.0, f64::max);
    for j in 0..ny {
        for i in 0..nx {
            let v = counts[j * nx + i];
            let (a, b) = (x0 + dx * i as f64, y0 + dy * j as f64);
            p.cell(a, b, a + dx, b + dy, 0.08 + 0.4 * v / cmax, false);
            p.label(a + dx / 2.0, b + dy / 2.0, 0.0, 5.0, "middle", "axp-bt", &n6(v));
        }
    }
    for i in 0..=nx {
        let a = x0 + dx * i as f64;
        p.line(&[(a, y0), (a, y1)], 3, "");
    }
    for j in 0..=ny {
        let b = y0 + dy * j as f64;
        p.line(&[(x0, b), (x1, b)], 3, "");
    }
    p.dots(&pts_of(&x, &y), 0, &format!("{} points", x.len()));
    let body = p.finish("x", "y", "");
    Some(s.chart("map", "Quadrat counts of a point pattern",
                 &format!("The {} points in a {} × {} grid of quadrats over their extent, each quadrat labelled with the \
                           count the model returns (row by row from the bottom left: {}). χ² = {} on {} degrees of freedom, \
                           p = {}; variance-to-mean ratio {} ({}).", x.len(), nx, ny,
                          counts.iter().map(|v| n6(*v)).collect::<Vec<_>>().join(", "), n6(s.out("chi2")?), n6(s.out("df")?),
                          n6(s.out("p_value")?), n6(s.out("vmr")?),
                          if s.out("vmr")? < 1.0 { "below 1: more even than random" } else { "above 1: more clustered than random" }),
                 &body))
}

pub fn band_links(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (w, nb, th) = (om(s.a, "W")?, s.out_list("neighbours")?, s.inp("threshold")?);
    if w.len() != x.len() || nb.len() != x.len() {
        return None;
    }
    let mut xs = x.clone();
    let mut ys = y.clone();
    xs.extend(x.iter().map(|v| v - th).chain(x.iter().map(|v| v + th)));
    ys.extend(y.iter().map(|v| v - th).chain(y.iter().map(|v| v + th)));
    let mut p = Plot::equal(&xs, &ys)?;
    for i in 0..x.len() {
        p.circle(x[i], y[i], th, 3, true, "");
        for j in i + 1..x.len() {
            if w[i][j] != 0.0 || w[j][i] != 0.0 {
                p.line(&[(x[i], y[i]), (x[j], y[j])], 0, "neighbours (W = 1)");
            }
        }
    }
    p.dots(&pts_of(&x, &y), 0, "");
    for i in 0..x.len() {
        p.label(x[i], y[i], 0.0, if i % 2 == 0 { -10.0 } else { 20.0 }, "middle", "axp-lt", &format!("{}: {}", i + 1, n6(nb[i])));
    }
    let body = p.finish("x", "y", "");
    Some(s.chart("map", "Distance-band neighbours",
                 &format!("Points at x = {}, y = {} with a distance band of {} (dashed circles of that radius round each \
                           point): the links are the pairs the model's weights matrix joins; labels give each point's number \
                           of neighbours as returned. Returned: {} without neighbours, S0 = {}.", lst(&x), lst(&y), n6(th),
                          n6(s.out("islands")?), n6(s.out("s0")?)), &body))
}

pub fn knn_links(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let w = om(s.a, "W")?;
    if w.len() != x.len() {
        return None;
    }
    let kk = iv(s, "k")?;
    let mut p = Plot::equal(&x, &y)?;
    for i in 0..x.len() {
        for j in 0..x.len() {
            if i != j && w[i][j] != 0.0 {
                p.arrow((x[i], y[i]), (x[j], y[j]), 0, 6.0);
            }
        }
    }
    p.key(&format!("to each of the {} nearest", n6(kk)), 0);
    p.dots(&pts_of(&x, &y), 3, "");
    for i in 0..x.len() {
        p.label(x[i], y[i], 0.0, -10.0, "middle", "axp-bt", &format!("{}", i + 1));
    }
    let body = p.finish("x", "y", "");
    let sym = ot(s.a, "symmetric").unwrap_or_default();
    Some(s.chart("map", "k-nearest-neighbour links",
                 &format!("k-nearest-neighbour weights (k = {}) for points x = {}, y = {}: an arrow from each point to every \
                           neighbour the model's matrix gives it. Returned symmetric: {} ({}); S0 = {}.", n6(kk), lst(&x),
                          lst(&y), sym, if sym == "True" { "every link is mutual" } else { "some links point one way only" },
                          n6(s.out("s0")?)), &body))
}

pub fn detour(s: &Solved) -> Option<Figure> {
    let xy_ = s.mat("coords")?;
    if xy_.iter().any(|r| r.len() != 2) {
        return None;
    }
    let es = edges(s, "edges")?;
    let (a, b) = (iv(s, "origin")? as usize, iv(s, "destination")? as usize);
    if a >= xy_.len() || b >= xy_.len() || es.iter().any(|e| e.0 >= xy_.len() || e.1 >= xy_.len()) {
        return None;
    }
    let (nd, ed, di) = (s.out("network_distance")?, s.out("euclidean_distance")?, s.out("detour_index")?);
    let xs: Vec<f64> = xy_.iter().map(|r| r[0]).collect();
    let ys: Vec<f64> = xy_.iter().map(|r| r[1]).collect();
    let mut p = Plot::equal(&xs, &ys)?;
    for e in &es {
        p.line(&[(xs[e.0], ys[e.0]), (xs[e.1], ys[e.1])], 2, &format!("network, path {}", n4(nd)));
    }
    p.dashed(&[(xs[a], ys[a]), (xs[b], ys[b])], 0, &format!("straight line {}", n4(ed)));
    p.dots(&pts_of(&xs, &ys), 3, "");
    for i in 0..xs.len() {
        let t = if i == a { format!("{i} origin") } else if i == b { format!("{i} destination") } else { format!("{i}") };
        p.label(xs[i], ys[i], 0.0, -10.0, "middle", "axp-lt", &t);
    }
    let body = p.finish("x", "y", &format!("detour index {}", n4(di)));
    Some(s.chart("map", "Network path against the straight line",
                 &format!("Nodes at the stated coordinates joined by the edges given: from node {a} to node {b} the model \
                           returns a network distance {} against a straight-line distance {}, a detour index of {}.", n6(nd),
                          n6(ed), n6(di)), &body))
}

pub fn frechet(s: &Solved) -> Option<Figure> {
    let (x1, y1, x2, y2) = (s.vec("x1")?, s.vec("y1")?, s.vec("x2")?, s.vec("y2")?);
    if x1.len() != y1.len() || x2.len() != y2.len() || x1.is_empty() || x2.is_empty() {
        return None;
    }
    let d = s.out("distance")?;
    let (a, b) = (pts_of(&x1, &y1), pts_of(&x2, &y2));
    // the leash: a vertex pair at exactly the returned distance (the coupling's longest link)
    let mut leash = None;
    for p in &a {
        for q in &b {
            if agrees(((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2)).sqrt(), d) && leash.is_none() {
                leash = Some((*p, *q));
            }
        }
    }
    let (lp, lq) = leash?;
    let xs: Vec<f64> = a.iter().chain(b.iter()).map(|p| p.0).collect();
    let ys: Vec<f64> = a.iter().chain(b.iter()).map(|p| p.1).collect();
    let mut p = Plot::equal(&xs, &ys)?;
    p.line_dots(&a, 0, "first line");
    p.line_dots(&b, 2, "second line");
    p.dashed(&[lp, lq], 3, "");
    p.label((lp.0 + lq.0) / 2.0, (lp.1 + lq.1) / 2.0, 8.0, 4.0, "start", "axp-vt", &format!("leash {}", n4(d)));
    let body = p.finish("x", "y", &format!("Fréchet distance {}", n4(d)));
    Some(s.chart("lines", "Discrete Fréchet distance between two lines",
                 &format!("Two polylines, ({}, {}) and ({}, {}), and the discrete Fréchet distance the model returns, {}: the \
                           shortest leash that lets a walker on each line advance vertex by vertex, never backwards, while \
                           staying connected. Dashed: a vertex pair exactly that far apart, where the leash is longest.",
                          lst(&x1), lst(&y1), lst(&x2), lst(&y2), n6(d)), &body))
}

pub fn simplification(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let t = s.inp("tolerance")?;
    let kept = s.out_list("kept")?;
    let mut finer = None;
    let mut tf = t;
    for f in [0.4, 0.1, 0.02] {
        if let Some(k2) = s.with(&[("tolerance", t * f)]).and_then(|a| ol(&a, "kept")) {
            tf = t * f;
            let differs = k2.len() != kept.len();
            finer = Some(k2);
            if differs {
                break;
            }
        }
    }
    let finer = finer?;
    let pick = |idx: &[f64]| -> Option<Vec<(f64, f64)>> {
        idx.iter().map(|i| { let i = *i as usize; (i < x.len()).then(|| (x[i], y[i])) }).collect()
    };
    let (k1, k2) = (pick(&kept)?, pick(&finer)?);
    let all = pts_of(&x, &y);
    let mut p = Plot::equal(&x, &y)?;
    p.line_dots(&all, 3, &format!("original line ({} vertices)", x.len()));
    p.line_dots(&k2, 2, &format!("tolerance {}: {} kept", n4(tf), finer.len()));
    p.line_dots(&k1, 0, &format!("tolerance {}: {} kept", n4(t), kept.len()));
    let body = p.finish("x", "y", "");
    let ix = |v: &[f64]| v.iter().map(|i| n6(*i)).collect::<Vec<_>>().join(", ");
    Some(s.chart("shape", "Line simplification at two tolerances",
                 &format!("A {}-vertex line simplified by the Douglas-Peucker algorithm at two tolerances, the kept vertices \
                           as returned by the model: tolerance {} keeps {} vertices ({}); tolerance {} keeps {} ({}).",
                          x.len(), n6(tf), finer.len(), ix(&finer), n6(t), kept.len(), ix(&kept)), &body))
}

pub fn compactness(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let refs: [(&str, Vec<f64>, Vec<f64>); 4] = [
        ("square", vec![0.0, 1.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 1.0]),
        ("long strip", vec![0.0, 6.0, 6.0, 0.0], vec![0.0, 0.0, 1.0, 1.0]),
        ("L shape", vec![0.0, 2.0, 2.0, 1.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 1.0, 2.0, 2.0]),
        ("triangle", vec![0.0, 2.0, 1.0], vec![0.0, 0.0, 3f64.sqrt()]),
    ];
    let mut shapes = vec![("stated polygon".to_string(), x.clone(), y.clone(), s.out("polsby_popper")?, s.out("reock")?)];
    for (n, a, b) in refs.iter() {
        let r = s.solve_other(s.key, &[("x", jl(a)), ("y", jl(b))])?;
        shapes.push((n.to_string(), a.clone(), b.clone(), o(&r, "polsby_popper")?, o(&r, "reock")?));
    }
    let mut c = Canvas::new(250.0);
    let cw = 520.0 / shapes.len() as f64;
    for (i, (n, a, b, pp, re)) in shapes.iter().enumerate() {
        let (xl, xh) = (a.iter().cloned().fold(f64::MAX, f64::min), a.iter().cloned().fold(f64::MIN, f64::max));
        let (yl, yh) = (b.iter().cloned().fold(f64::MAX, f64::min), b.iter().cloned().fold(f64::MIN, f64::max));
        let sc = 80.0 / (xh - xl).max(yh - yl).max(1e-12);
        let cx = cw * (i as f64 + 0.5);
        let (ox, oy) = (cx - sc * (xh - xl) / 2.0, 120.0 + sc * (yh - yl) / 2.0);
        let mut d = String::new();
        for (j, (u, v)) in a.iter().zip(b).enumerate() {
            d.push_str(&format!("{}{:.1},{:.1} ", if j == 0 { "M" } else { "L" }, ox + sc * (u - xl), oy - sc * (v - yl)));
        }
        let cl = if i == 0 { ("axp-s", "axp-c") } else { ("axp-s4", "axp-c4") };
        c.raw(&format!("<path class=\"{}\" d=\"{d}Z\"/><path class=\"{}\" d=\"{d}Z\"/>", cl.0, cl.1));
        c.bold(cx, 40.0, "middle", n);
        c.value(cx, 200.0, "middle", &format!("PP {}", n4(*pp)));
        c.value(cx, 218.0, "middle", &format!("Reock {}", n4(*re)));
    }
    let list: Vec<String> = shapes.iter().map(|(n, _, _, pp, re)| format!("{n}: Polsby-Popper {}, Reock {}", n6(*pp), n6(*re)))
        .collect();
    Some(s.figure("polygons", "Compactness of five shapes",
                  &format!("Five polygons and the compactness scores the model returns for each (1 = a circle): {}. The \
                            stated polygon ({}; {}) has area {}, perimeter {}, Schwartzberg {}, convex-hull ratio {}.",
                           list.join("; "), lst(&x), lst(&y), n6(s.out("area")?), n6(s.out("perimeter")?),
                           n6(s.out("schwartzberg")?), n6(s.out("convex_hull_ratio")?)), c.body(), c.h))
}

/// Shaded cells of a grid of values on a plot (darker = larger), with the low and high values noted.
fn shade(p: &mut Plot, xs: &[f64], ys: &[f64], vals: &[Vec<Option<f64>>]) -> (f64, f64) {
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let lo = all.iter().cloned().fold(f64::MAX, f64::min);
    let hi = all.iter().cloned().fold(f64::MIN, f64::max);
    let (dx, dy) = ((xs[1] - xs[0]) / 2.0, (ys[1] - ys[0]) / 2.0);
    for (i, col) in vals.iter().enumerate() {
        for (j, v) in col.iter().enumerate() {
            if let Some(v) = v {
                let op = if hi > lo { 0.06 + 0.8 * (v - lo) / (hi - lo) } else { 0.4 };
                p.cell(xs[i] - dx, ys[j] - dy, xs[i] + dx, ys[j] + dy, op, false);
            }
        }
    }
    (lo, hi)
}

fn grid(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let d = (hi - lo) / n as f64;
    (0..n).map(|i| lo + d * (i as f64 + 0.5)).collect()
}

/// The answer at every cell centre of a grid over the samples (each a run of the model), the visitor's point marked.
pub fn surface(s: &Solved, out: &str, title: &str) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (px, py, v) = (s.inp("px")?, s.inp("py")?, s.out(out)?);
    let pad = s.inp("bandwidth").unwrap_or(0.0);
    let (xl, xh) = (x.iter().cloned().fold(px, f64::min) - pad, x.iter().cloned().fold(px, f64::max) + pad);
    let (yl, yh) = (y.iter().cloned().fold(py, f64::min) - pad, y.iter().cloned().fold(py, f64::max) + pad);
    let side = (xh - xl).max(yh - yl).max(1e-9);
    let (xl, xh, yl, yh) = (xl - side * 0.05, xh + side * 0.05, yl - side * 0.05, yh + side * 0.05);
    let n = 18;
    let (gx, gy) = (grid(xl, xh, n), grid(yl, yh, n));
    let mut vals = vec![];
    let mut runs = 0;
    for a in &gx {
        let mut col = vec![];
        for b in &gy {
            let r = s.with(&[("px", *a), ("py", *b)]).and_then(|r| o(&r, out));
            runs += r.is_some() as usize;
            col.push(r);
        }
        vals.push(col);
    }
    if runs < 20 {
        return None;
    }
    let mut p = Plot::equal(&[xl, xh], &[yl, yh])?;
    let (lo, hi) = shade(&mut p, &gx, &gy, &vals);
    p.dots(&pts_of(&x, &y), 3, "");
    if let Some(z) = s.vec("z").filter(|z| z.len() == x.len()) {
        for i in 0..x.len() {
            p.label(x[i], y[i], 0.0, -9.0, "middle", "axp-lt", &format!("z = {}", n4(z[i])));
        }
    }
    p.mark(px, py, &n6(v));
    let body = p.finish("x", "y", &format!("{out}: light {} to dark {}", n4(lo), n4(hi)));
    let what = if out == "density" {
        format!("Kernel density of {} events (x = {}, y = {}) with a {} kernel of bandwidth {}", x.len(), lst(&x), lst(&y),
                s.txt("kernel").unwrap_or_default(), n6(s.inp("bandwidth")?))
    } else {
        format!("Inverse-distance-weighted surface (power {}) from {} samples z = {} at x = {}, y = {}", n6(s.inp("power")?),
                x.len(), lst(&s.vec("z")?), lst(&x), lst(&y))
    };
    Some(s.chart("map", title,
                 &format!("{what}: each of the {runs} cells is shaded by the {out} the model returns at its centre (light {} \
                           to dark {}). Marked: ({}, {}), {out} {}.", n6(lo), n6(hi), n6(px), n6(py), n6(v)), &body))
}

pub fn kriging(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (px, py, est, var) = (s.inp("px")?, s.inp("py")?, s.out("estimate")?, s.out("variance")?);
    let (xl, xh) = (x.iter().cloned().fold(px, f64::min), x.iter().cloned().fold(px, f64::max));
    let (yl, yh) = (y.iter().cloned().fold(py, f64::min), y.iter().cloned().fold(py, f64::max));
    let side = (xh - xl).max(yh - yl).max(1e-9);
    let (xl, xh, yl, yh) = (xl - side * 0.05, xh + side * 0.05, yl - side * 0.05, yh + side * 0.05);
    let n = 16;
    let (gx, gy) = (grid(xl, xh, n), grid(yl, yh, n));
    let (mut ve, mut vv) = (vec![], vec![]);
    let mut runs = 0;
    for a in &gx {
        let (mut ce, mut cv) = (vec![], vec![]);
        for b in &gy {
            let r = s.with(&[("px", *a), ("py", *b)]);
            let e = r.as_ref().and_then(|r| o(r, "estimate"));
            runs += e.is_some() as usize;
            ce.push(e);
            cv.push(r.as_ref().and_then(|r| o(r, "variance")));
        }
        ve.push(ce);
        vv.push(cv);
    }
    if runs < 20 {
        return None;
    }
    let z = s.vec("z")?;
    let panel = |vals: &[Vec<Option<f64>>], name: &str, mine: f64| -> Option<String> {
        let mut p = Plot::equal(&[xl, xh], &[yl, yh])?;
        let (lo, hi) = shade(&mut p, &gx, &gy, vals);
        p.dots(&pts_of(&x, &y), 3, "");
        for i in 0..x.len().min(z.len()) {
            p.label(x[i], y[i], 0.0, -9.0, "middle", "axp-lt", &format!("z = {}", n4(z[i])));
        }
        p.mark(px, py, &n6(mine));
        Some(p.finish("x", "y", &format!("{name}: light {} to dark {}", n4(lo), n4(hi))))
    };
    let (body, h) = k::stack(&[panel(&ve, "estimate", est)?, panel(&vv, "variance", var)?]);
    Some(s.figure("map", "Kriging estimate and its variance",
                  &format!("Ordinary kriging from {} samples z = {} ({} variogram, nugget {}, sill {}, range {}): top, the \
                            estimate the model returns at each of {} cell centres; bottom, the kriging variance, smallest at \
                            the samples and largest away from them. Marked: ({}, {}), estimate {}, variance {}.", x.len(),
                           lst(&z), s.txt("model").unwrap_or_default(), n6(s.inp("nugget")?), n6(s.inp("sill")?),
                           n6(s.inp("range")?), runs, n6(px), n6(py), n6(est), n6(var)), &body, h))
}

/// The polynomial trend surface's value from its coefficients (order 1: 1, x, y; order 2: 1, x, y, x², xy, y²).
fn trend_eval(c: &[f64], x: f64, y: f64) -> Option<f64> {
    match c.len() {
        3 => Some(c[0] + c[1] * x + c[2] * y),
        6 => Some(c[0] + c[1] * x + c[2] * y + c[3] * x * x + c[4] * x * y + c[5] * y * y),
        _ => None,
    }
}

pub fn trend_surface(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (z, c, r2) = (s.vec("z")?, s.out_list("coefficients")?, s.out("r_squared")?);
    let fit: Vec<f64> = x.iter().zip(&y).map(|(a, b)| trend_eval(&c, *a, *b)).collect::<Option<Vec<f64>>>()?;
    // refit the model to the surface's own values: the same coefficients must come back
    let again = s.with_input(&[("z", jl(&fit))]).and_then(|a| ol(&a, "coefficients"))?;
    if !same_fit(&again, &c) {
        return None;
    }
    let (xl, xh) = (x.iter().cloned().fold(f64::MAX, f64::min), x.iter().cloned().fold(f64::MIN, f64::max));
    let (yl, yh) = (y.iter().cloned().fold(f64::MAX, f64::min), y.iter().cloned().fold(f64::MIN, f64::max));
    let side = (xh - xl).max(yh - yl).max(1e-9);
    let (xl, xh, yl, yh) = (xl - side * 0.1, xh + side * 0.1, yl - side * 0.1, yh + side * 0.1);
    let n = 24;
    let (gx, gy) = (grid(xl, xh, n), grid(yl, yh, n));
    let vals: Vec<Vec<Option<f64>>> = gx.iter().map(|a| gy.iter().map(|b| trend_eval(&c, *a, *b)).collect()).collect();
    let mut p = Plot::equal(&[xl, xh], &[yl, yh])?;
    let (lo, hi) = shade(&mut p, &gx, &gy, &vals);
    p.dots(&pts_of(&x, &y), 3, "");
    for i in 0..x.len().min(z.len()) {
        p.label(x[i], y[i], 0.0, -9.0, "middle", "axp-lt", &format!("z = {}", n4(z[i])));
    }
    let body = p.finish("x", "y", &format!("fitted z: light {} to dark {}", n4(lo), n4(hi)));
    let eqn = if c.len() == 3 {
        format!("z = {} + {}·x + {}·y", n6(c[0]), n6(c[1]), n6(c[2]))
    } else {
        format!("z = {} + {}·x + {}·y + {}·x² + {}·x·y + {}·y²", n6(c[0]), n6(c[1]), n6(c[2]), n6(c[3]), n6(c[4]), n6(c[5]))
    };
    Some(s.chart("map", "Trend surface through the samples",
                 &format!("A trend surface fitted to {} samples (z = {} at the drawn points): the shading is {eqn} with the \
                           coefficients the model returns (evaluated on a grid; refitting the model to the surface's own \
                           values gives the same coefficients); R² = {}.", x.len(), lst(&z), n6(r2)), &body))
}

pub fn semivariogram(s: &Solved) -> Option<Figure> {
    let (x, y) = xy(s)?;
    let (z, edges_) = (s.vec("z")?, s.vec("bin_edges")?);
    let (g, cnt) = (s.out_list("gamma")?, s.out_list("counts")?);
    if z.len() != x.len() || edges_.len() != g.len() + 1 || cnt.len() != g.len() {
        return None;
    }
    let flat = y.iter().all(|v| (v - y[0]).abs() < 1e-12);
    let top = if flat {
        let mut p = Plot::new(&x, &z, false, false)?;
        p.line_dots(&pts_of(&x, &z), 2, "samples");
        p.finish("x (samples along a line)", "z", "")
    } else {
        let mut p = Plot::equal(&x, &y)?;
        p.dots(&pts_of(&x, &y), 2, "samples");
        for i in 0..x.len() {
            p.label(x[i], y[i], 0.0, -9.0, "middle", "axp-lt", &format!("z = {}", n4(z[i])));
        }
        p.finish("x", "y", "")
    };
    let gs: Vec<f64> = g.iter().cloned().filter(|v| v.is_finite()).chain([0.0]).collect();
    let mut p = Plot::new(&edges_, &gs, true, false)?;
    for i in 0..g.len() {
        if !g[i].is_finite() {
            continue;
        }
        p.polygon(&[(edges_[i], 0.0), (edges_[i + 1], 0.0), (edges_[i + 1], g[i]), (edges_[i], g[i])], 0, "");
        p.note((edges_[i] + edges_[i + 1]) / 2.0, g[i], -8.0, &n4(g[i]));
        p.label((edges_[i] + edges_[i + 1]) / 2.0, g[i], 0.0, 17.0, "middle", "axp-lt", &format!("{} pairs", n6(cnt[i])));
    }
    let bottom = p.finish("lag distance h", "semivariance γ(h)", "");
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("variogram", "Samples and their empirical semivariogram",
                  &format!("Top: the samples z = {} at x = {}, y = {}. Bottom: the empirical semivariogram the model returns \
                            for lag bins {}: each bar spans its bin and has the returned semivariance as height, labelled with \
                            the number of pairs ({}).", lst(&z), lst(&x), lst(&y), lst(&edges_), lst(&g)), &body, h))
}

/// The variogram model's value at lag h from its parameters (practical range: the model reaches 95 % of its partial
/// sill there for the exponential and gaussian forms).
fn vario(model: &str, nug: f64, ps: f64, r: f64, h: f64) -> Option<f64> {
    if !(r > 0.0) {
        return None;
    }
    Some(match model {
        "exponential" => nug + ps * (1.0 - (-3.0 * h / r).exp()),
        "gaussian" => nug + ps * (1.0 - (-3.0 * h * h / (r * r)).exp()),
        "spherical" => nug + ps * if h < r { 1.5 * h / r - 0.5 * (h / r).powi(3) } else { 1.0 },
        _ => return None,
    })
}

pub fn variogram_fit(s: &Solved) -> Option<Figure> {
    let (h, g) = (s.vec("h")?, s.vec("gamma")?);
    let model = s.txt("model")?.to_lowercase();
    let (nug, ps, r, sill) = (s.out("nugget")?, s.out("psill")?, s.out("range")?, s.out("sill")?);
    let fit: Vec<f64> = h.iter().map(|x| vario(&model, nug, ps, r, *x)).collect::<Option<Vec<f64>>>()?;
    let again = s.with_input(&[("gamma", jl(&fit))])?;
    if !same_fit(&[o(&again, "nugget")?, o(&again, "psill")?, o(&again, "range")?], &[nug, ps, r]) {
        return None;
    }
    let hmax = h.iter().cloned().fold(0.0, f64::max);
    let curve: Vec<(f64, f64)> = k::span(0.0, hmax * 1.05, 80, false).iter()
        .filter_map(|x| vario(&model, nug, ps, r, *x).map(|y| (*x, y))).collect();
    let (xs, ys): (Vec<f64>, Vec<f64>) = curve.iter().copied().chain(pts_of(&h, &g)).unzip();
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.line(&curve, 0, &format!("fitted {model} model"));
    p.dots(&pts_of(&h, &g), 2, "empirical values (input)");
    p.hline_at(sill, &format!("sill {}", n4(sill)), true);
    p.vline(r, &format!("range {}", n4(r)));
    let body = p.finish("lag distance h", "semivariance γ(h)", "");
    Some(s.chart("variogram", &format!("The {model} variogram model fitted to the data"),
                 &format!("The empirical semivariances γ = {} at lags h = {} and the {model} model with the parameters the \
                           model fits: nugget {}, partial sill {}, sill {}, range {}; the curve is evaluated from those \
                           returned values, and refitting the model to the curve's own values gives the same parameters.",
                          lst(&g), lst(&h), n6(nug), n6(ps), n6(sill), n6(r)), &body))
}

pub fn density_gradient(s: &Solved) -> Option<Figure> {
    let (r, d) = (s.vec("r")?, s.vec("D")?);
    let (d0, b, r2, half) = (s.out("D0")?, s.out("b")?, s.out("r_squared")?, s.out("half_distance")?);
    let fit: Vec<f64> = r.iter().map(|x| d0 * (-b * x).exp()).collect();
    let again = s.with_input(&[("D", jl(&fit))])?;
    if !same_fit(&[o(&again, "D0")?, o(&again, "b")?], &[d0, b]) || !agrees(half * b, std::f64::consts::LN_2) {
        return None;
    }
    let rmax = r.iter().cloned().fold(half, f64::max);
    let curve: Vec<(f64, f64)> = k::span(0.0, rmax * 1.05, 60, false).iter().map(|x| (*x, d0 * (-b * x).exp())).collect();
    let (xs, ys): (Vec<f64>, Vec<f64>) = curve.iter().copied().chain(pts_of(&r, &d)).unzip();
    let mut p = Plot::logs(&xs, &ys, false, true)?;
    p.line(&curve, 0, &format!("D = {}·e^(−{} r)", n4(d0), n4(b)));
    p.dots(&pts_of(&r, &d), 2, "observed (input)");
    p.mark(half, d0 / 2.0, &format!("half at r = {}", n4(half)));
    let body = p.finish("distance from the centre r", "density D", "");
    Some(s.chart("fit", "Urban density gradient",
                 &format!("Population densities D = {} at distances r = {} on a logarithmic density axis, and the \
                           negative-exponential gradient with the parameters the model returns (D0 = {}, b = {}, R² = {}; \
                           the line is evaluated from them, and refitting the model to the line's own values gives the same \
                           parameters). Marked: the returned half distance {}, where density falls to D0/2.", lst(&d),
                          lst(&r), n6(d0), n6(b), n6(r2), n6(half)), &body))
}

pub fn rank_size(s: &Solved) -> Option<Figure> {
    let mut pop = s.vec("populations")?;
    pop.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let (q, p1, r2, prim) = (s.out("q")?, s.out("P1_fit")?, s.out("r_squared")?, s.out("primacy")?);
    let fit: Vec<f64> = (1..=pop.len()).map(|i| p1 * (i as f64).powf(-q)).collect();
    let again = s.with_input(&[("populations", jl(&fit))])?;
    if !same_fit(&[o(&again, "q")?, o(&again, "P1_fit")?], &[q, p1]) {
        return None;
    }
    let pts: Vec<(f64, f64)> = pop.iter().enumerate().map(|(i, v)| ((i + 1) as f64, *v)).collect();
    let line: Vec<(f64, f64)> = k::span(1.0, pop.len() as f64, 40, true).iter().map(|x| (*x, p1 * x.powf(-q))).collect();
    let (xs, ys): (Vec<f64>, Vec<f64>) = line.iter().copied().chain(pts.iter().copied()).unzip();
    let mut p = Plot::logs(&xs, &ys, true, true)?;
    p.line(&line, 0, &format!("P = {}·rank^(−{})", n4(p1), n4(q)));
    p.dots(&pts, 2, "settlements (input, ranked)");
    p.mark(1.0, pop[0], &format!("primacy {}", n4(prim)));
    let body = p.finish("rank", "population", "");
    Some(s.chart("fit", "Rank-size rule of a settlement system",
                 &format!("Settlement populations ranked from largest to smallest on logarithmic axes, with the rank-size \
                           line of the returned exponent q = {} and fitted largest size {} (R² = {}; the line is evaluated \
                           from them, and refitting the model to the line's own values gives the same parameters). Marked: \
                           the largest settlement, primacy (largest over second) {}.", n6(q), n6(p1), n6(r2), n6(prim)), &body))
}

pub fn gravity_fit(s: &Solved) -> Option<Figure> {
    let (t, o_, d_, dist) = (s.vec("T")?, s.vec("O")?, s.vec("D")?, s.vec("d")?);
    let n = t.len();
    if o_.len() != n || d_.len() != n || dist.len() != n {
        return None;
    }
    let (kk, al, be, ga) = (s.out("k")?, s.out("alpha")?, s.out("beta")?, s.out("gamma")?);
    let fit: Vec<f64> = (0..n).map(|i| kk * o_[i].powf(al) * d_[i].powf(be) * dist[i].powf(-ga)).collect();
    let again = s.with_input(&[("T", jl(&fit))])?;
    if !same_fit(&[o(&again, "k")?, o(&again, "alpha")?, o(&again, "beta")?, o(&again, "gamma")?], &[kk, al, be, ga]) {
        return None;
    }
    let pts: Vec<(f64, f64)> = t.iter().zip(&fit).map(|(a, b)| (*a, *b)).collect();
    let lo = t.iter().chain(&fit).cloned().fold(f64::MAX, f64::min);
    let hi = t.iter().chain(&fit).cloned().fold(f64::MIN, f64::max);
    let mut p = Plot::logs(&[lo, hi], &[lo, hi], true, true)?;
    p.dashed(&[(lo, lo), (hi, hi)], 3, "perfect fit");
    p.dots(&pts, 0, "origin-destination pairs");
    let body = p.finish("observed flow T", "fitted flow", "");
    Some(s.chart("fit", "Gravity model: observed against fitted flows",
                 &format!("Observed flows T = {} against the flows of the gravity model fitted by the model, \
                           T̂ = k·O^α·D^β·d^(−γ) with k = {}, α = {}, β = {}, γ = {} (the fitted values are evaluated from \
                           these returned parameters and the inputs O, D, d; refitting the model to them gives the same \
                           parameters); points near the diagonal are well explained.", lst(&t), n6(kk), n6(al), n6(be),
                          n6(ga)), &body))
}

/// The row-standardised spatial lag of a variable (the model's spatial-lag capability).
fn lag(s: &Solved, v: &[f64], w: &[Vec<f64>]) -> Option<Vec<f64>> {
    let a = s.solve_other("geo_spatial_lag", &[("y", jl(v)), ("W", jm(w)), ("transform", Input::Text("row".into()))])?;
    let l = ol(&a, "lag")?;
    (l.len() == v.len()).then_some(l)
}

fn moran_axes(p: &mut Plot) {
    let (xl, xh, yl, yh) = p.bounds();
    p.dashed(&[(xl, 0.0), (xh, 0.0)], 3, "");
    p.dashed(&[(0.0, yl), (0.0, yh)], 3, "");
}

pub fn moran_scatter(s: &Solved) -> Option<Figure> {
    let (y, w) = (s.vec("y")?, s.mat("W")?);
    let l = lag(s, &y, &w)?;
    let m = y.iter().sum::<f64>() / y.len() as f64;
    let pts: Vec<(f64, f64)> = y.iter().zip(&l).map(|(a, b)| (a - m, b - m)).collect();
    let i_ = s.out("I")?;
    let row = s.txt("transform").map(|t| t == "row").unwrap_or(true);
    let (mut xs, mut ys): (Vec<f64>, Vec<f64>) = pts.iter().copied().unzip();
    xs.push(0.0);
    ys.push(0.0);
    let mut p = Plot::equal(&xs, &ys)?;
    moran_axes(&mut p);
    if row {
        let (xl, xh, _, _) = p.bounds();
        p.line(&[(xl, i_ * xl), (xh, i_ * xh)], 0, &format!("slope I = {}", n4(i_)));
    }
    p.dots(&pts, 2, "units");
    for (i, q) in pts.iter().enumerate() {
        p.label(q.0, q.1, 7.0, -6.0, "start", "axp-lt", &format!("{}", i + 1));
    }
    let body = p.finish("value − mean", "spatial lag − mean", "");
    Some(s.chart("scatter", "Moran scatter plot",
                 &format!("Moran scatter plot for y = {}: each unit's deviation from the mean against the deviation of its \
                           row-standardised spatial lag (lags returned by the model's spatial-lag capability). {}Expected \
                           without autocorrelation {}; z = {}, p = {} under normality. Points in the upper-right and \
                           lower-left quadrants are like their neighbours.", lst(&y),
                          if row { format!("The line through the origin has the returned Moran's I = {} as its slope. ", n6(i_)) }
                          else { format!("Returned Moran's I = {} (no slope line: the weights are not row-standardised). ", n6(i_)) },
                          n6(s.out("EI")?), n6(s.out("z_norm")?), n6(s.out("p_norm")?)), &body))
}

pub fn local_moran_scatter(s: &Solved) -> Option<Figure> {
    let (y, w) = (s.vec("y")?, s.mat("W")?);
    let (is, quad) = (s.out_list("Is")?, s.out_list("quadrant")?);
    if is.len() != y.len() || quad.len() != y.len() {
        return None;
    }
    let l = lag(s, &y, &w)?;
    let m = y.iter().sum::<f64>() / y.len() as f64;
    let pts: Vec<(f64, f64)> = y.iter().zip(&l).map(|(a, b)| (a - m, b - m)).collect();
    let (mut xs, mut ys): (Vec<f64>, Vec<f64>) = pts.iter().copied().unzip();
    xs.push(0.0);
    ys.push(0.0);
    let mut p = Plot::equal(&xs, &ys)?;
    moran_axes(&mut p);
    let names = ["", "quadrant 1 (high-high)", "quadrant 2 (low-high)", "quadrant 3 (low-low)", "quadrant 4 (high-low)"];
    for (i, q) in pts.iter().enumerate() {
        let qd = quad[i] as usize;
        let c = [3, 0, 1, 2, 3][qd.min(4)];
        p.dots(&[*q], c, names.get(qd).copied().unwrap_or(""));
        p.label(q.0, q.1, 7.0, if i % 2 == 0 { -6.0 } else { 14.0 }, "start", "axp-lt", &format!("{}: I = {}", i + 1, n4(is[i])));
    }
    let body = p.finish("value − mean", "spatial lag − mean", "");
    let list: Vec<String> = (0..y.len()).map(|i| format!("unit {} I = {} (quadrant {})", i + 1, n6(is[i]), n6(quad[i]))).collect();
    Some(s.chart("scatter", "Local Moran's I by unit and quadrant",
                 &format!("Local Moran's I for y = {}: each unit in the Moran scatter (deviation from the mean against its \
                           row-standardised lag, lags from the model's spatial-lag capability), coloured by the quadrant and \
                           labelled with the local I the model returns: {}.", lst(&y), list.join(", ")), &body))
}

pub fn bivariate_scatter(s: &Solved) -> Option<Figure> {
    let (x, y, w) = (s.vec("x")?, s.vec("y")?, s.mat("W")?);
    if x.len() != y.len() {
        return None;
    }
    let l = lag(s, &y, &w)?;
    let ixy = s.out("I_xy")?;
    let pts: Vec<(f64, f64)> = x.iter().zip(&l).map(|(a, b)| (*a, *b)).collect();
    let (xs, ys): (Vec<f64>, Vec<f64>) = pts.iter().copied().unzip();
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.dots(&pts, 2, "units");
    for (i, q) in pts.iter().enumerate() {
        p.label(q.0, q.1, 7.0, -6.0, "start", "axp-lt", &format!("{}", i + 1));
    }
    let body = p.finish("x at the unit", "spatial lag of y (neighbours' mean)", &format!("bivariate I = {}", n4(ixy)));
    Some(s.chart("scatter", "x against the neighbours' y",
                 &format!("Each unit's x = {} against the mean of y = {} over its neighbours (the row-standardised lag \
                           returned by the model's spatial-lag capability, with the stated weights). The model returns the \
                           bivariate Moran's I = {} ({}).", lst(&x), lst(&y), n6(ixy),
                          if ixy > 0.0 { "high x sits beside high y" } else { "high x sits beside low y" }), &body))
}

/// Node positions on a circle (a layout without meaning) for n nodes.
fn ring_layout(n: usize, cx: f64, cy: f64, r: f64) -> Vec<(f64, f64)> {
    // an even ring turned by half a step: no node at the far left or right, where a long label would cross an edge
    let turn = if n % 2 == 0 { std::f64::consts::PI / n as f64 } else { 0.0 };
    (0..n).map(|i| {
        let a = std::f64::consts::PI * 2.0 * i as f64 / n as f64 - std::f64::consts::FRAC_PI_2 + turn;
        (cx + r * a.cos(), cy + r * a.sin())
    }).collect()
}

struct Net {
    c: Canvas,
    pos: Vec<(f64, f64)>,
    centre: (f64, f64),
}

impl Net {
    fn new(n: usize, h: f64) -> Net {
        let centre = (260.0, 52.0 + (h - 90.0) / 2.0);
        let r = ((h - 150.0) / 2.0).min(150.0);
        Net { c: Canvas::new(h), pos: ring_layout(n, centre.0, centre.1, r), centre }
    }
    fn edge(&mut self, a: usize, b: usize, class: &str, label: &str) {
        let (p, q) = (self.pos[a], self.pos[b]);
        self.c.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p.0, p.1, q.0, q.1));
        if !label.is_empty() {
            let m = ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
            self.c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"16\" rx=\"3\"/>",
                                m.0 - 4.0 - 3.6 * label.chars().count() as f64, m.1 - 12.0, 8.0 + 7.2 * label.chars().count() as f64));
            self.c.value(m.0, m.1, "middle", label);
        }
    }
    fn node(&mut self, i: usize, class: &str, ring: bool, label: &str) {
        let p = self.pos[i];
        if ring {
            self.c.raw(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"15\"/>", p.0, p.1));
        }
        self.c.raw(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"10\"/>", p.0, p.1));
        // the label outside the ring, away from the centre
        let (dx, dy) = (p.0 - self.centre.0, p.1 - self.centre.1);
        let d = (dx * dx + dy * dy).sqrt().max(1e-9);
        let (ux, uy) = (dx / d, dy / d);
        let anchor = if ux > 0.3 { "start" } else if ux < -0.3 { "end" } else { "middle" };
        // a long label on two lines ("name (details)"), kept inside the canvas at a generous 7.8 px a character
        let (first, second) = match label.find(" (") {
            Some(i) => (&label[..i], label[i + 1..].to_string()),
            None => (label, String::new()),
        };
        let w = 7.8 * first.chars().count().max(second.chars().count()) as f64;
        let x = p.0 + ux * 22.0;
        let left = match anchor {
            "start" => x,
            "end" => x - w,
            _ => x - w / 2.0,
        };
        let x = x + if left < 6.0 { 6.0 - left } else if left + w > 514.0 { 514.0 - left - w } else { 0.0 };
        let y = p.1 + uy * 22.0 + 5.0 - if !second.is_empty() && uy < -0.3 { 15.0 } else { 0.0 };
        self.c.bold(x, y, anchor, first);
        if !second.is_empty() {
            self.c.name(x, y + 15.0, anchor, &second);
        }
    }
    fn legend(&mut self, items: &[(&str, &str)]) {
        let mut x = 16.0;
        let y = self.c.h - 12.0;
        for (class, t) in items {
            self.c.raw(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", x + 6.0, y - 4.0));
            self.c.name(x + 16.0, y, "start", t);
            x += 30.0 + 7.2 * t.chars().count() as f64;
        }
    }
}

/// Units linked by their weights matrix (a layout without meaning), each with its value and what the model returns.
pub fn unit_network(s: &Solved) -> Option<Figure> {
    let (y, w) = (s.vec("y")?, s.mat("W")?);
    let n = y.len();
    if w.len() != n || n < 2 || n > 24 || w.iter().any(|r| r.len() != n) {
        return None;
    }
    let m = y.iter().sum::<f64>() / n as f64;
    let mut net = Net::new(n, 420.0);
    let cap;
    let title = match s.key {
        "geo_gearys_c" => {
            for i in 0..n {
                for j in i + 1..n {
                    if w[i][j] != 0.0 || w[j][i] != 0.0 {
                        net.edge(i, j, "axp-c4", &format!("Δ² = {}", n4((y[i] - y[j]).powi(2))));
                    }
                }
            }
            for i in 0..n {
                net.node(i, if y[i] >= m { "axp-d" } else { "axp-d3" }, false, &format!("{}: y = {}", i + 1, n4(y[i])));
            }
            net.legend(&[("axp-d", "above the mean"), ("axp-d3", "below the mean")]);
            let c = s.out("C")?;
            cap = format!("The {n} units y = {} as a network of their neighbour links (W; a layout without meaning); on \
                           each link the squared difference of the two values (from the inputs). Geary's C compares these \
                           with the overall variance; the model returns C = {} ({}), z = {} under normality, {} under \
                           randomisation.", lst(&y), n6(c), if c < 1.0 { "below 1: neighbours are alike" } else {
                               "above 1: neighbours differ" }, n6(s.out("z_norm")?), n6(s.out("z_rand")?));
            "Neighbour differences behind Geary's C"
        }
        "geo_getis_ord_general_g" => {
            for i in 0..n {
                for j in i + 1..n {
                    if w[i][j] != 0.0 || w[j][i] != 0.0 {
                        net.edge(i, j, "axp-c4", "");
                    }
                }
            }
            for i in 0..n {
                net.node(i, if y[i] >= m { "axp-d" } else { "axp-d3" }, false, &format!("{}: {}", i + 1, n4(y[i])));
            }
            net.legend(&[("axp-d", "above the mean"), ("axp-d3", "below the mean")]);
            let (g, eg) = (s.out("G")?, s.out("EG")?);
            cap = format!("Units with values y = {} linked by the stated weights (a layout without meaning). The model \
                           returns General G = {} against its expectation {} (variance {}, z = {}): {}.", lst(&y), n6(g),
                          n6(eg), n6(s.out("VG")?), n6(s.out("z_norm")?),
                          if g > eg { "high values cluster" } else { "low values cluster" });
            "Clustering of high values: General G"
        }
        "geo_join_counts" => {
            let (mut bb, mut bw, mut ww) = (0.0, 0.0, 0.0);
            for i in 0..n {
                for j in i + 1..n {
                    if w[i][j] != 0.0 || w[j][i] != 0.0 {
                        let (a, b) = (y[i] != 0.0, y[j] != 0.0);
                        let (cl, t) = if a && b { bb += 1.0; ("axp-c", "BB") } else if a || b { bw += 1.0; ("axp-c3", "BW") }
                                      else { ww += 1.0; ("axp-c2", "WW") };
                        net.edge(i, j, cl, t);
                    }
                }
            }
            // the joins drawn must be the model's own counts
            if bb != s.out("BB")? || bw != s.out("BW")? || ww != s.out("WW")? {
                return None;
            }
            for i in 0..n {
                net.node(i, if y[i] != 0.0 { "axp-kd" } else { "axp-ka" }, true,
                         &format!("{}: {}", i + 1, if y[i] != 0.0 { "B" } else { "W" }));
            }
            net.legend(&[("axp-kd", "B (1)"), ("axp-ka", "W (0)")]);
            cap = format!("Binary map y = {} (1 = black B, 0 = white W) on the links of the stated weights (a layout \
                           without meaning); each link is labelled with the kind of join it is. The model counts BB = {}, \
                           BW = {}, WW = {} of J = {} joins.", lst(&y), n6(bb), n6(bw), n6(ww), n6(s.out("J")?));
            "Join counts on a binary map"
        }
        _ => {
            let (gs, zs) = (s.out_list("Gs")?, s.out_list("Zs")?);
            if gs.len() != n || zs.len() != n {
                return None;
            }
            for i in 0..n {
                for j in i + 1..n {
                    if w[i][j] != 0.0 || w[j][i] != 0.0 {
                        net.edge(i, j, "axp-c4", "");
                    }
                }
            }
            for i in 0..n {
                let cl = if zs[i] > 1.96 { "axp-d" } else if zs[i] < -1.96 { "axp-d3" } else { "axp-d4" };
                net.node(i, cl, false, &format!("{}: y={}, z={}", i + 1, n4(y[i]), n4(zs[i])));
            }
            net.legend(&[("axp-d", "hot spot, z > 1.96"), ("axp-d3", "cold spot, z < −1.96"), ("axp-d4", "neither")]);
            cap = format!("Local Getis-Ord statistics ({}) on the units with values y = {} linked by W (a layout without \
                           meaning): each node shows its value and the z-score the model returns, coloured hot (z > 1.96) \
                           or cold (z < −1.96); returned G values: {}.", s.txt("statistic").unwrap_or_default(), lst(&y),
                          lst(&gs));
            "Hot and cold spots: local Gi"
        }
    };
    Some(s.figure("network", title, &cap, net.c.body(), net.c.h))
}

/// A network given by its edges (lengths printed; a layout without meaning), labelled with what the model returns.
pub fn edge_network(s: &Solved) -> Option<Figure> {
    let es = edges(s, "edges")?;
    let n = es.iter().map(|e| e.0.max(e.1)).max()? + 1;
    if n > 24 {
        return None;
    }
    let mut net = Net::new(n, 430.0);
    for e in &es {
        net.edge(e.0, e.1, "axp-c4", &e.2.map(n4).unwrap_or_default());
    }
    let idx = |v: &[f64]| -> Option<Vec<usize>> { v.iter().map(|x| (*x >= 0.0 && (*x as usize) < n).then_some(*x as usize)).collect() };
    let (title, cap) = match s.key {
        "geo_closest_facility" => {
            let fac = idx(&s.vec("facilities")?)?;
            let (near, dist) = (s.out_list("nearest")?, s.out_list("distance")?);
            if near.len() != n || dist.len() != n {
                return None;
            }
            let cls = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];
            for i in 0..n {
                let f = fac.iter().position(|x| *x as f64 == near[i]).unwrap_or(0);
                let t = if fac.contains(&i) { format!("{i} facility") } else { format!("{i}: → {}, {}", n6(near[i]), n4(dist[i])) };
                net.node(i, cls[f % 4], fac.contains(&i), &t);
            }
            let items: Vec<(String, String)> = fac.iter().enumerate().take(4)
                .map(|(k_, f)| (cls[k_].to_string(), format!("served by facility {f}"))).collect();
            let it: Vec<(&str, &str)> = items.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
            net.legend(&it);
            ("Each node's closest facility",
             format!("The stated network (edge lengths printed; a layout without meaning) with facilities at nodes {}: each \
                      node is coloured by the nearest facility the model returns and labelled with it and the shortest-path \
                      distance; mean distance {}.", lst(&s.vec("facilities")?), n6(s.out("mean_distance")?)))
        }
        "geo_gravity_accessibility" | "geo_two_step_floating_catchment" => {
            let acc = s.out_list("accessibility")?;
            if acc.len() != n {
                return None;
            }
            let best = (0..n).max_by(|a, b| acc[*a].partial_cmp(&acc[*b]).unwrap_or(std::cmp::Ordering::Equal))?;
            let supply = if s.key == "geo_two_step_floating_catchment" { idx(&s.vec("supply_nodes")?)? } else { vec![] };
            for i in 0..n {
                net.node(i, if i == best { "axp-d" } else { "axp-d4" }, supply.contains(&i), &format!("{i}: {}", n4(acc[i])));
            }
            net.legend(&[("axp-d", &format!("highest: node {best}"))]);
            if s.key == "geo_two_step_floating_catchment" {
                ("Two-step floating catchment accessibility",
                 format!("The stated network (edge lengths printed; a layout without meaning; supply nodes ringed) with each \
                          node labelled by the accessibility (supply per person) the model returns: {}; highest at node \
                          {best}.", acc.iter().enumerate().map(|(i, v)| format!("node {i} {}", n6(*v))).collect::<Vec<_>>().join(", ")))
            } else {
                ("Gravity accessibility of each node",
                 format!("The stated network (edge lengths printed; a layout without meaning) with each node labelled by the \
                          gravity accessibility the model returns: {}; highest at node {best}.",
                         acc.iter().enumerate().map(|(i, v)| format!("node {i} {}", n6(*v))).collect::<Vec<_>>().join(", ")))
            }
        }
        "geo_huff_model" => {
            let org = iv(s, "origin")? as usize;
            let stores = idx(&s.vec("stores")?)?;
            let att = s.vec("attractiveness")?;
            let (pr, ds) = (s.out_list("probabilities")?, s.out_list("distances")?);
            if pr.len() != stores.len() || ds.len() != stores.len() || att.len() != stores.len() || org >= n {
                return None;
            }
            for i in 0..n {
                if let Some(k_) = stores.iter().position(|x| *x == i) {
                    net.node(i, "axp-d", true, &format!("{i}: P = {} (A {}, d {})", n4(pr[k_]), n4(att[k_]), n4(ds[k_])));
                } else if i == org {
                    net.node(i, "axp-d2", false, &format!("{i}: origin"));
                } else {
                    net.node(i, "axp-d4", false, &format!("{i}"));
                }
            }
            net.legend(&[("axp-d", "stores"), ("axp-d2", "shopper's origin")]);
            ("Huff store-choice probabilities",
             format!("Huff model on the stated network (edge lengths printed; a layout without meaning): a shopper at node \
                      {org} chooses among stores {} with attractiveness {} (α = {}, distance decay β = {}). The model returns \
                      shortest-path distances {} and patronage probabilities {}.", lst(&s.vec("stores")?), lst(&att),
                     n6(s.inp("alpha")?), n6(s.inp("beta")?), lst(&ds), lst(&pr)))
        }
        "geo_p_median" => {
            let fac = idx(&s.out_list("facilities")?)?;
            let dem = s.vec("demand")?;
            if dem.len() != n {
                return None;
            }
            for i in 0..n {
                net.node(i, if fac.contains(&i) { "axp-d" } else { "axp-d4" }, fac.contains(&i), &format!("{i}: demand {}", n4(dem[i])));
            }
            net.legend(&[("axp-d", &format!("chosen facilities ({})", fac.len())), ("axp-d4", "demand nodes")]);
            ("p-median facility locations",
             format!("The p-median solution (p = {}) on the stated network (edge lengths printed; a layout without meaning; \
                      demand at each node): the model places facilities at nodes {}, with total demand-weighted distance {} \
                      (mean {}).", n6(iv(s, "p")?), lst(&s.out_list("facilities")?), n6(s.out("objective")?),
                     n6(s.out("mean_distance")?)))
        }
        "geo_service_area" => {
            let src = iv(s, "source")? as usize;
            let reach = idx(&s.out_list("reachable")?)?;
            for i in 0..n {
                let cl = if i == src { "axp-d2" } else if reach.contains(&i) { "axp-d" } else { "axp-d4" };
                net.node(i, cl, i == src, &format!("{i}"));
            }
            net.legend(&[("axp-d2", "source"), ("axp-d", &format!("within {}", n4(s.inp("cutoff")?))), ("axp-d4", "beyond")]);
            ("Service area within a network cutoff",
             format!("Service area from node {src} with cutoff {} on the stated network (edge lengths printed; a layout \
                      without meaning): the nodes the model returns as reachable ({}; {} nodes, farthest at {}) are \
                      highlighted.", n6(s.inp("cutoff")?), lst(&s.out_list("reachable")?), n6(s.out("count")?),
                     n6(s.out("farthest")?)))
        }
        _ => {
            let sh = s.out_list("shimbel")?;
            if sh.len() != n {
                return None;
            }
            let best = s.out("most_accessible")? as usize;
            for i in 0..n {
                net.node(i, if i == best { "axp-d" } else { "axp-d4" }, false, &format!("{i}: Σd = {}", n4(sh[i])));
            }
            net.legend(&[("axp-d", &format!("most accessible: node {best}"))]);
            ("Shimbel accessibility of each node",
             format!("The Shimbel index of each node of the stated network (edge lengths printed; a layout without meaning): \
                      the sum of shortest-path distances to all other nodes, as returned: {} (total {}); the smallest sum, \
                      node {best}, is the most accessible.", sh.iter().enumerate().map(|(i, v)| format!("node {i} {}", n6(*v)))
                         .collect::<Vec<_>>().join(", "), n6(s.out("total")?)))
        }
    };
    Some(s.figure("network", title, &cap, net.c.body(), net.c.h))
}

/// Covered demand as the number of facilities grows (each a run of the model).
pub fn covering_by_p(s: &Solved) -> Option<Figure> {
    let cands = s.vec("candidates")?;
    let p0 = iv(s, "p")?;
    let cov0 = s.out("covered")?;
    let mut pts = vec![];
    for p in 1..=cands.len().min(12) {
        if let Some(a) = s.with_input(&[("p", Input::Int(p as i64))]) {
            if let Some(c) = o(&a, "covered") {
                pts.push((p as f64, c));
            }
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let total = s.vec("demand")?.iter().sum::<f64>();
    let (xs, ys): (Vec<f64>, Vec<f64>) = pts.iter().copied().chain([(p0, cov0), (1.0, 0.0)]).unzip();
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(total, &format!("all demand {}", n4(total)));
    p.line_dots(&pts, 0, &format!("covered within {}", n4(s.inp("coverage")?)));
    p.mark(p0, cov0, &format!("p = {}: {}", n6(p0), n4(cov0)));
    let body = p.finish("facilities p", "covered demand", "");
    Some(s.chart("xy", "Covered demand against the number of facilities",
                 &format!("Maximal covering location on the stated network (demand {}, candidates {}, coverage distance {}): \
                           the demand the model covers with p = 1 to {} facilities, each a run of the model ({}). Marked: p = \
                           {}, covered {} of {} (fraction {}).", lst(&s.vec("demand")?), lst(&cands), n6(s.inp("coverage")?),
                          pts.len(), pts.iter().map(|q| format!("p = {}: {}", n6(q.0), n6(q.1))).collect::<Vec<_>>().join(", "),
                          n6(p0), n6(cov0), n6(total), n6(s.out("fraction")?)), &body))
}
