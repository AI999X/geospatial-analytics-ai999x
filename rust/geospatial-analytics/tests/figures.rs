//! Every Geospatial Analytics capability's own live figures (src/figures.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use geospatial_analytics_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/geospatial-analytics-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("geo_albers_equal_area_forward", &["graticule"]),
    ("geo_albers_equal_area_inverse", &["graticule"]),
    ("geo_azimuthal_equidistant_forward", &["compass", "graticule"]),
    ("geo_bivariate_morans_i", &["scatter"]),
    ("geo_classification_accuracy", &["bars", "matrix"]),
    ("geo_closest_facility", &["network"]),
    ("geo_cylindrical_equal_area_forward", &["graticule"]),
    ("geo_d8_flow_accumulation", &["flow", "heatmap"]),
    ("geo_d8_flow_direction", &["flow"]),
    ("geo_decimal_to_dms", &["scale"]),
    ("geo_density_gradient", &["fit"]),
    ("geo_detour_index", &["map"]),
    ("geo_discrete_frechet_distance", &["lines"]),
    ("geo_distance_band_weights", &["map"]),
    ("geo_dms_to_decimal", &["scale"]),
    ("geo_douglas_peucker_simplification", &["shape"]),
    ("geo_ecef_to_geodetic", &["section"]),
    ("geo_empirical_semivariogram", &["variogram"]),
    ("geo_enu_to_geodetic", &["frames"]),
    ("geo_equirectangular_forward", &["graticule"]),
    ("geo_gearys_c", &["network"]),
    ("geo_geodesic_destination", &["map", "map-2"]),
    ("geo_geodesic_midpoint", &["map"]),
    ("geo_geodesic_polygon_area", &["polygons"]),
    ("geo_geodetic_to_ecef", &["frames"]),
    ("geo_geodetic_to_enu", &["frames"]),
    ("geo_geohash_decode", &["cells"]),
    ("geo_getis_ord_general_g", &["network"]),
    ("geo_gnomonic_forward", &["graticule"]),
    ("geo_gravity_accessibility", &["network"]),
    ("geo_gravity_model_poisson_fit", &["fit"]),
    ("geo_hillshade", &["compass"]),
    ("geo_huff_model", &["network"]),
    ("geo_idw_interpolation", &["map"]),
    ("geo_join_counts", &["network"]),
    ("geo_kernel_density", &["map"]),
    ("geo_knn_weights", &["map"]),
    ("geo_lambert_azimuthal_equal_area_forward", &["graticule"]),
    ("geo_lambert_conformal_conic_forward", &["graticule"]),
    ("geo_lambert_conformal_conic_inverse", &["graticule"]),
    ("geo_line_of_sight_profile", &["profile"]),
    ("geo_local_getis_ord_gi", &["network"]),
    ("geo_local_morans_i", &["scatter"]),
    ("geo_look_angles", &["sky"]),
    ("geo_map_scale_from_distances", &["field"]),
    ("geo_map_scale_ground_distance", &["field"]),
    ("geo_maximal_covering_location", &["graph", "xy"]),
    ("geo_mean_center_standard_distance", &["map", "map-2"]),
    ("geo_mercator_forward", &["graticule"]),
    ("geo_mercator_inverse", &["graticule"]),
    ("geo_meridian_arc", &["field"]),
    ("geo_mgrs_encode", &["zones"]),
    ("geo_mollweide_forward", &["projection"]),
    ("geo_morans_i", &["scatter"]),
    ("geo_nearest_neighbour_index", &["map"]),
    ("geo_network_connectivity_indices", &["graph"]),
    ("geo_ordinary_kriging", &["map"]),
    ("geo_orthographic_forward", &["graticule"]),
    ("geo_p_median", &["network"]),
    ("geo_polar_stereographic_forward", &["graticule"]),
    ("geo_polygon_compactness", &["polygons"]),
    ("geo_quadrat_test", &["map"]),
    ("geo_rank_size_rule", &["fit"]),
    ("geo_rhumb_line_destination", &["map"]),
    ("geo_rhumb_line_inverse", &["map"]),
    ("geo_segregation_evenness", &["bars"]),
    ("geo_segregation_exposure", &["bars"]),
    ("geo_service_area", &["network"]),
    ("geo_shimbel_index", &["network"]),
    ("geo_sinusoidal_forward", &["graticule"]),
    ("geo_slope_aspect_horn", &["window"]),
    ("geo_spatial_lag", &["bars"]),
    ("geo_standard_deviational_ellipse", &["map"]),
    ("geo_terrain_curvature", &["matrix"]),
    ("geo_terrain_ruggedness", &["matrix"]),
    ("geo_tissot_indicatrix", &["ellipses"]),
    ("geo_transverse_mercator_forward", &["graticule"]),
    ("geo_trend_surface", &["map"]),
    ("geo_two_step_floating_catchment", &["network"]),
    ("geo_utm_forward", &["graticule"]),
    ("geo_utm_inverse", &["graticule"]),
    ("geo_utm_zone", &["zones"]),
    ("geo_variogram_model_fit", &["variogram"]),
    ("geo_vegetation_indices", &["bars", "field"]),
    ("geo_water_burn_built_indices", &["bars"]),
    ("geo_watershed_delineation", &["mask"]),
    ("geo_web_map_tile", &["tiles"]),
    ("geo_web_mercator_forward", &["graticule"]),
    ("geo_web_mercator_inverse", &["graticule"]),
    ("geo_weighted_mean_center", &["map"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("geo_geohash_decode", "geohash", "gcpvj0e5"),
    ("geo_shimbel_index", "edges", "0-1 3; 1-2 3; 0-2 6; 2-3 2; 3-4 1"),
    ("geo_closest_facility", "facilities", "1, 3"),
    ("geo_service_area", "source", "2"),
    ("geo_detour_index", "destination", "3"),
    ("geo_tissot_indicatrix", "projection", "sinusoidal"),
    ("geo_join_counts", "y", "1, 0, 1, 0, 1, 1"),
    ("geo_classification_accuracy", "matrix", "50 10 5; 8 60 12; 4 6 70"),
    ("geo_network_connectivity_indices", "edges", "0-1; 1-2; 2-0; 2-3; 3-4"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
pub fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '[' {
            // a list or a matrix, written as the form takes it ("8, 6, 6" or "0 1 0; 1 0 1")
            let s0 = i;
            let mut depth = 0;
            loop {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                i += 1;
            }
            let j: serde_json::Value = serde_json::from_str(&cs[s0..=i].iter().collect::<String>()).expect("a list");
            i += 1;
            let num = |v: &serde_json::Value| v.to_string();
            let arr = j.as_array().expect("a list");
            value = if arr.first().map(|r| r.is_array()).unwrap_or(false) {
                arr.iter().map(|r| r.as_array().unwrap().iter().map(num).collect::<Vec<_>>().join(" "))
                    .collect::<Vec<_>>().join("; ")
            } else {
                arr.iter().map(num).collect::<Vec<_>>().join(", ")
            };
        } else if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

/// Every figure is the same on every call: the web-map tiles' labels came out in a different order on each run (they
/// were collected in a HashMap), so the package's, the program's and the browser's figures differed (2026-11-03).
#[test]
fn every_figure_is_the_same_on_every_call() {
    let p = pack();
    // the model's own figures (a generic "-live-" sweep keeps the native wall-clock limit here, so its point count
    // may change with the machine's load)
    let own = |key: &str, given: &[(String, Input)]| -> Vec<String> {
        figures(p, key, given, None, None).into_iter().filter(|f| !f.slug.contains("-live-")).map(|f| f.svg).collect()
    };
    for (key, _kinds) in KINDS {
        let given = example(key);
        let a = own(key, &given);
        for _ in 0..3 {
            let b = own(key, &given);
            assert_eq!(a, b, "{key}: the figures differ between two calls");
        }
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                // a kind the generic engine already draws (a graph, bars of a list) keeps its generic slug; the
                // model's own figures carry "<capability>-<kind>"
                Some(f) if f.slug.contains("-live-") => {
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                }
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                    let cap = &f.caption;
                    for bad in ["we ", "We ", " our ", " us ", "—", "package", &["open", "stax"].concat()] {
                        assert!(!cap.contains(bad), "{key} {kind}: the caption holds '{bad}'");
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a latitude beyond the pole and lists of different lengths: when the model refuses, nothing is drawn
    let p = pack();
    for (key, inputs) in [("geo_geodesic_destination", vec![("lat1", "95"), ("lon1", "0"), ("azimuth", "45"),
                                                             ("distance", "100000"), ("ellipsoid", "WGS84")]),
                          ("geo_mean_center_standard_distance", vec![("x", "0, 4, 4"), ("y", "0, 0")])] {
        let given: Vec<(String, Input)> = inputs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
        let a = ax_engine::solve(p, key, &given, None);
        let figs = figures(p, key, &given, None, None);
        if !a.solved() {
            assert!(figs.is_empty(), "{key}: a figure for a refused input");
        }
    }
}
