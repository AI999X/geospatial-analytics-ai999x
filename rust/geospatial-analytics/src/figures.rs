//! Geospatial Analytics' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: graticules traced through the projections, geodesic and rhumb paths, frames, point patterns, networks,
//! Moran scatters, fitted models, rasters and spectral indices. Every value drawn is an output of this request or of
//! a further run of the model; a fitted curve is drawn only when refitting the model to the curve's own values gives
//! back the same parameters; a figure the model cannot support for these inputs is left out (docs/FIGURE_PARITY.md).

use ax_engine::figkit::{num, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input, Val};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    use crate::figures_geo as g;
    use crate::figures_proj as pj;
    use crate::figures_space as sp;
    use crate::figures_terrain as tr;
    let v: Vec<Option<Figure>> = match s.key {
        // projections: the graticule traced through the model's own forward projection
        "geo_albers_equal_area_forward" | "geo_cylindrical_equal_area_forward" | "geo_equirectangular_forward"
        | "geo_gnomonic_forward" | "geo_lambert_azimuthal_equal_area_forward" | "geo_lambert_conformal_conic_forward"
        | "geo_mercator_forward" | "geo_orthographic_forward" | "geo_polar_stereographic_forward"
        | "geo_sinusoidal_forward" | "geo_transverse_mercator_forward" | "geo_utm_forward" | "geo_web_mercator_forward"
        | "geo_albers_equal_area_inverse" | "geo_lambert_conformal_conic_inverse" | "geo_mercator_inverse"
        | "geo_web_mercator_inverse" | "geo_utm_inverse" | "geo_mollweide_forward" => vec![pj::graticule(s)],
        "geo_azimuthal_equidistant_forward" => vec![pj::azimuth_compass(s), pj::graticule(s)],
        "geo_tissot_indicatrix" => vec![pj::tissot(s)],
        "geo_utm_zone" => vec![pj::utm_zones(s)],
        "geo_mgrs_encode" => vec![pj::mgrs_squares(s)],
        "geo_web_map_tile" => vec![pj::tiles(s)],
        "geo_geohash_decode" => vec![pj::geohash_cells(s)],
        // geodesy
        "geo_geodesic_destination" => vec![g::destination_paths(s), g::destination_circles(s)],
        "geo_geodesic_midpoint" => vec![g::midpoint_path(s)],
        "geo_geodesic_polygon_area" => vec![g::polygon_shifts(s)],
        "geo_rhumb_line_destination" => vec![g::rhumb_rays(s)],
        "geo_rhumb_line_inverse" => vec![g::rhumb_vs_geodesic(s)],
        "geo_look_angles" => vec![g::sky(s)],
        "geo_geodetic_to_ecef" => vec![g::ecef_frames(s)],
        "geo_ecef_to_geodetic" => vec![g::ecef_section(s)],
        "geo_geodetic_to_enu" => vec![g::enu_frames(s, false)],
        "geo_enu_to_geodetic" => vec![g::enu_frames(s, true)],
        "geo_decimal_to_dms" => vec![g::dms_scale(s, true)],
        "geo_dms_to_decimal" => vec![g::dms_scale(s, false)],
        "geo_meridian_arc" => vec![g::two_input_field(s, "lat2", "lat1", "distance", "m", (0.0, 90.0), (0.0, 90.0))],
        "geo_map_scale_from_distances" => vec![g::two_input_field(s, "map_distance", "ground_distance", "scale", "",
                                                                   (0.5, 1.5), (0.5, 1.5))],
        "geo_map_scale_ground_distance" => vec![g::two_input_field(s, "map_distance", "scale", "ground_distance", "m",
                                                                    (0.5, 1.5), (0.5, 1.5))],
        // point patterns and lines
        "geo_mean_center_standard_distance" => vec![sp::mean_centre(s), sp::mean_centre_leave_one_out(s)],
        "geo_weighted_mean_center" => vec![sp::weighted_centre(s)],
        "geo_standard_deviational_ellipse" => vec![sp::sd_ellipse(s)],
        "geo_nearest_neighbour_index" => vec![sp::nearest_neighbours(s)],
        "geo_quadrat_test" => vec![sp::quadrats(s)],
        "geo_distance_band_weights" => vec![sp::band_links(s)],
        "geo_knn_weights" => vec![sp::knn_links(s)],
        "geo_detour_index" => vec![sp::detour(s)],
        "geo_discrete_frechet_distance" => vec![sp::frechet(s)],
        "geo_douglas_peucker_simplification" => vec![sp::simplification(s)],
        "geo_polygon_compactness" => vec![sp::compactness(s)],
        "geo_idw_interpolation" => vec![sp::surface(s, "estimate", "Inverse-distance-weighted surface")],
        "geo_kernel_density" => vec![sp::surface(s, "density", "Kernel density surface of the events")],
        "geo_ordinary_kriging" => vec![sp::kriging(s)],
        "geo_trend_surface" => vec![sp::trend_surface(s)],
        "geo_empirical_semivariogram" => vec![sp::semivariogram(s)],
        "geo_variogram_model_fit" => vec![sp::variogram_fit(s)],
        "geo_density_gradient" => vec![sp::density_gradient(s)],
        "geo_rank_size_rule" => vec![sp::rank_size(s)],
        "geo_gravity_model_poisson_fit" => vec![sp::gravity_fit(s)],
        // spatial autocorrelation
        "geo_morans_i" => vec![sp::moran_scatter(s)],
        "geo_local_morans_i" => vec![sp::local_moran_scatter(s)],
        "geo_bivariate_morans_i" => vec![sp::bivariate_scatter(s)],
        "geo_gearys_c" | "geo_getis_ord_general_g" | "geo_join_counts" | "geo_local_getis_ord_gi" => {
            vec![sp::unit_network(s)]
        }
        // networks
        "geo_closest_facility" | "geo_gravity_accessibility" | "geo_huff_model" | "geo_p_median" | "geo_service_area"
        | "geo_shimbel_index" | "geo_two_step_floating_catchment" => vec![sp::edge_network(s)],
        "geo_maximal_covering_location" => vec![sp::covering_by_p(s)],
        // rasters, terrain and indices
        "geo_d8_flow_direction" => vec![tr::flow_directions(s)],
        "geo_d8_flow_accumulation" => vec![tr::stream_network(s), tr::accumulation_panels(s)],
        "geo_watershed_delineation" => vec![tr::watersheds(s)],
        "geo_slope_aspect_horn" => vec![tr::slope_window(s)],
        "geo_terrain_curvature" | "geo_terrain_ruggedness" => vec![tr::window_matrix(s)],
        "geo_hillshade" => vec![tr::hillshade_compass(s)],
        "geo_line_of_sight_profile" => vec![tr::sight_profile(s)],
        "geo_classification_accuracy" => vec![tr::accuracy_bars(s), tr::accuracy_matrix(s)],
        "geo_segregation_evenness" | "geo_segregation_exposure" => vec![tr::group_bars(s)],
        "geo_vegetation_indices" => vec![tr::index_bars(s), tr::ndvi_field(s)],
        "geo_water_burn_built_indices" => vec![tr::index_bars(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ shared helpers
/// A number to 6 significant digits, rounding noise shown as 0.
pub(crate) fn n6(x: f64) -> String {
    ax_engine::num::py_g(if x.abs() < 1e-12 { 0.0 } else { x }, 6)
}

/// A number to 4 significant digits.
pub(crate) fn n4(x: f64) -> String {
    num(if x.abs() < 1e-12 { 0.0 } else { x })
}

/// An angle in degrees, 6 digits.
pub(crate) fn dg(x: f64) -> String {
    format!("{}°", n6(x))
}

/// A list as "[1, 2, 3]".
pub(crate) fn lst(v: &[f64]) -> String {
    format!("[{}]", v.iter().map(|x| n6(*x)).collect::<Vec<_>>().join(", "))
}

/// The visitor's input as given (to pass it on to another run).
pub(crate) fn given(s: &Solved, sym: &str) -> Option<Input> {
    s.given.iter().find(|g| g.0 == sym).map(|g| g.1.clone())
}

/// Inputs passed on unchanged (those given), plus set ones.
pub(crate) fn pass<'a>(s: &Solved, syms: &[&'a str], set: Vec<(&'a str, Input)>) -> Vec<(&'a str, Input)> {
    let mut v: Vec<(&str, Input)> = syms.iter().filter_map(|k| given(s, k).map(|x| (*k, x))).collect();
    for (k, x) in set {
        match v.iter_mut().find(|e| e.0 == k) {
            Some(e) => e.1 = x,
            None => v.push((k, x)),
        }
    }
    v
}

/// A number or whole-number input as read (an integer input is not in the SI list).
pub(crate) fn iv(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| match s.val(sym)? {
        Val::Int(i) => Some(i as f64),
        Val::Num(x) if x.is_finite() => Some(x),
        _ => None,
    })
}

pub(crate) fn fl(x: f64) -> Input {
    Input::Float(x)
}

pub(crate) fn jl(v: &[f64]) -> Input {
    Input::Json(serde_json::json!(v))
}

pub(crate) fn jm(m: &[Vec<f64>]) -> Input {
    Input::Json(serde_json::json!(m))
}

/// A number output of an answer.
pub(crate) fn o(a: &Answer, sym: &str) -> Option<f64> {
    ax_engine::figkit::num_of(a, sym)
}

/// A list output of an answer.
pub(crate) fn ol(a: &Answer, sym: &str) -> Option<Vec<f64>> {
    ax_engine::figkit::list_of(a, sym)
}

/// A matrix output (a list of number lists).
pub(crate) fn om(a: &Answer, sym: &str) -> Option<Vec<Vec<f64>>> {
    let v = a.values.iter().find(|x| x.0 == sym)?.1.as_array()?;
    let m: Vec<Vec<f64>> = v.iter().map(|r| {
        r.as_array().map(|r| r.iter().filter_map(|x| x.as_f64()).collect::<Vec<f64>>()).unwrap_or_default()
    }).collect();
    let c = m.first()?.len();
    (c > 0 && m.iter().all(|r| r.len() == c && r.iter().all(|x| x.is_finite()))).then_some(m)
}

/// A text output.
pub(crate) fn ot(a: &Answer, sym: &str) -> Option<String> {
    match &a.values.iter().find(|x| x.0 == sym)?.1 {
        serde_json::Value::String(t) => Some(t.clone()),
        serde_json::Value::Bool(b) => Some(if *b { "True".into() } else { "False".into() }),
        serde_json::Value::Number(n) => Some(n6(n.as_f64()?)),
        _ => None,
    }
}

/// A graph's edges as read: (u, v, length) with node numbers; None when a node is not a whole number 0..60.
pub(crate) fn edges(s: &Solved, sym: &str) -> Option<Vec<(usize, usize, Option<f64>)>> {
    match s.val(sym)? {
        Val::Edges(es) => {
            let mut out = vec![];
            for (a, b, w) in es {
                let (a, b): (usize, usize) = (a.trim().parse().ok()?, b.trim().parse().ok()?);
                if a > 60 || b > 60 {
                    return None;
                }
                out.push((a, b, w));
            }
            (!out.is_empty()).then_some(out)
        }
        _ => None,
    }
}

/// Two numbers as close as their 6-digit printing (a check that a drawing reproduces the model's own value).
pub(crate) fn agrees(a: f64, b: f64) -> bool {
    a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-6 * a.abs().max(b.abs()).max(1e-9)
}

/// Refitting agreement: every pair within a relative 1e-4 (a fit run on its own model's values).
pub(crate) fn same_fit(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-4 * x.abs().max(y.abs()).max(1e-6))
}
