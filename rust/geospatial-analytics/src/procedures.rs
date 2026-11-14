//! geospatial_analytics: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/geospatial_analytics/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "geo_albers_equal_area_forward",
    "geo_albers_equal_area_inverse",
    "geo_auxiliary_latitudes",
    "geo_azimuthal_equidistant_forward",
    "geo_bivariate_morans_i",
    "geo_brightness_temperature",
    "geo_classification_accuracy",
    "geo_closest_facility",
    "geo_cylindrical_equal_area_forward",
    "geo_d8_flow_accumulation",
    "geo_d8_flow_direction",
    "geo_decimal_to_dms",
    "geo_density_gradient",
    "geo_detour_index",
    "geo_discrete_frechet_distance",
    "geo_distance_band_weights",
    "geo_dms_to_decimal",
    "geo_douglas_peucker_simplification",
    "geo_ecef_to_geodetic",
    "geo_empirical_semivariogram",
    "geo_enu_to_geodetic",
    "geo_equirectangular_forward",
    "geo_gearys_c",
    "geo_geodesic_destination",
    "geo_geodesic_midpoint",
    "geo_geodesic_polygon_area",
    "geo_geodetic_to_ecef",
    "geo_geodetic_to_enu",
    "geo_geohash_decode",
    "geo_geohash_encode",
    "geo_getis_ord_general_g",
    "geo_gnomonic_forward",
    "geo_graticule_cell_area",
    "geo_gravity_accessibility",
    "geo_gravity_model_poisson_fit",
    "geo_ground_sample_distance",
    "geo_hillshade",
    "geo_huff_model",
    "geo_idw_interpolation",
    "geo_join_counts",
    "geo_kernel_density",
    "geo_knn_weights",
    "geo_lambert_azimuthal_equal_area_forward",
    "geo_lambert_conformal_conic_forward",
    "geo_lambert_conformal_conic_inverse",
    "geo_line_of_sight_profile",
    "geo_local_getis_ord_gi",
    "geo_local_morans_i",
    "geo_look_angles",
    "geo_map_scale_from_distances",
    "geo_map_scale_ground_distance",
    "geo_maximal_covering_location",
    "geo_mean_center_standard_distance",
    "geo_mercator_forward",
    "geo_mercator_inverse",
    "geo_meridian_arc",
    "geo_mgrs_encode",
    "geo_mollweide_forward",
    "geo_morans_i",
    "geo_nearest_neighbour_index",
    "geo_network_connectivity_indices",
    "geo_ordinary_kriging",
    "geo_orthographic_forward",
    "geo_p_median",
    "geo_parallel_arc",
    "geo_polar_stereographic_forward",
    "geo_polygon_compactness",
    "geo_quadrat_test",
    "geo_radii_of_curvature",
    "geo_rank_size_rule",
    "geo_rhumb_line_destination",
    "geo_rhumb_line_inverse",
    "geo_ripley_k",
    "geo_segregation_evenness",
    "geo_segregation_exposure",
    "geo_service_area",
    "geo_shimbel_index",
    "geo_sinusoidal_forward",
    "geo_slope_aspect_horn",
    "geo_spatial_lag",
    "geo_standard_deviational_ellipse",
    "geo_terrain_curvature",
    "geo_terrain_ruggedness",
    "geo_tissot_indicatrix",
    "geo_toa_reflectance",
    "geo_transverse_mercator_forward",
    "geo_trend_surface",
    "geo_two_step_floating_catchment",
    "geo_utm_forward",
    "geo_utm_inverse",
    "geo_utm_zone",
    "geo_variogram_model_fit",
    "geo_vegetation_indices",
    "geo_water_burn_built_indices",
    "geo_watershed_delineation",
    "geo_web_map_tile",
    "geo_web_mercator_forward",
    "geo_web_mercator_inverse",
    "geo_weighted_mean_center",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "geo_albers_equal_area_forward" => {
            let r = f_albers_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_45(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_albers_equal_area_inverse" => {
            let r = f_albers_inverse(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_137(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_auxiliary_latitudes" => {
            let r = f_auxiliary_latitudes(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_203(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_azimuthal_equidistant_forward" => {
            let r = f_aeqd_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_306(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_bivariate_morans_i" => {
            let r = f_bivariate_morans_i(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_403(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_brightness_temperature" => {
            let r = f_brightness_temperature(match v.get("radiance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_462(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_classification_accuracy" => {
            let r = f_classification_accuracy(match v.get("matrix") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_545(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_closest_facility" => {
            let r = f_closest_facility(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("facilities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_655(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_cylindrical_equal_area_forward" => {
            let r = f_cea_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat_ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_732(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_d8_flow_accumulation" => {
            let r = f_flow_accumulation(match v.get("dem") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cell_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_841(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_d8_flow_direction" => {
            let r = f_d8_flow_direction(match v.get("dem") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cell_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_923(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_decimal_to_dms" => {
            let r = f_decimal_to_dms(match v.get("decimal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("axis") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_989(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_density_gradient" => {
            let r = f_density_gradient(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1074(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_detour_index" => {
            let r = f_detour_index(match v.get("coords") { Some(x) => py::from_val(x), None => { return None } }, match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("origin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("destination") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1199(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_discrete_frechet_distance" => {
            let r = f_frechet_distance(match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1263(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_distance_band_weights" => {
            let r = f_distance_band_weights(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("threshold") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1325(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_dms_to_decimal" => {
            let r = f_dms_to_decimal(match v.get("degrees") { Some(x) => py::from_val(x), None => { return None } }, match v.get("minutes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("seconds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hemisphere") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1397(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_douglas_peucker_simplification" => {
            let r = f_douglas_peucker(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tolerance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1466(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_ecef_to_geodetic" => {
            let r = f_ecef_to_geodetic(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1534(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_empirical_semivariogram" => {
            let r = f_empirical_semivariogram(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bin_edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1617(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_enu_to_geodetic" => {
            let r = f_enu_to_geodetic(match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1707(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_equirectangular_forward" => {
            let r = f_equirectangular_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat_ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1784(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_gearys_c" => {
            let r = f_gearys_c(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("transform") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1885(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_geodesic_destination" => {
            let r = f_geodesic_destination(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("distance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_geodesic_midpoint" => {
            let r = f_geodesic_midpoint(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2094(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_geodesic_polygon_area" => {
            let r = f_geodesic_polygon_area(match v.get("lats") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lons") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2211(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_geodetic_to_ecef" => {
            let r = f_geodetic_to_ecef(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2285(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_geodetic_to_enu" => {
            let r = f_geodetic_to_enu(match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2367(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_geohash_decode" => {
            let r = f_geohash_decode(match v.get("geohash") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2418(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_geohash_encode" => {
            let r = f_geohash_encode(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("precision") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2491(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_getis_ord_general_g" => {
            let r = f_general_g(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2598(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_gnomonic_forward" => {
            let r = f_gnomonic_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2678(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_graticule_cell_area" => {
            let r = f_graticule_cell_area(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2751(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_gravity_accessibility" => {
            let r = f_gravity_accessibility(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("opportunities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("impedance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2857(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_gravity_model_poisson_fit" => {
            let r = f_gravity_poisson(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("O") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2974(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_ground_sample_distance" => {
            let r = f_ground_sample_distance(match v.get("pixel_pitch") { Some(x) => py::from_val(x), None => { return None } }, match v.get("focal_length") { Some(x) => py::from_val(x), None => { return None } }, match v.get("altitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pixels") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3036(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_hillshade" => {
            let r = f_hillshade(match v.get("window") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cell_size") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sun_azimuth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sun_altitude") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3115(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_huff_model" => {
            let r = f_huff_model(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("origin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stores") { Some(x) => py::from_val(x), None => { return None } }, match v.get("attractiveness") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3241(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_idw_interpolation" => {
            let r = f_idw(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("py") { Some(x) => py::from_val(x), None => { return None } }, match v.get("power") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3320(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_join_counts" => {
            let r = f_join_counts(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3408(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_kernel_density" => {
            let r = f_kernel_density(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("py") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bandwidth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kernel") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3487(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_knn_weights" => {
            let r = f_knn_weights(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3551(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_lambert_azimuthal_equal_area_forward" => {
            let r = f_laea_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3630(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_lambert_conformal_conic_forward" => {
            let r = f_lcc_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_lambert_conformal_conic_inverse" => {
            let r = f_lcc_inverse(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3809(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_line_of_sight_profile" => {
            let r = f_line_of_sight(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("observer_height") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_height") { Some(x) => py::from_val(x), None => { return None } }, match v.get("curvature") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3882(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_local_getis_ord_gi" => {
            let r = f_local_g(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("statistic") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3986(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_local_morans_i" => {
            let r = f_local_morans_i(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4098(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_look_angles" => {
            let r = f_look_angles(match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4191(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_map_scale_from_distances" => {
            let r = f_map_scale_from_distances(match v.get("map_distance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ground_distance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4250(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_map_scale_ground_distance" => {
            let r = f_map_scale_ground_distance(match v.get("map_distance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scale") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4297(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_maximal_covering_location" => {
            let r = f_max_covering(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("candidates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("coverage") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4422(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_mean_center_standard_distance" => {
            let r = f_mean_center(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_mercator_forward" => {
            let r = f_mercator_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat_ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4559(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_mercator_inverse" => {
            let r = f_mercator_inverse(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat_ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4640(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_meridian_arc" => {
            let r = f_meridian_arc(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4709(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_mgrs_encode" => {
            let r = f_mgrs_encode(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::str("WGS84")) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4813(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_mollweide_forward" => {
            let r = f_mollweide_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4895(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_morans_i" => {
            let r = f_morans_i(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("transform") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4997(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_nearest_neighbour_index" => {
            let r = f_nearest_neighbour_index(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("area") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5066(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_network_connectivity_indices" => {
            let r = f_network_indices(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5133(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_ordinary_kriging" => {
            let r = f_ordinary_kriging(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("py") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nugget") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sill") { Some(x) => py::from_val(x), None => { return None } }, match v.get("range") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5252(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_orthographic_forward" => {
            let r = f_orthographic_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5332(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_p_median" => {
            let r = f_p_median(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("candidates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5458(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_parallel_arc" => {
            let r = f_parallel_arc(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5532(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_polar_stereographic_forward" => {
            let r = f_polar_stereographic_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pole") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat_ts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5615(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_polygon_compactness" => {
            let r = f_compactness(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5736(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_quadrat_test" => {
            let r = f_quadrat_test(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ny") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5816(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_radii_of_curvature" => {
            let r = f_radii_of_curvature(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5882(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_rank_size_rule" => {
            let r = f_rank_size(match v.get("populations") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5956(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_rhumb_line_destination" => {
            let r = f_rhumb_destination(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bearing") { Some(x) => py::from_val(x), None => { return None } }, match v.get("distance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6044(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_rhumb_line_inverse" => {
            let r = f_rhumb_inverse(match v.get("lat1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6133(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_ripley_k" => {
            let r = f_ripley_k(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6206(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_segregation_evenness" => {
            let r = f_segregation_evenness(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6295(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_segregation_exposure" => {
            let r = f_segregation_exposure(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6378(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_service_area" => {
            let r = f_service_area(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cutoff") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6479(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_shimbel_index" => {
            let r = f_shimbel_index(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6567(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_sinusoidal_forward" => {
            let r = f_sinusoidal_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6642(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_slope_aspect_horn" => {
            let r = f_slope_aspect(match v.get("window") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cell_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6719(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_spatial_lag" => {
            let r = f_spatial_lag(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("transform") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6806(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_standard_deviational_ellipse" => {
            let r = f_std_ellipse(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6879(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_terrain_curvature" => {
            let r = f_curvature(match v.get("window") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cell_size") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6953(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_terrain_ruggedness" => {
            let r = f_terrain_ruggedness(match v.get("window") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7022(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_tissot_indicatrix" => {
            let r = f_tissot_indicatrix(match v.get("projection") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_toa_reflectance" => {
            let r = f_toa_reflectance(match v.get("radiance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("esun") { Some(x) => py::from_val(x), None => { return None } }, match v.get("doy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sun_elevation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_transverse_mercator_forward" => {
            let r = f_tm_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lat0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FE") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("FN") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7275(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_trend_surface" => {
            let r = f_trend_surface(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("order") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7371(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_two_step_floating_catchment" => {
            let r = f_two_step_fca(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("population") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_nodes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("threshold") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7497(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_utm_forward" => {
            let r = f_utm_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7610(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_utm_inverse" => {
            let r = f_utm_inverse(match v.get("easting") { Some(x) => py::from_val(x), None => { return None } }, match v.get("northing") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zone") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hemisphere") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ellipsoid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7699(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_utm_zone" => {
            let r = f_utm_zone(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7784(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_variogram_model_fit" => {
            let r = f_variogram_fit(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7934(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_vegetation_indices" => {
            let r = f_vegetation_indices(match v.get("red") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nir") { Some(x) => py::from_val(x), None => { return None } }, match v.get("blue") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8006(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_water_burn_built_indices" => {
            let r = f_water_indices(match v.get("green") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nir") { Some(x) => py::from_val(x), None => { return None } }, match v.get("swir1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("swir2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8068(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_watershed_delineation" => {
            let r = f_watershed(match v.get("dem") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cell_size") { Some(x) => py::from_val(x), None => { return None } }, match v.get("row") { Some(x) => py::from_val(x), None => { return None } }, match v.get("col") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8186(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_web_map_tile" => {
            let r = f_web_map_tile(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zoom") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8250(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_web_mercator_forward" => {
            let r = f_web_mercator_forward(match v.get("lat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lon") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8321(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_web_mercator_inverse" => {
            let r = f_web_mercator_inverse(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8382(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geo_weighted_mean_center" => {
            let r = f_weighted_mean_center(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8457(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_DEG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.017453292519943295)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ELLIPSOIDS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("WGS84"), Py::tuple(vec![Py::Float(6378137.0), Py::Float(0.0033528106647474805)])), (Py::str("GRS80"), Py::tuple(vec![Py::Float(6378137.0), Py::Float(0.003352810681182319)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_K_REFRACTION() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.13)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_EARTH_LOS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6371000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_WEB_MERCATOR_A() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6378137.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BANDS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::str("CDEFGHJKLMNPQRSTUVWXX")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__D8() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), Py::Int(2i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), Py::Int(4i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(-1i64), Py::Int(8i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(-1i64), Py::Int(16i64)]), Py::tuple(vec![Py::Int(-1i64), Py::Int(-1i64), Py::Int(32i64)]), Py::tuple(vec![Py::Int(-1i64), Py::Int(0i64), Py::Int(64i64)]), Py::tuple(vec![Py::Int(-1i64), Py::Int(1i64), Py::Int(128i64)])])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GH() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::str("0123456789bcdefghjkmnpqrstuvwxyz")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GLW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.027152459411754058), Py::Float(0.062253523938647776), Py::Float(0.0951585116824929), Py::Float(0.12462897125553395), Py::Float(0.14959598881657682), Py::Float(0.16915651939500256), Py::Float(0.1826034150449236), Py::Float(0.18945061045506847), Py::Float(0.18945061045506847), Py::Float(0.1826034150449236), Py::Float(0.16915651939500256), Py::Float(0.14959598881657682), Py::Float(0.12462897125553395), Py::Float(0.0951585116824929), Py::Float(0.062253523938647776), Py::Float(0.027152459411754058)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GLX() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.9894009349916499), Py::Float(0.9445750230732326), Py::Float(0.8656312023878318), Py::Float(0.755404408355003), Py::Float(0.6178762444026438), Py::Float(0.45801677765722737), Py::Float(0.2816035507792589), Py::Float(0.09501250983763744), Py::Float(-0.09501250983763744), Py::Float(-0.2816035507792589), Py::Float(-0.45801677765722737), Py::Float(-0.6178762444026438), Py::Float(-0.755404408355003), Py::Float(-0.8656312023878318), Py::Float(-0.9445750230732326), Py::Float(-0.9894009349916499)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MGRS_COLS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("ABCDEFGH"), Py::str("JKLMNPQR"), Py::str("STUVWXYZ")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MGRS_ROWS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::str("ABCDEFGHJKLMNPQRSTUV")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TISSOT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("mercator"), Py::str("equirectangular"), Py::str("cylindrical_equal_area"), Py::str("sinusoidal"), Py::str("mollweide")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__M(mut v_E: Py, mut v_phi: Py) -> R {
    return Ok(py::truediv(&py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("e2"))?)?)?, &py::pow(&py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &v_phi.clone())?, &Py::Int(2i64))?)?)?, &Py::Float(1.5))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__N(mut v_E: Py, mut v_phi: Py) -> R {
    return Ok(py::truediv(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &v_phi.clone())?, &Py::Int(2i64))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__aea_setup(mut v_E: Py, mut v_lat1: Py, mut v_lat2: Py, mut v_lat0: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t95 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("lat1"), v_lat1.clone()]), Py::tuple(vec![Py::str("lat2"), v_lat2.clone()]), Py::tuple(vec![Py::str("lat0"), v_lat0.clone()])])))? {
        let t104 = (t95).items()?;
        if t104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t104[0].clone();
        v_v = t104[1].clone();
        let _ = f__lat(v_nm.clone(), v_v.clone(), Py::Bool(false))?;
        let _ = f__need({ let t102 = py::abs(&v_v.clone())?; Py::Bool({ let t103 = Py::Int(89i64); if py::le(&t102, &t103)? { true } else { false } }) }, Py::str(&[py::format(&v_nm.clone(), "")?, " must lie within 89 degrees of the equator".to_string()].concat()))?;
    }
    let _ = f__need({ let t105 = py::abs(&py::add(&v_lat1.clone(), &v_lat2.clone())?)?; Py::Bool({ let t106 = Py::Float(1e-06); if py::gt(&t105, &t106)? { true } else { false } }) }, Py::str("the standard parallels must not be symmetric about the equator"))?;
    v_m = { let v_E = v_E.clone(); let v_lat0 = v_lat0.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_nm = v_nm.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::math1("cos", &v_p.clone())?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &v_p.clone())?, &Py::Int(2i64))?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_q = { let v_E = v_E.clone(); let v_lat0 = v_lat0.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_m = v_m.clone(); let v_nm = v_nm.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__q_auth(v_E.clone(), v_p.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t107 = (Py::tuple(vec![py::mul(&v_lat1.clone(), &c_DEG()?)?, py::mul(&v_lat2.clone(), &c_DEG()?)?])).items()?;
    if t107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t107[0].clone();
    v_p2 = t107[1].clone();
    v_n = (if ({ let t108 = py::abs(&py::sub(&v_lat1.clone(), &v_lat2.clone())?)?; Py::Bool({ let t109 = Py::Float(1e-10); if py::lt(&t108, &t109)? { true } else { false } }) }).truthy() { py::math1("sin", &v_p1.clone())? } else { py::truediv(&py::sub(&py::pow(&py::call(&v_m.clone(), &[v_p1.clone()])?, &Py::Int(2i64))?, &py::pow(&py::call(&v_m.clone(), &[v_p2.clone()])?, &Py::Int(2i64))?)?, &py::sub(&py::call(&v_q.clone(), &[v_p2.clone()])?, &py::call(&v_q.clone(), &[v_p1.clone()])?)?)? });
    v_C = py::add(&py::pow(&py::call(&v_m.clone(), &[v_p1.clone()])?, &Py::Int(2i64))?, &py::mul(&v_n.clone(), &py::call(&v_q.clone(), &[v_p1.clone()])?)?)?;
    v_rho = { let v_C = v_C.clone(); let v_E = v_E.clone(); let v_lat0 = v_lat0.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_nm = v_nm.clone(); let v_p1 = v_p1.clone(); let v_p2 = v_p2.clone(); let v_q = v_q.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::math1("sqrt", &py::sub(&v_C.clone(), &py::mul(&v_n.clone(), &py::call(&v_q.clone(), &[v_p.clone()])?)?)?)?)?, &v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::tuple(vec![v_n.clone(), v_C.clone(), v_rho.clone(), v_m.clone(), v_q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__all_dist(mut v_adj: Py) -> R {
    return Ok({ let mut out = vec![]; for t6546 in py::range_iter(&[(py::len(&v_adj.clone())?).i()?]) { let mut v_i: Py; 
v_i = t6546;
out.push(f__dijkstra(v_adj.clone(), v_i.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__az(mut v_a: Py) -> R {
    let mut v_x: Py = Py::None;
    v_x = py::fmod(&v_a.clone(), &Py::Float(360.0))?;
    if ({ let t6703 = v_x.clone(); Py::Bool({ let t6704 = Py::Int(0i64); if py::lt(&t6703, &t6704)? { true } else { false } }) }).truthy() {
        v_x = py::iadd(&v_x, &Py::Float(360.0))?;
    }
    if ({ let t6705 = v_x.clone(); Py::Bool({ let t6706 = Py::Float(360.0); if py::ge(&t6705, &t6706)? { true } else { false } }) }).truthy() {
        v_x = py::sub(&v_x, &Py::Float(360.0))?;
    }
    return Ok(py::add(&v_x.clone(), &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chi2_sf(mut v_x: Py, mut v_df: Py) -> R {
    return Ok(py::float(&ax_engine::pymp::call("gammainc", &[py::truediv(&v_df.clone(), &Py::Float(2.0))?, py::truediv(&v_x.clone(), &Py::Float(2.0))?, ax_engine::pymp::constant("inf")?], &[(String::from("regularized"), Py::Bool(true))])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__d8(mut v_dem: Py, mut v_cell_size: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cd: Py = Py::None;
    let mut v_code: Py = Py::None;
    let mut v_di: Py = Py::None;
    let mut v_dj: Py = Py::None;
    let mut v_drop: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t8125 = (f__dem(v_dem.clone())?).items()?;
    if t8125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t8125[0].clone();
    v_C = t8125[1].clone();
    let _ = f__pos(Py::str("the cell size"), Py::tuple(vec![v_cell_size.clone()]))?;
    v_out = { let mut out = vec![]; for t8132 in py::range_iter(&[(v_R.clone()).i()?]) { let mut v__: Py; 
v__ = t8132;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_C.clone())?); } Py::list(out) };
    for t8133 in py::range_iter(&[(v_R.clone()).i()?]) {
        v_i = t8133;
        for t8134 in py::range_iter(&[(v_C.clone()).i()?]) {
            v_j = t8134;
            let t8135 = (Py::tuple(vec![Py::Float(0.0), Py::Int(0i64)])).items()?;
            if t8135.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_best = t8135[0].clone();
            v_code = t8135[1].clone();
            for t8136 in py::iter(&(c__D8()?))? {
                let t8149 = (t8136).items()?;
                if t8149.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_di = t8149[0].clone();
                v_dj = t8149[1].clone();
                v_cd = t8149[2].clone();
                let t8137 = (Py::tuple(vec![py::add(&v_i.clone(), &v_di.clone())?, py::add(&v_j.clone(), &v_dj.clone())?])).items()?;
                if t8137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_a = t8137[0].clone();
                v_b = t8137[1].clone();
                if ({ let t8144 = { let t8138 = Py::Int(0i64); Py::Bool({ let t8139 = v_a.clone(); if py::le(&t8138, &t8139)? { { let t8140 = v_R.clone(); if py::lt(&t8139, &t8140)? { true } else { false } } } else { false } }) }; if !t8144.truthy() { t8144 } else { { let t8141 = Py::Int(0i64); Py::Bool({ let t8142 = v_b.clone(); if py::le(&t8141, &t8142)? { { let t8143 = v_C.clone(); if py::lt(&t8142, &t8143)? { true } else { false } } } else { false } }) } } }).truthy() {
                    v_drop = py::truediv(&py::sub(&py::getitem(&py::getitem(&v_dem.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_dem.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::mul(&v_cell_size.clone(), &(if ({ let t8145 = v_di.clone(); if !t8145.truthy() { t8145 } else { v_dj.clone() } }).truthy() { py::math1("sqrt", &Py::Int(2i64))? } else { Py::Float(1.0) }))?)?;
                    if ({ let t8146 = v_drop.clone(); Py::Bool({ let t8147 = v_best.clone(); if py::gt(&t8146, &t8147)? { true } else { false } }) }).truthy() {
                        let t8148 = (Py::tuple(vec![v_drop.clone(), v_cd.clone()])).items()?;
                        if t8148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_best = t8148[0].clone();
                        v_code = t8148[1].clone();
                    }
                }
            }
            py::setitem(&py::getitem(&v_out.clone(), &v_i.clone())?, &v_j.clone(), py::float(&v_code.clone())?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dem(mut v_dem: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_r = py::len(&v_dem.clone())?;
    let _ = f__need({ let t8117 = { let t8109 = v_r.clone(); Py::Bool({ let t8110 = Py::Int(2i64); if py::ge(&t8109, &t8110)? { true } else { false } }) }; if !t8117.truthy() { t8117 } else { { let t8116 = Py::Bool(({ let mut out = vec![]; for t8111 in py::iter(&(v_dem.clone()))? { let mut v_row: Py; 
v_row = t8111;
out.push({ let t8112 = py::len(&v_row.clone())?; Py::Bool({ let t8113 = py::len(&py::getitem(&v_dem.clone(), &Py::Int(0i64))?)?; if py::eq(&t8112, &t8113) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t8116.truthy() { t8116 } else { { let t8114 = py::len(&py::getitem(&v_dem.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t8115 = Py::Int(2i64); if py::ge(&t8114, &t8115)? { true } else { false } }) } } } } }, Py::str("the elevation raster must be a rectangular matrix of at least 2 x 2"))?;
    let _ = f__need({ let t8118 = py::mul(&v_r.clone(), &py::len(&py::getitem(&v_dem.clone(), &Py::Int(0i64))?)?)?; Py::Bool({ let t8119 = Py::Int(900i64); if py::le(&t8118, &t8119)? { true } else { false } }) }, Py::str("the raster may have at most 900 cells"))?;
    for t8120 in py::iter(&(v_dem.clone()))? {
        v_row = t8120;
        for t8121 in py::iter(&(v_row.clone()))? {
            v_v = t8121;
            let _ = f__fin(Py::str("each elevation"), Py::tuple(vec![v_v.clone()]))?;
        }
    }
    return Ok(Py::tuple(vec![v_r.clone(), py::len(&py::getitem(&v_dem.clone(), &Py::Int(0i64))?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dijkstra(mut v_adj: Py, mut v_s: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_nd: Py = Py::None;
    let mut v_pq: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_dist = py::mul(&Py::list(vec![Py::Float(f64::INFINITY)]), &py::len(&v_adj.clone())?)?;
    py::setitem(&v_dist.clone(), &v_s.clone(), Py::Float(0.0))?;
    v_pq = Py::list(vec![Py::tuple(vec![Py::Float(0.0), v_s.clone()])]);
    while (v_pq.clone()).truthy() {
        let t7469 = (py::heapq("heappop", &[v_pq.clone()])?).items()?;
        if t7469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d = t7469[0].clone();
        v_u = t7469[1].clone();
        if ({ let t7470 = v_d.clone(); Py::Bool({ let t7471 = py::getitem(&v_dist.clone(), &v_u.clone())?; if py::gt(&t7470, &t7471)? { true } else { false } }) }).truthy() {
            continue;
        }
        for t7472 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            let t7475 = (t7472).items()?;
            if t7475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t7475[0].clone();
            v_w = t7475[1].clone();
            v_nd = py::add(&v_d.clone(), &v_w.clone())?;
            if ({ let t7473 = v_nd.clone(); Py::Bool({ let t7474 = py::getitem(&v_dist.clone(), &v_v.clone())?; if py::lt(&t7473, &t7474)? { true } else { false } }) }).truthy() {
                py::setitem(&v_dist.clone(), &v_v.clone(), v_nd.clone())?;
                let _ = py::heapq("heappush", &[v_pq.clone(), Py::tuple(vec![v_nd.clone(), v_v.clone()])])?;
            }
        }
    }
    return Ok(v_dist.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dvmodel(mut v_model: Py, mut v_h: Py, mut v_a: Py) -> R {
    let mut v_c: Py = Py::None;
    if ({ let t7872 = v_model.clone(); Py::Bool({ let t7873 = Py::str("exponential"); if py::eq(&t7872, &t7873) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::mul(&py::mul(&py::neg(&py::math1("exp", &py::truediv(&py::mul(&py::neg(&Py::Int(3i64))?, &v_h.clone())?, &v_a.clone())?)?)?, &Py::Int(3i64))?, &v_h.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?);
    }
    v_c = py::truediv(&Py::Float(4.0), &Py::Float(7.0))?;
    return Ok(py::truediv(&py::mul(&py::mul(&py::mul(&py::neg(&py::math1("exp", &py::neg(&py::pow(&py::truediv(&v_h.clone(), &py::mul(&v_a.clone(), &v_c.clone())?)?, &Py::Int(2i64))?)?)?)?, &Py::Int(2i64))?, &v_h.clone())?, &v_h.clone())?, &py::mul(&py::mul(&v_c.clone(), &v_c.clone())?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ecef(mut v_E: Py, mut v_lat: Py, mut v_lon: Py, mut v_h: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t4159 = (f__sincos_deg(v_lat.clone())?).items()?;
    if t4159.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sp = t4159[0].clone();
    v_cp = t4159[1].clone();
    let t4160 = (f__sincos_deg(v_lon.clone())?).items()?;
    if t4160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sl = t4160[0].clone();
    v_cl = t4160[1].clone();
    v_N = py::truediv(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &v_sp.clone())?, &v_sp.clone())?)?)?)?;
    return Ok(Py::tuple(vec![py::mul(&py::mul(&py::add(&v_N.clone(), &v_h.clone())?, &v_cp.clone())?, &v_cl.clone())?, py::mul(&py::mul(&py::add(&v_N.clone(), &v_h.clone())?, &v_cp.clone())?, &v_sl.clone())?, py::mul(&py::add(&py::mul(&v_N.clone(), &py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("e2"))?)?)?, &v_h.clone())?, &v_sp.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ellipsoid(mut v_name: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__need({ let t7651 = v_name.clone(); Py::Bool({ let t7652 = c_ELLIPSOIDS()?; if py::contains(&t7652, &t7651)? { true } else { false } }) }, Py::str("the ellipsoid must be WGS84 or GRS80"))?;
    let t7653 = (py::getitem(&c_ELLIPSOIDS()?, &v_name.clone())?).items()?;
    if t7653.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t7653[0].clone();
    v_f = t7653[1].clone();
    v_e2 = py::mul(&v_f.clone(), &py::sub(&Py::Int(2i64), &v_f.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("f"), v_f.clone()), (Py::str("b"), py::mul(&v_a.clone(), &py::sub(&Py::Int(1i64), &v_f.clone())?)?), (Py::str("e2"), v_e2.clone()), (Py::str("e"), py::math1("sqrt", &v_e2.clone())?), (Py::str("ep2"), py::truediv(&v_e2.clone(), &py::sub(&Py::Int(1i64), &v_e2.clone())?)?), (Py::str("n"), py::truediv(&v_f.clone(), &py::sub(&Py::Int(2i64), &v_f.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__enu_rot(mut v_lat0: Py, mut v_lon0: Py) -> R {
    let mut v_cl: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t4162 = (f__sincos_deg(v_lat0.clone())?).items()?;
    if t4162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sp = t4162[0].clone();
    v_cp = t4162[1].clone();
    let t4163 = (f__sincos_deg(v_lon0.clone())?).items()?;
    if t4163.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sl = t4163[0].clone();
    v_cl = t4163[1].clone();
    return Ok(Py::list(vec![Py::list(vec![py::neg(&v_sl.clone())?, v_cl.clone(), Py::Float(0.0)]), Py::list(vec![py::mul(&py::neg(&v_sp.clone())?, &v_cl.clone())?, py::mul(&py::neg(&v_sp.clone())?, &v_sl.clone())?, v_cp.clone()]), Py::list(vec![py::mul(&v_cp.clone(), &v_cl.clone())?, py::mul(&v_cp.clone(), &v_sl.clone())?, v_sp.clone()])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t8362 in py::iter(&(v_vals.clone()))? {
        v_v = t8362;
        let _ = f__need({ let t8364 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t8364.truthy() { t8364 } else { { let t8363 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t8363.truthy() { t8363 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__geo_point(mut v_lat: Py, mut v_lon: Py, mut v_h: Py, mut v_pre: Py) -> R {
    let _ = f__lat(py::add(&v_pre.clone(), &Py::str("lat"))?, v_lat.clone(), Py::Bool(true))?;
    let _ = f__lon(py::add(&v_pre.clone(), &Py::str("lon"))?, v_lon.clone())?;
    let _ = f__fin(py::add(&v_pre.clone(), &Py::str("h"))?, Py::tuple(vec![v_h.clone()]))?;
    let _ = f__need({ let t4154 = py::neg(&Py::Float(100000.0))?; Py::Bool({ let t4155 = v_h.clone(); if py::le(&t4154, &t4155)? { { let t4156 = Py::Float(100000000.0); if py::le(&t4155, &t4156)? { true } else { false } } } else { false } }) }, Py::str("the height must lie between -100 km and 100000 km"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__geod_direct(mut v_E: Py, mut v_lat1: Py, mut v_lon1: Py, mut v_az1: Py, mut v_s: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_az2: Py = Py::None;
    let mut v_ca0: Py = Py::None;
    let mut v_cb1: Py = Py::None;
    let mut v_cb2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_lat2: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_sa0: Py = Py::None;
    let mut v_sb1: Py = Py::None;
    let mut v_sb2: Py = Py::None;
    let mut v_target: Py = Py::None;
    let t2074 = (f__reduced(v_E.clone(), v_lat1.clone())?).items()?;
    if t2074.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sb1 = t2074[0].clone();
    v_cb1 = t2074[1].clone();
    let t2075 = (f__geodesic_frame(v_E.clone(), v_sb1.clone(), v_cb1.clone(), py::mul(&v_az1.clone(), &c_DEG()?)?)?).items()?;
    if t2075.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_sa0 = t2075[0].clone();
    v_ca0 = t2075[1].clone();
    v_s1 = t2075[2].clone();
    v_k2 = t2075[3].clone();
    v_g = { let v_E = v_E.clone(); let v_az1 = v_az1.clone(); let v_ca0 = v_ca0.clone(); let v_cb1 = v_cb1.clone(); let v_k2 = v_k2.clone(); let v_lat1 = v_lat1.clone(); let v_lon1 = v_lon1.clone(); let v_s = v_s.clone(); let v_s1 = v_s1.clone(); let v_sa0 = v_sa0.clone(); let v_sb1 = v_sb1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_k2.clone(), &py::pow(&py::math1("sin", &v_t.clone())?, &Py::Int(2i64))?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_target = py::truediv(&v_s.clone(), &py::getitem(&v_E.clone(), &Py::str("b"))?)?;
    v_s2 = py::add(&v_s1.clone(), &v_target.clone())?;
    for t2076 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
        v__ = t2076;
        v_F = py::sub(&f__integ(v_g.clone(), v_s1.clone(), v_s2.clone())?, &v_target.clone())?;
        v_d = py::truediv(&v_F.clone(), &py::call(&v_g.clone(), &[v_s2.clone()])?)?;
        v_s2 = py::sub(&v_s2, &v_d.clone())?;
        if ({ let t2077 = py::abs(&v_d.clone())?; Py::Bool({ let t2078 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::abs(&v_s2.clone())?], true)?)?; if py::le(&t2077, &t2078)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_sb2 = py::mul(&v_ca0.clone(), &py::math1("sin", &v_s2.clone())?)?;
    v_cb2 = py::hypot(&[v_sa0.clone(), py::mul(&v_ca0.clone(), &py::math1("cos", &v_s2.clone())?)?])?;
    v_lat2 = py::truediv(&py::atan2(&v_sb2.clone(), &py::mul(&py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("f"))?)?, &v_cb2.clone())?)?, &c_DEG()?)?;
    v_az2 = py::truediv(&py::atan2(&v_sa0.clone(), &py::mul(&v_ca0.clone(), &py::math1("cos", &v_s2.clone())?)?)?, &c_DEG()?)?;
    v_lam = f__lam_integral(v_E.clone(), v_sa0.clone(), v_k2.clone(), v_s1.clone(), v_s2.clone())?;
    return Ok(Py::tuple(vec![v_lat2.clone(), f__nlon(py::add(&v_lon1.clone(), &py::truediv(&v_lam.clone(), &c_DEG()?)?)?)?, f__az(v_az2.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__geod_inverse(mut v_E: Py, mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_az1: Py = Py::None;
    let mut v_az2: Py = Py::None;
    let mut v_ca0: Py = Py::None;
    let mut v_cb1: Py = Py::None;
    let mut v_cb2: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_cw: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_sa0: Py = Py::None;
    let mut v_sb1: Py = Py::None;
    let mut v_sb2: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let mut v_sw: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_w_new: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let c_k2 = py::cell_new();
    v_L = py::mul(&f__nlon(py::sub(&v_lon2.clone(), &v_lon1.clone())?)?, &c_DEG()?)?;
    let t2171 = (f__reduced(v_E.clone(), v_lat1.clone())?).items()?;
    if t2171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sb1 = t2171[0].clone();
    v_cb1 = t2171[1].clone();
    let t2172 = (f__reduced(v_E.clone(), v_lat2.clone())?).items()?;
    if t2172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sb2 = t2172[0].clone();
    v_cb2 = t2172[1].clone();
    v_f = py::getitem(&v_E.clone(), &Py::str("f"))?;
    v_w = v_L.clone();
    v_res = Py::None;
    let mut t2173 = false;
    for t2174 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v_it = t2174;
        let t2175 = (Py::tuple(vec![py::math1("sin", &v_w.clone())?, py::math1("cos", &v_w.clone())?])).items()?;
        if t2175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sw = t2175[0].clone();
        v_cw = t2175[1].clone();
        v_x = py::mul(&v_cb2.clone(), &v_sw.clone())?;
        v_y = py::sub(&py::mul(&v_cb1.clone(), &v_sb2.clone())?, &py::mul(&py::mul(&v_sb1.clone(), &v_cb2.clone())?, &v_cw.clone())?)?;
        v_ss = py::hypot(&[v_x.clone(), v_y.clone()])?;
        v_cs = py::add(&py::mul(&v_sb1.clone(), &v_sb2.clone())?, &py::mul(&py::mul(&v_cb1.clone(), &v_cb2.clone())?, &v_cw.clone())?)?;
        v_sig = py::atan2(&v_ss.clone(), &v_cs.clone())?;
        let _ = f__need({ let t2176 = v_ss.clone(); Py::Bool({ let t2177 = Py::Int(0i64); if py::gt(&t2176, &t2177)? { true } else { false } }) }, Py::str("the two points must be distinct"))?;
        v_az1 = py::atan2(&v_x.clone(), &v_y.clone())?;
        let t2179 = (f__geodesic_frame(v_E.clone(), v_sb1.clone(), v_cb1.clone(), v_az1.clone())?).items()?;
        if t2179.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_sa0 = t2179[0].clone();
        v_ca0 = t2179[1].clone();
        v_s1 = t2179[2].clone();
        py::cell_set(&c_k2, t2179[3].clone());
        v_J = f__integ({ let v_E = v_E.clone(); let v_L = v_L.clone(); let v_az1 = v_az1.clone(); let v_ca0 = v_ca0.clone(); let v_cb1 = v_cb1.clone(); let v_cb2 = v_cb2.clone(); let v_cs = v_cs.clone(); let v_cw = v_cw.clone(); let v_f = v_f.clone(); let v_it = v_it.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_lon1 = v_lon1.clone(); let v_lon2 = v_lon2.clone(); let v_res = v_res.clone(); let v_s1 = v_s1.clone(); let v_sa0 = v_sa0.clone(); let v_sb1 = v_sb1.clone(); let v_sb2 = v_sb2.clone(); let v_sig = v_sig.clone(); let v_ss = v_ss.clone(); let v_sw = v_sw.clone(); let v_w = v_w.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); let c_k2 = c_k2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::sub(&Py::Int(2i64), &v_f.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::sub(&Py::Int(1i64), &v_f.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&py::cell_get(&c_k2)?, &py::pow(&py::math1("sin", &v_t.clone())?, &Py::Int(2i64))?)?)?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_s1.clone(), py::add(&v_s1.clone(), &v_sig.clone())?)?;
        v_w_new = py::add(&v_L.clone(), &py::mul(&py::mul(&v_f.clone(), &v_sa0.clone())?, &v_J.clone())?)?;
        v_res = Py::tuple(vec![v_az1.clone(), v_sa0.clone(), v_ca0.clone(), v_s1.clone(), py::cell_get(&c_k2)?, v_sig.clone(), v_w_new.clone()]);
        if ({ let t2186 = py::abs(&py::sub(&v_w_new.clone(), &v_w.clone())?)?; Py::Bool({ let t2187 = Py::Float(1e-15); if py::le(&t2186, &t2187)? { true } else { false } }) }).truthy() {
            v_w = v_w_new.clone();
            { t2173 = true; break; }
        }
        v_w = v_w_new.clone();
    }
    if !t2173 {
        return Err(py::value_error(py::to_str(&Py::str("the geodesic iteration did not converge (points nearly antipodal)"))));
    }
    let t2188 = (Py::tuple(vec![py::math1("sin", &v_w.clone())?, py::math1("cos", &v_w.clone())?])).items()?;
    if t2188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sw = t2188[0].clone();
    v_cw = t2188[1].clone();
    v_x = py::mul(&v_cb2.clone(), &v_sw.clone())?;
    v_y = py::sub(&py::mul(&v_cb1.clone(), &v_sb2.clone())?, &py::mul(&py::mul(&v_sb1.clone(), &v_cb2.clone())?, &v_cw.clone())?)?;
    v_sig = py::atan2(&py::hypot(&[v_x.clone(), v_y.clone()])?, &py::add(&py::mul(&v_sb1.clone(), &v_sb2.clone())?, &py::mul(&py::mul(&v_cb1.clone(), &v_cb2.clone())?, &v_cw.clone())?)?)?;
    v_az1 = py::atan2(&v_x.clone(), &v_y.clone())?;
    let t2189 = (f__geodesic_frame(v_E.clone(), v_sb1.clone(), v_cb1.clone(), v_az1.clone())?).items()?;
    if t2189.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_sa0 = t2189[0].clone();
    v_ca0 = t2189[1].clone();
    v_s1 = t2189[2].clone();
    py::cell_set(&c_k2, t2189[3].clone());
    v_s = py::mul(&py::getitem(&v_E.clone(), &Py::str("b"))?, &f__integ({ let v_E = v_E.clone(); let v_J = v_J.clone(); let v_L = v_L.clone(); let v_az1 = v_az1.clone(); let v_ca0 = v_ca0.clone(); let v_cb1 = v_cb1.clone(); let v_cb2 = v_cb2.clone(); let v_cs = v_cs.clone(); let v_cw = v_cw.clone(); let v_f = v_f.clone(); let v_it = v_it.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_lon1 = v_lon1.clone(); let v_lon2 = v_lon2.clone(); let v_res = v_res.clone(); let v_s1 = v_s1.clone(); let v_sa0 = v_sa0.clone(); let v_sb1 = v_sb1.clone(); let v_sb2 = v_sb2.clone(); let v_sig = v_sig.clone(); let v_ss = v_ss.clone(); let v_sw = v_sw.clone(); let v_w = v_w.clone(); let v_w_new = v_w_new.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); let c_k2 = c_k2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&py::cell_get(&c_k2)?, &py::pow(&py::math1("sin", &v_t.clone())?, &Py::Int(2i64))?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_s1.clone(), py::add(&v_s1.clone(), &v_sig.clone())?)?)?;
    v_az2 = py::atan2(&py::mul(&v_cb1.clone(), &v_sw.clone())?, &py::add(&py::mul(&py::neg(&v_sb1.clone())?, &v_cb2.clone())?, &py::mul(&py::mul(&v_cb1.clone(), &v_sb2.clone())?, &v_cw.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("s"), v_s.clone()), (Py::str("az1"), f__az(py::truediv(&v_az1.clone(), &c_DEG()?)?)?), (Py::str("az2"), f__az(py::truediv(&v_az2.clone(), &c_DEG()?)?)?), (Py::str("sa0"), v_sa0.clone()), (Py::str("ca0"), v_ca0.clone()), (Py::str("s1"), v_s1.clone()), (Py::str("k2"), py::cell_get(&c_k2)?), (Py::str("sig"), v_sig.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__geodesic_frame(mut v_E: Py, mut v_sb1: Py, mut v_cb1: Py, mut v_az1: Py) -> R {
    let mut v_ca0: Py = Py::None;
    let mut v_ca1: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_sa0: Py = Py::None;
    let mut v_sa1: Py = Py::None;
    let mut v_sig1: Py = Py::None;
    let t2178 = (Py::tuple(vec![py::math1("sin", &v_az1.clone())?, py::math1("cos", &v_az1.clone())?])).items()?;
    if t2178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sa1 = t2178[0].clone();
    v_ca1 = t2178[1].clone();
    v_sa0 = py::mul(&v_sa1.clone(), &v_cb1.clone())?;
    v_ca0 = py::hypot(&[v_ca1.clone(), py::mul(&v_sa1.clone(), &v_sb1.clone())?])?;
    v_sig1 = py::atan2(&v_sb1.clone(), &py::mul(&v_ca1.clone(), &v_cb1.clone())?)?;
    v_k2 = py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("ep2"))?, &v_ca0.clone())?, &v_ca0.clone())?;
    return Ok(Py::tuple(vec![v_sa0.clone(), v_ca0.clone(), v_sig1.clone(), v_k2.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__geodetic(mut v_E: Py, mut v_X: Py, mut v_Y: Py, mut v_Z: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_nphi: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_phi: Py = Py::None;
    v_p = py::hypot(&[v_X.clone(), v_Y.clone()])?;
    let _ = f__need({ let t1689 = v_p.clone(); Py::Bool({ let t1690 = Py::Float(0.001); if py::gt(&t1689, &t1690)? { true } else { false } }) }, Py::str("the point lies on the polar axis: its longitude is undefined"))?;
    v_phi = py::atan2(&v_Z.clone(), &py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("e2"))?)?)?)?;
    for t1691 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t1691;
        v_N = f__N(v_E.clone(), v_phi.clone())?;
        v_nphi = py::atan2(&py::add(&v_Z.clone(), &py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &v_N.clone())?, &py::math1("sin", &v_phi.clone())?)?)?, &v_p.clone())?;
        v_d = py::sub(&v_nphi.clone(), &v_phi.clone())?;
        v_phi = v_nphi.clone();
        if ({ let t1692 = py::abs(&v_d.clone())?; Py::Bool({ let t1693 = Py::Float(1e-16); if py::lt(&t1692, &t1693)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_N = f__N(v_E.clone(), v_phi.clone())?;
    v_h = py::sub(&py::add(&py::mul(&v_p.clone(), &py::math1("cos", &v_phi.clone())?)?, &py::mul(&v_Z.clone(), &py::math1("sin", &v_phi.clone())?)?)?, &py::truediv(&py::pow(&py::getitem(&v_E.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &v_N.clone())?)?;
    return Ok(Py::tuple(vec![py::truediv(&v_phi.clone(), &c_DEG()?)?, py::add(&py::truediv(&py::atan2(&v_Y.clone(), &v_X.clone())?, &c_DEG()?)?, &Py::Float(0.0))?, v_h.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gh_bounds(mut v_code: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_bit: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_even: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_lon: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_rng_: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2412 = (Py::tuple(vec![Py::list(vec![py::neg(&Py::Float(90.0))?, Py::Float(90.0)]), Py::list(vec![py::neg(&Py::Float(180.0))?, Py::Float(180.0)])])).items()?;
    if t2412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t2412[0].clone();
    v_lon = t2412[1].clone();
    v_even = Py::Bool(true);
    for t2413 in py::iter(&(v_code.clone()))? {
        v_ch = t2413;
        let _ = f__need({ let t2414 = v_ch.clone(); Py::Bool({ let t2415 = c__GH()?; if py::contains(&t2415, &t2414)? { true } else { false } }) }, Py::str("a geohash uses the characters 0-9 and b-z without a, i, l, o"))?;
        v_v = py::method(&c__GH()?, "index", &[v_ch.clone()])?;
        for t2416 in py::range_iter(&[(Py::Int(4i64)).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_b = t2416;
            v_bit = py::bitand(&py::rshift(&v_v.clone(), &v_b.clone())?, &Py::Int(1i64))?;
            v_rng_ = (if (v_even.clone()).truthy() { v_lon.clone() } else { v_lat.clone() });
            v_mid = py::truediv(&py::add(&py::getitem(&v_rng_.clone(), &Py::Int(0i64))?, &py::getitem(&v_rng_.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?;
            if (v_bit.clone()).truthy() {
                py::setitem(&v_rng_.clone(), &Py::Int(0i64), v_mid.clone())?;
            } else {
                py::setitem(&v_rng_.clone(), &Py::Int(1i64), v_mid.clone())?;
            }
            v_even = py::not(&v_even.clone());
        }
    }
    return Ok(Py::tuple(vec![v_lat.clone(), v_lon.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__horn(mut v_w: Py, mut v_L: Py) -> R {
    let mut v_dzdx: Py = Py::None;
    let mut v_dzdy: Py = Py::None;
    v_dzdx = py::truediv(&py::sub(&py::add(&py::add(&py::getitem(&py::getitem(&v_w.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?)?, &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::add(&py::add(&py::getitem(&py::getitem(&v_w.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?, &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?, &py::mul(&Py::Int(8i64), &v_L.clone())?)?;
    v_dzdy = py::truediv(&py::sub(&py::add(&py::add(&py::getitem(&py::getitem(&v_w.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?)?, &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?)?, &py::add(&py::add(&py::getitem(&py::getitem(&v_w.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?, &py::getitem(&py::getitem(&v_w.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?)?, &py::mul(&Py::Int(8i64), &v_L.clone())?)?;
    return Ok(Py::tuple(vec![v_dzdx.clone(), v_dzdy.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hull(mut v_P: Py) -> R {
    let mut v_cross: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_up: Py = Py::None;
    v_P = py::sorted(&py::set(&v_P.clone())?, (Py::Bool(false)).truthy())?;
    if ({ let t5697 = py::len(&v_P.clone())?; Py::Bool({ let t5698 = Py::Int(2i64); if py::le(&t5697, &t5698)? { true } else { false } }) }).truthy() {
        return Ok(v_P.clone());
    }
    v_cross = { let v_P = v_P.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_o: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t5699 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t5699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t5699[0].clone();
    v_up = t5699[1].clone();
    for t5700 in py::iter(&(v_P.clone()))? {
        v_p = t5700;
        while ({ let t5705 = { let t5701 = py::len(&v_lo.clone())?; Py::Bool({ let t5702 = Py::Int(2i64); if py::ge(&t5701, &t5702)? { true } else { false } }) }; if !t5705.truthy() { t5705 } else { { let t5703 = py::call(&v_cross.clone(), &[py::getitem(&v_lo.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_lo.clone(), &py::neg(&Py::Int(1i64))?)?, v_p.clone()])?; Py::Bool({ let t5704 = Py::Int(0i64); if py::le(&t5703, &t5704)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_lo.clone(), "pop", &[])?;
        }
        let _ = py::method(&v_lo.clone(), "append", &[v_p.clone()])?;
    }
    for t5706 in py::iter(&(py::reversed(&v_P.clone())?))? {
        v_p = t5706;
        while ({ let t5711 = { let t5707 = py::len(&v_up.clone())?; Py::Bool({ let t5708 = Py::Int(2i64); if py::ge(&t5707, &t5708)? { true } else { false } }) }; if !t5711.truthy() { t5711 } else { { let t5709 = py::call(&v_cross.clone(), &[py::getitem(&v_up.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_up.clone(), &py::neg(&Py::Int(1i64))?)?, v_p.clone()])?; Py::Bool({ let t5710 = Py::Int(0i64); if py::le(&t5709, &t5710)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_up.clone(), "pop", &[])?;
        }
        let _ = py::method(&v_up.clone(), "append", &[v_p.clone()])?;
    }
    return Ok(py::add(&py::slice(&v_lo.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, &py::slice(&v_up.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__integ(mut v_f: Py, mut v_x0: Py, mut v_x1: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pieces: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t2180 = v_x1.clone(); Py::Bool({ let t2181 = v_x0.clone(); if py::eq(&t2180, &t2181) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_pieces = py::minmax(&[Py::Int(1i64), py::int(&py::math1("ceil", &py::truediv(&py::abs(&py::sub(&v_x1.clone(), &v_x0.clone())?)?, &py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?)?)?)?], true)?;
    v_h = py::truediv(&py::sub(&v_x1.clone(), &v_x0.clone())?, &v_pieces.clone())?;
    v_terms = Py::list(vec![]);
    for t2182 in py::range_iter(&[(v_pieces.clone()).i()?]) {
        v_p = t2182;
        v_a = py::add(&v_x0.clone(), &py::mul(&v_p.clone(), &v_h.clone())?)?;
        let t2183 = (Py::tuple(vec![py::add(&v_a.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::truediv(&v_h.clone(), &Py::Int(2i64))?])).items()?;
        if t2183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t2183[0].clone();
        v_r = t2183[1].clone();
        for t2184 in py::iter(&(py::zip(&[c__GLX()?, c__GLW()?])?))? {
            let t2185 = (t2184).items()?;
            if t2185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t2185[0].clone();
            v_w = t2185[1].clone();
            let _ = py::method(&v_terms.clone(), "append", &[py::mul(&py::mul(&v_w.clone(), &v_r.clone())?, &py::call(&v_f.clone(), &[py::add(&v_c.clone(), &py::mul(&v_r.clone(), &v_x.clone())?)?])?)?])?;
        }
    }
    return Ok(py::fsum(&v_terms.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kr(mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_n3: Py = Py::None;
    let mut v_n4: Py = Py::None;
    let mut v_n5: Py = Py::None;
    let mut v_n6: Py = Py::None;
    let t7671 = (Py::tuple(vec![py::pow(&v_n.clone(), &Py::Int(2i64))?, py::pow(&v_n.clone(), &Py::Int(3i64))?, py::pow(&v_n.clone(), &Py::Int(4i64))?, py::pow(&v_n.clone(), &Py::Int(5i64))?, py::pow(&v_n.clone(), &Py::Int(6i64))?])).items()?;
    if t7671.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_n2 = t7671[0].clone();
    v_n3 = t7671[1].clone();
    v_n4 = t7671[2].clone();
    v_n5 = t7671[3].clone();
    v_n6 = t7671[4].clone();
    v_alpha = Py::list(vec![py::add(&py::sub(&py::add(&py::add(&py::sub(&py::truediv(&v_n.clone(), &Py::Int(2i64))?, &py::truediv(&py::mul(&Py::Int(2i64), &v_n2.clone())?, &Py::Int(3i64))?)?, &py::truediv(&py::mul(&Py::Int(5i64), &v_n3.clone())?, &Py::Int(16i64))?)?, &py::truediv(&py::mul(&Py::Int(41i64), &v_n4.clone())?, &Py::Int(180i64))?)?, &py::truediv(&py::mul(&Py::Int(127i64), &v_n5.clone())?, &Py::Int(288i64))?)?, &py::truediv(&py::mul(&Py::Int(7891i64), &v_n6.clone())?, &Py::Int(37800i64))?)?, py::sub(&py::add(&py::add(&py::sub(&py::truediv(&py::mul(&Py::Int(13i64), &v_n2.clone())?, &Py::Int(48i64))?, &py::truediv(&py::mul(&Py::Int(3i64), &v_n3.clone())?, &Py::Int(5i64))?)?, &py::truediv(&py::mul(&Py::Int(557i64), &v_n4.clone())?, &Py::Int(1440i64))?)?, &py::truediv(&py::mul(&Py::Int(281i64), &v_n5.clone())?, &Py::Int(630i64))?)?, &py::truediv(&py::mul(&Py::Int(1983433i64), &v_n6.clone())?, &Py::Int(1935360i64))?)?, py::add(&py::add(&py::sub(&py::truediv(&py::mul(&Py::Int(61i64), &v_n3.clone())?, &Py::Int(240i64))?, &py::truediv(&py::mul(&Py::Int(103i64), &v_n4.clone())?, &Py::Int(140i64))?)?, &py::truediv(&py::mul(&Py::Int(15061i64), &v_n5.clone())?, &Py::Int(26880i64))?)?, &py::truediv(&py::mul(&Py::Int(167603i64), &v_n6.clone())?, &Py::Int(181440i64))?)?, py::add(&py::sub(&py::truediv(&py::mul(&Py::Int(49561i64), &v_n4.clone())?, &Py::Int(161280i64))?, &py::truediv(&py::mul(&Py::Int(179i64), &v_n5.clone())?, &Py::Int(168i64))?)?, &py::truediv(&py::mul(&Py::Int(6601661i64), &v_n6.clone())?, &Py::Int(7257600i64))?)?, py::sub(&py::truediv(&py::mul(&Py::Int(34729i64), &v_n5.clone())?, &Py::Int(80640i64))?, &py::truediv(&py::mul(&Py::Int(3418889i64), &v_n6.clone())?, &Py::Int(1995840i64))?)?, py::truediv(&py::mul(&Py::Int(212378941i64), &v_n6.clone())?, &Py::Int(319334400i64))?]);
    v_beta = Py::list(vec![py::add(&py::sub(&py::sub(&py::add(&py::sub(&py::truediv(&v_n.clone(), &Py::Int(2i64))?, &py::truediv(&py::mul(&Py::Int(2i64), &v_n2.clone())?, &Py::Int(3i64))?)?, &py::truediv(&py::mul(&Py::Int(37i64), &v_n3.clone())?, &Py::Int(96i64))?)?, &py::truediv(&v_n4.clone(), &Py::Int(360i64))?)?, &py::truediv(&py::mul(&Py::Int(81i64), &v_n5.clone())?, &Py::Int(512i64))?)?, &py::truediv(&py::mul(&Py::Int(96199i64), &v_n6.clone())?, &Py::Int(604800i64))?)?, py::sub(&py::add(&py::sub(&py::add(&py::truediv(&v_n2.clone(), &Py::Int(48i64))?, &py::truediv(&v_n3.clone(), &Py::Int(15i64))?)?, &py::truediv(&py::mul(&Py::Int(437i64), &v_n4.clone())?, &Py::Int(1440i64))?)?, &py::truediv(&py::mul(&Py::Int(46i64), &v_n5.clone())?, &Py::Int(105i64))?)?, &py::truediv(&py::mul(&Py::Int(1118711i64), &v_n6.clone())?, &Py::Int(3870720i64))?)?, py::add(&py::sub(&py::sub(&py::truediv(&py::mul(&Py::Int(17i64), &v_n3.clone())?, &Py::Int(480i64))?, &py::truediv(&py::mul(&Py::Int(37i64), &v_n4.clone())?, &Py::Int(840i64))?)?, &py::truediv(&py::mul(&Py::Int(209i64), &v_n5.clone())?, &Py::Int(4480i64))?)?, &py::truediv(&py::mul(&Py::Int(5569i64), &v_n6.clone())?, &Py::Int(90720i64))?)?, py::sub(&py::sub(&py::truediv(&py::mul(&Py::Int(4397i64), &v_n4.clone())?, &Py::Int(161280i64))?, &py::truediv(&py::mul(&Py::Int(11i64), &v_n5.clone())?, &Py::Int(504i64))?)?, &py::truediv(&py::mul(&Py::Int(830251i64), &v_n6.clone())?, &Py::Int(7257600i64))?)?, py::sub(&py::truediv(&py::mul(&Py::Int(4583i64), &v_n5.clone())?, &Py::Int(161280i64))?, &py::truediv(&py::mul(&Py::Int(108847i64), &v_n6.clone())?, &Py::Int(3991680i64))?)?, py::truediv(&py::mul(&Py::Int(20648693i64), &v_n6.clone())?, &Py::Int(638668800i64))?]);
    v_A = py::mul(&py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_n.clone())?)?, &py::add(&py::add(&py::add(&Py::Int(1i64), &py::truediv(&v_n2.clone(), &Py::Int(4i64))?)?, &py::truediv(&v_n4.clone(), &Py::Int(64i64))?)?, &py::truediv(&v_n6.clone(), &Py::Int(256i64))?)?)?;
    return Ok(Py::tuple(vec![v_alpha.clone(), v_beta.clone(), v_A.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lam_integral(mut v_E: Py, mut v_sa0: Py, mut v_k2: Py, mut v_s1: Py, mut v_s2: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_f: Py = Py::None;
    v_f = py::getitem(&v_E.clone(), &Py::str("f"))?;
    v_J = f__integ({ let v_E = v_E.clone(); let v_f = v_f.clone(); let v_k2 = v_k2.clone(); let v_s1 = v_s1.clone(); let v_s2 = v_s2.clone(); let v_sa0 = v_sa0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::sub(&Py::Int(2i64), &v_f.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::sub(&Py::Int(1i64), &v_f.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_k2.clone(), &py::pow(&py::math1("sin", &v_t.clone())?, &Py::Int(2i64))?)?)?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_s1.clone(), v_s2.clone())?;
    return Ok(py::sub(&py::sub(&f__omega(v_sa0.clone(), v_s2.clone())?, &f__omega(v_sa0.clone(), v_s1.clone())?)?, &py::mul(&py::mul(&v_f.clone(), &v_sa0.clone())?, &v_J.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lat(mut v_name: Py, mut v_v: Py, mut v_polar_ok: Py) -> R {
    let _ = f__fin(v_name.clone(), Py::tuple(vec![v_v.clone()]))?;
    if (v_polar_ok.clone()).truthy() {
        let _ = f__need({ let t8294 = py::neg(&Py::Int(90i64))?; Py::Bool({ let t8295 = v_v.clone(); if py::le(&t8294, &t8295)? { { let t8296 = Py::Int(90i64); if py::le(&t8295, &t8296)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie between -90 and 90 degrees".to_string()].concat()))?;
    } else {
        let _ = f__need({ let t8297 = py::neg(&Py::Int(90i64))?; Py::Bool({ let t8298 = v_v.clone(); if py::lt(&t8297, &t8298)? { { let t8299 = Py::Int(90i64); if py::lt(&t8298, &t8299)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie strictly between -90 and 90 degrees".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lcc_setup(mut v_E: Py, mut v_lat1: Py, mut v_lat2: Py, mut v_lat0: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t3767 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("lat1"), v_lat1.clone()]), Py::tuple(vec![Py::str("lat2"), v_lat2.clone()]), Py::tuple(vec![Py::str("lat0"), v_lat0.clone()])])))? {
        let t3776 = (t3767).items()?;
        if t3776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t3776[0].clone();
        v_v = t3776[1].clone();
        let _ = f__lat(v_nm.clone(), v_v.clone(), Py::Bool(false))?;
        let _ = f__need({ let t3774 = py::abs(&v_v.clone())?; Py::Bool({ let t3775 = Py::Int(89i64); if py::le(&t3774, &t3775)? { true } else { false } }) }, Py::str(&[py::format(&v_nm.clone(), "")?, " must lie within 89 degrees of the equator".to_string()].concat()))?;
    }
    let _ = f__need({ let t3777 = py::abs(&py::add(&v_lat1.clone(), &v_lat2.clone())?)?; Py::Bool({ let t3778 = Py::Float(1e-06); if py::gt(&t3777, &t3778)? { true } else { false } }) }, Py::str("the standard parallels must not be symmetric about the equator"))?;
    v_m = { let v_E = v_E.clone(); let v_lat0 = v_lat0.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_nm = v_nm.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::math1("cos", &v_p.clone())?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &v_p.clone())?, &Py::Int(2i64))?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t3779 = (Py::tuple(vec![py::mul(&v_lat1.clone(), &c_DEG()?)?, py::mul(&v_lat2.clone(), &c_DEG()?)?])).items()?;
    if t3779.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t3779[0].clone();
    v_p2 = t3779[1].clone();
    if ({ let t3780 = py::abs(&py::sub(&v_lat1.clone(), &v_lat2.clone())?)?; Py::Bool({ let t3781 = Py::Float(1e-10); if py::lt(&t3780, &t3781)? { true } else { false } }) }).truthy() {
        v_n = py::math1("sin", &v_p1.clone())?;
    } else {
        v_n = py::truediv(&py::sub(&py::math1("log", &py::call(&v_m.clone(), &[v_p1.clone()])?)?, &py::math1("log", &py::call(&v_m.clone(), &[v_p2.clone()])?)?)?, &py::sub(&f__psi(v_E.clone(), v_p2.clone())?, &f__psi(v_E.clone(), v_p1.clone())?)?)?;
    }
    v_F = py::truediv(&py::mul(&py::call(&v_m.clone(), &[v_p1.clone()])?, &py::math1("exp", &py::mul(&v_n.clone(), &f__psi(v_E.clone(), v_p1.clone())?)?)?)?, &v_n.clone())?;
    v_rho = { let v_E = v_E.clone(); let v_F = v_F.clone(); let v_lat0 = v_lat0.clone(); let v_lat1 = v_lat1.clone(); let v_lat2 = v_lat2.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_nm = v_nm.clone(); let v_p1 = v_p1.clone(); let v_p2 = v_p2.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &v_F.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_n.clone())?, &f__psi(v_E.clone(), v_p.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::tuple(vec![v_n.clone(), v_F.clone(), v_rho.clone(), v_m.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__linfit(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_c1: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let t7853 = (Py::tuple(vec![py::truediv(&py::fsum(&v_x.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?])).items()?;
    if t7853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t7853[0].clone();
    v_my = t7853[1].clone();
    v_sxx = py::fsum(&{ let mut out = vec![]; for t7854 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t7854;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_syy = py::fsum(&{ let mut out = vec![]; for t7855 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t7855;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_sxy = py::fsum(&{ let mut out = vec![]; for t7856 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t7857 = (t7856).items()?;
if t7857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t7857[0].clone();
v_b = t7857[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) })?;
    let _ = f__need({ let t7858 = v_sxx.clone(); Py::Bool({ let t7859 = Py::Int(0i64); if py::gt(&t7858, &t7859)? { true } else { false } }) }, Py::str("the x values must not all be equal"))?;
    v_c1 = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_r2 = (if ({ let t7860 = v_syy.clone(); Py::Bool({ let t7861 = Py::Int(0i64); if py::gt(&t7860, &t7861)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)? } else { Py::Float(1.0) });
    return Ok(Py::tuple(vec![py::sub(&v_my.clone(), &py::mul(&v_c1.clone(), &v_mx.clone())?)?, v_c1.clone(), v_r2.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lon(mut v_name: Py, mut v_v: Py) -> R {
    let _ = f__fin(v_name.clone(), Py::tuple(vec![v_v.clone()]))?;
    let _ = f__need({ let t8300 = py::neg(&Py::Int(540i64))?; Py::Bool({ let t8301 = v_v.clone(); if py::le(&t8300, &t8301)? { { let t8302 = Py::Int(540i64); if py::le(&t8301, &t8302)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a longitude in degrees (-540 to 540)".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mec(mut v_P: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_c3: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_inside: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_c2 = { let v_P = v_P.clone(); let v_c2 = v_c2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_cx: Py = Py::None;
            let mut v_cy: Py = Py::None;
            let t5712 = (Py::tuple(vec![py::truediv(&py::add(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?])).items()?;
            if t5712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cx = t5712[0].clone();
            v_cy = t5712[1].clone();
            return Ok(Py::tuple(vec![v_cx.clone(), v_cy.clone(), py::hypot(&[py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &v_cx.clone())?, py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &v_cy.clone())?])?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_c3 = { let v_P = v_P.clone(); let v_c2 = v_c2.clone(); let v_c3 = v_c3.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_cands: Py = Py::None;
            let mut v_d: Py = Py::None;
            let mut v_ux: Py = Py::None;
            let mut v_uy: Py = Py::None;
            v_d = py::mul(&Py::Int(2i64), &py::add(&py::add(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?)?)?;
            if ({ let t5713 = v_d.clone(); Py::Bool({ let t5714 = Py::Int(0i64); if py::eq(&t5713, &t5714) { true } else { false } }) }).truthy() {
                v_cands = Py::list(vec![py::call(&v_c2.clone(), &[v_a.clone(), v_b.clone()])?, py::call(&v_c2.clone(), &[v_a.clone(), v_c.clone()])?, py::call(&v_c2.clone(), &[v_b.clone(), v_c.clone()])?]);
                return Ok(py::minmax_key(&[v_cands.clone()], &{ let v_P = v_P.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_c2 = v_c2.clone(); let v_c3 = v_c3.clone(); let v_cands = v_cands.clone(); let v_d = v_d.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_t.clone(), &Py::Int(2i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?);
            }
            v_ux = py::truediv(&py::add(&py::add(&py::mul(&py::add(&py::pow(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::add(&py::pow(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&py::add(&py::pow(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?)?, &v_d.clone())?;
            v_uy = py::truediv(&py::add(&py::add(&py::mul(&py::add(&py::pow(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::add(&py::pow(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?)?)?, &py::mul(&py::add(&py::pow(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?)?)?, &v_d.clone())?;
            return Ok(Py::tuple(vec![v_ux.clone(), v_uy.clone(), py::hypot(&[py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &v_ux.clone())?, py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &v_uy.clone())?])?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_inside = { let v_P = v_P.clone(); let v_c2 = v_c2.clone(); let v_c3 = v_c3.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_p: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok({ let t5715 = py::hypot(&[py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?])?; Py::Bool({ let t5716 = py::mul(&py::getitem(&v_c.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t5715, &t5716)? { true } else { false } }) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_c = Py::None;
    for t5717 in py::iter(&(py::enumerate(&v_P.clone(), 0)?))? {
        let t5723 = (t5717).items()?;
        if t5723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t5723[0].clone();
        v_p = t5723[1].clone();
        if ({ let t5720 = { let t5718 = v_c.clone(); Py::Bool({ let t5719 = Py::None; if py::is(&t5718, &t5719) { true } else { false } }) }; if t5720.truthy() { t5720 } else { py::not(&py::call(&v_inside.clone(), &[v_c.clone(), v_p.clone()])?) } }).truthy() {
            v_c = Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?, Py::Float(0.0)]);
            for t5721 in py::range_iter(&[(v_i.clone()).i()?]) {
                v_j = t5721;
                v_q = py::getitem(&v_P.clone(), &v_j.clone())?;
                if (py::not(&py::call(&v_inside.clone(), &[v_c.clone(), v_q.clone()])?)).truthy() {
                    v_c = py::call(&v_c2.clone(), &[v_p.clone(), v_q.clone()])?;
                    for t5722 in py::range_iter(&[(v_j.clone()).i()?]) {
                        v_k = t5722;
                        if (py::not(&py::call(&v_inside.clone(), &[v_c.clone(), py::getitem(&v_P.clone(), &v_k.clone())?])?)).truthy() {
                            v_c = py::call(&v_c3.clone(), &[v_p.clone(), v_q.clone(), py::getitem(&v_P.clone(), &v_k.clone())?])?;
                        }
                    }
                }
            }
        }
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__merc_k0(mut v_E: Py, mut v_lat_ts: Py) -> R {
    return Ok(py::truediv(&py::math1("cos", &py::mul(&v_lat_ts.clone(), &c_DEG()?)?)?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &py::mul(&v_lat_ts.clone(), &c_DEG()?)?)?, &Py::Int(2i64))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__merid(mut v_E: Py, mut v_phi: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_n3: Py = Py::None;
    let mut v_n4: Py = Py::None;
    v_n = py::getitem(&v_E.clone(), &Py::str("n"))?;
    let t6641 = (Py::tuple(vec![py::mul(&v_n.clone(), &v_n.clone())?, py::pow(&v_n.clone(), &Py::Int(3i64))?, py::pow(&v_n.clone(), &Py::Int(4i64))?])).items()?;
    if t6641.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n2 = t6641[0].clone();
    v_n3 = t6641[1].clone();
    v_n4 = t6641[2].clone();
    return Ok(py::mul(&py::truediv(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::add(&Py::Int(1i64), &v_n.clone())?)?, &py::add(&py::sub(&py::add(&py::sub(&py::mul(&py::add(&py::add(&Py::Int(1i64), &py::truediv(&v_n2.clone(), &Py::Int(4i64))?)?, &py::truediv(&v_n4.clone(), &Py::Int(64i64))?)?, &v_phi.clone())?, &py::mul(&py::mul(&Py::Float(1.5), &py::sub(&v_n.clone(), &py::truediv(&v_n3.clone(), &Py::Int(8i64))?)?)?, &py::math1("sin", &py::mul(&Py::Int(2i64), &v_phi.clone())?)?)?)?, &py::mul(&py::mul(&py::truediv(&Py::Float(15.0), &Py::Int(16i64))?, &py::sub(&v_n2.clone(), &py::truediv(&v_n4.clone(), &Py::Int(4i64))?)?)?, &py::math1("sin", &py::mul(&Py::Int(4i64), &v_phi.clone())?)?)?)?, &py::mul(&py::mul(&py::truediv(&Py::Float(35.0), &Py::Int(48i64))?, &v_n3.clone())?, &py::math1("sin", &py::mul(&Py::Int(6i64), &v_phi.clone())?)?)?)?, &py::mul(&py::mul(&py::truediv(&Py::Float(315.0), &Py::Int(512i64))?, &v_n4.clone())?, &py::math1("sin", &py::mul(&Py::Int(8i64), &v_phi.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__merid_inv(mut v_E: Py, mut v_m: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_phi: Py = Py::None;
    v_phi = py::truediv(&py::mul(&py::truediv(&v_m.clone(), &f__merid(v_E.clone(), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?)?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?;
    for t6041 in py::range_iter(&[(Py::Int(50i64)).i()?]) {
        v__ = t6041;
        v_d = py::truediv(&py::sub(&f__merid(v_E.clone(), v_phi.clone())?, &v_m.clone())?, &f__M(v_E.clone(), v_phi.clone())?)?;
        v_phi = py::sub(&v_phi, &v_d.clone())?;
        if ({ let t6042 = py::abs(&v_d.clone())?; Py::Bool({ let t6043 = Py::Float(1e-16); if py::lt(&t6042, &t6043)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(v_phi.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nd(mut v_a: Py, mut v_b: Py) -> R {
    let _ = f__need({ let t8066 = py::add(&v_a.clone(), &v_b.clone())?; Py::Bool({ let t8067 = Py::Int(0i64); if py::ne(&t8066, &t8067) { true } else { false } }) }, Py::str("a normalized difference needs a non-zero sum of the bands"))?;
    return Ok(py::truediv(&py::sub(&v_a.clone(), &v_b.clone())?, &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__net(mut v_edges: Py, mut v_n: Py, mut v_weighted: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__need({ let t7412 = py::len(&v_edges.clone())?; Py::Bool({ let t7413 = Py::Int(1i64); if py::ge(&t7412, &t7413)? { true } else { false } }) }, Py::str("give at least one edge"))?;
    v_E = Py::list(vec![]);
    for t7414 in py::iter(&(v_edges.clone()))? {
        v_e = t7414;
        let t7417 = (Py::tuple(vec![f__node(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::str("a node"))?, f__node(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::str("a node"))?])).items()?;
        if t7417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t7417[0].clone();
        v_v = t7417[1].clone();
        if (v_weighted.clone()).truthy() {
            let _ = f__need({ let t7418 = py::len(&v_e.clone())?; Py::Bool({ let t7419 = Py::Int(3i64); if py::eq(&t7418, &t7419) { true } else { false } }) }, Py::str("every edge needs a length ('0-1 4.5')"))?;
            v_w = py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)?;
            let _ = f__need({ let t7422 = py::math1("isfinite", &v_w.clone())?; if !t7422.truthy() { t7422 } else { { let t7420 = v_w.clone(); Py::Bool({ let t7421 = Py::Int(0i64); if py::gt(&t7420, &t7421)? { true } else { false } }) } } }, Py::str("edge lengths must be positive"))?;
        } else {
            v_w = (if ({ let t7423 = py::len(&v_e.clone())?; Py::Bool({ let t7424 = Py::Int(3i64); if py::eq(&t7423, &t7424) { true } else { false } }) }).truthy() { py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)? } else { Py::Float(1.0) });
        }
        let _ = f__need({ let t7425 = v_u.clone(); Py::Bool({ let t7426 = v_v.clone(); if py::ne(&t7425, &t7426) { true } else { false } }) }, Py::str("an edge must join two different nodes"))?;
        let _ = py::method(&v_E.clone(), "append", &[Py::tuple(vec![v_u.clone(), v_v.clone(), v_w.clone()])])?;
    }
    v_m = py::add(&py::minmax(&[{ let mut out = vec![]; for t7427 in py::iter(&(v_E.clone()))? { let mut v__: Py; let mut v_u: Py; let mut v_v: Py; 
let t7428 = (t7427).items()?;
if t7428.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t7428[0].clone();
v_v = t7428[1].clone();
v__ = t7428[2].clone();
out.push(py::minmax(&[v_u.clone(), v_v.clone()], true)?); } Py::list(out) }], true)?, &Py::Int(1i64))?;
    if ({ let t7429 = v_n.clone(); Py::Bool({ let t7430 = Py::None; if py::is(&t7429, &t7430) { true } else { false } }) }).truthy() {
        v_n = v_m.clone();
    }
    let _ = f__need({ let t7431 = v_m.clone(); Py::Bool({ let t7432 = v_n.clone(); if py::le(&t7431, &t7432)? { true } else { false } }) }, Py::str(&["the edges use node ".to_string(), py::format(&py::sub(&v_m.clone(), &Py::Int(1i64))?, "")?, ", but only nodes 0 .. ".to_string(), py::format(&py::sub(&v_n.clone(), &Py::Int(1i64))?, "")?, " exist".to_string()].concat()))?;
    let _ = f__need({ let t7433 = v_n.clone(); Py::Bool({ let t7434 = Py::Int(500i64); if py::le(&t7433, &t7434)? { true } else { false } }) }, Py::str("the network may have at most 500 nodes"))?;
    v_adj = { let mut out = vec![]; for t7435 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t7435;
out.push(Py::list(vec![])); } Py::list(out) };
    for t7436 in py::iter(&(v_E.clone()))? {
        let t7437 = (t7436).items()?;
        if t7437.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t7437[0].clone();
        v_v = t7437[1].clone();
        v_w = t7437[2].clone();
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_u.clone())?, "append", &[Py::tuple(vec![v_v.clone(), v_w.clone()])])?;
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_v.clone())?, "append", &[Py::tuple(vec![v_u.clone(), v_w.clone()])])?;
    }
    return Ok(v_adj.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nlon(mut v_lon: Py) -> R {
    let mut v_x: Py = Py::None;
    v_x = py::fmod(&v_lon.clone(), &Py::Float(360.0))?;
    if ({ let t8317 = v_x.clone(); Py::Bool({ let t8318 = Py::Float(180.0); if py::gt(&t8317, &t8318)? { true } else { false } }) }).truthy() {
        v_x = py::sub(&v_x, &Py::Float(360.0))?;
    } else {
        if ({ let t8319 = v_x.clone(); Py::Bool({ let t8320 = py::neg(&Py::Float(180.0))?; if py::le(&t8319, &t8320)? { true } else { false } }) }).truthy() {
            v_x = py::iadd(&v_x, &Py::Float(360.0))?;
        }
    }
    return Ok(py::add(&v_x.clone(), &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__node(mut v_v: Py, mut v_name: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::method(&Py::str(&py::to_str(&v_v.clone())), "strip", &[])?;
    let _ = f__need({ let t7415 = ax_engine::pyre::module_call("fullmatch", &Py::str("\\d+"), &[v_s.clone()], 0)?; Py::Bool({ let t7416 = Py::None; if !py::is(&t7415, &t7416) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number (nodes are numbered 0 .. n-1)".to_string()].concat()))?;
    return Ok(py::int(&v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t8430 in py::iter(&(v_vals.clone()))? {
        v_v = t8430;
        let _ = f__need({ let t8435 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t8435.truthy() { t8435 } else { { let t8434 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t8434.truthy() { t8434 } else { { let t8433 = py::math1("isfinite", &v_v.clone())?; if !t8433.truthy() { t8433 } else { { let t8431 = v_v.clone(); Py::Bool({ let t8432 = Py::Int(0i64); if py::ge(&t8431, &t8432)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ols(mut v_X: Py, mut v_y: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = py::len(&v_X.clone())?;
    v_A = { let mut out = vec![]; for t7332 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t7332;
out.push({ let mut out = vec![]; for t7333 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t7333;
out.push(py::fsum(&{ let mut out = vec![]; for t7334 in py::range_iter(&[(py::len(&v_y.clone())?).i()?]) { let mut v_k: Py; 
v_k = t7334;
out.push(py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_b = { let mut out = vec![]; for t7335 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t7335;
out.push(py::fsum(&{ let mut out = vec![]; for t7336 in py::range_iter(&[(py::len(&v_y.clone())?).i()?]) { let mut v_k: Py; 
v_k = t7336;
out.push(py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_y.clone(), &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(f__solve_lin(v_A.clone(), v_b.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__omega(mut v_s_a0: Py, mut v_sigma: Py) -> R {
    let mut v_w: Py = Py::None;
    v_w = py::atan2(&py::mul(&py::abs(&v_s_a0.clone())?, &py::math1("sin", &v_sigma.clone())?)?, &py::math1("cos", &v_sigma.clone())?)?;
    v_w = py::iadd(&v_w, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::round(&py::truediv(&py::sub(&v_sigma.clone(), &v_w.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, None)?)?)?;
    return Ok((if ({ let t2079 = v_s_a0.clone(); Py::Bool({ let t2080 = Py::Int(0i64); if py::ne(&t2079, &t2080) { true } else { false } }) }).truthy() { py::mul(&py::copysign(&Py::Float(1.0), &v_s_a0.clone())?, &v_w.clone())? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t8445 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t8446 = (t8445).items()?;
if t8446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t8446[0].clone();
v_v = t8446[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__p_two(mut v_z: Py) -> R {
    return Ok(py::math1("erfc", &py::truediv(&py::abs(&v_z.clone())?, &py::math1("sqrt", &Py::Float(2.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__polygon_simple(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cross: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_d3: Py = Py::None;
    let mut v_d4: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    v_cross = { let v_cross = v_cross.clone(); let v_n = v_n.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_o: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_P = Py::list((py::zip(&[v_x.clone(), v_y.clone()])?).items()?);
    for t5663 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5663;
        let t5664 = (Py::tuple(vec![py::getitem(&v_P.clone(), &v_i.clone())?, py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t5664.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t5664[0].clone();
        v_b = t5664[1].clone();
        for t5665 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t5665;
            if ({ let t5673 = { let t5666 = v_j.clone(); Py::Bool({ let t5667 = v_i.clone(); if py::eq(&t5666, &t5667) { true } else { false } }) }; if t5673.truthy() { t5673 } else { { let t5672 = { let t5668 = py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?; Py::Bool({ let t5669 = v_i.clone(); if py::eq(&t5668, &t5669) { true } else { false } }) }; if t5672.truthy() { t5672 } else { { let t5670 = v_j.clone(); Py::Bool({ let t5671 = py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?; if py::eq(&t5670, &t5671) { true } else { false } }) } } } } }).truthy() {
                continue;
            }
            let t5674 = (Py::tuple(vec![py::getitem(&v_P.clone(), &v_j.clone())?, py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
            if t5674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t5674[0].clone();
            v_d = t5674[1].clone();
            let t5675 = (Py::tuple(vec![py::call(&v_cross.clone(), &[v_a.clone(), v_b.clone(), v_c.clone()])?, py::call(&v_cross.clone(), &[v_a.clone(), v_b.clone(), v_d.clone()])?, py::call(&v_cross.clone(), &[v_c.clone(), v_d.clone(), v_a.clone()])?, py::call(&v_cross.clone(), &[v_c.clone(), v_d.clone(), v_b.clone()])?])).items()?;
            if t5675.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_d1 = t5675[0].clone();
            v_d2 = t5675[1].clone();
            v_d3 = t5675[2].clone();
            v_d4 = t5675[3].clone();
            if ({ let t5692 = { let t5676 = { let t5678 = v_d1.clone(); Py::Bool({ let t5679 = Py::Int(0i64); if py::gt(&t5678, &t5679)? { true } else { false } }) }; Py::Bool({ let t5677 = { let t5680 = v_d2.clone(); Py::Bool({ let t5681 = Py::Int(0i64); if py::gt(&t5680, &t5681)? { true } else { false } }) }; if py::ne(&t5676, &t5677) { true } else { false } }) }; if !t5692.truthy() { t5692 } else { { let t5691 = { let t5682 = { let t5684 = v_d3.clone(); Py::Bool({ let t5685 = Py::Int(0i64); if py::gt(&t5684, &t5685)? { true } else { false } }) }; Py::Bool({ let t5683 = { let t5686 = v_d4.clone(); Py::Bool({ let t5687 = Py::Int(0i64); if py::gt(&t5686, &t5687)? { true } else { false } }) }; if py::ne(&t5682, &t5683) { true } else { false } }) }; if !t5691.truthy() { t5691 } else { { let t5690 = v_d1.clone(); if !t5690.truthy() { t5690 } else { { let t5689 = v_d2.clone(); if !t5689.truthy() { t5689 } else { { let t5688 = v_d3.clone(); if !t5688.truthy() { t5688 } else { v_d4.clone() } } } } } } } } } }).truthy() {
                return Ok(Py::Bool(false));
            }
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t8126 in py::iter(&(v_vals.clone()))? {
        v_v = t8126;
        let _ = f__need({ let t8131 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t8131.truthy() { t8131 } else { { let t8130 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t8130.truthy() { t8130 } else { { let t8129 = py::math1("isfinite", &v_v.clone())?; if !t8129.truthy() { t8129 } else { { let t8127 = v_v.clone(); Py::Bool({ let t8128 = Py::Int(0i64); if py::gt(&t8127, &t8128)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__psi(mut v_E: Py, mut v_phi: Py) -> R {
    return Ok(py::sub(&py::math1("asinh", &py::math1("tan", &v_phi.clone())?)?, &py::mul(&py::getitem(&v_E.clone(), &Py::str("e"))?, &py::math1("atanh", &py::mul(&py::getitem(&v_E.clone(), &Py::str("e"))?, &py::math1("sin", &v_phi.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pt(mut v_lat: Py, mut v_lon: Py, mut v_lat_lim: Py) -> R {
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon"), v_lon.clone())?;
    let _ = f__need({ let t8303 = py::abs(&v_lat.clone())?; Py::Bool({ let t8304 = v_lat_lim.clone(); if py::le(&t8303, &t8304)? { true } else { false } }) }, Py::str(&["the latitude must lie within ".to_string(), py::format(&v_lat_lim.clone(), "g")?, " degrees of the equator".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__q_auth(mut v_E: Py, mut v_phi: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::math1("sin", &v_phi.clone())?;
    v_e = py::getitem(&v_E.clone(), &Py::str("e"))?;
    return Ok(py::mul(&py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("e2"))?)?, &py::add(&py::truediv(&v_s.clone(), &py::sub(&Py::Int(1i64), &py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &v_s.clone())?, &v_s.clone())?)?)?, &py::truediv(&py::math1("atanh", &py::mul(&v_e.clone(), &v_s.clone())?)?, &v_e.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t8450 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t8450.truthy() { t8450 } else { { let t8449 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t8449.truthy() { t8449 } else { { let t8447 = v_x.clone(); Py::Bool({ let t8448 = Py::None; if py::is(&t8447, &t8448) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t8451 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t8451;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t8454 = { let t8452 = v_x.clone(); Py::Bool({ let t8453 = Py::Int(0i64); if py::eq(&t8452, &t8453) { true } else { false } }) }; if t8454.truthy() { t8454 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t8455 = v_x.clone(); Py::Bool({ let t8456 = Py::Int(0i64); if py::eq(&t8455, &t8456) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__receivers(mut v_dirs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_cd: Py = Py::None;
    let mut v_di: Py = Py::None;
    let mut v_dj: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_rec: Py = Py::None;
    let mut v_step: Py = Py::None;
    let t8160 = (Py::tuple(vec![py::len(&v_dirs.clone())?, py::len(&py::getitem(&v_dirs.clone(), &Py::Int(0i64))?)?])).items()?;
    if t8160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t8160[0].clone();
    v_C = t8160[1].clone();
    v_step = { let mut out = vec![]; for t8161 in py::iter(&(c__D8()?))? { let mut v_cd: Py; let mut v_di: Py; let mut v_dj: Py; 
let t8162 = (t8161).items()?;
if t8162.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_di = t8162[0].clone();
v_dj = t8162[1].clone();
v_cd = t8162[2].clone();
out.push((v_cd.clone(), Py::tuple(vec![v_di.clone(), v_dj.clone()]))); } Py::dict(out) };
    v_rec = Py::dict(vec![]);
    for t8163 in py::range_iter(&[(v_R.clone()).i()?]) {
        v_i = t8163;
        for t8164 in py::range_iter(&[(v_C.clone()).i()?]) {
            v_j = t8164;
            v_cd = py::int(&py::getitem(&py::getitem(&v_dirs.clone(), &v_i.clone())?, &v_j.clone())?)?;
            if (v_cd.clone()).truthy() {
                let t8165 = (py::getitem(&v_step.clone(), &v_cd.clone())?).items()?;
                if t8165.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_di = t8165[0].clone();
                v_dj = t8165[1].clone();
                py::setitem(&v_rec.clone(), &Py::tuple(vec![v_i.clone(), v_j.clone()]), Py::tuple(vec![py::add(&v_i.clone(), &v_di.clone())?, py::add(&v_j.clone(), &v_dj.clone())?]))?;
            }
        }
    }
    return Ok(v_rec.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__reduced(mut v_E: Py, mut v_lat: Py) -> R {
    let mut v_cb: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sb: Py = Py::None;
    v_p = py::mul(&v_lat.clone(), &c_DEG()?)?;
    let t2170 = (Py::tuple(vec![py::mul(&py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("f"))?)?, &py::math1("sin", &v_p.clone())?)?, py::math1("cos", &v_p.clone())?])).items()?;
    if t2170.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sb = t2170[0].clone();
    v_cb = t2170[1].clone();
    v_r = py::hypot(&[v_sb.clone(), v_cb.clone()])?;
    return Ok(Py::tuple(vec![py::truediv(&v_sb.clone(), &v_r.clone())?, py::truediv(&v_cb.clone(), &v_r.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__refl(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t8047 in py::iter(&(v_vals.clone()))? {
        v_v = t8047;
        let _ = f__fin(v_name.clone(), Py::tuple(vec![v_v.clone()]))?;
        let _ = f__need({ let t8051 = Py::Int(0i64); Py::Bool({ let t8052 = v_v.clone(); if py::le(&t8051, &t8052)? { { let t8053 = Py::Int(1i64); if py::le(&t8052, &t8053)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a reflectance between 0 and 1".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t8469 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t8469.truthy() { t8469 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t8473 = py::not(&{ let t8470 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t8470.truthy() { t8470 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t8473.truthy() { t8473 } else { { let t8471 = py::len(&v_a.clone())?; Py::Bool({ let t8472 = py::len(&v_b.clone())?; if py::ne(&t8471, &t8472) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t8474 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t8475 = (t8474).items()?;
if t8475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t8475[0].clone();
v_y = t8475[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t8482 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t8482.truthy() { t8482 } else { { let t8481 = Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))); if t8481.truthy() { t8481 } else { { let t8480 = { let t8476 = v_a.clone(); Py::Bool({ let t8477 = Py::None; if py::is(&t8476, &t8477) { true } else { false } }) }; if t8480.truthy() { t8480 } else { { let t8478 = v_b.clone(); Py::Bool({ let t8479 = Py::None; if py::is(&t8478, &t8479) { true } else { false } }) } } } } } } }).truthy() {
        return Ok((if ({ let t8483 = v_a.clone(); Py::Bool({ let t8484 = v_b.clone(); if py::eq(&t8483, &t8484) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t8486: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t8485 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t8485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t8485[0].clone();
            v_b = t8485[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8486 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&{ let t8487 = py::math1("isfinite", &v_a.clone())?; if !t8487.truthy() { t8487 } else { py::math1("isfinite", &v_b.clone())? } })).truthy() {
        return Ok((if ({ let t8488 = v_a.clone(); Py::Bool({ let t8489 = v_b.clone(); if py::eq(&t8488, &t8489) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t8490 = v_d.clone(); Py::Bool({ let t8491 = Py::Int(0i64); if py::eq(&t8490, &t8491) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t8496 = { let t8492 = py::abs(&v_a.clone())?; Py::Bool({ let t8493 = Py::Float(1e-15); if py::lt(&t8492, &t8493)? { true } else { false } }) }; if !t8496.truthy() { t8496 } else { { let t8494 = py::abs(&v_b.clone())?; Py::Bool({ let t8495 = Py::Float(1e-15); if py::lt(&t8494, &t8495)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__s012(mut v_W: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    v_n = py::len(&v_W.clone())?;
    v_s0 = py::fsum(&{ let mut out = vec![]; for t4974 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t4974;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) })?;
    v_s1 = py::mul(&Py::Float(0.5), &py::fsum(&{ let mut out = vec![]; for t4975 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4975;
for t4976 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4976;
out.push(py::pow(&py::add(&py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_W.clone(), &v_j.clone())?, &v_i.clone())?)?, &Py::Int(2i64))?); }} Py::list(out) })?)?;
    v_s2 = py::fsum(&{ let mut out = vec![]; for t4977 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4977;
out.push(py::pow(&py::add(&py::fsum(&py::getitem(&v_W.clone(), &v_i.clone())?)?, &py::fsum(&{ let mut out = vec![]; for t4978 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4978;
out.push(py::getitem(&py::getitem(&v_W.clone(), &v_j.clone())?, &v_i.clone())?); } Py::list(out) })?)?, &Py::Int(2i64))?); } Py::list(out) })?;
    return Ok(Py::tuple(vec![v_s0.clone(), v_s1.clone(), v_s2.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seg_dist(mut v_px: Py, mut v_py: Py, mut v_ax: Py, mut v_ay: Py, mut v_bx: Py, mut v_by: Py) -> R {
    let mut v_L2: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t1454 = (Py::tuple(vec![py::sub(&v_bx.clone(), &v_ax.clone())?, py::sub(&v_by.clone(), &v_ay.clone())?])).items()?;
    if t1454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t1454[0].clone();
    v_dy = t1454[1].clone();
    v_L2 = py::add(&py::mul(&v_dx.clone(), &v_dx.clone())?, &py::mul(&v_dy.clone(), &v_dy.clone())?)?;
    if ({ let t1455 = v_L2.clone(); Py::Bool({ let t1456 = Py::Int(0i64); if py::eq(&t1455, &t1456) { true } else { false } }) }).truthy() {
        return Ok(py::hypot(&[py::sub(&v_px.clone(), &v_ax.clone())?, py::sub(&v_py.clone(), &v_ay.clone())?])?);
    }
    v_t = py::minmax(&[Py::Float(0.0), py::minmax(&[Py::Float(1.0), py::truediv(&py::add(&py::mul(&py::sub(&v_px.clone(), &v_ax.clone())?, &v_dx.clone())?, &py::mul(&py::sub(&v_py.clone(), &v_ay.clone())?, &v_dy.clone())?)?, &v_L2.clone())?], false)?], true)?;
    return Ok(py::hypot(&[py::sub(&v_px.clone(), &py::add(&v_ax.clone(), &py::mul(&v_t.clone(), &v_dx.clone())?)?)?, py::sub(&v_py.clone(), &py::add(&v_ay.clone(), &py::mul(&v_t.clone(), &v_dy.clone())?)?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seg_in(mut v_x: Py, mut v_t: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__need({ let t6340 = { let t6336 = py::len(&v_x.clone())?; Py::Bool({ let t6337 = py::len(&v_t.clone())?; if py::eq(&t6336, &t6337) { true } else { false } }) }; if !t6340.truthy() { t6340 } else { { let t6338 = py::len(&v_x.clone())?; Py::Bool({ let t6339 = Py::Int(2i64); if py::ge(&t6338, &t6339)? { true } else { false } }) } } }, Py::str("give the group count and the total of every areal unit (at least 2 units)"))?;
    for t6341 in py::iter(&(py::zip(&[v_x.clone(), v_t.clone()])?))? {
        let t6356 = (t6341).items()?;
        if t6356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t6356[0].clone();
        v_b = t6356[1].clone();
        let _ = f__nonneg(Py::str("each count"), Py::tuple(vec![v_a.clone()]))?;
        let _ = f__pos(Py::str("each unit total"), Py::tuple(vec![v_b.clone()]))?;
        let _ = f__need({ let t6354 = v_a.clone(); Py::Bool({ let t6355 = v_b.clone(); if py::le(&t6354, &t6355)? { true } else { false } }) }, Py::str("a group count cannot exceed its unit's total"))?;
    }
    let t6357 = (Py::tuple(vec![py::fsum(&v_x.clone())?, py::fsum(&v_t.clone())?])).items()?;
    if t6357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t6357[0].clone();
    v_T = t6357[1].clone();
    v_P = py::truediv(&v_X.clone(), &v_T.clone())?;
    let _ = f__need({ let t6358 = Py::Int(0i64); Py::Bool({ let t6359 = v_P.clone(); if py::lt(&t6358, &t6359)? { { let t6360 = Py::Int(1i64); if py::lt(&t6359, &t6360)? { true } else { false } } } else { false } }) }, Py::str("the group must be present but not the whole population"))?;
    return Ok(Py::tuple(vec![v_X.clone(), v_T.clone(), v_P.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shoelace(mut v_P: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::fsum(&{ let mut out = vec![]; for t5693 in py::range_iter(&[(py::len(&v_P.clone())?).i()?]) { let mut v_i: Py; 
v_i = t5693;
out.push(py::sub(&py::mul(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_P.clone())?)?)?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_P.clone())?)?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sincos_deg(mut v_angle_deg: Py) -> R {
    let mut v_x: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_x = py::truediv(&py::mul(&ax_engine::pymp::call("mpf", &[v_angle_deg.clone()], &[])?, &ax_engine::pymp::constant("pi")?)?, &Py::Int(180i64))?;
        return Ok(Py::tuple(vec![py::float(&ax_engine::pymp::call("sin", &[v_x.clone()], &[])?)?, py::float(&ax_engine::pymp::call("cos", &[v_x.clone()], &[])?)?]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve_lin(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t7337 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t7338 = (t7337).items()?;
if t7338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t7338[0].clone();
v_row = t7338[1].clone();
out.push(py::add(&Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_row.clone())?).items()?), &Py::list(vec![py::float(&py::getitem(&v_b.clone(), &v_i.clone())?)?]))?); } Py::list(out) };
    v_scale = { let t7341 = { let t = { let mut out = vec![]; for t7339 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t7339;
for t7340 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t7340;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(1.0) } else { py::minmax(&[t], true)? } }; if t7341.truthy() { t7341 } else { Py::Float(1.0) } };
    for t7342 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t7342;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        let _ = f__need({ let t7343 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t7344 = py::mul(&Py::Float(1e-13), &v_scale.clone())?; if py::gt(&t7343, &t7344)? { true } else { false } }) }, Py::str("the equations are singular (the data do not determine the parameters)"))?;
        let t7345 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t7345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t7345[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t7345[1].clone())?;
        for t7346 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t7346;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t7347 = v_f.clone(); Py::Bool({ let t7348 = Py::Int(0i64); if py::ne(&t7347, &t7348) { true } else { false } }) }).truthy() {
                for t7349 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t7349;
                    let t7350 = py::getitem(&v_M.clone(), &v_r.clone())?; let t7351 = v_k.clone();
                    py::setitem(&t7350, &t7351, py::sub(&py::getitem(&t7350, &t7351)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t7352 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t7352;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::fsum(&{ let mut out = vec![]; for t7353 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t7353;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__spencer(mut v_doy: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::sub(&v_doy.clone(), &Py::Int(1i64))?)?, &Py::Float(365.0))?;
    return Ok(py::add(&py::add(&py::add(&py::add(&Py::Float(1.00011), &py::mul(&Py::Float(0.034221), &py::math1("cos", &v_g.clone())?)?)?, &py::mul(&Py::Float(0.00128), &py::math1("sin", &v_g.clone())?)?)?, &py::mul(&Py::Float(0.000719), &py::math1("cos", &py::mul(&Py::Int(2i64), &v_g.clone())?)?)?)?, &py::mul(&Py::Float(7.7e-05), &py::math1("sin", &py::mul(&Py::Int(2i64), &v_g.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sph_sep(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py) -> R {
    let mut v_dl: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let t3613 = (Py::tuple(vec![py::mul(&v_lat1.clone(), &c_DEG()?)?, py::mul(&v_lat2.clone(), &c_DEG()?)?, py::mul(&py::sub(&v_lon2.clone(), &v_lon1.clone())?, &c_DEG()?)?])).items()?;
    if t3613.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t3613[0].clone();
    v_p2 = t3613[1].clone();
    v_dl = t3613[2].clone();
    return Ok(py::truediv(&py::atan2(&py::hypot(&[py::mul(&py::math1("cos", &v_p2.clone())?, &py::math1("sin", &v_dl.clone())?)?, py::sub(&py::mul(&py::math1("cos", &v_p1.clone())?, &py::math1("sin", &v_p2.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_p1.clone())?, &py::math1("cos", &v_p2.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?])?, &py::add(&py::mul(&py::math1("sin", &v_p1.clone())?, &py::math1("sin", &v_p2.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_p1.clone())?, &py::math1("cos", &v_p2.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?)?, &c_DEG()?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sphere_R(mut v_R: Py) -> R {
    let _ = f__pos(Py::str("the radius R"), Py::tuple(vec![v_R.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tau_of_taup(mut v_E: Py, mut v_taup: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_tp: Py = Py::None;
    v_e2 = py::getitem(&v_E.clone(), &Py::str("e2"))?;
    v_tau = py::truediv(&v_taup.clone(), &py::sub(&Py::Int(1i64), &v_e2.clone())?)?;
    for t7677 in py::range_iter(&[(Py::Int(50i64)).i()?]) {
        v__ = t7677;
        v_tp = f__taup(v_E.clone(), v_tau.clone())?;
        v_d = py::truediv(&py::mul(&py::truediv(&py::sub(&v_taup.clone(), &v_tp.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_tp.clone(), &v_tp.clone())?)?)?)?, &py::add(&Py::Int(1i64), &py::mul(&py::mul(&py::sub(&Py::Int(1i64), &v_e2.clone())?, &v_tau.clone())?, &v_tau.clone())?)?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_e2.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_tau.clone(), &v_tau.clone())?)?)?)?)?;
        v_tau = py::iadd(&v_tau, &v_d.clone())?;
        if ({ let t7678 = py::abs(&v_d.clone())?; Py::Bool({ let t7679 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::abs(&v_tau.clone())?], true)?)?; if py::le(&t7678, &t7679)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(v_tau.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__taup(mut v_E: Py, mut v_tau: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_sig: Py = Py::None;
    v_e = py::getitem(&v_E.clone(), &Py::str("e"))?;
    v_sig = py::math1("sinh", &py::mul(&v_e.clone(), &py::math1("atanh", &py::truediv(&py::mul(&v_e.clone(), &v_tau.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_tau.clone(), &v_tau.clone())?)?)?)?)?)?)?;
    return Ok(py::sub(&py::mul(&v_tau.clone(), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_sig.clone(), &v_sig.clone())?)?)?)?, &py::mul(&v_sig.clone(), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_tau.clone(), &v_tau.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tissot_derivs(mut v_projection: Py, mut v_p: Py, mut v_l: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dth: Py = Py::None;
    let mut v_th: Py = Py::None;
    if ({ let t7080 = v_projection.clone(); Py::Bool({ let t7081 = Py::str("mercator"); if py::eq(&t7080, &t7081) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(1.0), Py::Float(0.0), Py::Float(0.0), py::truediv(&Py::Int(1i64), &py::math1("cos", &v_p.clone())?)?]));
    }
    if ({ let t7082 = v_projection.clone(); Py::Bool({ let t7083 = Py::str("equirectangular"); if py::eq(&t7082, &t7083) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(1.0), Py::Float(0.0), Py::Float(0.0), Py::Float(1.0)]));
    }
    if ({ let t7084 = v_projection.clone(); Py::Bool({ let t7085 = Py::str("cylindrical_equal_area"); if py::eq(&t7084, &t7085) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(1.0), Py::Float(0.0), Py::Float(0.0), py::math1("cos", &v_p.clone())?]));
    }
    if ({ let t7086 = v_projection.clone(); Py::Bool({ let t7087 = Py::str("sinusoidal"); if py::eq(&t7086, &t7087) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![py::math1("cos", &v_p.clone())?, Py::Float(0.0), py::mul(&py::neg(&v_l.clone())?, &py::math1("sin", &v_p.clone())?)?, Py::Float(1.0)]));
    }
    v_th = v_p.clone();
    for t7088 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t7088;
        v_d = py::truediv(&py::sub(&py::add(&py::mul(&Py::Int(2i64), &v_th.clone())?, &py::math1("sin", &py::mul(&Py::Int(2i64), &v_th.clone())?)?)?, &py::mul(&Py::Float(std::f64::consts::PI), &py::math1("sin", &v_p.clone())?)?)?, &py::add(&Py::Int(2i64), &py::mul(&Py::Int(2i64), &py::math1("cos", &py::mul(&Py::Int(2i64), &v_th.clone())?)?)?)?)?;
        v_th = py::sub(&v_th, &v_d.clone())?;
        if ({ let t7089 = py::abs(&v_d.clone())?; Py::Bool({ let t7090 = Py::Float(1e-15); if py::lt(&t7089, &t7090)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_dth = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::math1("cos", &v_p.clone())?)?, &py::mul(&Py::Int(4i64), &py::pow(&py::math1("cos", &v_th.clone())?, &Py::Int(2i64))?)?)?;
    v_c = py::truediv(&py::mul(&Py::Int(2i64), &py::math1("sqrt", &Py::Int(2i64))?)?, &Py::Float(std::f64::consts::PI))?;
    return Ok(Py::tuple(vec![py::mul(&v_c.clone(), &py::math1("cos", &v_th.clone())?)?, Py::Float(0.0), py::mul(&py::mul(&py::mul(&py::neg(&v_c.clone())?, &v_l.clone())?, &py::math1("sin", &v_th.clone())?)?, &v_dth.clone())?, py::mul(&py::mul(&py::math1("sqrt", &Py::Int(2i64))?, &py::math1("cos", &v_th.clone())?)?, &v_dth.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tm_fwd(mut v_E: Py, mut v_phi: Py, mut v_lam: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A1: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_al: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_etap: Py = Py::None;
    let mut v_gamma: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_sh: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_taup: Py = Py::None;
    let mut v_xi: Py = Py::None;
    let mut v_xip: Py = Py::None;
    let t7588 = (f__kr(py::getitem(&v_E.clone(), &Py::str("n"))?)?).items()?;
    if t7588.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_alpha = t7588[0].clone();
    v__ = t7588[1].clone();
    v_A1 = t7588[2].clone();
    v_A = py::mul(&v_A1.clone(), &py::getitem(&v_E.clone(), &Py::str("a"))?)?;
    v_tau = py::math1("tan", &v_phi.clone())?;
    v_taup = f__taup(v_E.clone(), v_tau.clone())?;
    let t7589 = (Py::tuple(vec![py::math1("cos", &v_lam.clone())?, py::math1("sin", &v_lam.clone())?])).items()?;
    if t7589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cl = t7589[0].clone();
    v_sl = t7589[1].clone();
    v_xip = py::atan2(&v_taup.clone(), &v_cl.clone())?;
    v_etap = py::math1("asinh", &py::truediv(&v_sl.clone(), &py::hypot(&[v_taup.clone(), v_cl.clone()])?)?)?;
    let t7590 = (Py::tuple(vec![v_xip.clone(), v_etap.clone(), Py::Float(1.0), Py::Float(0.0)])).items()?;
    if t7590.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_xi = t7590[0].clone();
    v_eta = t7590[1].clone();
    v_p = t7590[2].clone();
    v_q = t7590[3].clone();
    for t7591 in py::iter(&(py::enumerate(&v_alpha.clone(), (Py::Int(1i64)).i()?)?))? {
        let t7594 = (t7591).items()?;
        if t7594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t7594[0].clone();
        v_al = t7594[1].clone();
        let t7592 = (Py::tuple(vec![py::math1("cos", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_xip.clone())?)?, py::math1("sin", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_xip.clone())?)?])).items()?;
        if t7592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c2 = t7592[0].clone();
        v_s2 = t7592[1].clone();
        let t7593 = (Py::tuple(vec![py::math1("cosh", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_etap.clone())?)?, py::math1("sinh", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_etap.clone())?)?])).items()?;
        if t7593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ch = t7593[0].clone();
        v_sh = t7593[1].clone();
        v_xi = py::iadd(&v_xi, &py::mul(&py::mul(&v_al.clone(), &v_s2.clone())?, &v_ch.clone())?)?;
        v_eta = py::iadd(&v_eta, &py::mul(&py::mul(&v_al.clone(), &v_c2.clone())?, &v_sh.clone())?)?;
        v_p = py::iadd(&v_p, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_al.clone())?, &v_c2.clone())?, &v_ch.clone())?)?;
        v_q = py::iadd(&v_q, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_al.clone())?, &v_s2.clone())?, &v_sh.clone())?)?;
    }
    v_gamma = py::add(&py::math1("atan", &py::mul(&py::truediv(&v_taup.clone(), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_taup.clone(), &v_taup.clone())?)?)?)?, &py::math1("tan", &v_lam.clone())?)?)?, &py::atan2(&v_q.clone(), &v_p.clone())?)?;
    v_k = py::mul(&py::truediv(&py::mul(&v_A1.clone(), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&py::mul(&py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("e2"))?)?, &v_tau.clone())?, &v_tau.clone())?)?)?)?, &py::hypot(&[v_taup.clone(), v_cl.clone()])?)?, &py::hypot(&[v_p.clone(), v_q.clone()])?)?;
    return Ok(Py::tuple(vec![py::mul(&v_A.clone(), &v_xi.clone())?, py::mul(&v_A.clone(), &v_eta.clone())?, v_gamma.clone(), v_k.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tm_inv(mut v_E: Py, mut v_X: Py, mut v_Y: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A1: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_be: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_etap: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_taup: Py = Py::None;
    let mut v_xi: Py = Py::None;
    let mut v_xip: Py = Py::None;
    let t7672 = (f__kr(py::getitem(&v_E.clone(), &Py::str("n"))?)?).items()?;
    if t7672.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__ = t7672[0].clone();
    v_beta = t7672[1].clone();
    v_A1 = t7672[2].clone();
    v_A = py::mul(&v_A1.clone(), &py::getitem(&v_E.clone(), &Py::str("a"))?)?;
    let t7673 = (Py::tuple(vec![py::truediv(&v_X.clone(), &v_A.clone())?, py::truediv(&v_Y.clone(), &v_A.clone())?])).items()?;
    if t7673.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xi = t7673[0].clone();
    v_eta = t7673[1].clone();
    let t7674 = (Py::tuple(vec![v_xi.clone(), v_eta.clone()])).items()?;
    if t7674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xip = t7674[0].clone();
    v_etap = t7674[1].clone();
    for t7675 in py::iter(&(py::enumerate(&v_beta.clone(), (Py::Int(1i64)).i()?)?))? {
        let t7676 = (t7675).items()?;
        if t7676.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t7676[0].clone();
        v_be = t7676[1].clone();
        v_xip = py::sub(&v_xip, &py::mul(&py::mul(&v_be.clone(), &py::math1("sin", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_xi.clone())?)?)?, &py::math1("cosh", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_eta.clone())?)?)?)?;
        v_etap = py::sub(&v_etap, &py::mul(&py::mul(&v_be.clone(), &py::math1("cos", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_xi.clone())?)?)?, &py::math1("sinh", &py::mul(&py::mul(&Py::Int(2i64), &v_j.clone())?, &v_eta.clone())?)?)?)?;
    }
    v_taup = py::truediv(&py::math1("sin", &v_xip.clone())?, &py::hypot(&[py::math1("sinh", &v_etap.clone())?, py::math1("cos", &v_xip.clone())?])?)?;
    v_lam = py::atan2(&py::math1("sinh", &v_etap.clone())?, &py::math1("cos", &v_xip.clone())?)?;
    return Ok(Py::tuple(vec![py::math1("atan", &f__tau_of_taup(v_E.clone(), v_taup.clone())?)?, v_lam.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_points(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py) -> R {
    let mut v_sep: Py = Py::None;
    let _ = f__lat(Py::str("lat1"), v_lat1.clone(), Py::Bool(false))?;
    let _ = f__lat(Py::str("lat2"), v_lat2.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon1"), v_lon1.clone())?;
    let _ = f__lon(Py::str("lon2"), v_lon2.clone())?;
    v_sep = f__sph_sep(v_lat1.clone(), v_lon1.clone(), v_lat2.clone(), v_lon2.clone())?;
    let _ = f__need({ let t2042 = v_sep.clone(); Py::Bool({ let t2043 = Py::Float(1e-07); if py::gt(&t2042, &t2043)? { true } else { false } }) }, Py::str("the two points must be distinct"))?;
    let _ = f__need({ let t2044 = v_sep.clone(); Py::Bool({ let t2045 = Py::Int(150i64); if py::le(&t2044, &t2045)? { true } else { false } }) }, Py::str("the points must be at most 150 degrees apart (the iteration converges slowly towards antipodes)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__utm_zone(mut v_lat: Py, mut v_lon: Py) -> R {
    let mut v_z: Py = Py::None;
    v_lon = f__nlon(v_lon.clone())?;
    if ({ let t7759 = v_lon.clone(); Py::Bool({ let t7760 = Py::Float(180.0); if py::eq(&t7759, &t7760) { true } else { false } }) }).truthy() {
        v_lon = py::neg(&Py::Float(180.0))?;
    }
    v_z = py::add(&py::int(&py::math1("floor", &py::truediv(&py::add(&v_lon.clone(), &Py::Float(180.0))?, &Py::Float(6.0))?)?)?, &Py::Int(1i64))?;
    if ({ let t7767 = { let t7761 = Py::Int(56i64); Py::Bool({ let t7762 = v_lat.clone(); if py::le(&t7761, &t7762)? { { let t7763 = Py::Int(64i64); if py::lt(&t7762, &t7763)? { true } else { false } } } else { false } }) }; if !t7767.truthy() { t7767 } else { { let t7764 = Py::Int(3i64); Py::Bool({ let t7765 = v_lon.clone(); if py::le(&t7764, &t7765)? { { let t7766 = Py::Int(12i64); if py::lt(&t7765, &t7766)? { true } else { false } } } else { false } }) } } }).truthy() {
        v_z = Py::Int(32i64);
    }
    if ({ let t7773 = { let t7768 = Py::Int(72i64); Py::Bool({ let t7769 = v_lat.clone(); if py::le(&t7768, &t7769)? { { let t7770 = Py::Int(84i64); if py::le(&t7769, &t7770)? { true } else { false } } } else { false } }) }; if !t7773.truthy() { t7773 } else { { let t7771 = v_lon.clone(); Py::Bool({ let t7772 = Py::Int(0i64); if py::ge(&t7771, &t7772)? { true } else { false } }) } } }).truthy() {
        if ({ let t7774 = v_lon.clone(); Py::Bool({ let t7775 = Py::Int(9i64); if py::lt(&t7774, &t7775)? { true } else { false } }) }).truthy() {
            v_z = Py::Int(31i64);
        } else {
            if ({ let t7776 = v_lon.clone(); Py::Bool({ let t7777 = Py::Int(21i64); if py::lt(&t7776, &t7777)? { true } else { false } }) }).truthy() {
                v_z = Py::Int(33i64);
            } else {
                if ({ let t7778 = v_lon.clone(); Py::Bool({ let t7779 = Py::Int(33i64); if py::lt(&t7778, &t7779)? { true } else { false } }) }).truthy() {
                    v_z = Py::Int(35i64);
                } else {
                    if ({ let t7780 = v_lon.clone(); Py::Bool({ let t7781 = Py::Int(42i64); if py::lt(&t7780, &t7781)? { true } else { false } }) }).truthy() {
                        v_z = Py::Int(37i64);
                    }
                }
            }
        }
    }
    return Ok(v_z.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vfit(mut v_h: Py, mut v_gamma: Py, mut v_model: Py, mut v_zero_nugget: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_am: Py = Py::None;
    let mut v_c0m: Py = Py::None;
    let mut v_cm: Py = Py::None;
    let mut v_dshape: Py = Py::None;
    let mut v_g0: Py = Py::None;
    let mut v_g1: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_gmp: Py = Py::None;
    let mut v_grad: Py = Py::None;
    let mut v_grid: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_hi_: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lin: Py = Py::None;
    let mut v_linfit: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_lo_: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_shape: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_n = py::len(&v_h.clone())?;
    v_linfit = { let v_gamma = v_gamma.clone(); let v_h = v_h.clone(); let v_linfit = v_linfit.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v__: Py = Py::None;
            let mut v_c: Py = Py::None;
            let mut v_c0: Py = Py::None;
            let mut v_f: Py = Py::None;
            let mut v_res: Py = Py::None;
            v_f = { let mut out = vec![]; for t7843 in py::iter(&(v_h.clone()))? { let mut v_t: Py; 
v_t = t7843;
out.push(f__vmodel(v_model.clone(), v_t.clone(), v_a.clone())?); } Py::list(out) };
            if (v_zero_nugget.clone()).truthy() {
                v_c0 = Py::Float(0.0);
                v_c = py::truediv(&py::fsum(&{ let mut out = vec![]; for t7850 in py::iter(&(py::zip(&[v_f.clone(), v_gamma.clone()])?))? { let mut v_fv: Py; let mut v_g: Py; 
let t7851 = (t7850).items()?;
if t7851.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_fv = t7851[0].clone();
v_g = t7851[1].clone();
out.push(py::mul(&v_fv.clone(), &v_g.clone())?); } Py::list(out) })?, &py::fsum(&{ let mut out = vec![]; for t7852 in py::iter(&(v_f.clone()))? { let mut v_fv: Py; 
v_fv = t7852;
out.push(py::mul(&v_fv.clone(), &v_fv.clone())?); } Py::list(out) })?)?;
            } else {
                let t7862 = (f__linfit(v_f.clone(), v_gamma.clone())?).items()?;
                if t7862.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_c0 = t7862[0].clone();
                v_c = t7862[1].clone();
                v__ = t7862[2].clone();
            }
            v_res = { let mut out = vec![]; for t7863 in py::iter(&(py::zip(&[v_gamma.clone(), v_f.clone()])?))? { let mut v_fv: Py; let mut v_g: Py; 
let t7864 = (t7863).items()?;
if t7864.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t7864[0].clone();
v_fv = t7864[1].clone();
out.push(py::sub(&py::sub(&v_g.clone(), &v_c0.clone())?, &py::mul(&v_c.clone(), &v_fv.clone())?)?); } Py::list(out) };
            return Ok(Py::tuple(vec![v_c0.clone(), v_c.clone(), v_res.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_sse = { let v_gamma = v_gamma.clone(); let v_h = v_h.clone(); let v_linfit = v_linfit.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_sse = v_sse.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_f: Py = Py::None;
            v_f = { let mut out = vec![]; for t7865 in py::iter(&(v_h.clone()))? { let mut v_t: Py; 
v_t = t7865;
out.push(f__vmodel(v_model.clone(), v_t.clone(), v_a.clone())?); } Py::list(out) };
            if ({ let t7866 = py::sub(&py::minmax(&[v_f.clone()], true)?, &py::minmax(&[v_f.clone()], false)?)?; Py::Bool({ let t7867 = Py::Float(1e-09); if py::lt(&t7866, &t7867)? { true } else { false } }) }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
            return Ok(py::fsum(&{ let mut out = vec![]; for t7868 in py::iter(&(py::getitem(&py::call(&v_linfit.clone(), &[v_a.clone()])?, &Py::Int(2i64))?))? { let mut v_r: Py; 
v_r = t7868;
out.push(py::mul(&v_r.clone(), &v_r.clone())?); } Py::list(out) })?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_grad = { let v_gamma = v_gamma.clone(); let v_grad = v_grad.clone(); let v_h = v_h.clone(); let v_linfit = v_linfit.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_sse = v_sse.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_c: Py = Py::None;
            let mut v_c0: Py = Py::None;
            let mut v_res: Py = Py::None;
            let t7869 = (py::call(&v_linfit.clone(), &[v_a.clone()])?).items()?;
            if t7869.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_c0 = t7869[0].clone();
            v_c = t7869[1].clone();
            v_res = t7869[2].clone();
            return Ok(py::mul(&py::neg(&Py::Int(2i64))?, &py::fsum(&{ let mut out = vec![]; for t7870 in py::iter(&(py::zip(&[v_res.clone(), v_h.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t7871 = (t7870).items()?;
if t7871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t7871[0].clone();
v_t = t7871[1].clone();
out.push(py::mul(&py::mul(&v_r.clone(), &v_c.clone())?, &f__dvmodel(v_model.clone(), v_t.clone(), v_a.clone())?)?); } Py::list(out) })?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t7874 = (Py::tuple(vec![py::truediv(&py::minmax(&[v_h.clone()], false)?, &Py::Int(20i64))?, py::mul(&py::minmax(&[v_h.clone()], true)?, &Py::Int(20i64))?])).items()?;
    if t7874.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t7874[0].clone();
    v_hi = t7874[1].clone();
    v_grid = { let mut out = vec![]; for t7875 in py::range_iter(&[(Py::Int(401i64)).i()?]) { let mut v_i: Py; 
v_i = t7875;
out.push(py::mul(&v_lo.clone(), &py::pow(&py::truediv(&v_hi.clone(), &v_lo.clone())?, &py::truediv(&v_i.clone(), &Py::Int(400i64))?)?)?); } Py::list(out) };
    v_vals = { let mut out = vec![]; for t7876 in py::iter(&(v_grid.clone()))? { let mut v_a: Py; 
v_a = t7876;
out.push(py::call(&v_sse.clone(), &[v_a.clone()])?); } Py::list(out) };
    v_i = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_grid.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_gamma = v_gamma.clone(); let v_grad = v_grad.clone(); let v_grid = v_grid.clone(); let v_h = v_h.clone(); let v_hi = v_hi.clone(); let v_linfit = v_linfit.clone(); let v_lo = v_lo.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_sse = v_sse.clone(); let v_vals = v_vals.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_vals.clone(), &v_k.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
    let _ = f__need({ let t7877 = Py::Int(0i64); Py::Bool({ let t7878 = v_i.clone(); if py::lt(&t7877, &t7878)? { { let t7879 = py::sub(&py::len(&v_grid.clone())?, &Py::Int(1i64))?; if py::lt(&t7878, &t7879)? { true } else { false } } } else { false } }) }, Py::str("the fit does not determine the range (no interior minimum)"))?;
    let t7880 = (Py::tuple(vec![py::getitem(&v_grid.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])).items()?;
    if t7880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a0 = t7880[0].clone();
    v_a1 = t7880[1].clone();
    let t7881 = (Py::tuple(vec![py::call(&v_grad.clone(), &[v_a0.clone()])?, py::call(&v_grad.clone(), &[v_a1.clone()])?])).items()?;
    if t7881.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g0 = t7881[0].clone();
    v_g1 = t7881[1].clone();
    let _ = f__need({ let t7882 = v_g0.clone(); Py::Bool({ let t7883 = Py::Int(0i64); if py::lt(&t7882, &t7883)? { { let t7884 = v_g1.clone(); if py::lt(&t7883, &t7884)? { true } else { false } } } else { false } }) }, Py::str("the fit does not determine the range"))?;
    for t7885 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t7885;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a0.clone(), &v_a1.clone())?)?;
        if ({ let t7890 = { let t7886 = v_m.clone(); Py::Bool({ let t7887 = v_a0.clone(); if py::eq(&t7886, &t7887) { true } else { false } }) }; if t7890.truthy() { t7890 } else { { let t7888 = v_m.clone(); Py::Bool({ let t7889 = v_a1.clone(); if py::eq(&t7888, &t7889) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_gm = py::call(&v_grad.clone(), &[v_m.clone()])?;
        if ({ let t7891 = v_gm.clone(); Py::Bool({ let t7892 = Py::Int(0i64); if py::lt(&t7891, &t7892)? { true } else { false } }) }).truthy() {
            v_a0 = v_m.clone();
        } else {
            v_a1 = v_m.clone();
        }
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_H = { let mut out = vec![]; for t7893 in py::iter(&(v_h.clone()))? { let mut v_t: Py; 
v_t = t7893;
out.push(ax_engine::pymp::call("mpf", &[Py::str(&py::repr(&py::float(&v_t.clone())?))], &[])?); } Py::list(out) };
        v_G = { let mut out = vec![]; for t7894 in py::iter(&(v_gamma.clone()))? { let mut v_v: Py; 
v_v = t7894;
out.push(ax_engine::pymp::call("mpf", &[Py::str(&py::repr(&py::float(&v_v.clone())?))], &[])?); } Py::list(out) };
        v_q = py::truediv(&ax_engine::pymp::call("mpf", &[Py::Int(4i64)], &[])?, &Py::Int(7i64))?;
        v_shape = { let v_G = v_G.clone(); let v_H = v_H.clone(); let v__ = v__.clone(); let v_a0 = v_a0.clone(); let v_a1 = v_a1.clone(); let v_g0 = v_g0.clone(); let v_g1 = v_g1.clone(); let v_gamma = v_gamma.clone(); let v_gm = v_gm.clone(); let v_grad = v_grad.clone(); let v_grid = v_grid.clone(); let v_h = v_h.clone(); let v_hi = v_hi.clone(); let v_i = v_i.clone(); let v_linfit = v_linfit.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_q = v_q.clone(); let v_shape = v_shape.clone(); let v_sse = v_sse.clone(); let v_vals = v_vals.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t7895 = v_model.clone(); Py::Bool({ let t7896 = Py::str("exponential"); if py::eq(&t7895, &t7896) { true } else { false } }) }).truthy() { py::sub(&Py::Int(1i64), &ax_engine::pymp::call("exp", &[py::truediv(&py::mul(&py::neg(&Py::Int(3i64))?, &v_t.clone())?, &v_a.clone())?], &[])?)? } else { py::sub(&Py::Int(1i64), &ax_engine::pymp::call("exp", &[py::neg(&py::pow(&py::truediv(&v_t.clone(), &py::mul(&v_a.clone(), &v_q.clone())?)?, &Py::Int(2i64))?)?], &[])?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_dshape = { let v_G = v_G.clone(); let v_H = v_H.clone(); let v__ = v__.clone(); let v_a0 = v_a0.clone(); let v_a1 = v_a1.clone(); let v_dshape = v_dshape.clone(); let v_g0 = v_g0.clone(); let v_g1 = v_g1.clone(); let v_gamma = v_gamma.clone(); let v_gm = v_gm.clone(); let v_grad = v_grad.clone(); let v_grid = v_grid.clone(); let v_h = v_h.clone(); let v_hi = v_hi.clone(); let v_i = v_i.clone(); let v_linfit = v_linfit.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_q = v_q.clone(); let v_shape = v_shape.clone(); let v_sse = v_sse.clone(); let v_vals = v_vals.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t7897 = v_model.clone(); Py::Bool({ let t7898 = Py::str("exponential"); if py::eq(&t7897, &t7898) { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&py::neg(&ax_engine::pymp::call("exp", &[py::truediv(&py::mul(&py::neg(&Py::Int(3i64))?, &v_t.clone())?, &v_a.clone())?], &[])?)?, &Py::Int(3i64))?, &v_t.clone())?, &py::pow(&v_a.clone(), &Py::Int(2i64))?)? } else { py::truediv(&py::mul(&py::mul(&py::mul(&py::neg(&ax_engine::pymp::call("exp", &[py::neg(&py::pow(&py::truediv(&v_t.clone(), &py::mul(&v_a.clone(), &v_q.clone())?)?, &Py::Int(2i64))?)?], &[])?)?, &Py::Int(2i64))?, &v_t.clone())?, &v_t.clone())?, &py::mul(&py::mul(&v_q.clone(), &v_q.clone())?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_lin = { let v_G = v_G.clone(); let v_H = v_H.clone(); let v__ = v__.clone(); let v_a0 = v_a0.clone(); let v_a1 = v_a1.clone(); let v_dshape = v_dshape.clone(); let v_g0 = v_g0.clone(); let v_g1 = v_g1.clone(); let v_gamma = v_gamma.clone(); let v_gm = v_gm.clone(); let v_grad = v_grad.clone(); let v_grid = v_grid.clone(); let v_h = v_h.clone(); let v_hi = v_hi.clone(); let v_i = v_i.clone(); let v_lin = v_lin.clone(); let v_linfit = v_linfit.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_q = v_q.clone(); let v_shape = v_shape.clone(); let v_sse = v_sse.clone(); let v_vals = v_vals.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_F: Py = Py::None;
            let mut v_cc: Py = Py::None;
            let mut v_mf: Py = Py::None;
            let mut v_mg: Py = Py::None;
            v_F = { let mut out = vec![]; for t7899 in py::iter(&(v_H.clone()))? { let mut v_t: Py; 
v_t = t7899;
out.push(py::call(&v_shape.clone(), &[v_t.clone(), v_a.clone()])?); } Py::list(out) };
            if (v_zero_nugget.clone()).truthy() {
                return Ok(Py::tuple(vec![ax_engine::pymp::call("mpf", &[Py::Int(0i64)], &[])?, py::truediv(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t7900 in py::iter(&(py::zip(&[v_F.clone(), v_G.clone()])?))? { let mut v_fv: Py; let mut v_g: Py; 
let t7901 = (t7900).items()?;
if t7901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_fv = t7901[0].clone();
v_g = t7901[1].clone();
out.push(py::mul(&v_fv.clone(), &v_g.clone())?); } Py::list(out) }], &[])?, &ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t7902 in py::iter(&(v_F.clone()))? { let mut v_fv: Py; 
v_fv = t7902;
out.push(py::mul(&v_fv.clone(), &v_fv.clone())?); } Py::list(out) }], &[])?)?, v_F.clone()]));
            }
            let t7903 = (Py::tuple(vec![py::truediv(&ax_engine::pymp::call("fsum", &[v_F.clone()], &[])?, &v_n.clone())?, py::truediv(&ax_engine::pymp::call("fsum", &[v_G.clone()], &[])?, &v_n.clone())?])).items()?;
            if t7903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_mf = t7903[0].clone();
            v_mg = t7903[1].clone();
            v_cc = py::truediv(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t7904 in py::iter(&(py::zip(&[v_F.clone(), v_G.clone()])?))? { let mut v_fv: Py; let mut v_g: Py; 
let t7905 = (t7904).items()?;
if t7905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_fv = t7905[0].clone();
v_g = t7905[1].clone();
out.push(py::mul(&py::sub(&v_fv.clone(), &v_mf.clone())?, &py::sub(&v_g.clone(), &v_mg.clone())?)?); } Py::list(out) }], &[])?, &ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t7906 in py::iter(&(v_F.clone()))? { let mut v_fv: Py; 
v_fv = t7906;
out.push(py::pow(&py::sub(&v_fv.clone(), &v_mf.clone())?, &Py::Int(2i64))?); } Py::list(out) }], &[])?)?;
            return Ok(Py::tuple(vec![py::sub(&v_mg.clone(), &py::mul(&v_cc.clone(), &v_mf.clone())?)?, v_cc.clone(), v_F.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_gmp = { let v_G = v_G.clone(); let v_H = v_H.clone(); let v__ = v__.clone(); let v_a0 = v_a0.clone(); let v_a1 = v_a1.clone(); let v_dshape = v_dshape.clone(); let v_g0 = v_g0.clone(); let v_g1 = v_g1.clone(); let v_gamma = v_gamma.clone(); let v_gm = v_gm.clone(); let v_gmp = v_gmp.clone(); let v_grad = v_grad.clone(); let v_grid = v_grid.clone(); let v_h = v_h.clone(); let v_hi = v_hi.clone(); let v_i = v_i.clone(); let v_lin = v_lin.clone(); let v_linfit = v_linfit.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_q = v_q.clone(); let v_shape = v_shape.clone(); let v_sse = v_sse.clone(); let v_vals = v_vals.clone(); let v_zero_nugget = v_zero_nugget.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_F: Py = Py::None;
            let mut v_c0m: Py = Py::None;
            let mut v_cm: Py = Py::None;
            let t7907 = (py::call(&v_lin.clone(), &[v_a.clone()])?).items()?;
            if t7907.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_c0m = t7907[0].clone();
            v_cm = t7907[1].clone();
            v_F = t7907[2].clone();
            return Ok(py::mul(&py::neg(&Py::Int(2i64))?, &ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t7908 in py::iter(&(py::zip(&[v_G.clone(), v_F.clone(), v_H.clone()])?))? { let mut v_fv: Py; let mut v_g: Py; let mut v_t: Py; 
let t7909 = (t7908).items()?;
if t7909.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_g = t7909[0].clone();
v_fv = t7909[1].clone();
v_t = t7909[2].clone();
out.push(py::mul(&py::mul(&py::sub(&py::sub(&v_g.clone(), &v_c0m.clone())?, &py::mul(&v_cm.clone(), &v_fv.clone())?)?, &v_cm.clone())?, &py::call(&v_dshape.clone(), &[v_t.clone(), v_a.clone()])?)?); } Py::list(out) }], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        let t7910 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[Py::str(&py::repr(&v_a0.clone()))], &[])?, ax_engine::pymp::call("mpf", &[Py::str(&py::repr(&v_a1.clone()))], &[])?])).items()?;
        if t7910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo_ = t7910[0].clone();
        v_hi_ = t7910[1].clone();
        if ({ let t7911 = py::call(&v_gmp.clone(), &[v_lo_.clone()])?; Py::Bool({ let t7912 = Py::Int(0i64); if py::lt(&t7911, &t7912)? { { let t7913 = py::call(&v_gmp.clone(), &[v_hi_.clone()])?; if py::lt(&t7912, &t7913)? { true } else { false } } } else { false } }) }).truthy() {
            for t7914 in py::range_iter(&[(Py::Int(80i64)).i()?]) {
                v__ = t7914;
                v_mid = py::truediv(&py::add(&v_lo_.clone(), &v_hi_.clone())?, &Py::Int(2i64))?;
                if ({ let t7915 = py::call(&v_gmp.clone(), &[v_mid.clone()])?; Py::Bool({ let t7916 = Py::Int(0i64); if py::lt(&t7915, &t7916)? { true } else { false } }) }).truthy() {
                    v_lo_ = v_mid.clone();
                } else {
                    v_hi_ = v_mid.clone();
                }
            }
        }
        v_am = py::truediv(&py::add(&v_lo_.clone(), &v_hi_.clone())?, &Py::Int(2i64))?;
        let t7917 = (py::call(&v_lin.clone(), &[v_am.clone()])?).items()?;
        if t7917.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_c0m = t7917[0].clone();
        v_cm = t7917[1].clone();
        v__ = t7917[2].clone();
        return Ok(f__out(Py::dict(vec![(Py::str("nugget"), py::float(&v_c0m.clone())?), (Py::str("psill"), py::float(&v_cm.clone())?), (Py::str("range"), py::float(&v_am.clone())?), (Py::str("sill"), py::float(&py::add(&v_c0m.clone(), &v_cm.clone())?)?)]))?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vmodel(mut v_model: Py, mut v_h: Py, mut v_a: Py) -> R {
    if ({ let t7844 = v_model.clone(); Py::Bool({ let t7845 = Py::str("exponential"); if py::eq(&t7844, &t7845) { true } else { false } }) }).truthy() {
        return Ok(py::sub(&Py::Int(1i64), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&Py::Int(3i64))?, &v_h.clone())?, &v_a.clone())?)?)?);
    }
    if ({ let t7846 = v_model.clone(); Py::Bool({ let t7847 = Py::str("gaussian"); if py::eq(&t7846, &t7847) { true } else { false } }) }).truthy() {
        return Ok(py::sub(&Py::Int(1i64), &py::math1("exp", &py::neg(&py::pow(&py::truediv(&v_h.clone(), &py::truediv(&py::mul(&v_a.clone(), &Py::Float(4.0))?, &Py::Float(7.0))?)?, &Py::Int(2i64))?)?)?)?);
    }
    return Ok((if ({ let t7848 = v_h.clone(); Py::Bool({ let t7849 = v_a.clone(); if py::le(&t7848, &t7849)? { true } else { false } }) }).truthy() { py::sub(&py::truediv(&py::mul(&Py::Float(1.5), &v_h.clone())?, &v_a.clone())?, &py::mul(&Py::Float(0.5), &py::pow(&py::truediv(&v_h.clone(), &v_a.clone())?, &Py::Int(3i64))?)?)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__weights(mut v_W: Py, mut v_n: Py, mut v_transform: Py, mut v_binary: Py, mut v_symmetric: Py, mut v_islands_ok: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let _ = f__need({ let t6773 = { let t6768 = py::len(&v_W.clone())?; Py::Bool({ let t6769 = v_n.clone(); if py::eq(&t6768, &t6769) { true } else { false } }) }; if !t6773.truthy() { t6773 } else { Py::Bool(({ let mut out = vec![]; for t6770 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t6770;
out.push({ let t6771 = py::len(&v_r.clone())?; Py::Bool({ let t6772 = v_n.clone(); if py::eq(&t6771, &t6772) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str(&["the weights matrix must be ".to_string(), py::format(&v_n.clone(), "")?, " x ".to_string(), py::format(&v_n.clone(), "")?, " (one row and column per observation)".to_string()].concat()))?;
    for t6774 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t6774;
        let _ = f__need({ let t6775 = py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_i.clone())?; Py::Bool({ let t6776 = Py::Int(0i64); if py::eq(&t6775, &t6776) { true } else { false } }) }, Py::str("the weights matrix must have a zero diagonal"))?;
        for t6777 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t6777;
            let _ = f__fin(Py::str("each weight"), Py::tuple(vec![py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone())?]))?;
            let _ = f__need({ let t6778 = py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t6779 = Py::Int(0i64); if py::ge(&t6778, &t6779)? { true } else { false } }) }, Py::str("weights must be zero or positive"))?;
            if (v_binary.clone()).truthy() {
                let _ = f__need({ let t6780 = py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t6781 = Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]); if py::contains(&t6781, &t6780)? { true } else { false } }) }, Py::str("the weights must be binary (0 or 1)"))?;
            }
            if (v_symmetric.clone()).truthy() {
                let _ = f__need({ let t6782 = py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t6783 = py::getitem(&py::getitem(&v_W.clone(), &v_j.clone())?, &v_i.clone())?; if py::eq(&t6782, &t6783) { true } else { false } }) }, Py::str("the weights matrix must be symmetric"))?;
            }
        }
    }
    v_rows = { let mut out = vec![]; for t6784 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t6784;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) };
    if (py::not(&v_islands_ok.clone())).truthy() {
        let _ = f__need(Py::Bool(({ let mut out = vec![]; for t6785 in py::iter(&(v_rows.clone()))? { let mut v_s: Py; 
v_s = t6785;
out.push({ let t6786 = v_s.clone(); Py::Bool({ let t6787 = Py::Int(0i64); if py::gt(&t6786, &t6787)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("every observation needs at least one neighbour (no islands)"))?;
    }
    if ({ let t6788 = v_transform.clone(); Py::Bool({ let t6789 = Py::str("row"); if py::eq(&t6788, &t6789) { true } else { false } }) }).truthy() {
        return Ok({ let mut out = vec![]; for t6790 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6790;
out.push({ let mut out = vec![]; for t6791 in py::iter(&(py::getitem(&v_W.clone(), &v_i.clone())?))? { let mut v_w: Py; 
v_w = t6791;
out.push(py::truediv(&v_w.clone(), &py::getitem(&v_rows.clone(), &v_i.clone())?)?); } Py::list(out) }); } Py::list(out) });
    }
    return Ok({ let mut out = vec![]; for t6792 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t6792;
out.push({ let mut out = vec![]; for t6793 in py::iter(&(v_r.clone()))? { let mut v_w: Py; 
v_w = t6793;
out.push(py::float(&v_w.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__window(mut v_w: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t6999 = { let t6994 = py::len(&v_w.clone())?; Py::Bool({ let t6995 = Py::Int(3i64); if py::eq(&t6994, &t6995) { true } else { false } }) }; if !t6999.truthy() { t6999 } else { Py::Bool(({ let mut out = vec![]; for t6996 in py::iter(&(v_w.clone()))? { let mut v_r: Py; 
v_r = t6996;
out.push({ let t6997 = py::len(&v_r.clone())?; Py::Bool({ let t6998 = Py::Int(3i64); if py::eq(&t6997, &t6998) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the elevation window must be 3 x 3 (row 0 to the north)"))?;
    for t7000 in py::iter(&(v_w.clone()))? {
        v_r = t7000;
        for t7001 in py::iter(&(v_r.clone()))? {
            v_v = t7001;
            let _ = f__fin(Py::str("each elevation"), Py::tuple(vec![v_v.clone()]))?;
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__y_ok(mut v_y: Py, mut v_lo: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t4936 = py::len(&v_y.clone())?; Py::Bool({ let t4937 = v_lo.clone(); if py::ge(&t4936, &t4937)? { true } else { false } }) }, Py::str(&["give at least ".to_string(), py::format(&v_lo.clone(), "")?, " observations".to_string()].concat()))?;
    for t4938 in py::iter(&(v_y.clone()))? {
        v_v = t4938;
        let _ = f__fin(Py::str("each value"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_m = py::truediv(&py::fsum(&v_y.clone())?, &py::len(&v_y.clone())?)?;
    let _ = f__need({ let t4942 = py::fsum(&{ let mut out = vec![]; for t4944 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t4944;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) })?; Py::Bool({ let t4943 = Py::Int(0i64); if py::gt(&t4942, &t4943)? { true } else { false } }) }, Py::str("the values must not all be equal"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_aeqd_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_inv: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let _ = f__two_points(v_lat0.clone(), v_lon0.clone(), v_lat.clone(), v_lon.clone())?;
    let _ = f__need({ let t264 = py::abs(&v_lat0.clone())?; Py::Bool({ let t265 = Py::Int(89i64); if py::le(&t264, &t265)? { true } else { false } }) }, Py::str("the centre must lie within 89 degrees of the equator"))?;
    v_inv = f__geod_inverse(v_E.clone(), v_lat0.clone(), v_lon0.clone(), v_lat.clone(), v_lon.clone())?;
    v_a = py::mul(&py::getitem(&v_inv.clone(), &Py::str("az1"))?, &c_DEG()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::add(&v_FE.clone(), &py::mul(&py::getitem(&v_inv.clone(), &Py::str("s"))?, &py::math1("sin", &v_a.clone())?)?)?), (Py::str("y"), py::add(&v_FN.clone(), &py::mul(&py::getitem(&v_inv.clone(), &Py::str("s"))?, &py::math1("cos", &v_a.clone())?)?)?), (Py::str("distance"), py::getitem(&v_inv.clone(), &Py::str("s"))?), (Py::str("azimuth"), py::getitem(&v_inv.clone(), &Py::str("az1"))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_albers_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat1: Py, mut v_lat2: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Int(89i64))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let t27 = (f__aea_setup(v_E.clone(), v_lat1.clone(), v_lat2.clone(), v_lat0.clone())?).items()?;
    if t27.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_n = t27[0].clone();
    v_C = t27[1].clone();
    v_rho = t27[2].clone();
    v_m = t27[3].clone();
    v_q = t27[4].clone();
    let t28 = (Py::tuple(vec![py::call(&v_rho.clone(), &[py::mul(&v_lat.clone(), &c_DEG()?)?])?, py::call(&v_rho.clone(), &[py::mul(&v_lat0.clone(), &c_DEG()?)?])?])).items()?;
    if t28.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t28[0].clone();
    v_r0 = t28[1].clone();
    v_th = py::mul(&py::mul(&v_n.clone(), &f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?)?, &c_DEG()?)?;
    v_k = py::truediv(&py::mul(&v_r.clone(), &v_n.clone())?, &py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::call(&v_m.clone(), &[py::mul(&v_lat.clone(), &c_DEG()?)?])?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::add(&v_FE.clone(), &py::mul(&v_r.clone(), &py::math1("sin", &v_th.clone())?)?)?), (Py::str("y"), py::sub(&py::add(&v_FN.clone(), &v_r0.clone())?, &py::mul(&v_r.clone(), &py::math1("cos", &v_th.clone())?)?)?), (Py::str("k"), v_k.clone()), (Py::str("h"), py::truediv(&Py::Int(1i64), &v_k.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_albers_inverse(mut v_x: Py, mut v_y: Py, mut v_lat1: Py, mut v_lat2: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qp: Py = Py::None;
    let mut v_qv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_xp: Py = Py::None;
    let mut v_yp: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__fin(Py::str("x and y"), Py::tuple(vec![v_x.clone(), v_y.clone()]))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let t110 = (f__aea_setup(v_E.clone(), v_lat1.clone(), v_lat2.clone(), v_lat0.clone())?).items()?;
    if t110.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_n = t110[0].clone();
    v_C = t110[1].clone();
    v_rho = t110[2].clone();
    v_m = t110[3].clone();
    v_q = t110[4].clone();
    v_r0 = py::call(&v_rho.clone(), &[py::mul(&v_lat0.clone(), &c_DEG()?)?])?;
    let t111 = (Py::tuple(vec![py::sub(&v_x.clone(), &v_FE.clone())?, py::sub(&v_r0.clone(), &py::sub(&v_y.clone(), &v_FN.clone())?)?])).items()?;
    if t111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xp = t111[0].clone();
    v_yp = t111[1].clone();
    v_sg = (if ({ let t112 = v_n.clone(); Py::Bool({ let t113 = Py::Int(0i64); if py::gt(&t112, &t113)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    v_r = py::hypot(&[v_xp.clone(), v_yp.clone()])?;
    v_th = py::atan2(&py::mul(&v_sg.clone(), &v_xp.clone())?, &py::mul(&v_sg.clone(), &v_yp.clone())?)?;
    v_qv = py::truediv(&py::sub(&v_C.clone(), &py::pow(&py::truediv(&py::mul(&v_r.clone(), &v_n.clone())?, &py::getitem(&v_E.clone(), &Py::str("a"))?)?, &Py::Int(2i64))?)?, &v_n.clone())?;
    v_qp = f__q_auth(v_E.clone(), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?;
    let _ = f__need({ let t114 = py::abs(&v_qv.clone())?; Py::Bool({ let t115 = py::mul(&v_qp.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::lt(&t114, &t115)? { true } else { false } }) }, Py::str("the point lies outside the projected area"))?;
    v_phi = py::math1("asin", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&v_qv.clone(), &Py::Int(2i64))?], false)?], true)?)?;
    for t116 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
        v__ = t116;
        v_s = py::math1("sin", &v_phi.clone())?;
        v_d = py::truediv(&py::mul(&py::sub(&f__q_auth(v_E.clone(), v_phi.clone())?, &v_qv.clone())?, &py::pow(&py::sub(&Py::Int(1i64), &py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &v_s.clone())?, &v_s.clone())?)?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("e2"))?)?)?, &py::math1("cos", &v_phi.clone())?)?)?;
        v_phi = py::sub(&v_phi, &v_d.clone())?;
        if ({ let t117 = py::abs(&v_d.clone())?; Py::Bool({ let t118 = Py::Float(1e-16); if py::lt(&t117, &t118)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    let _ = f__need({ let t119 = py::abs(&py::truediv(&v_phi.clone(), &c_DEG()?)?)?; Py::Bool({ let t120 = Py::Int(89i64); if py::le(&t119, &t120)? { true } else { false } }) }, Py::str("the point lies too close to a pole"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("lat"), py::truediv(&v_phi.clone(), &c_DEG()?)?), (Py::str("lon"), f__nlon(py::add(&v_lon0.clone(), &py::truediv(&py::truediv(&v_th.clone(), &v_n.clone())?, &c_DEG()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_auxiliary_latitudes(mut v_lat: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_auth: Py = Py::None;
    let mut v_conf: Py = Py::None;
    let mut v_geoc: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rect: Py = Py::None;
    let mut v_red: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(false))?;
    v_p = py::mul(&v_lat.clone(), &c_DEG()?)?;
    v_geoc = py::atan2(&py::mul(&py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("e2"))?)?, &py::math1("sin", &v_p.clone())?)?, &py::math1("cos", &v_p.clone())?)?;
    v_red = py::atan2(&py::mul(&py::sub(&Py::Int(1i64), &py::getitem(&v_E.clone(), &Py::str("f"))?)?, &py::math1("sin", &v_p.clone())?)?, &py::math1("cos", &v_p.clone())?)?;
    v_conf = py::math1("atan", &f__taup(v_E.clone(), py::math1("tan", &v_p.clone())?)?)?;
    v_auth = py::math1("asin", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&f__q_auth(v_E.clone(), v_p.clone())?, &f__q_auth(v_E.clone(), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?)?], false)?], true)?)?;
    v_rect = py::truediv(&py::mul(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &f__merid(v_E.clone(), v_p.clone())?)?, &f__merid(v_E.clone(), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("geocentric"), py::truediv(&v_geoc.clone(), &c_DEG()?)?), (Py::str("reduced"), py::truediv(&v_red.clone(), &c_DEG()?)?), (Py::str("conformal"), py::truediv(&v_conf.clone(), &c_DEG()?)?), (Py::str("authalic"), py::truediv(&v_auth.clone(), &c_DEG()?)?), (Py::str("rectifying"), py::truediv(&v_rect.clone(), &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bivariate_morans_i(mut v_x: Py, mut v_y: Py, mut v_W: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_Wt: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_zx: Py = Py::None;
    let mut v_zy: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__need({ let t347 = py::len(&v_x.clone())?; Py::Bool({ let t348 = v_n.clone(); if py::eq(&t347, &t348) { true } else { false } }) }, Py::str("give one x per y"))?;
    let _ = f__y_ok(v_y.clone(), Py::Int(4i64))?;
    let _ = f__y_ok(v_x.clone(), Py::Int(4i64))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), Py::str("row"), Py::Bool(false), Py::Bool(false), Py::Bool(false))?;
    let t384 = (Py::tuple(vec![py::truediv(&py::fsum(&v_x.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?])).items()?;
    if t384.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t384[0].clone();
    v_my = t384[1].clone();
    v_sx = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t385 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t385;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    v_sy = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t386 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t386;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    v_zx = { let mut out = vec![]; for t387 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t387;
out.push(py::truediv(&py::sub(&v_v.clone(), &v_mx.clone())?, &v_sx.clone())?); } Py::list(out) };
    v_zy = { let mut out = vec![]; for t388 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t388;
out.push(py::truediv(&py::sub(&v_v.clone(), &v_my.clone())?, &v_sy.clone())?); } Py::list(out) };
    v_I = py::truediv(&py::fsum(&{ let mut out = vec![]; for t389 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t389;
for t390 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t390;
out.push(py::mul(&py::mul(&py::getitem(&v_zx.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_zy.clone(), &v_j.clone())?)?); }} Py::list(out) })?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("I_xy"), v_I.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_brightness_temperature(mut v_radiance: Py, mut v_K1: Py, mut v_K2: Py) -> R {
    let _ = f__pos(Py::str("the radiance"), Py::tuple(vec![v_radiance.clone()]))?;
    let _ = f__pos(Py::str("K1 and K2"), Py::tuple(vec![v_K1.clone(), v_K2.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), py::truediv(&v_K2.clone(), &py::math1("log", &py::add(&py::truediv(&v_K1.clone(), &v_radiance.clone())?, &Py::Int(1i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cea_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat_ts: Py, mut v_lon0: Py, mut v_R: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Int(90i64))?;
    let _ = f__lat(Py::str("lat_ts"), v_lat_ts.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__sphere_R(v_R.clone())?;
    v_c = py::math1("cos", &py::mul(&v_lat_ts.clone(), &c_DEG()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::mul(&py::mul(&py::mul(&v_R.clone(), &f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?)?, &c_DEG()?)?, &v_c.clone())?), (Py::str("y"), py::truediv(&py::mul(&v_R.clone(), &py::math1("sin", &py::mul(&v_lat.clone(), &c_DEG()?)?)?)?, &v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_classification_accuracy(mut v_matrix: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_pe: Py = Py::None;
    let mut v_po: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_k = py::len(&v_matrix.clone())?;
    let _ = f__need({ let t508 = { let t503 = v_k.clone(); Py::Bool({ let t504 = Py::Int(2i64); if py::ge(&t503, &t504)? { true } else { false } }) }; if !t508.truthy() { t508 } else { Py::Bool(({ let mut out = vec![]; for t505 in py::iter(&(v_matrix.clone()))? { let mut v_r: Py; 
v_r = t505;
out.push({ let t506 = py::len(&v_r.clone())?; Py::Bool({ let t507 = v_k.clone(); if py::eq(&t506, &t507) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the confusion matrix must be square with at least 2 classes"))?;
    for t509 in py::iter(&(v_matrix.clone()))? {
        v_r = t509;
        for t510 in py::iter(&(v_r.clone()))? {
            v_v = t510;
            let _ = f__need({ let t515 = { let t511 = v_v.clone(); Py::Bool({ let t512 = Py::Int(0i64); if py::ge(&t511, &t512)? { true } else { false } }) }; if !t515.truthy() { t515 } else { { let t513 = py::float(&v_v.clone())?; Py::Bool({ let t514 = py::int(&v_v.clone())?; if py::eq(&t513, &t514) { true } else { false } }) } } }, Py::str("the matrix entries must be whole counts"))?;
        }
    }
    v_N = py::fsum(&{ let mut out = vec![]; for t516 in py::iter(&(v_matrix.clone()))? { let mut v_r: Py; 
v_r = t516;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) })?;
    let _ = f__need({ let t517 = v_N.clone(); Py::Bool({ let t518 = Py::Int(0i64); if py::gt(&t517, &t518)? { true } else { false } }) }, Py::str("the matrix must contain samples"))?;
    v_rows = { let mut out = vec![]; for t519 in py::iter(&(v_matrix.clone()))? { let mut v_r: Py; 
v_r = t519;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) };
    v_cols = { let mut out = vec![]; for t520 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t520;
out.push(py::fsum(&{ let mut out = vec![]; for t521 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t521;
out.push(py::getitem(&py::getitem(&v_matrix.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) })?); } Py::list(out) };
    let _ = f__need({ let t528 = Py::Bool(({ let mut out = vec![]; for t522 in py::iter(&(v_cols.clone()))? { let mut v_c: Py; 
v_c = t522;
out.push({ let t523 = v_c.clone(); Py::Bool({ let t524 = Py::Int(0i64); if py::gt(&t523, &t524)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t528.truthy() { t528 } else { Py::Bool(({ let mut out = vec![]; for t525 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t525;
out.push({ let t526 = v_r.clone(); Py::Bool({ let t527 = Py::Int(0i64); if py::gt(&t526, &t527)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("every class needs reference and map samples"))?;
    v_po = py::truediv(&py::fsum(&{ let mut out = vec![]; for t529 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t529;
out.push(py::getitem(&py::getitem(&v_matrix.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) })?, &v_N.clone())?;
    v_pe = py::truediv(&py::fsum(&{ let mut out = vec![]; for t530 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t530;
out.push(py::mul(&py::getitem(&v_rows.clone(), &v_i.clone())?, &py::getitem(&v_cols.clone(), &v_i.clone())?)?); } Py::list(out) })?, &py::mul(&v_N.clone(), &v_N.clone())?)?;
    let _ = f__need({ let t531 = v_pe.clone(); Py::Bool({ let t532 = Py::Int(1i64); if py::lt(&t531, &t532)? { true } else { false } }) }, Py::str("kappa is undefined when chance agreement is 1"))?;
    return Ok(Py::dict(vec![(Py::str("overall"), f__r(v_po.clone())?), (Py::str("kappa"), f__r(py::truediv(&py::sub(&v_po.clone(), &v_pe.clone())?, &py::sub(&Py::Int(1i64), &v_pe.clone())?)?)?), (Py::str("producers"), f__r({ let mut out = vec![]; for t543 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t543;
out.push(py::truediv(&py::getitem(&py::getitem(&v_matrix.clone(), &v_i.clone())?, &v_i.clone())?, &py::getitem(&v_cols.clone(), &v_i.clone())?)?); } Py::list(out) })?), (Py::str("users"), f__r({ let mut out = vec![]; for t544 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t544;
out.push(py::truediv(&py::getitem(&py::getitem(&v_matrix.clone(), &v_i.clone())?, &v_i.clone())?, &py::getitem(&v_rows.clone(), &v_i.clone())?)?); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_closest_facility(mut v_edges: Py, mut v_facilities: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_near: Py = Py::None;
    let mut v_others: Py = Py::None;
    v_adj = f__net(v_edges.clone(), Py::None, Py::Bool(true))?;
    v_n = py::len(&v_adj.clone())?;
    v_F = { let mut out = vec![]; for t612 in py::iter(&(v_facilities.clone()))? { let mut v_f: Py; 
v_f = t612;
out.push(py::int(&v_f.clone())?); } Py::list(out) };
    let _ = f__need({ let t623 = { let t613 = py::len(&v_F.clone())?; Py::Bool({ let t614 = Py::Int(1i64); if py::ge(&t613, &t614)? { true } else { false } }) }; if !t623.truthy() { t623 } else { Py::Bool(({ let mut out = vec![]; for t615 in py::iter(&(py::zip(&[v_facilities.clone(), v_F.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t616 = (t615).items()?;
if t616.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t616[0].clone();
v_b = t616[1].clone();
out.push({ let t622 = { let t617 = py::float(&v_a.clone())?; Py::Bool({ let t618 = v_b.clone(); if py::eq(&t617, &t618) { true } else { false } }) }; if !t622.truthy() { t622 } else { { let t619 = Py::Int(0i64); Py::Bool({ let t620 = v_b.clone(); if py::le(&t619, &t620)? { { let t621 = v_n.clone(); if py::lt(&t620, &t621)? { true } else { false } } } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the facilities must be nodes of the network"))?;
    let _ = f__need({ let t624 = py::len(&py::set(&v_F.clone())?)?; Py::Bool({ let t625 = py::len(&v_F.clone())?; if py::eq(&t624, &t625) { true } else { false } }) }, Py::str("each facility may be listed once"))?;
    v_D = { let mut out = vec![]; for t626 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t626;
out.push(f__dijkstra(v_adj.clone(), v_f.clone())?); } Py::list(out) };
    let t634 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t634.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_near = t634[0].clone();
    v_dist = t634[1].clone();
    for t635 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t635;
        v_best = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_F.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_D = v_D.clone(); let v_F = v_F.clone(); let v_adj = v_adj.clone(); let v_dist = v_dist.clone(); let v_edges = v_edges.clone(); let v_facilities = v_facilities.clone(); let v_i = v_i.clone(); let v_n = v_n.clone(); let v_near = v_near.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&py::getitem(&v_D.clone(), &v_k.clone())?, &v_i.clone())?, py::getitem(&v_F.clone(), &v_k.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
        let _ = f__need(py::math1("isfinite", &py::getitem(&py::getitem(&v_D.clone(), &v_best.clone())?, &v_i.clone())?)?, Py::str("every node must be able to reach a facility"))?;
        v_others = py::sorted(&{ let mut out = vec![]; for t636 in py::range_iter(&[(py::len(&v_F.clone())?).i()?]) { let mut v_k: Py; 
v_k = t636;
out.push(py::getitem(&py::getitem(&v_D.clone(), &v_k.clone())?, &v_i.clone())?); } Py::list(out) }, (Py::Bool(false)).truthy())?;
        let _ = f__need({ let t644 = { let t637 = py::len(&v_F.clone())?; Py::Bool({ let t638 = Py::Int(1i64); if py::eq(&t637, &t638) { true } else { false } }) }; if t644.truthy() { t644 } else { { let t643 = { let t639 = py::getitem(&v_others.clone(), &Py::Int(1i64))?; Py::Bool({ let t640 = py::getitem(&v_others.clone(), &Py::Int(0i64))?; if py::gt(&t639, &t640)? { true } else { false } }) }; if t643.truthy() { t643 } else { { let t641 = py::getitem(&py::getitem(&v_D.clone(), &v_best.clone())?, &v_i.clone())?; Py::Bool({ let t642 = Py::Int(0i64); if py::eq(&t641, &t642) { true } else { false } }) } } } } }, Py::str("two facilities are equally near a node: the nearest is not unique"))?;
        let _ = py::method(&v_near.clone(), "append", &[py::float(&py::getitem(&v_F.clone(), &v_best.clone())?)?])?;
        let _ = py::method(&v_dist.clone(), "append", &[py::getitem(&py::getitem(&v_D.clone(), &v_best.clone())?, &v_i.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("nearest"), v_near.clone()), (Py::str("distance"), f__r(v_dist.clone())?), (Py::str("mean_distance"), f__r(py::truediv(&py::fsum(&v_dist.clone())?, &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compactness(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t5656 = v_n.clone(); Py::Bool({ let t5657 = py::len(&v_y.clone())?; if py::eq(&t5656, &t5657) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t5658 = Py::Int(3i64); Py::Bool({ let t5659 = v_n.clone(); if py::le(&t5658, &t5659)? { { let t5660 = Py::Int(200i64); if py::le(&t5659, &t5660)? { true } else { false } } } else { false } }) }, Py::str("a polygon needs 3 to 200 vertices"))?;
    v_P = Py::list((py::zip(&[v_x.clone(), v_y.clone()])?).items()?);
    let _ = f__need({ let t5661 = py::len(&py::set(&v_P.clone())?)?; Py::Bool({ let t5662 = v_n.clone(); if py::eq(&t5661, &t5662) { true } else { false } }) }, Py::str("the vertices must be distinct (do not repeat the first vertex)"))?;
    let _ = f__need(f__polygon_simple(v_x.clone(), v_y.clone())?, Py::str("the polygon must be simple (edges must not cross)"))?;
    v_A = py::abs(&f__shoelace(v_P.clone())?)?;
    let _ = f__need({ let t5694 = v_A.clone(); Py::Bool({ let t5695 = Py::Int(0i64); if py::gt(&t5694, &t5695)? { true } else { false } }) }, Py::str("the polygon has no area"))?;
    v_L = py::fsum(&{ let mut out = vec![]; for t5696 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5696;
out.push(py::hypot(&[py::sub(&py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &Py::Int(1i64))?)?])?); } Py::list(out) })?;
    v_H = py::abs(&f__shoelace(f__hull(v_P.clone())?)?)?;
    v_r = py::getitem(&f__mec(v_P.clone())?, &Py::Int(2i64))?;
    v_pp = py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_A.clone())?, &py::mul(&v_L.clone(), &v_L.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("area"), v_A.clone()), (Py::str("perimeter"), v_L.clone()), (Py::str("polsby_popper"), v_pp.clone()), (Py::str("schwartzberg"), py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_pp.clone())?)?), (Py::str("convex_hull_ratio"), py::truediv(&v_A.clone(), &v_H.clone())?), (Py::str("reock"), py::truediv(&v_A.clone(), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_r.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_curvature(mut v_window: Py, mut v_cell_size: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_gg: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__window(v_window.clone())?;
    let _ = f__pos(Py::str("the cell size"), Py::tuple(vec![v_cell_size.clone()]))?;
    v_z = { let mut out = vec![]; for t6937 in py::iter(&(v_window.clone()))? { let mut v_r: Py; 
v_r = t6937;
for t6938 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t6938;
out.push(v_v.clone()); }} Py::list(out) };
    v_L = v_cell_size.clone();
    v_D = py::truediv(&py::sub(&py::truediv(&py::add(&py::getitem(&v_z.clone(), &Py::Int(3i64))?, &py::getitem(&v_z.clone(), &Py::Int(5i64))?)?, &Py::Int(2i64))?, &py::getitem(&v_z.clone(), &Py::Int(4i64))?)?, &py::pow(&v_L.clone(), &Py::Int(2i64))?)?;
    v_E = py::truediv(&py::sub(&py::truediv(&py::add(&py::getitem(&v_z.clone(), &Py::Int(1i64))?, &py::getitem(&v_z.clone(), &Py::Int(7i64))?)?, &Py::Int(2i64))?, &py::getitem(&v_z.clone(), &Py::Int(4i64))?)?, &py::pow(&v_L.clone(), &Py::Int(2i64))?)?;
    v_F = py::truediv(&py::sub(&py::add(&py::add(&py::neg(&py::getitem(&v_z.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_z.clone(), &Py::Int(2i64))?)?, &py::getitem(&v_z.clone(), &Py::Int(6i64))?)?, &py::getitem(&v_z.clone(), &Py::Int(8i64))?)?, &py::mul(&Py::Int(4i64), &py::pow(&v_L.clone(), &Py::Int(2i64))?)?)?;
    v_G = py::truediv(&py::add(&py::neg(&py::getitem(&v_z.clone(), &Py::Int(3i64))?)?, &py::getitem(&v_z.clone(), &Py::Int(5i64))?)?, &py::mul(&Py::Int(2i64), &v_L.clone())?)?;
    v_H = py::truediv(&py::sub(&py::getitem(&v_z.clone(), &Py::Int(1i64))?, &py::getitem(&v_z.clone(), &Py::Int(7i64))?)?, &py::mul(&Py::Int(2i64), &v_L.clone())?)?;
    v_gg = py::add(&py::mul(&v_G.clone(), &v_G.clone())?, &py::mul(&v_H.clone(), &v_H.clone())?)?;
    let _ = f__need({ let t6939 = v_gg.clone(); Py::Bool({ let t6940 = Py::Int(0i64); if py::gt(&t6939, &t6940)? { true } else { false } }) }, Py::str("profile and plan curvature are undefined on a flat centre (zero gradient)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("curvature"), py::mul(&py::neg(&Py::Int(2i64))?, &py::add(&v_D.clone(), &v_E.clone())?)?), (Py::str("profile"), py::truediv(&py::mul(&py::neg(&Py::Int(2i64))?, &py::add(&py::add(&py::mul(&py::mul(&v_D.clone(), &v_G.clone())?, &v_G.clone())?, &py::mul(&py::mul(&v_E.clone(), &v_H.clone())?, &v_H.clone())?)?, &py::mul(&py::mul(&v_F.clone(), &v_G.clone())?, &v_H.clone())?)?)?, &v_gg.clone())?), (Py::str("plan"), py::truediv(&py::mul(&Py::Int(2i64), &py::sub(&py::add(&py::mul(&py::mul(&v_D.clone(), &v_H.clone())?, &v_H.clone())?, &py::mul(&py::mul(&v_E.clone(), &v_G.clone())?, &v_G.clone())?)?, &py::mul(&py::mul(&v_F.clone(), &v_G.clone())?, &v_H.clone())?)?)?, &v_gg.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_d8_flow_direction(mut v_dem: Py, mut v_cell_size: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("direction"), f__d8(v_dem.clone(), v_cell_size.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decimal_to_dms(mut v_decimal: Py, mut v_axis: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_hemi: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__fin(Py::str("the angle"), Py::tuple(vec![v_decimal.clone()]))?;
    let _ = f__need({ let t967 = v_axis.clone(); Py::Bool({ let t968 = Py::tuple(vec![Py::str("latitude"), Py::str("longitude")]); if py::contains(&t968, &t967)? { true } else { false } }) }, Py::str("the axis must be latitude or longitude"))?;
    let _ = f__need({ let t969 = py::abs(&v_decimal.clone())?; Py::Bool({ let t970 = (if ({ let t971 = v_axis.clone(); Py::Bool({ let t972 = Py::str("latitude"); if py::eq(&t971, &t972) { true } else { false } }) }).truthy() { Py::Int(90i64) } else { Py::Int(180i64) }); if py::le(&t969, &t970)? { true } else { false } }) }, Py::str("a latitude is at most 90 and a longitude at most 180 degrees in size"))?;
    v_v = py::abs(&py::fraction(&[Py::str(&py::repr(&py::float(&v_decimal.clone())?))])?)?;
    v_d = py::int(&v_v.clone())?;
    v_m = py::int(&py::mul(&py::sub(&v_v.clone(), &v_d.clone())?, &Py::Int(60i64))?)?;
    v_s = py::mul(&py::sub(&py::sub(&v_v.clone(), &v_d.clone())?, &py::fraction(&[v_m.clone(), Py::Int(60i64)])?)?, &Py::Int(3600i64))?;
    v_hemi = (if ({ let t973 = v_axis.clone(); Py::Bool({ let t974 = Py::str("latitude"); if py::eq(&t973, &t974) { true } else { false } }) }).truthy() { (if ({ let t975 = v_decimal.clone(); Py::Bool({ let t976 = Py::Int(0i64); if py::ge(&t975, &t976)? { true } else { false } }) }).truthy() { Py::str("N") } else { Py::str("S") }) } else { (if ({ let t977 = v_decimal.clone(); Py::Bool({ let t978 = Py::Int(0i64); if py::ge(&t977, &t978)? { true } else { false } }) }).truthy() { Py::str("E") } else { Py::str("W") }) });
    return Ok(Py::dict(vec![(Py::str("degrees"), v_d.clone()), (Py::str("minutes"), v_m.clone()), (Py::str("seconds"), f__r(py::float(&v_s.clone())?)?), (Py::str("hemisphere"), v_hemi.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_density_gradient(mut v_r: Py, mut v_D: Py) -> R {
    let mut v_c0: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_r.clone())?;
    let _ = f__need({ let t1034 = { let t1030 = v_n.clone(); Py::Bool({ let t1031 = py::len(&v_D.clone())?; if py::eq(&t1030, &t1031) { true } else { false } }) }; if !t1034.truthy() { t1034 } else { { let t1032 = v_n.clone(); Py::Bool({ let t1033 = Py::Int(3i64); if py::ge(&t1032, &t1033)? { true } else { false } }) } } }, Py::str("give one density per distance (at least 3)"))?;
    for t1035 in py::iter(&(v_r.clone()))? {
        v_v = t1035;
        let _ = f__nonneg(Py::str("each distance"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t1042 in py::iter(&(v_D.clone()))? {
        v_v = t1042;
        let _ = f__pos(Py::str("each density"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t1059 = (f__linfit(v_r.clone(), { let mut out = vec![]; for t1058 in py::iter(&(v_D.clone()))? { let mut v_v: Py; 
v_v = t1058;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) })?).items()?;
    if t1059.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c0 = t1059[0].clone();
    v_c1 = t1059[1].clone();
    v_r2 = t1059[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("D0"), py::math1("exp", &v_c0.clone())?), (Py::str("b"), py::neg(&v_c1.clone())?), (Py::str("r_squared"), v_r2.clone()), (Py::str("half_distance"), (if ({ let t1072 = v_c1.clone(); Py::Bool({ let t1073 = Py::Int(0i64); if py::lt(&t1072, &t1073)? { true } else { false } }) }).truthy() { py::truediv(&py::math1("log", &Py::Int(2i64))?, &py::neg(&v_c1.clone())?)? } else { Py::None }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_detour_index(mut v_coords: Py, mut v_edges: Py, mut v_origin: Py, mut v_destination: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_n = py::len(&v_coords.clone())?;
    let _ = f__need({ let t1120 = { let t1115 = v_n.clone(); Py::Bool({ let t1116 = Py::Int(2i64); if py::ge(&t1115, &t1116)? { true } else { false } }) }; if !t1120.truthy() { t1120 } else { Py::Bool(({ let mut out = vec![]; for t1117 in py::iter(&(v_coords.clone()))? { let mut v_r: Py; 
v_r = t1117;
out.push({ let t1118 = py::len(&v_r.clone())?; Py::Bool({ let t1119 = Py::Int(2i64); if py::eq(&t1118, &t1119) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("give the x and y of every node (an n x 2 matrix)"))?;
    v_E = (if (Py::Bool(({ let mut out = vec![]; for t1121 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t1121;
out.push({ let t1128 = { let t1122 = f__node(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::str("a node"))?; Py::Bool({ let t1123 = v_n.clone(); if py::lt(&t1122, &t1123)? { true } else { false } }) }; if !t1128.truthy() { t1128 } else { { let t1126 = f__node(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::str("a node"))?; Py::Bool({ let t1127 = v_n.clone(); if py::lt(&t1126, &t1127)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() { { let mut out = vec![]; for t1129 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t1129;
if ({ let t1134 = { let t1130 = f__node(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::str("a node"))?; Py::Bool({ let t1131 = v_n.clone(); if py::lt(&t1130, &t1131)? { true } else { false } }) }; if !t1134.truthy() { t1134 } else { { let t1132 = f__node(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::str("a node"))?; Py::Bool({ let t1133 = v_n.clone(); if py::lt(&t1132, &t1133)? { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(0i64))?, py::getitem(&v_e.clone(), &Py::Int(1i64))?, py::hypot(&[py::sub(&py::getitem(&py::getitem(&v_coords.clone(), &f__node(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::str("a node"))?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_coords.clone(), &f__node(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::str("a node"))?)?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_coords.clone(), &f__node(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::str("a node"))?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_coords.clone(), &f__node(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::str("a node"))?)?, &Py::Int(1i64))?)?])?])); }} Py::list(out) } } else { Py::None });
    let _ = f__need({ let t1135 = v_E.clone(); Py::Bool({ let t1136 = Py::None; if !py::is(&t1135, &t1136) { true } else { false } }) }, Py::str("every edge must join nodes that have coordinates"))?;
    v_adj = f__net(v_E.clone(), v_n.clone(), Py::Bool(true))?;
    let t1161 = (Py::tuple(vec![py::int(&v_origin.clone())?, py::int(&v_destination.clone())?])).items()?;
    if t1161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_o = t1161[0].clone();
    v_t = t1161[1].clone();
    let _ = f__need({ let t1177 = { let t1162 = py::float(&v_origin.clone())?; Py::Bool({ let t1163 = v_o.clone(); if py::eq(&t1162, &t1163) { true } else { false } }) }; if !t1177.truthy() { t1177 } else { { let t1176 = { let t1164 = py::float(&v_destination.clone())?; Py::Bool({ let t1165 = v_t.clone(); if py::eq(&t1164, &t1165) { true } else { false } }) }; if !t1176.truthy() { t1176 } else { { let t1175 = { let t1166 = Py::Int(0i64); Py::Bool({ let t1167 = v_o.clone(); if py::le(&t1166, &t1167)? { { let t1168 = v_n.clone(); if py::lt(&t1167, &t1168)? { true } else { false } } } else { false } }) }; if !t1175.truthy() { t1175 } else { { let t1174 = { let t1169 = Py::Int(0i64); Py::Bool({ let t1170 = v_t.clone(); if py::le(&t1169, &t1170)? { { let t1171 = v_n.clone(); if py::lt(&t1170, &t1171)? { true } else { false } } } else { false } }) }; if !t1174.truthy() { t1174 } else { { let t1172 = v_o.clone(); Py::Bool({ let t1173 = v_t.clone(); if py::ne(&t1172, &t1173) { true } else { false } }) } } } } } } } } }, Py::str("the origin and destination must be two different nodes"))?;
    v_d = py::getitem(&f__dijkstra(v_adj.clone(), v_o.clone())?, &v_t.clone())?;
    let _ = f__need(py::math1("isfinite", &v_d.clone())?, Py::str("the destination is not reachable from the origin"))?;
    v_e = py::hypot(&[py::sub(&py::getitem(&py::getitem(&v_coords.clone(), &v_o.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_coords.clone(), &v_t.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_coords.clone(), &v_o.clone())?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_coords.clone(), &v_t.clone())?, &Py::Int(1i64))?)?])?;
    let _ = f__need({ let t1185 = v_e.clone(); Py::Bool({ let t1186 = Py::Int(0i64); if py::gt(&t1185, &t1186)? { true } else { false } }) }, Py::str("the origin and destination coincide in space"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("network_distance"), v_d.clone()), (Py::str("euclidean_distance"), v_e.clone()), (Py::str("detour_index"), py::truediv(&v_d.clone(), &v_e.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_band_weights(mut v_x: Py, mut v_y: Py, mut v_threshold: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_card: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t1304 = v_n.clone(); Py::Bool({ let t1305 = py::len(&v_y.clone())?; if py::eq(&t1304, &t1305) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t1306 = v_n.clone(); Py::Bool({ let t1307 = Py::Int(2i64); if py::ge(&t1306, &t1307)? { true } else { false } }) }, Py::str("give at least 2 points"))?;
    let _ = f__pos(Py::str("the threshold"), Py::tuple(vec![v_threshold.clone()]))?;
    v_W = { let mut out = vec![]; for t1314 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1314;
out.push({ let mut out = vec![]; for t1315 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1315;
out.push((if ({ let t1320 = { let t1316 = v_i.clone(); Py::Bool({ let t1317 = v_j.clone(); if py::ne(&t1316, &t1317) { true } else { false } }) }; if !t1320.truthy() { t1320 } else { { let t1318 = py::hypot(&[py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?])?; Py::Bool({ let t1319 = v_threshold.clone(); if py::le(&t1318, &t1319)? { true } else { false } }) } } }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    v_card = { let mut out = vec![]; for t1321 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t1321;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("W"), v_W.clone()), (Py::str("neighbours"), v_card.clone()), (Py::str("s0"), py::fsum(&v_card.clone())?), (Py::str("islands"), py::float(&py::sum(&{ let mut out = vec![]; for t1322 in py::iter(&(v_card.clone()))? { let mut v_c: Py; 
v_c = t1322;
if ({ let t1323 = v_c.clone(); Py::Bool({ let t1324 = Py::Int(0i64); if py::eq(&t1323, &t1324) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dms_to_decimal(mut v_degrees: Py, mut v_minutes: Py, mut v_seconds: Py, mut v_hemisphere: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t1368 = Py::Bool(matches!(v_degrees.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1368.truthy() { t1368 } else { { let t1366 = v_degrees.clone(); Py::Bool({ let t1367 = Py::Int(0i64); if py::ge(&t1366, &t1367)? { true } else { false } }) } } }, Py::str("degrees must be a whole number >= 0 (the hemisphere gives the sign)"))?;
    let _ = f__need({ let t1372 = Py::Bool(matches!(v_minutes.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1372.truthy() { t1372 } else { { let t1369 = Py::Int(0i64); Py::Bool({ let t1370 = v_minutes.clone(); if py::le(&t1369, &t1370)? { { let t1371 = Py::Int(59i64); if py::le(&t1370, &t1371)? { true } else { false } } } else { false } }) } } }, Py::str("minutes must be a whole number from 0 to 59"))?;
    let _ = f__fin(Py::str("seconds"), Py::tuple(vec![v_seconds.clone()]))?;
    let _ = f__need({ let t1376 = Py::Int(0i64); Py::Bool({ let t1377 = v_seconds.clone(); if py::le(&t1376, &t1377)? { { let t1378 = Py::Int(60i64); if py::lt(&t1377, &t1378)? { true } else { false } } } else { false } }) }, Py::str("seconds must lie in [0, 60)"))?;
    let _ = f__need({ let t1379 = v_hemisphere.clone(); Py::Bool({ let t1380 = Py::tuple(vec![Py::str("N"), Py::str("S"), Py::str("E"), Py::str("W")]); if py::contains(&t1380, &t1379)? { true } else { false } }) }, Py::str("the hemisphere must be N, S, E or W"))?;
    v_v = py::add(&py::add(&py::fraction(&[v_degrees.clone()])?, &py::fraction(&[v_minutes.clone(), Py::Int(60i64)])?)?, &py::truediv(&py::fraction(&[Py::str(&py::repr(&py::float(&v_seconds.clone())?))])?, &Py::Int(3600i64))?)?;
    let _ = f__need({ let t1381 = v_v.clone(); Py::Bool({ let t1382 = (if ({ let t1383 = v_hemisphere.clone(); Py::Bool({ let t1384 = Py::str("NS"); if py::contains(&t1384, &t1383)? { true } else { false } }) }).truthy() { Py::Int(90i64) } else { Py::Int(180i64) }); if py::le(&t1381, &t1382)? { true } else { false } }) }, Py::str("a latitude is at most 90 and a longitude at most 180 degrees"))?;
    if ({ let t1385 = v_hemisphere.clone(); Py::Bool({ let t1386 = Py::str("SW"); if py::contains(&t1386, &t1385)? { true } else { false } }) }).truthy() {
        v_v = py::neg(&v_v.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("decimal"), py::add(&py::float(&v_v.clone())?, &Py::Float(0.0))?), (Py::str("radians"), f__r(py::mul(&py::float(&v_v.clone())?, &c_DEG()?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_douglas_peucker(mut v_x: Py, mut v_y: Py, mut v_tolerance: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_keep: Py = Py::None;
    let mut v_kept: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_stack: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t1438 = v_n.clone(); Py::Bool({ let t1439 = py::len(&v_y.clone())?; if py::eq(&t1438, &t1439) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t1440 = v_n.clone(); Py::Bool({ let t1441 = Py::Int(3i64); if py::ge(&t1440, &t1441)? { true } else { false } }) }, Py::str("a line needs at least 3 vertices to simplify"))?;
    let _ = f__pos(Py::str("the tolerance"), Py::tuple(vec![v_tolerance.clone()]))?;
    let _ = f__need({ let t1448 = py::len(&py::set(&py::zip(&[v_x.clone(), v_y.clone()])?)?)?; Py::Bool({ let t1449 = v_n.clone(); if py::eq(&t1448, &t1449) { true } else { false } }) }, Py::str("the vertices must be distinct"))?;
    v_keep = py::mul(&Py::list(vec![Py::Bool(false)]), &v_n.clone())?;
    let t1450 = Py::Bool(true);
    py::setitem(&v_keep.clone(), &Py::Int(0i64), t1450.clone())?;
    py::setitem(&v_keep.clone(), &py::neg(&Py::Int(1i64))?, t1450.clone())?;
    v_stack = Py::list(vec![Py::tuple(vec![Py::Int(0i64), py::sub(&v_n.clone(), &Py::Int(1i64))?])]);
    while (v_stack.clone()).truthy() {
        let t1451 = (py::method(&v_stack.clone(), "pop", &[])?).items()?;
        if t1451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1451[0].clone();
        v_j = t1451[1].clone();
        let t1452 = (Py::tuple(vec![py::neg(&Py::Float(1.0))?, py::neg(&Py::Int(1i64))?])).items()?;
        if t1452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_best = t1452[0].clone();
        v_idx = t1452[1].clone();
        for t1453 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_j.clone()).i()?]) {
            v_k = t1453;
            v_d = f__seg_dist(py::getitem(&v_x.clone(), &v_k.clone())?, py::getitem(&v_y.clone(), &v_k.clone())?, py::getitem(&v_x.clone(), &v_i.clone())?, py::getitem(&v_y.clone(), &v_i.clone())?, py::getitem(&v_x.clone(), &v_j.clone())?, py::getitem(&v_y.clone(), &v_j.clone())?)?;
            if ({ let t1457 = v_d.clone(); Py::Bool({ let t1458 = v_best.clone(); if py::gt(&t1457, &t1458)? { true } else { false } }) }).truthy() {
                let t1459 = (Py::tuple(vec![v_d.clone(), v_k.clone()])).items()?;
                if t1459.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_best = t1459[0].clone();
                v_idx = t1459[1].clone();
            }
        }
        if ({ let t1464 = { let t1460 = v_idx.clone(); Py::Bool({ let t1461 = Py::Int(0i64); if py::ge(&t1460, &t1461)? { true } else { false } }) }; if !t1464.truthy() { t1464 } else { { let t1462 = v_best.clone(); Py::Bool({ let t1463 = v_tolerance.clone(); if py::gt(&t1462, &t1463)? { true } else { false } }) } } }).truthy() {
            py::setitem(&v_keep.clone(), &v_idx.clone(), Py::Bool(true))?;
            let _ = py::method(&v_stack.clone(), "append", &[Py::tuple(vec![v_i.clone(), v_idx.clone()])])?;
            let _ = py::method(&v_stack.clone(), "append", &[Py::tuple(vec![v_idx.clone(), v_j.clone()])])?;
        }
    }
    v_kept = { let mut out = vec![]; for t1465 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1465;
if (py::getitem(&v_keep.clone(), &v_i.clone())?).truthy() { out.push(py::float(&v_i.clone())?); }} Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("kept"), v_kept.clone()), (Py::str("n_kept"), py::len(&v_kept.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ecef_to_geodetic(mut v_X: Py, mut v_Y: Py, mut v_Z: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_lon: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__fin(Py::str("X, Y and Z"), Py::tuple(vec![v_X.clone(), v_Y.clone(), v_Z.clone()]))?;
    v_r = py::math1("sqrt", &py::add(&py::add(&py::mul(&v_X.clone(), &v_X.clone())?, &py::mul(&v_Y.clone(), &v_Y.clone())?)?, &py::mul(&v_Z.clone(), &v_Z.clone())?)?)?;
    let _ = f__need({ let t1513 = Py::Float(6200000.0); Py::Bool({ let t1514 = v_r.clone(); if py::le(&t1513, &t1514)? { { let t1515 = Py::Float(100000000.0); if py::le(&t1514, &t1515)? { true } else { false } } } else { false } }) }, Py::str("the point must lie between 6200 km and 100000 km from the Earth's centre"))?;
    let t1521 = (f__geodetic(v_E.clone(), v_X.clone(), v_Y.clone(), v_Z.clone())?).items()?;
    if t1521.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t1521[0].clone();
    v_lon = t1521[1].clone();
    v_h = t1521[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("lat"), v_lat.clone()), (Py::str("lon"), v_lon.clone()), (Py::str("h"), v_h.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_empirical_semivariogram(mut v_x: Py, mut v_y: Py, mut v_z: Py, mut v_bin_edges: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_counts: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sums: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t1580 = { let t1575 = v_n.clone(); Py::Bool({ let t1576 = py::len(&v_y.clone())?; if py::eq(&t1575, &t1576) { { let t1577 = py::len(&v_z.clone())?; if py::eq(&t1576, &t1577) { true } else { false } } } else { false } }) }; if !t1580.truthy() { t1580 } else { { let t1578 = v_n.clone(); Py::Bool({ let t1579 = Py::Int(3i64); if py::ge(&t1578, &t1579)? { true } else { false } }) } } }, Py::str("give one y and one value per x (at least 3 samples)"))?;
    v_e = Py::list((v_bin_edges.clone()).items()?);
    let _ = f__need({ let t1590 = { let t1581 = py::len(&v_e.clone())?; Py::Bool({ let t1582 = Py::Int(2i64); if py::ge(&t1581, &t1582)? { true } else { false } }) }; if !t1590.truthy() { t1590 } else { { let t1589 = Py::Bool(({ let mut out = vec![]; for t1583 in py::iter(&(py::zip(&[v_e.clone(), py::slice(&v_e.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1584 = (t1583).items()?;
if t1584.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1584[0].clone();
v_b = t1584[1].clone();
out.push({ let t1585 = v_b.clone(); Py::Bool({ let t1586 = v_a.clone(); if py::gt(&t1585, &t1586)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1589.truthy() { t1589 } else { { let t1587 = py::getitem(&v_e.clone(), &Py::Int(0i64))?; Py::Bool({ let t1588 = Py::Int(0i64); if py::ge(&t1587, &t1588)? { true } else { false } }) } } } } }, Py::str("the bin edges must increase from a non-negative start"))?;
    v_k = py::sub(&py::len(&v_e.clone())?, &Py::Int(1i64))?;
    let t1592 = (Py::tuple(vec![{ let mut out = vec![]; for t1591 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v__: Py; 
v__ = t1591;
out.push(Py::list(vec![])); } Py::list(out) }, py::mul(&Py::list(vec![Py::Int(0i64)]), &v_k.clone())?])).items()?;
    if t1592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sums = t1592[0].clone();
    v_counts = t1592[1].clone();
    for t1593 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1593;
        for t1594 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t1594;
            v_d = py::hypot(&[py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?])?;
            for t1595 in py::range_iter(&[(v_k.clone()).i()?]) {
                v_b = t1595;
                if ({ let t1596 = py::getitem(&v_e.clone(), &v_b.clone())?; Py::Bool({ let t1597 = v_d.clone(); if py::le(&t1596, &t1597)? { { let t1598 = py::getitem(&v_e.clone(), &py::add(&v_b.clone(), &Py::Int(1i64))?)?; if py::lt(&t1597, &t1598)? { true } else { false } } } else { false } }) }).truthy() {
                    let _ = py::method(&py::getitem(&v_sums.clone(), &v_b.clone())?, "append", &[py::pow(&py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::getitem(&v_z.clone(), &v_j.clone())?)?, &Py::Int(2i64))?])?;
                    let t1599 = v_counts.clone(); let t1600 = v_b.clone();
                    py::setitem(&t1599, &t1600, py::iadd(&py::getitem(&t1599, &t1600)?, &Py::Int(1i64))?)?;
                    break;
                }
            }
        }
    }
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1601 in py::iter(&(v_counts.clone()))? { let mut v_c: Py; 
v_c = t1601;
out.push({ let t1602 = v_c.clone(); Py::Bool({ let t1603 = Py::Int(0i64); if py::gt(&t1602, &t1603)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("every lag bin must contain at least one pair"))?;
    return Ok(Py::dict(vec![(Py::str("gamma"), f__r({ let mut out = vec![]; for t1614 in py::iter(&(py::zip(&[v_sums.clone(), v_counts.clone()])?))? { let mut v_c: Py; let mut v_s: Py; 
let t1615 = (t1614).items()?;
if t1615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t1615[0].clone();
v_c = t1615[1].clone();
out.push(py::truediv(&py::fsum(&v_s.clone())?, &py::mul(&Py::Int(2i64), &v_c.clone())?)?); } Py::list(out) })?), (Py::str("counts"), { let mut out = vec![]; for t1616 in py::iter(&(v_counts.clone()))? { let mut v_c: Py; 
v_c = t1616;
out.push(py::float(&v_c.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_enu_to_geodetic(mut v_lat0: Py, mut v_lon0: Py, mut v_h0: Py, mut v_e: Py, mut v_n: Py, mut v_u: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_lon: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__geo_point(v_lat0.clone(), v_lon0.clone(), v_h0.clone(), Py::str("the origin's "))?;
    let _ = f__need({ let t1676 = py::abs(&v_lat0.clone())?; Py::Bool({ let t1677 = Py::Int(90i64); if py::lt(&t1676, &t1677)? { true } else { false } }) }, Py::str("the origin must not be a pole"))?;
    let _ = f__fin(Py::str("e, n and u"), Py::tuple(vec![v_e.clone(), v_n.clone(), v_u.clone()]))?;
    let _ = f__need({ let t1678 = py::minmax(&[py::abs(&v_e.clone())?, py::abs(&v_n.clone())?, py::abs(&v_u.clone())?], true)?; Py::Bool({ let t1679 = Py::Float(50000000.0); if py::le(&t1678, &t1679)? { true } else { false } }) }, Py::str("the local coordinates must be at most 50000 km"))?;
    v_o = f__ecef(v_E.clone(), v_lat0.clone(), v_lon0.clone(), v_h0.clone())?;
    v_R = f__enu_rot(v_lat0.clone(), v_lon0.clone())?;
    v_v = Py::tuple(vec![v_e.clone(), v_n.clone(), v_u.clone()]);
    let t1686 = ({ let mut out = vec![]; for t1684 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t1684;
out.push(py::add(&py::getitem(&v_o.clone(), &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t1685 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t1685;
out.push(py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_v.clone(), &v_i.clone())?)?); } Py::list(out) })?)?); } Py::list(out) }).items()?;
    if t1686.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t1686[0].clone();
    v_Y = t1686[1].clone();
    v_Z = t1686[2].clone();
    let _ = f__need({ let t1687 = py::math1("sqrt", &py::add(&py::add(&py::mul(&v_X.clone(), &v_X.clone())?, &py::mul(&v_Y.clone(), &v_Y.clone())?)?, &py::mul(&v_Z.clone(), &v_Z.clone())?)?)?; Py::Bool({ let t1688 = Py::Float(6200000.0); if py::ge(&t1687, &t1688)? { true } else { false } }) }, Py::str("the point would lie deep inside the Earth"))?;
    let t1694 = (f__geodetic(v_E.clone(), v_X.clone(), v_Y.clone(), v_Z.clone())?).items()?;
    if t1694.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t1694[0].clone();
    v_lon = t1694[1].clone();
    v_h = t1694[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("lat"), v_lat.clone()), (Py::str("lon"), v_lon.clone()), (Py::str("h"), v_h.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equirectangular_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat_ts: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_R: Py) -> R {
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Int(90i64))?;
    let _ = f__lat(Py::str("lat_ts"), v_lat_ts.clone(), Py::Bool(false))?;
    let _ = f__lat(Py::str("lat0"), v_lat0.clone(), Py::Bool(true))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__sphere_R(v_R.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::mul(&py::mul(&py::mul(&v_R.clone(), &f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?)?, &c_DEG()?)?, &py::math1("cos", &py::mul(&v_lat_ts.clone(), &c_DEG()?)?)?)?), (Py::str("y"), py::mul(&py::mul(&v_R.clone(), &py::sub(&v_lat.clone(), &v_lat0.clone())?)?, &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flow_accumulation(mut v_dem: Py, mut v_cell_size: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_acc: Py = Py::None;
    let mut v_dirs: Py = Py::None;
    let mut v_indeg: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_rec: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_dirs = f__d8(v_dem.clone(), v_cell_size.clone())?;
    let t814 = (Py::tuple(vec![py::len(&v_dem.clone())?, py::len(&py::getitem(&v_dem.clone(), &Py::Int(0i64))?)?])).items()?;
    if t814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t814[0].clone();
    v_C = t814[1].clone();
    v_rec = f__receivers(v_dirs.clone())?;
    v_indeg = { let mut out = vec![]; for t821 in py::range_iter(&[(v_R.clone()).i()?]) { let mut v_i: Py; 
v_i = t821;
for t822 in py::range_iter(&[(v_C.clone()).i()?]) { let mut v_j: Py; 
v_j = t822;
out.push((Py::tuple(vec![v_i.clone(), v_j.clone()]), Py::Int(0i64))); }} Py::dict(out) };
    for t823 in py::iter(&(py::method(&v_rec.clone(), "values", &[])?))? {
        v_v = t823;
        let t824 = v_indeg.clone(); let t825 = v_v.clone();
        py::setitem(&t824, &t825, py::iadd(&py::getitem(&t824, &t825)?, &Py::Int(1i64))?)?;
    }
    v_acc = { let mut out = vec![]; for t826 in py::iter(&(v_indeg.clone()))? { let mut v_k: Py; 
v_k = t826;
out.push((v_k.clone(), Py::Int(1i64))); } Py::dict(out) };
    v_stack = { let mut out = vec![]; for t827 in py::iter(&(py::method(&v_indeg.clone(), "items", &[])?))? { let mut v_d: Py; let mut v_k: Py; 
let t828 = (t827).items()?;
if t828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t828[0].clone();
v_d = t828[1].clone();
if ({ let t829 = v_d.clone(); Py::Bool({ let t830 = Py::Int(0i64); if py::eq(&t829, &t830) { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    while (v_stack.clone()).truthy() {
        v_k = py::method(&v_stack.clone(), "pop", &[])?;
        if ({ let t831 = v_k.clone(); Py::Bool({ let t832 = v_rec.clone(); if py::contains(&t832, &t831)? { true } else { false } }) }).truthy() {
            v_t = py::getitem(&v_rec.clone(), &v_k.clone())?;
            let t833 = v_acc.clone(); let t834 = v_t.clone();
            py::setitem(&t833, &t834, py::iadd(&py::getitem(&t833, &t834)?, &py::getitem(&v_acc.clone(), &v_k.clone())?)?)?;
            let t835 = v_indeg.clone(); let t836 = v_t.clone();
            py::setitem(&t835, &t836, py::sub(&py::getitem(&t835, &t836)?, &Py::Int(1i64))?)?;
            if ({ let t837 = py::getitem(&v_indeg.clone(), &v_t.clone())?; Py::Bool({ let t838 = Py::Int(0i64); if py::eq(&t837, &t838) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_stack.clone(), "append", &[v_t.clone()])?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("accumulation"), { let mut out = vec![]; for t839 in py::range_iter(&[(v_R.clone()).i()?]) { let mut v_i: Py; 
v_i = t839;
out.push({ let mut out = vec![]; for t840 in py::range_iter(&[(v_C.clone()).i()?]) { let mut v_j: Py; 
v_j = t840;
out.push(py::float(&py::getitem(&v_acc.clone(), &Py::tuple(vec![v_i.clone(), v_j.clone()]))?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("max_accumulation"), py::float(&py::minmax(&[py::method(&v_acc.clone(), "values", &[])?], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_frechet_distance(mut v_x1: Py, mut v_y1: Py, mut v_x2: Py, mut v_y2: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__need({ let t1244 = { let t1240 = py::len(&v_x1.clone())?; Py::Bool({ let t1241 = py::len(&v_y1.clone())?; if py::eq(&t1240, &t1241) { true } else { false } }) }; if !t1244.truthy() { t1244 } else { { let t1242 = py::len(&v_x2.clone())?; Py::Bool({ let t1243 = py::len(&v_y2.clone())?; if py::eq(&t1242, &t1243) { true } else { false } }) } } }, Py::str("give one y per x"))?;
    let _ = f__need({ let t1249 = { let t1245 = py::len(&v_x1.clone())?; Py::Bool({ let t1246 = Py::Int(2i64); if py::ge(&t1245, &t1246)? { true } else { false } }) }; if !t1249.truthy() { t1249 } else { { let t1247 = py::len(&v_x2.clone())?; Py::Bool({ let t1248 = Py::Int(2i64); if py::ge(&t1247, &t1248)? { true } else { false } }) } } }, Py::str("each line needs at least 2 vertices"))?;
    let t1250 = (Py::tuple(vec![py::len(&v_x1.clone())?, py::len(&v_x2.clone())?])).items()?;
    if t1250.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1250[0].clone();
    v_m = t1250[1].clone();
    v_c = { let mut out = vec![]; for t1251 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t1251;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?); } Py::list(out) };
    for t1252 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1252;
        for t1253 in py::range_iter(&[(v_m.clone()).i()?]) {
            v_j = t1253;
            v_d = py::hypot(&[py::sub(&py::getitem(&v_x1.clone(), &v_i.clone())?, &py::getitem(&v_x2.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_y1.clone(), &v_i.clone())?, &py::getitem(&v_y2.clone(), &v_j.clone())?)?])?;
            if ({ let t1258 = { let t1254 = v_i.clone(); Py::Bool({ let t1255 = Py::Int(0i64); if py::eq(&t1254, &t1255) { true } else { false } }) }; if !t1258.truthy() { t1258 } else { { let t1256 = v_j.clone(); Py::Bool({ let t1257 = Py::Int(0i64); if py::eq(&t1256, &t1257) { true } else { false } }) } } }).truthy() {
                py::setitem(&py::getitem(&v_c.clone(), &v_i.clone())?, &v_j.clone(), v_d.clone())?;
            } else {
                if ({ let t1259 = v_i.clone(); Py::Bool({ let t1260 = Py::Int(0i64); if py::eq(&t1259, &t1260) { true } else { false } }) }).truthy() {
                    py::setitem(&py::getitem(&v_c.clone(), &v_i.clone())?, &v_j.clone(), py::minmax(&[py::getitem(&py::getitem(&v_c.clone(), &v_i.clone())?, &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, v_d.clone()], true)?)?;
                } else {
                    if ({ let t1261 = v_j.clone(); Py::Bool({ let t1262 = Py::Int(0i64); if py::eq(&t1261, &t1262) { true } else { false } }) }).truthy() {
                        py::setitem(&py::getitem(&v_c.clone(), &v_i.clone())?, &v_j.clone(), py::minmax(&[py::getitem(&py::getitem(&v_c.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &v_j.clone())?, v_d.clone()], true)?)?;
                    } else {
                        py::setitem(&py::getitem(&v_c.clone(), &v_i.clone())?, &v_j.clone(), py::minmax(&[py::minmax(&[py::getitem(&py::getitem(&v_c.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &v_j.clone())?, py::getitem(&py::getitem(&v_c.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, py::getitem(&py::getitem(&v_c.clone(), &v_i.clone())?, &py::sub(&v_j.clone(), &Py::Int(1i64))?)?], false)?, v_d.clone()], true)?)?;
                    }
                }
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("distance"), py::getitem(&py::getitem(&v_c.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &py::sub(&v_m.clone(), &Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gearys_c(mut v_y: Py, mut v_W: Py, mut v_transform: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_VCn: Py = Py::None;
    let mut v_VCr: Py = Py::None;
    let mut v_Wt: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__y_ok(v_y.clone(), Py::Int(4i64))?;
    let _ = f__need({ let t1834 = v_transform.clone(); Py::Bool({ let t1835 = Py::tuple(vec![Py::str("row"), Py::str("none")]); if py::contains(&t1835, &t1834)? { true } else { false } }) }, Py::str("the transform must be row or none"))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), v_transform.clone(), Py::Bool(false), Py::Bool(false), Py::Bool(false))?;
    v_m = py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?;
    v_z = { let mut out = vec![]; for t1862 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t1862;
out.push(py::sub(&v_v.clone(), &v_m.clone())?); } Py::list(out) };
    let t1868 = (f__s012(v_Wt.clone())?).items()?;
    if t1868.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s0 = t1868[0].clone();
    v_s1 = t1868[1].clone();
    v_s2 = t1868[2].clone();
    v_m2 = py::fsum(&{ let mut out = vec![]; for t1869 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t1869;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    v_C = py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &py::fsum(&{ let mut out = vec![]; for t1870 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1870;
for t1871 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1871;
out.push(py::mul(&py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?, &py::pow(&py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?, &Py::Int(2i64))?)?); }} Py::list(out) })?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_s0.clone())?, &v_m2.clone())?)?;
    v_VCn = py::truediv(&py::sub(&py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_s1.clone())?, &v_s2.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &py::mul(&py::mul(&Py::Int(4i64), &v_s0.clone())?, &v_s0.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &v_s0.clone())?, &v_s0.clone())?)?;
    v_k = py::truediv(&py::truediv(&py::fsum(&{ let mut out = vec![]; for t1872 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t1872;
out.push(py::pow(&v_v.clone(), &Py::Int(4i64))?); } Py::list(out) })?, &v_n.clone())?, &py::pow(&py::truediv(&v_m2.clone(), &v_n.clone())?, &Py::Int(2i64))?)?;
    v_VCr = py::truediv(&py::add(&py::sub(&py::mul(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_s1.clone())?, &py::sub(&py::add(&py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &py::mul(&Py::Int(3i64), &v_n.clone())?)?, &Py::Int(3i64))?, &py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?)?)?, &py::mul(&py::mul(&py::mul(&Py::Float(0.25), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &v_s2.clone())?, &py::sub(&py::sub(&py::add(&py::mul(&v_n.clone(), &v_n.clone())?, &py::mul(&Py::Int(3i64), &v_n.clone())?)?, &Py::Int(6i64))?, &py::mul(&py::add(&py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &v_n.clone())?, &Py::Int(2i64))?, &v_k.clone())?)?)?)?, &py::mul(&py::mul(&v_s0.clone(), &v_s0.clone())?, &py::sub(&py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &Py::Int(3i64))?, &py::mul(&py::pow(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_k.clone())?)?)?)?, &py::mul(&py::mul(&py::mul(&py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(2i64))?)?, &py::sub(&v_n.clone(), &Py::Int(3i64))?)?, &v_s0.clone())?, &v_s0.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("VC_norm"), v_VCn.clone()), (Py::str("z_norm"), py::truediv(&py::sub(&v_C.clone(), &Py::Int(1i64))?, &py::math1("sqrt", &v_VCn.clone())?)?), (Py::str("VC_rand"), v_VCr.clone()), (Py::str("z_rand"), py::truediv(&py::sub(&v_C.clone(), &Py::Int(1i64))?, &py::math1("sqrt", &v_VCr.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_general_g(mut v_y: Py, mut v_W: Py) -> R {
    let mut v_EG: Py = Py::None;
    let mut v_EG2: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_VG: Py = Py::None;
    let mut v_Wt: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_b3: Py = Py::None;
    let mut v_b4: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_m3: Py = Py::None;
    let mut v_m4: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__y_ok(v_y.clone(), Py::Int(4i64))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2541 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t2541;
out.push({ let t2542 = v_v.clone(); Py::Bool({ let t2543 = Py::Int(0i64); if py::gt(&t2542, &t2543)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the values must be positive"))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), Py::str("none"), Py::Bool(true), Py::Bool(false), Py::Bool(false))?;
    let t2575 = (f__s012(v_Wt.clone())?).items()?;
    if t2575.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s0 = t2575[0].clone();
    v_s1 = t2575[1].clone();
    v_s2 = t2575[2].clone();
    v_num = py::fsum(&{ let mut out = vec![]; for t2576 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2576;
for t2577 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2577;
out.push(py::mul(&py::mul(&py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_y.clone(), &v_i.clone())?)?, &py::getitem(&v_y.clone(), &v_j.clone())?)?); }} Py::list(out) })?;
    let t2580 = ({ let mut out = vec![]; for t2578 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64), Py::Int(4i64)])))? { let mut v_p: Py; 
v_p = t2578;
out.push(py::fsum(&{ let mut out = vec![]; for t2579 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t2579;
out.push(py::pow(&v_v.clone(), &v_p.clone())?); } Py::list(out) })?); } Py::list(out) }).items()?;
    if t2580.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m1 = t2580[0].clone();
    v_m2 = t2580[1].clone();
    v_m3 = t2580[2].clone();
    v_m4 = t2580[3].clone();
    v_G = py::truediv(&v_num.clone(), &py::sub(&py::mul(&v_m1.clone(), &v_m1.clone())?, &v_m2.clone())?)?;
    v_EG = py::truediv(&v_s0.clone(), &py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    v_b0 = py::add(&py::sub(&py::mul(&py::add(&py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &py::mul(&Py::Int(3i64), &v_n.clone())?)?, &Py::Int(3i64))?, &v_s1.clone())?, &py::mul(&v_n.clone(), &v_s2.clone())?)?, &py::mul(&py::mul(&Py::Int(3i64), &v_s0.clone())?, &v_s0.clone())?)?;
    v_b1 = py::neg(&py::add(&py::sub(&py::mul(&py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &v_n.clone())?, &v_s1.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_n.clone())?, &v_s2.clone())?)?, &py::mul(&py::mul(&Py::Int(6i64), &v_s0.clone())?, &v_s0.clone())?)?)?;
    v_b2 = py::neg(&py::add(&py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_n.clone())?, &v_s1.clone())?, &py::mul(&py::add(&v_n.clone(), &Py::Int(3i64))?, &v_s2.clone())?)?, &py::mul(&py::mul(&Py::Int(6i64), &v_s0.clone())?, &v_s0.clone())?)?)?;
    v_b3 = py::add(&py::sub(&py::mul(&py::mul(&Py::Int(4i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &v_s1.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &v_s2.clone())?)?, &py::mul(&py::mul(&Py::Int(8i64), &v_s0.clone())?, &v_s0.clone())?)?;
    v_b4 = py::add(&py::sub(&v_s1.clone(), &v_s2.clone())?, &py::mul(&v_s0.clone(), &v_s0.clone())?)?;
    v_EG2 = py::truediv(&py::add(&py::add(&py::add(&py::add(&py::mul(&py::mul(&v_b0.clone(), &v_m2.clone())?, &v_m2.clone())?, &py::mul(&v_b1.clone(), &v_m4.clone())?)?, &py::mul(&py::mul(&py::mul(&v_b2.clone(), &v_m1.clone())?, &v_m1.clone())?, &v_m2.clone())?)?, &py::mul(&py::mul(&v_b3.clone(), &v_m1.clone())?, &v_m3.clone())?)?, &py::mul(&v_b4.clone(), &py::pow(&v_m1.clone(), &Py::Int(4i64))?)?)?, &py::mul(&py::mul(&py::mul(&py::mul(&py::pow(&py::sub(&py::mul(&v_m1.clone(), &v_m1.clone())?, &v_m2.clone())?, &Py::Int(2i64))?, &v_n.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &py::sub(&v_n.clone(), &Py::Int(2i64))?)?, &py::sub(&v_n.clone(), &Py::Int(3i64))?)?)?;
    v_VG = py::sub(&v_EG2.clone(), &py::mul(&v_EG.clone(), &v_EG.clone())?)?;
    let _ = f__need({ let t2585 = { let t2581 = v_s0.clone(); Py::Bool({ let t2582 = py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?; if py::lt(&t2581, &t2582)? { true } else { false } }) }; if !t2585.truthy() { t2585 } else { { let t2583 = v_VG.clone(); Py::Bool({ let t2584 = py::mul(&py::mul(&Py::Float(1e-12), &v_EG.clone())?, &v_EG.clone())?; if py::gt(&t2583, &t2584)? { true } else { false } }) } } }, Py::str("with every pair of locations neighbours G is constant (variance zero, z undefined)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("EG"), v_EG.clone()), (Py::str("VG"), v_VG.clone()), (Py::str("z_norm"), py::truediv(&py::sub(&v_G.clone(), &v_EG.clone())?, &py::math1("sqrt", &v_VG.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geodesic_destination(mut v_lat1: Py, mut v_lon1: Py, mut v_azimuth: Py, mut v_distance: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_az2: Py = Py::None;
    let mut v_lat2: Py = Py::None;
    let mut v_lon2: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat1"), v_lat1.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon1"), v_lon1.clone())?;
    let _ = f__fin(Py::str("azimuth"), Py::tuple(vec![v_azimuth.clone()]))?;
    let _ = f__pos(Py::str("the distance"), Py::tuple(vec![v_distance.clone()]))?;
    let _ = f__need({ let t1947 = v_distance.clone(); Py::Bool({ let t1948 = Py::Float(15000000.0); if py::le(&t1947, &t1948)? { true } else { false } }) }, Py::str("the distance must be at most 15000 km"))?;
    let t1972 = (f__geod_direct(v_E.clone(), v_lat1.clone(), v_lon1.clone(), v_azimuth.clone(), v_distance.clone())?).items()?;
    if t1972.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lat2 = t1972[0].clone();
    v_lon2 = t1972[1].clone();
    v_az2 = t1972[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("lat2"), v_lat2.clone()), (Py::str("lon2"), v_lon2.clone()), (Py::str("azimuth2"), v_az2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geodesic_midpoint(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_az: Py = Py::None;
    let mut v_inv: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_lon: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__two_points(v_lat1.clone(), v_lon1.clone(), v_lat2.clone(), v_lon2.clone())?;
    v_inv = f__geod_inverse(v_E.clone(), v_lat1.clone(), v_lon1.clone(), v_lat2.clone(), v_lon2.clone())?;
    let t2081 = (f__geod_direct(v_E.clone(), v_lat1.clone(), v_lon1.clone(), py::getitem(&v_inv.clone(), &Py::str("az1"))?, py::truediv(&py::getitem(&v_inv.clone(), &Py::str("s"))?, &Py::Int(2i64))?)?).items()?;
    if t2081.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t2081[0].clone();
    v_lon = t2081[1].clone();
    v_az = t2081[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("lat_mid"), v_lat.clone()), (Py::str("lon_mid"), v_lon.clone()), (Py::str("distance"), py::getitem(&v_inv.clone(), &Py::str("s"))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geodesic_polygon_area(mut v_lats: Py, mut v_lons: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_area: Py = Py::None;
    let mut v_ca0: Py = Py::None;
    let mut v_dlam_sum: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_integrand: Py = Py::None;
    let mut v_inv: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_perim: Py = Py::None;
    let mut v_qc: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_sa0: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    v_n = py::len(&v_lats.clone())?;
    let _ = f__need({ let t2138 = v_n.clone(); Py::Bool({ let t2139 = py::len(&v_lons.clone())?; if py::eq(&t2138, &t2139) { true } else { false } }) }, Py::str("give one longitude per latitude"))?;
    let _ = f__need({ let t2140 = Py::Int(3i64); Py::Bool({ let t2141 = v_n.clone(); if py::le(&t2140, &t2141)? { { let t2142 = Py::Int(50i64); if py::le(&t2141, &t2142)? { true } else { false } } } else { false } }) }, Py::str("a polygon needs 3 to 50 vertices"))?;
    for t2143 in py::iter(&(v_lats.clone()))? {
        v_v = t2143;
        let _ = f__lat(Py::str("each latitude"), v_v.clone(), Py::Bool(false))?;
        let _ = f__need({ let t2153 = py::abs(&v_v.clone())?; Py::Bool({ let t2154 = Py::Int(88i64); if py::le(&t2153, &t2154)? { true } else { false } }) }, Py::str("vertices must lie within 88 degrees of the equator (polygons around a pole are not supported)"))?;
    }
    for t2155 in py::iter(&(v_lons.clone()))? {
        v_v = t2155;
        let _ = f__lon(Py::str("each longitude"), v_v.clone())?;
    }
    v_a2 = py::pow(&py::getitem(&v_E.clone(), &Py::str("a"))?, &Py::Int(2i64))?;
    v_qc = f__q_auth(v_E.clone(), py::mul(&py::getitem(&v_lats.clone(), &Py::Int(0i64))?, &c_DEG()?)?)?;
    v_f = py::getitem(&v_E.clone(), &Py::str("f"))?;
    let t2159 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::Float(0.0)])).items()?;
    if t2159.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_total = t2159[0].clone();
    v_perim = t2159[1].clone();
    v_dlam_sum = t2159[2].clone();
    for t2160 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2160;
        v_j = py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?;
        let _ = f__need({ let t2161 = f__sph_sep(py::getitem(&v_lats.clone(), &v_i.clone())?, py::getitem(&v_lons.clone(), &v_i.clone())?, py::getitem(&v_lats.clone(), &v_j.clone())?, py::getitem(&v_lons.clone(), &v_j.clone())?)?; Py::Bool({ let t2162 = Py::Float(1e-07); if py::gt(&t2161, &t2162)? { true } else { false } }) }, Py::str("consecutive vertices must be distinct"))?;
        let _ = f__need({ let t2164 = f__sph_sep(py::getitem(&v_lats.clone(), &v_i.clone())?, py::getitem(&v_lons.clone(), &v_i.clone())?, py::getitem(&v_lats.clone(), &v_j.clone())?, py::getitem(&v_lons.clone(), &v_j.clone())?)?; Py::Bool({ let t2165 = Py::Int(90i64); if py::le(&t2164, &t2165)? { true } else { false } }) }, Py::str("each edge must span at most 90 degrees"))?;
        v_inv = f__geod_inverse(v_E.clone(), py::getitem(&v_lats.clone(), &v_i.clone())?, py::getitem(&v_lons.clone(), &v_i.clone())?, py::getitem(&v_lats.clone(), &v_j.clone())?, py::getitem(&v_lons.clone(), &v_j.clone())?)?;
        let _ = py::method(&v_perim.clone(), "append", &[py::getitem(&v_inv.clone(), &Py::str("s"))?])?;
        let t2194 = (Py::tuple(vec![py::getitem(&v_inv.clone(), &Py::str("sa0"))?, py::getitem(&v_inv.clone(), &Py::str("ca0"))?, py::getitem(&v_inv.clone(), &Py::str("s1"))?, py::getitem(&v_inv.clone(), &Py::str("k2"))?, py::getitem(&v_inv.clone(), &Py::str("sig"))?])).items()?;
        if t2194.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_sa0 = t2194[0].clone();
        v_ca0 = t2194[1].clone();
        v_s1 = t2194[2].clone();
        v_k2 = t2194[3].clone();
        v_sig = t2194[4].clone();
        v_integrand = { let v_E = v_E.clone(); let v_a2 = v_a2.clone(); let v_ca0 = v_ca0.clone(); let v_dlam_sum = v_dlam_sum.clone(); let v_ellipsoid = v_ellipsoid.clone(); let v_f = v_f.clone(); let v_i = v_i.clone(); let v_integrand = v_integrand.clone(); let v_inv = v_inv.clone(); let v_j = v_j.clone(); let v_k2 = v_k2.clone(); let v_lats = v_lats.clone(); let v_lons = v_lons.clone(); let v_n = v_n.clone(); let v_perim = v_perim.clone(); let v_qc = v_qc.clone(); let v_s1 = v_s1.clone(); let v_sa0 = v_sa0.clone(); let v_sig = v_sig.clone(); let v_total = v_total.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_cb2: Py = Py::None;
            let mut v_dl: Py = Py::None;
            let mut v_phi: Py = Py::None;
            let mut v_sb: Py = Py::None;
            v_sb = py::mul(&v_ca0.clone(), &py::math1("sin", &v_t.clone())?)?;
            v_cb2 = py::add(&py::mul(&v_sa0.clone(), &v_sa0.clone())?, &py::pow(&py::mul(&v_ca0.clone(), &py::math1("cos", &v_t.clone())?)?, &Py::Int(2i64))?)?;
            v_phi = py::atan2(&v_sb.clone(), &py::mul(&py::sub(&Py::Int(1i64), &v_f.clone())?, &py::math1("sqrt", &v_cb2.clone())?)?)?;
            v_dl = py::sub(&py::truediv(&v_sa0.clone(), &v_cb2.clone())?, &py::truediv(&py::mul(&py::mul(&v_f.clone(), &v_sa0.clone())?, &py::sub(&Py::Int(2i64), &v_f.clone())?)?, &py::add(&Py::Int(1i64), &py::mul(&py::sub(&Py::Int(1i64), &v_f.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_k2.clone(), &py::pow(&py::math1("sin", &v_t.clone())?, &Py::Int(2i64))?)?)?)?)?)?)?)?;
            return Ok(py::mul(&py::mul(&py::truediv(&v_a2.clone(), &Py::Int(2i64))?, &py::sub(&f__q_auth(v_E.clone(), v_phi.clone())?, &v_qc.clone())?)?, &v_dl.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        let _ = py::method(&v_total.clone(), "append", &[f__integ(v_integrand.clone(), v_s1.clone(), py::add(&v_s1.clone(), &v_sig.clone())?)?])?;
        v_dlam_sum = py::iadd(&v_dlam_sum, &f__nlon(py::sub(&py::getitem(&v_lons.clone(), &v_j.clone())?, &py::getitem(&v_lons.clone(), &v_i.clone())?)?)?)?;
    }
    let _ = f__need({ let t2195 = py::abs(&v_dlam_sum.clone())?; Py::Bool({ let t2196 = Py::Float(1.0); if py::lt(&t2195, &t2196)? { true } else { false } }) }, Py::str("the polygon must not enclose a pole"))?;
    v_area = py::neg(&py::fsum(&v_total.clone())?)?;
    let _ = f__need({ let t2197 = py::abs(&v_area.clone())?; Py::Bool({ let t2198 = Py::Int(0i64); if py::gt(&t2197, &t2198)? { true } else { false } }) }, Py::str("the polygon has no area"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("area"), py::abs(&v_area.clone())?), (Py::str("perimeter"), py::fsum(&v_perim.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geodetic_to_ecef(mut v_lat: Py, mut v_lon: Py, mut v_h: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_Z: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__geo_point(v_lat.clone(), v_lon.clone(), v_h.clone(), Py::str(""))?;
    let t2272 = (f__ecef(v_E.clone(), v_lat.clone(), v_lon.clone(), v_h.clone())?).items()?;
    if t2272.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t2272[0].clone();
    v_Y = t2272[1].clone();
    v_Z = t2272[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("X"), v_X.clone()), (Py::str("Y"), v_Y.clone()), (Py::str("Z"), v_Z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geodetic_to_enu(mut v_lat0: Py, mut v_lon0: Py, mut v_h0: Py, mut v_lat: Py, mut v_lon: Py, mut v_h: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__geo_point(v_lat0.clone(), v_lon0.clone(), v_h0.clone(), Py::str("the origin's "))?;
    let _ = f__geo_point(v_lat.clone(), v_lon.clone(), v_h.clone(), Py::str(""))?;
    let _ = f__need({ let t4157 = py::abs(&v_lat0.clone())?; Py::Bool({ let t4158 = Py::Int(90i64); if py::lt(&t4157, &t4158)? { true } else { false } }) }, Py::str("the origin must not be a pole"))?;
    v_o = f__ecef(v_E.clone(), v_lat0.clone(), v_lon0.clone(), v_h0.clone())?;
    v_t = f__ecef(v_E.clone(), v_lat.clone(), v_lon.clone(), v_h.clone())?;
    v_d = { let mut out = vec![]; for t4161 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t4161;
out.push(py::sub(&py::getitem(&v_t.clone(), &v_i.clone())?, &py::getitem(&v_o.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_R = f__enu_rot(v_lat0.clone(), v_lon0.clone())?;
    let t4166 = ({ let mut out = vec![]; for t4164 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t4164;
out.push(py::fsum(&{ let mut out = vec![]; for t4165 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t4165;
out.push(py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_d.clone(), &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }).items()?;
    if t4166.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t4166[0].clone();
    v_n = t4166[1].clone();
    v_u = t4166[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("e"), v_e.clone()), (Py::str("n"), v_n.clone()), (Py::str("u"), v_u.clone()), (Py::str("slant_range"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t4179 in py::iter(&(v_d.clone()))? { let mut v_x: Py; 
v_x = t4179;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) })?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geohash_decode(mut v_geohash: Py) -> R {
    let mut v_la: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let _ = f__need({ let t2411 = Py::Bool(matches!(v_geohash.clone(), Py::Str(_))); if !t2411.truthy() { t2411 } else { { let t2408 = Py::Int(1i64); Py::Bool({ let t2409 = py::len(&v_geohash.clone())?; if py::le(&t2408, &t2409)? { { let t2410 = Py::Int(12i64); if py::le(&t2409, &t2410)? { true } else { false } } } else { false } }) } } }, Py::str("a geohash has 1 to 12 characters"))?;
    let t2417 = (f__gh_bounds(py::method(&v_geohash.clone(), "lower", &[])?)?).items()?;
    if t2417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_la = t2417[0].clone();
    v_lo = t2417[1].clone();
    return Ok(Py::dict(vec![(Py::str("lat_center"), py::truediv(&py::add(&py::getitem(&v_la.clone(), &Py::Int(0i64))?, &py::getitem(&v_la.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?), (Py::str("lon_center"), py::truediv(&py::add(&py::getitem(&v_lo.clone(), &Py::Int(0i64))?, &py::getitem(&v_lo.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?), (Py::str("lat_error"), py::truediv(&py::sub(&py::getitem(&v_la.clone(), &Py::Int(1i64))?, &py::getitem(&v_la.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?), (Py::str("lon_error"), py::truediv(&py::sub(&py::getitem(&v_lo.clone(), &Py::Int(1i64))?, &py::getitem(&v_lo.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geohash_encode(mut v_lat: Py, mut v_lon: Py, mut v_precision: Py) -> R {
    let mut v_bits: Py = Py::None;
    let mut v_code: Py = Py::None;
    let mut v_even: Py = Py::None;
    let mut v_la: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(true))?;
    let _ = f__lon(Py::str("lon"), v_lon.clone())?;
    let _ = f__need({ let t2474 = Py::Bool(matches!(v_precision.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2474.truthy() { t2474 } else { { let t2471 = Py::Int(1i64); Py::Bool({ let t2472 = v_precision.clone(); if py::le(&t2471, &t2472)? { { let t2473 = Py::Int(12i64); if py::le(&t2472, &t2473)? { true } else { false } } } else { false } }) } } }, Py::str("the precision must be a whole number of characters from 1 to 12"))?;
    v_lon = (if ({ let t2475 = v_lon.clone(); Py::Bool({ let t2476 = Py::Int(180i64); if py::ne(&t2475, &t2476) { true } else { false } }) }).truthy() { f__nlon(v_lon.clone())? } else { Py::Float(180.0) });
    let t2481 = (Py::tuple(vec![Py::list(vec![py::neg(&Py::Float(90.0))?, Py::Float(90.0)]), Py::list(vec![py::neg(&Py::Float(180.0))?, Py::Float(180.0)])])).items()?;
    if t2481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_la = t2481[0].clone();
    v_lo = t2481[1].clone();
    let t2482 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64), Py::Int(0i64), Py::Bool(true)])).items()?;
    if t2482.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_code = t2482[0].clone();
    v_bits = t2482[1].clone();
    v_v = t2482[2].clone();
    v_even = t2482[3].clone();
    while ({ let t2489 = py::len(&v_code.clone())?; Py::Bool({ let t2490 = v_precision.clone(); if py::lt(&t2489, &t2490)? { true } else { false } }) }).truthy() {
        let t2483 = ((if (v_even.clone()).truthy() { Py::tuple(vec![v_lo.clone(), v_lon.clone()]) } else { Py::tuple(vec![v_la.clone(), v_lat.clone()]) })).items()?;
        if t2483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t2483[0].clone();
        v_x = t2483[1].clone();
        v_mid = py::truediv(&py::add(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::getitem(&v_r.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?;
        if ({ let t2484 = v_x.clone(); Py::Bool({ let t2485 = v_mid.clone(); if py::ge(&t2484, &t2485)? { true } else { false } }) }).truthy() {
            v_v = py::add(&py::mul(&v_v.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?;
            py::setitem(&v_r.clone(), &Py::Int(0i64), v_mid.clone())?;
        } else {
            v_v = py::mul(&v_v.clone(), &Py::Int(2i64))?;
            py::setitem(&v_r.clone(), &Py::Int(1i64), v_mid.clone())?;
        }
        v_even = py::not(&v_even.clone());
        v_bits = py::iadd(&v_bits, &Py::Int(1i64))?;
        if ({ let t2486 = v_bits.clone(); Py::Bool({ let t2487 = Py::Int(5i64); if py::eq(&t2486, &t2487) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_code.clone(), "append", &[py::getitem(&c__GH()?, &v_v.clone())?])?;
            let t2488 = (Py::tuple(vec![Py::Int(0i64), Py::Int(0i64)])).items()?;
            if t2488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_bits = t2488[0].clone();
            v_v = t2488[1].clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("geohash"), py::method(&Py::str(""), "join", &[v_code.clone()])?), (Py::str("lat_center"), py::truediv(&py::add(&py::getitem(&v_la.clone(), &Py::Int(0i64))?, &py::getitem(&v_la.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?), (Py::str("lon_center"), py::truediv(&py::add(&py::getitem(&v_lo.clone(), &Py::Int(0i64))?, &py::getitem(&v_lo.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?), (Py::str("lat_error"), py::truediv(&py::sub(&py::getitem(&v_la.clone(), &Py::Int(1i64))?, &py::getitem(&v_la.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?), (Py::str("lon_error"), py::truediv(&py::sub(&py::getitem(&v_lo.clone(), &Py::Int(1i64))?, &py::getitem(&v_lo.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gnomonic_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_R: Py) -> R {
    let mut v_cc: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Int(90i64))?;
    let _ = f__lat(Py::str("lat0"), v_lat0.clone(), Py::Bool(true))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__sphere_R(v_R.clone())?;
    let t2663 = (Py::tuple(vec![py::mul(&v_lat.clone(), &c_DEG()?)?, py::mul(&v_lat0.clone(), &c_DEG()?)?, py::mul(&f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?, &c_DEG()?)?])).items()?;
    if t2663.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2663[0].clone();
    v_p0 = t2663[1].clone();
    v_dl = t2663[2].clone();
    v_cc = py::add(&py::mul(&py::math1("sin", &v_p0.clone())?, &py::math1("sin", &v_p.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_p0.clone())?, &py::math1("cos", &v_p.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?;
    let _ = f__need({ let t2664 = v_cc.clone(); Py::Bool({ let t2665 = Py::Float(0.1); if py::ge(&t2664, &t2665)? { true } else { false } }) }, Py::str("the point must lie within about 84 degrees of the centre"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::truediv(&py::mul(&py::mul(&v_R.clone(), &py::math1("cos", &v_p.clone())?)?, &py::math1("sin", &v_dl.clone())?)?, &v_cc.clone())?), (Py::str("y"), py::truediv(&py::mul(&v_R.clone(), &py::sub(&py::mul(&py::math1("cos", &v_p0.clone())?, &py::math1("sin", &v_p.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_p0.clone())?, &py::math1("cos", &v_p.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?)?, &v_cc.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graticule_cell_area(mut v_lat1: Py, mut v_lat2: Py, mut v_lon1: Py, mut v_lon2: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_area: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_qa: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat1"), v_lat1.clone(), Py::Bool(true))?;
    let _ = f__lat(Py::str("lat2"), v_lat2.clone(), Py::Bool(true))?;
    let _ = f__need({ let t2731 = v_lat2.clone(); Py::Bool({ let t2732 = v_lat1.clone(); if py::gt(&t2731, &t2732)? { true } else { false } }) }, Py::str("lat2 must be north of lat1"))?;
    let _ = f__lon(Py::str("lon1"), v_lon1.clone())?;
    let _ = f__lon(Py::str("lon2"), v_lon2.clone())?;
    v_dl = py::sub(&v_lon2.clone(), &v_lon1.clone())?;
    let _ = f__need({ let t2736 = Py::Int(0i64); Py::Bool({ let t2737 = v_dl.clone(); if py::lt(&t2736, &t2737)? { { let t2738 = Py::Int(360i64); if py::le(&t2737, &t2738)? { true } else { false } } } else { false } }) }, Py::str("lon2 must lie east of lon1 by at most 360 degrees"))?;
    v_qa = py::mul(&py::truediv(&py::pow(&py::getitem(&v_E.clone(), &Py::str("a"))?, &Py::Int(2i64))?, &Py::Int(2i64))?, &py::sub(&f__q_auth(v_E.clone(), py::mul(&v_lat2.clone(), &c_DEG()?)?)?, &f__q_auth(v_E.clone(), py::mul(&v_lat1.clone(), &c_DEG()?)?)?)?)?;
    v_total = py::mul(&py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&py::getitem(&v_E.clone(), &Py::str("a"))?, &Py::Int(2i64))?)?, &Py::Int(2i64))?, &f__q_auth(v_E.clone(), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?)?, &Py::Int(2i64))?;
    v_area = py::mul(&py::mul(&v_qa.clone(), &v_dl.clone())?, &c_DEG()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("area"), v_area.clone()), (Py::str("fraction_of_ellipsoid"), py::truediv(&v_area.clone(), &v_total.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gravity_accessibility(mut v_edges: Py, mut v_opportunities: Py, mut v_beta: Py, mut v_impedance: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_opportunities.clone())?;
    v_adj = f__net(v_edges.clone(), v_n.clone(), Py::Bool(true))?;
    for t2818 in py::iter(&(v_opportunities.clone()))? {
        v_v = t2818;
        let _ = f__nonneg(Py::str("each opportunity"), Py::tuple(vec![v_v.clone()]))?;
    }
    let _ = f__pos(Py::str("beta"), Py::tuple(vec![v_beta.clone()]))?;
    let _ = f__need({ let t2831 = v_impedance.clone(); Py::Bool({ let t2832 = Py::tuple(vec![Py::str("power"), Py::str("exponential")]); if py::contains(&t2832, &t2831)? { true } else { false } }) }, Py::str("the impedance function must be power or exponential"))?;
    v_acc = Py::list(vec![]);
    for t2833 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2833;
        v_d = f__dijkstra(v_adj.clone(), v_i.clone())?;
        let _ = py::method(&v_acc.clone(), "append", &[py::fsum(&{ let mut out = vec![]; for t2841 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2841;
if ({ let t2844 = { let t2842 = v_j.clone(); Py::Bool({ let t2843 = v_i.clone(); if py::ne(&t2842, &t2843) { true } else { false } }) }; if !t2844.truthy() { t2844 } else { py::math1("isfinite", &py::getitem(&v_d.clone(), &v_j.clone())?)? } }).truthy() { out.push(py::mul(&py::getitem(&v_opportunities.clone(), &v_j.clone())?, &(if ({ let t2845 = v_impedance.clone(); Py::Bool({ let t2846 = Py::str("power"); if py::eq(&t2845, &t2846) { true } else { false } }) }).truthy() { py::pow(&py::getitem(&v_d.clone(), &v_j.clone())?, &py::neg(&v_beta.clone())?)? } else { py::math1("exp", &py::mul(&py::neg(&v_beta.clone())?, &py::getitem(&v_d.clone(), &v_j.clone())?)?)? }))?); }} Py::list(out) })?])?;
    }
    return Ok(Py::dict(vec![(Py::str("accessibility"), f__r(v_acc.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gravity_poisson(mut v_T: Py, mut v_O: Py, mut v_D: Py, mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_mT: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_zw: Py = Py::None;
    v_n = py::len(&v_T.clone())?;
    let _ = f__need({ let t2904 = { let t2898 = v_n.clone(); Py::Bool({ let t2899 = py::len(&v_O.clone())?; if py::eq(&t2898, &t2899) { { let t2900 = py::len(&v_D.clone())?; if py::eq(&t2899, &t2900) { { let t2901 = py::len(&v_d.clone())?; if py::eq(&t2900, &t2901) { true } else { false } } } else { false } } } else { false } }) }; if !t2904.truthy() { t2904 } else { { let t2902 = v_n.clone(); Py::Bool({ let t2903 = Py::Int(6i64); if py::ge(&t2902, &t2903)? { true } else { false } }) } } }, Py::str("give the flow, origin mass, destination mass and distance of each pair (at least 6 pairs)"))?;
    for t2905 in py::iter(&(v_T.clone()))? {
        v_v = t2905;
        let _ = f__nonneg(Py::str("each flow"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t2912 in py::iter(&(py::add(&py::add(&Py::list((v_O.clone()).items()?), &Py::list((v_D.clone()).items()?))?, &Py::list((v_d.clone()).items()?))?))? {
        v_v = t2912;
        let _ = f__pos(Py::str("masses and distances"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_X = Py::list(vec![py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?, { let mut out = vec![]; for t2919 in py::iter(&(v_O.clone()))? { let mut v_v: Py; 
v_v = t2919;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t2920 in py::iter(&(v_D.clone()))? { let mut v_v: Py; 
v_v = t2920;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t2921 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t2921;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }]);
    for t2922 in py::iter(&(py::slice(&v_X.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_c = t2922;
        let _ = f__need({ let t2923 = py::sub(&py::minmax(&[v_c.clone()], true)?, &py::minmax(&[v_c.clone()], false)?)?; Py::Bool({ let t2924 = Py::Int(0i64); if py::gt(&t2923, &t2924)? { true } else { false } }) }, Py::str("masses and distances must vary across the pairs"))?;
    }
    v_mT = py::truediv(&py::fsum(&v_T.clone())?, &v_n.clone())?;
    let _ = f__need({ let t2925 = v_mT.clone(); Py::Bool({ let t2926 = Py::Int(0i64); if py::gt(&t2925, &t2926)? { true } else { false } }) }, Py::str("the flows must not all be zero"))?;
    v_beta = Py::list(vec![py::math1("log", &v_mT.clone())?, Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]);
    v_delta = Py::Float(f64::INFINITY);
    for t2927 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v_it = t2927;
        v_eta = { let mut out = vec![]; for t2928 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2928;
out.push(py::fsum(&{ let mut out = vec![]; for t2929 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_k: Py; 
v_k = t2929;
out.push(py::mul(&py::getitem(&v_beta.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_k.clone())?, &v_i.clone())?)?); } Py::list(out) })?); } Py::list(out) };
        v_mu = { let mut out = vec![]; for t2930 in py::iter(&(v_eta.clone()))? { let mut v_e: Py; 
v_e = t2930;
out.push(py::math1("exp", &v_e.clone())?); } Py::list(out) };
        v_zw = { let mut out = vec![]; for t2931 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2931;
out.push(py::add(&py::getitem(&v_eta.clone(), &v_i.clone())?, &py::truediv(&py::sub(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::getitem(&v_mu.clone(), &v_i.clone())?)?, &py::getitem(&v_mu.clone(), &v_i.clone())?)?)?); } Py::list(out) };
        v_A = { let mut out = vec![]; for t2932 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_a: Py; 
v_a = t2932;
out.push({ let mut out = vec![]; for t2933 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_b: Py; 
v_b = t2933;
out.push(py::fsum(&{ let mut out = vec![]; for t2934 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2934;
out.push(py::mul(&py::mul(&py::getitem(&v_mu.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_a.clone())?, &v_i.clone())?)?, &py::getitem(&py::getitem(&v_X.clone(), &v_b.clone())?, &v_i.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
        v_rhs = { let mut out = vec![]; for t2935 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_a: Py; 
v_a = t2935;
out.push(py::fsum(&{ let mut out = vec![]; for t2936 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2936;
out.push(py::mul(&py::mul(&py::getitem(&v_mu.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_a.clone())?, &v_i.clone())?)?, &py::getitem(&v_zw.clone(), &v_i.clone())?)?); } Py::list(out) })?); } Py::list(out) };
        v_new = f__solve_lin(v_A.clone(), v_rhs.clone())?;
        v_delta = py::minmax(&[{ let mut out = vec![]; for t2954 in py::iter(&(py::zip(&[v_new.clone(), v_beta.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2955 = (t2954).items()?;
if t2955.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2955[0].clone();
v_b = t2955[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) }], true)?;
        v_beta = v_new.clone();
        if ({ let t2956 = v_delta.clone(); Py::Bool({ let t2957 = py::mul(&Py::Float(1e-14), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2958 in py::iter(&(v_beta.clone()))? { let mut v_v: Py; 
v_v = t2958;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?)?; if py::lt(&t2956, &t2957)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    let _ = f__need({ let t2959 = v_delta.clone(); Py::Bool({ let t2960 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2961 in py::iter(&(v_beta.clone()))? { let mut v_v: Py; 
v_v = t2961;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?)?; if py::lt(&t2959, &t2960)? { true } else { false } }) }, Py::str("the Poisson fit did not converge"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("k"), py::math1("exp", &py::getitem(&v_beta.clone(), &Py::Int(0i64))?)?), (Py::str("alpha"), py::getitem(&v_beta.clone(), &Py::Int(1i64))?), (Py::str("beta"), py::getitem(&v_beta.clone(), &Py::Int(2i64))?), (Py::str("gamma"), py::neg(&py::getitem(&v_beta.clone(), &Py::Int(3i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ground_sample_distance(mut v_pixel_pitch: Py, mut v_focal_length: Py, mut v_altitude: Py, mut v_pixels: Py) -> R {
    let mut v_g: Py = Py::None;
    let _ = f__pos(Py::str("the pixel pitch, focal length and altitude"), Py::tuple(vec![v_pixel_pitch.clone(), v_focal_length.clone(), v_altitude.clone()]))?;
    let _ = f__need({ let t3023 = Py::Bool(matches!(v_pixels.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3023.truthy() { t3023 } else { { let t3021 = v_pixels.clone(); Py::Bool({ let t3022 = Py::Int(1i64); if py::ge(&t3021, &t3022)? { true } else { false } }) } } }, Py::str("the number of pixels across must be a whole number >= 1"))?;
    v_g = py::truediv(&py::mul(&v_pixel_pitch.clone(), &v_altitude.clone())?, &v_focal_length.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("gsd"), v_g.clone()), (Py::str("swath"), py::mul(&v_g.clone(), &v_pixels.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hillshade(mut v_window: Py, mut v_cell_size: Py, mut v_sun_azimuth: Py, mut v_sun_altitude: Py) -> R {
    let mut v_asp: Py = Py::None;
    let mut v_ci: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_zen: Py = Py::None;
    let _ = f__window(v_window.clone())?;
    let _ = f__pos(Py::str("the cell size"), Py::tuple(vec![v_cell_size.clone()]))?;
    let _ = f__fin(Py::str("the sun azimuth and altitude"), Py::tuple(vec![v_sun_azimuth.clone(), v_sun_altitude.clone()]))?;
    let _ = f__need({ let t3094 = Py::Int(0i64); Py::Bool({ let t3095 = v_sun_altitude.clone(); if py::le(&t3094, &t3095)? { { let t3096 = Py::Int(90i64); if py::le(&t3095, &t3096)? { true } else { false } } } else { false } }) }, Py::str("the sun altitude must lie between 0 and 90 degrees"))?;
    let t3097 = (f__horn(v_window.clone(), v_cell_size.clone())?).items()?;
    if t3097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3097[0].clone();
    v_q = t3097[1].clone();
    v_s = py::math1("atan", &py::hypot(&[v_p.clone(), v_q.clone()])?)?;
    v_zen = py::mul(&py::sub(&Py::Int(90i64), &v_sun_altitude.clone())?, &c_DEG()?)?;
    if ({ let t3102 = { let t3098 = v_p.clone(); Py::Bool({ let t3099 = Py::Int(0i64); if py::eq(&t3098, &t3099) { true } else { false } }) }; if !t3102.truthy() { t3102 } else { { let t3100 = v_q.clone(); Py::Bool({ let t3101 = Py::Int(0i64); if py::eq(&t3100, &t3101) { true } else { false } }) } } }).truthy() {
        v_ci = py::math1("cos", &v_zen.clone())?;
    } else {
        v_asp = py::atan2(&py::neg(&v_p.clone())?, &py::neg(&v_q.clone())?)?;
        v_ci = py::add(&py::mul(&py::math1("cos", &v_zen.clone())?, &py::math1("cos", &v_s.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_zen.clone())?, &py::math1("sin", &v_s.clone())?)?, &py::math1("cos", &py::sub(&py::mul(&v_sun_azimuth.clone(), &c_DEG()?)?, &v_asp.clone())?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("illumination"), v_ci.clone()), (Py::str("hillshade"), py::mul(&Py::Int(255i64), &py::minmax(&[Py::Float(0.0), v_ci.clone()], true)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_huff_model(mut v_edges: Py, mut v_origin: Py, mut v_stores: Py, mut v_attractiveness: Py, mut v_alpha: Py, mut v_beta: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__net(v_edges.clone(), Py::None, Py::Bool(true))?;
    v_n = py::len(&v_adj.clone())?;
    v_o = py::int(&v_origin.clone())?;
    let _ = f__need({ let t3187 = { let t3182 = Py::Int(0i64); Py::Bool({ let t3183 = v_o.clone(); if py::le(&t3182, &t3183)? { { let t3184 = v_n.clone(); if py::lt(&t3183, &t3184)? { true } else { false } } } else { false } }) }; if !t3187.truthy() { t3187 } else { { let t3185 = py::float(&v_origin.clone())?; Py::Bool({ let t3186 = v_o.clone(); if py::eq(&t3185, &t3186) { true } else { false } }) } } }, Py::str("the origin must be a node of the network"))?;
    v_S = { let mut out = vec![]; for t3188 in py::iter(&(v_stores.clone()))? { let mut v_s: Py; 
v_s = t3188;
out.push(py::int(&v_s.clone())?); } Py::list(out) };
    let _ = f__need({ let t3199 = { let t3189 = py::len(&v_S.clone())?; Py::Bool({ let t3190 = Py::Int(1i64); if py::ge(&t3189, &t3190)? { true } else { false } }) }; if !t3199.truthy() { t3199 } else { Py::Bool(({ let mut out = vec![]; for t3191 in py::iter(&(py::zip(&[v_stores.clone(), v_S.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3192 = (t3191).items()?;
if t3192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3192[0].clone();
v_b = t3192[1].clone();
out.push({ let t3198 = { let t3193 = py::float(&v_a.clone())?; Py::Bool({ let t3194 = v_b.clone(); if py::eq(&t3193, &t3194) { true } else { false } }) }; if !t3198.truthy() { t3198 } else { { let t3195 = Py::Int(0i64); Py::Bool({ let t3196 = v_b.clone(); if py::le(&t3195, &t3196)? { { let t3197 = v_n.clone(); if py::lt(&t3196, &t3197)? { true } else { false } } } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the stores must be nodes of the network"))?;
    let _ = f__need({ let t3200 = py::len(&py::set(&v_S.clone())?)?; Py::Bool({ let t3201 = py::len(&v_S.clone())?; if py::eq(&t3200, &t3201) { true } else { false } }) }, Py::str("each store node may be listed once"))?;
    let _ = f__need({ let t3202 = py::len(&v_attractiveness.clone())?; Py::Bool({ let t3203 = py::len(&v_S.clone())?; if py::eq(&t3202, &t3203) { true } else { false } }) }, Py::str("give one attractiveness per store"))?;
    for t3204 in py::iter(&(v_attractiveness.clone()))? {
        v_v = t3204;
        let _ = f__pos(Py::str("each attractiveness"), Py::tuple(vec![v_v.clone()]))?;
    }
    let _ = f__fin(Py::str("alpha and beta"), Py::tuple(vec![v_alpha.clone(), v_beta.clone()]))?;
    let _ = f__need({ let t3214 = v_beta.clone(); Py::Bool({ let t3215 = Py::Int(0i64); if py::gt(&t3214, &t3215)? { true } else { false } }) }, Py::str("beta must be positive"))?;
    v_d = f__dijkstra(v_adj.clone(), v_o.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3223 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t3223;
out.push(py::math1("isfinite", &py::getitem(&v_d.clone(), &v_s.clone())?)?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("every store must be reachable from the origin"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3224 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t3224;
out.push({ let t3225 = py::getitem(&v_d.clone(), &v_s.clone())?; Py::Bool({ let t3226 = Py::Int(0i64); if py::gt(&t3225, &t3226)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("a store at the origin node has zero distance (the model is undefined)"))?;
    v_u = { let mut out = vec![]; for t3227 in py::iter(&(py::enumerate(&v_S.clone(), 0)?))? { let mut v_k: Py; let mut v_s: Py; 
let t3228 = (t3227).items()?;
if t3228.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3228[0].clone();
v_s = t3228[1].clone();
out.push(py::mul(&py::pow(&py::getitem(&v_attractiveness.clone(), &v_k.clone())?, &v_alpha.clone())?, &py::pow(&py::getitem(&v_d.clone(), &v_s.clone())?, &py::neg(&v_beta.clone())?)?)?); } Py::list(out) };
    v_t = py::fsum(&v_u.clone())?;
    return Ok(Py::dict(vec![(Py::str("probabilities"), f__r({ let mut out = vec![]; for t3239 in py::iter(&(v_u.clone()))? { let mut v_v: Py; 
v_v = t3239;
out.push(py::truediv(&v_v.clone(), &v_t.clone())?); } Py::list(out) })?), (Py::str("distances"), f__r({ let mut out = vec![]; for t3240 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t3240;
out.push(py::getitem(&v_d.clone(), &v_s.clone())?); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_idw(mut v_x: Py, mut v_y: Py, mut v_z: Py, mut v_px: Py, mut v_py: Py, mut v_power: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t3287 = { let t3282 = v_n.clone(); Py::Bool({ let t3283 = py::len(&v_y.clone())?; if py::eq(&t3282, &t3283) { { let t3284 = py::len(&v_z.clone())?; if py::eq(&t3283, &t3284) { true } else { false } } } else { false } }) }; if !t3287.truthy() { t3287 } else { { let t3285 = v_n.clone(); Py::Bool({ let t3286 = Py::Int(1i64); if py::ge(&t3285, &t3286)? { true } else { false } }) } } }, Py::str("give one y and one value per x"))?;
    let _ = f__fin(Py::str("the query point"), Py::tuple(vec![v_px.clone(), v_py.clone()]))?;
    let _ = f__pos(Py::str("the power"), Py::tuple(vec![v_power.clone()]))?;
    let _ = f__need({ let t3297 = v_power.clone(); Py::Bool({ let t3298 = Py::Int(10i64); if py::le(&t3297, &t3298)? { true } else { false } }) }, Py::str("the power must be at most 10"))?;
    v_d = { let mut out = vec![]; for t3299 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3300 = (t3299).items()?;
if t3300.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3300[0].clone();
v_b = t3300[1].clone();
out.push(py::hypot(&[py::sub(&v_a.clone(), &v_px.clone())?, py::sub(&v_b.clone(), &v_py.clone())?])?); } Py::list(out) };
    for t3301 in py::iter(&(py::enumerate(&v_d.clone(), 0)?))? {
        let t3316 = (t3301).items()?;
        if t3316.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t3316[0].clone();
        v_v = t3316[1].clone();
        if ({ let t3302 = v_v.clone(); Py::Bool({ let t3303 = Py::Int(0i64); if py::eq(&t3302, &t3303) { true } else { false } }) }).truthy() {
            return Ok(f__out(Py::dict(vec![(Py::str("estimate"), py::getitem(&v_z.clone(), &v_i.clone())?)]))?);
        }
    }
    v_w = { let mut out = vec![]; for t3317 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t3317;
out.push(py::pow(&v_v.clone(), &py::neg(&v_power.clone())?)?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("estimate"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t3318 in py::iter(&(py::zip(&[v_w.clone(), v_z.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3319 = (t3318).items()?;
if t3319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3319[0].clone();
v_b = t3319[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &py::fsum(&v_w.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_join_counts(mut v_y: Py, mut v_W: Py) -> R {
    let mut v_Wt: Py = Py::None;
    let mut v_bb: Py = Py::None;
    let mut v_bw: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ww: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__need({ let t3361 = v_n.clone(); Py::Bool({ let t3362 = Py::Int(3i64); if py::ge(&t3361, &t3362)? { true } else { false } }) }, Py::str("give at least 3 observations"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3363 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t3363;
out.push({ let t3364 = v_v.clone(); Py::Bool({ let t3365 = Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]); if py::contains(&t3365, &t3364)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the variable must be binary (1 black, 0 white)"))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), Py::str("none"), Py::Bool(true), Py::Bool(true), Py::Bool(true))?;
    let t3395 = Py::Int(0i64);
    v_bb = t3395.clone();
    v_bw = t3395.clone();
    v_ww = t3395.clone();
    for t3396 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3396;
        for t3397 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t3397;
            if (py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?).truthy() {
                if ({ let t3402 = { let t3398 = py::getitem(&v_y.clone(), &v_i.clone())?; Py::Bool({ let t3399 = Py::Int(1i64); if py::eq(&t3398, &t3399) { true } else { false } }) }; if !t3402.truthy() { t3402 } else { { let t3400 = py::getitem(&v_y.clone(), &v_j.clone())?; Py::Bool({ let t3401 = Py::Int(1i64); if py::eq(&t3400, &t3401) { true } else { false } }) } } }).truthy() {
                    v_bb = py::iadd(&v_bb, &Py::Int(1i64))?;
                } else {
                    if ({ let t3407 = { let t3403 = py::getitem(&v_y.clone(), &v_i.clone())?; Py::Bool({ let t3404 = Py::Int(0i64); if py::eq(&t3403, &t3404) { true } else { false } }) }; if !t3407.truthy() { t3407 } else { { let t3405 = py::getitem(&v_y.clone(), &v_j.clone())?; Py::Bool({ let t3406 = Py::Int(0i64); if py::eq(&t3405, &t3406) { true } else { false } }) } } }).truthy() {
                        v_ww = py::iadd(&v_ww, &Py::Int(1i64))?;
                    } else {
                        v_bw = py::iadd(&v_bw, &Py::Int(1i64))?;
                    }
                }
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("BB"), py::float(&v_bb.clone())?), (Py::str("BW"), py::float(&v_bw.clone())?), (Py::str("WW"), py::float(&v_ww.clone())?), (Py::str("J"), py::float(&py::add(&py::add(&v_bb.clone(), &v_bw.clone())?, &v_ww.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kernel_density(mut v_x: Py, mut v_y: Py, mut v_px: Py, mut v_py: Py, mut v_bandwidth: Py, mut v_kernel: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_u2: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t3453 = { let t3449 = v_n.clone(); Py::Bool({ let t3450 = py::len(&v_y.clone())?; if py::eq(&t3449, &t3450) { true } else { false } }) }; if !t3453.truthy() { t3453 } else { { let t3451 = v_n.clone(); Py::Bool({ let t3452 = Py::Int(1i64); if py::ge(&t3451, &t3452)? { true } else { false } }) } } }, Py::str("give one y per x"))?;
    let _ = f__fin(Py::str("the query point"), Py::tuple(vec![v_px.clone(), v_py.clone()]))?;
    let _ = f__pos(Py::str("the bandwidth"), Py::tuple(vec![v_bandwidth.clone()]))?;
    let _ = f__need({ let t3463 = v_kernel.clone(); Py::Bool({ let t3464 = Py::tuple(vec![Py::str("gaussian"), Py::str("epanechnikov"), Py::str("uniform")]); if py::contains(&t3464, &t3463)? { true } else { false } }) }, Py::str("the kernel must be gaussian, epanechnikov or uniform"))?;
    v_h = v_bandwidth.clone();
    v_terms = Py::list(vec![]);
    for t3465 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? {
        let t3474 = (t3465).items()?;
        if t3474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3474[0].clone();
        v_b = t3474[1].clone();
        v_u2 = py::truediv(&py::add(&py::pow(&py::sub(&v_a.clone(), &v_px.clone())?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_b.clone(), &v_py.clone())?, &Py::Int(2i64))?)?, &py::mul(&v_h.clone(), &v_h.clone())?)?;
        if ({ let t3466 = v_kernel.clone(); Py::Bool({ let t3467 = Py::str("gaussian"); if py::eq(&t3466, &t3467) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_terms.clone(), "append", &[py::truediv(&py::math1("exp", &py::truediv(&py::neg(&v_u2.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_h.clone())?, &v_h.clone())?)?])?;
        } else {
            if ({ let t3468 = v_kernel.clone(); Py::Bool({ let t3469 = Py::str("epanechnikov"); if py::eq(&t3468, &t3469) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_terms.clone(), "append", &[(if ({ let t3470 = v_u2.clone(); Py::Bool({ let t3471 = Py::Int(1i64); if py::lt(&t3470, &t3471)? { true } else { false } }) }).truthy() { py::mul(&py::truediv(&Py::Int(2i64), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_h.clone())?, &v_h.clone())?)?, &py::sub(&Py::Int(1i64), &v_u2.clone())?)? } else { Py::Float(0.0) })])?;
            } else {
                let _ = py::method(&v_terms.clone(), "append", &[(if ({ let t3472 = v_u2.clone(); Py::Bool({ let t3473 = Py::Int(1i64); if py::lt(&t3472, &t3473)? { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_h.clone())?, &v_h.clone())?)? } else { Py::Float(0.0) })])?;
            }
        }
    }
    v_s = py::fsum(&v_terms.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("density"), py::truediv(&v_s.clone(), &v_n.clone())?), (Py::str("intensity"), v_s.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_knn_weights(mut v_x: Py, mut v_y: Py, mut v_k: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t3528 = v_n.clone(); Py::Bool({ let t3529 = py::len(&v_y.clone())?; if py::eq(&t3528, &t3529) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t3533 = Py::Bool(matches!(v_k.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3533.truthy() { t3533 } else { { let t3530 = Py::Int(1i64); Py::Bool({ let t3531 = v_k.clone(); if py::le(&t3530, &t3531)? { { let t3532 = v_n.clone(); if py::lt(&t3531, &t3532)? { true } else { false } } } else { false } }) } } }, Py::str("k must be a whole number from 1 to n - 1"))?;
    let _ = f__need({ let t3534 = py::len(&py::set(&py::zip(&[v_x.clone(), v_y.clone()])?)?)?; Py::Bool({ let t3535 = v_n.clone(); if py::eq(&t3534, &t3535) { true } else { false } }) }, Py::str("the points must be distinct"))?;
    v_W = { let mut out = vec![]; for t3536 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3536;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t3537 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3537;
        v_d = py::sorted(&{ let mut out = vec![]; for t3538 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3538;
if ({ let t3539 = v_j.clone(); Py::Bool({ let t3540 = v_i.clone(); if py::ne(&t3539, &t3540) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![py::hypot(&[py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?])?, v_j.clone()])); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
        if ({ let t3541 = v_k.clone(); Py::Bool({ let t3542 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::lt(&t3541, &t3542)? { true } else { false } }) }).truthy() {
            let _ = f__need({ let t3543 = py::getitem(&py::getitem(&v_d.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?; Py::Bool({ let t3544 = py::getitem(&py::getitem(&v_d.clone(), &v_k.clone())?, &Py::Int(0i64))?; if py::lt(&t3543, &t3544)? { true } else { false } }) }, Py::str("the k-th nearest neighbour is tied: the weights are not unique"))?;
        }
        for t3545 in py::iter(&(py::slice(&v_d.clone(), None, Some((v_k.clone()).i()?), None)?))? {
            let t3546 = (t3545).items()?;
            if t3546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v__ = t3546[0].clone();
            v_j = t3546[1].clone();
            py::setitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone(), Py::Float(1.0))?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("W"), v_W.clone()), (Py::str("s0"), py::float(&py::mul(&v_n.clone(), &v_k.clone())?)?), (Py::str("symmetric"), Py::Bool(({ let mut out = vec![]; for t3547 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3547;
for t3548 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3548;
out.push({ let t3549 = py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t3550 = py::getitem(&py::getitem(&v_W.clone(), &v_j.clone())?, &v_i.clone())?; if py::eq(&t3549, &t3550) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laea_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_Rq: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qp: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Float(89.9))?;
    let _ = f__lat(Py::str("lat0"), v_lat0.clone(), Py::Bool(false))?;
    let _ = f__need({ let t3609 = py::abs(&v_lat0.clone())?; Py::Bool({ let t3610 = Py::Int(89i64); if py::le(&t3609, &t3610)? { true } else { false } }) }, Py::str("the centre must lie within 89 degrees of the equator (the polar aspect is not supported)"))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let _ = f__need({ let t3611 = f__sph_sep(v_lat0.clone(), v_lon0.clone(), v_lat.clone(), v_lon.clone())?; Py::Bool({ let t3612 = Py::Int(120i64); if py::le(&t3611, &t3612)? { true } else { false } }) }, Py::str("the point must lie within 120 degrees of the centre"))?;
    v_q = { let v_E = v_E.clone(); let v_FE = v_FE.clone(); let v_FN = v_FN.clone(); let v_ellipsoid = v_ellipsoid.clone(); let v_lat = v_lat.clone(); let v_lat0 = v_lat0.clone(); let v_lon = v_lon.clone(); let v_lon0 = v_lon0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__q_auth(v_E.clone(), v_p.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_qp = py::call(&v_q.clone(), &[py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?])?;
    v_Rq = py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::math1("sqrt", &py::truediv(&v_qp.clone(), &Py::Int(2i64))?)?)?;
    v_b1 = py::math1("asin", &py::truediv(&py::call(&v_q.clone(), &[py::mul(&v_lat0.clone(), &c_DEG()?)?])?, &v_qp.clone())?)?;
    v_b = py::math1("asin", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::call(&v_q.clone(), &[py::mul(&v_lat.clone(), &c_DEG()?)?])?, &v_qp.clone())?], false)?], true)?)?;
    v_m1 = py::truediv(&py::math1("cos", &py::mul(&v_lat0.clone(), &c_DEG()?)?)?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &py::mul(&v_lat0.clone(), &c_DEG()?)?)?, &Py::Int(2i64))?)?)?)?)?;
    v_D = py::truediv(&py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &v_m1.clone())?, &py::mul(&v_Rq.clone(), &py::math1("cos", &v_b1.clone())?)?)?;
    v_dl = py::mul(&f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?, &c_DEG()?)?;
    v_B = py::mul(&v_Rq.clone(), &py::math1("sqrt", &py::truediv(&Py::Int(2i64), &py::add(&py::add(&Py::Int(1i64), &py::mul(&py::math1("sin", &v_b1.clone())?, &py::math1("sin", &v_b.clone())?)?)?, &py::mul(&py::mul(&py::math1("cos", &v_b1.clone())?, &py::math1("cos", &v_b.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::add(&v_FE.clone(), &py::mul(&py::mul(&py::mul(&v_B.clone(), &v_D.clone())?, &py::math1("cos", &v_b.clone())?)?, &py::math1("sin", &v_dl.clone())?)?)?), (Py::str("y"), py::add(&v_FN.clone(), &py::mul(&py::truediv(&v_B.clone(), &v_D.clone())?, &py::sub(&py::mul(&py::math1("cos", &v_b1.clone())?, &py::math1("sin", &v_b.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_b1.clone())?, &py::math1("cos", &v_b.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lcc_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat1: Py, mut v_lat2: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Int(89i64))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let t3697 = (f__lcc_setup(v_E.clone(), v_lat1.clone(), v_lat2.clone(), v_lat0.clone())?).items()?;
    if t3697.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3697[0].clone();
    v_F = t3697[1].clone();
    v_rho = t3697[2].clone();
    v_m = t3697[3].clone();
    let _ = f__need({ let t3698 = py::mul(&v_lat.clone(), &v_n.clone())?; Py::Bool({ let t3699 = py::mul(&py::neg(&Py::Int(80i64))?, &py::abs(&v_n.clone())?)?; if py::gt(&t3698, &t3699)? { true } else { false } }) }, Py::str("the point lies too close to the pole opposite the cone's apex"))?;
    let t3700 = (Py::tuple(vec![py::call(&v_rho.clone(), &[py::mul(&v_lat.clone(), &c_DEG()?)?])?, py::call(&v_rho.clone(), &[py::mul(&v_lat0.clone(), &c_DEG()?)?])?])).items()?;
    if t3700.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3700[0].clone();
    v_r0 = t3700[1].clone();
    v_th = py::mul(&py::mul(&v_n.clone(), &f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?)?, &c_DEG()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::add(&v_FE.clone(), &py::mul(&v_r.clone(), &py::math1("sin", &v_th.clone())?)?)?), (Py::str("y"), py::sub(&py::add(&v_FN.clone(), &v_r0.clone())?, &py::mul(&v_r.clone(), &py::math1("cos", &v_th.clone())?)?)?), (Py::str("k"), py::truediv(&py::mul(&v_r.clone(), &v_n.clone())?, &py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::call(&v_m.clone(), &[py::mul(&v_lat.clone(), &c_DEG()?)?])?)?)?), (Py::str("gamma"), py::truediv(&v_th.clone(), &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lcc_inverse(mut v_x: Py, mut v_y: Py, mut v_lat1: Py, mut v_lat2: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_psi: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_xp: Py = Py::None;
    let mut v_yp: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__fin(Py::str("x and y"), Py::tuple(vec![v_x.clone(), v_y.clone()]))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let t3782 = (f__lcc_setup(v_E.clone(), v_lat1.clone(), v_lat2.clone(), v_lat0.clone())?).items()?;
    if t3782.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3782[0].clone();
    v_F = t3782[1].clone();
    v_rho = t3782[2].clone();
    v_m = t3782[3].clone();
    v_r0 = py::call(&v_rho.clone(), &[py::mul(&v_lat0.clone(), &c_DEG()?)?])?;
    let t3783 = (Py::tuple(vec![py::sub(&v_x.clone(), &v_FE.clone())?, py::sub(&v_r0.clone(), &py::sub(&v_y.clone(), &v_FN.clone())?)?])).items()?;
    if t3783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xp = t3783[0].clone();
    v_yp = t3783[1].clone();
    v_sg = (if ({ let t3784 = v_n.clone(); Py::Bool({ let t3785 = Py::Int(0i64); if py::gt(&t3784, &t3785)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    v_r = py::mul(&v_sg.clone(), &py::hypot(&[v_xp.clone(), v_yp.clone()])?)?;
    let _ = f__need({ let t3786 = py::mul(&v_r.clone(), &v_sg.clone())?; Py::Bool({ let t3787 = Py::Int(0i64); if py::gt(&t3786, &t3787)? { true } else { false } }) }, Py::str("the point is the apex of the cone"))?;
    v_th = py::atan2(&py::mul(&v_sg.clone(), &v_xp.clone())?, &py::mul(&v_sg.clone(), &v_yp.clone())?)?;
    v_psi = py::truediv(&py::neg(&py::math1("log", &py::truediv(&v_r.clone(), &py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &v_F.clone())?)?)?)?, &v_n.clone())?;
    v_lat = py::truediv(&py::math1("atan", &f__tau_of_taup(v_E.clone(), py::math1("sinh", &v_psi.clone())?)?)?, &c_DEG()?)?;
    let _ = f__need({ let t3791 = py::abs(&v_lat.clone())?; Py::Bool({ let t3792 = Py::Int(89i64); if py::le(&t3791, &t3792)? { true } else { false } }) }, Py::str("the point lies too close to a pole"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("lat"), v_lat.clone()), (Py::str("lon"), f__nlon(py::add(&v_lon0.clone(), &py::truediv(&py::truediv(&v_th.clone(), &v_n.clone())?, &c_DEG()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_of_sight(mut v_d: Py, mut v_z: Py, mut v_observer_height: Py, mut v_target_height: Py, mut v_curvature: Py) -> R {
    let mut v_eye: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_tip: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vis: Py = Py::None;
    let mut v_zc: Py = Py::None;
    v_n = py::len(&v_d.clone())?;
    let _ = f__need({ let t3854 = { let t3850 = v_n.clone(); Py::Bool({ let t3851 = py::len(&v_z.clone())?; if py::eq(&t3850, &t3851) { true } else { false } }) }; if !t3854.truthy() { t3854 } else { { let t3852 = v_n.clone(); Py::Bool({ let t3853 = Py::Int(2i64); if py::ge(&t3852, &t3853)? { true } else { false } }) } } }, Py::str("give one elevation per distance (at least 2 points)"))?;
    let _ = f__need({ let t3861 = { let t3855 = py::getitem(&v_d.clone(), &Py::Int(0i64))?; Py::Bool({ let t3856 = Py::Int(0i64); if py::eq(&t3855, &t3856) { true } else { false } }) }; if !t3861.truthy() { t3861 } else { Py::Bool(({ let mut out = vec![]; for t3857 in py::iter(&(py::zip(&[v_d.clone(), py::slice(&v_d.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3858 = (t3857).items()?;
if t3858.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3858[0].clone();
v_b = t3858[1].clone();
out.push({ let t3859 = v_b.clone(); Py::Bool({ let t3860 = v_a.clone(); if py::gt(&t3859, &t3860)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("distances must start at 0 (the observer) and increase"))?;
    for t3862 in py::iter(&(v_z.clone()))? {
        v_v = t3862;
        let _ = f__fin(Py::str("each elevation"), Py::tuple(vec![v_v.clone()]))?;
    }
    let _ = f__pos(Py::str("the observer and target heights"), Py::tuple(vec![v_observer_height.clone(), v_target_height.clone()]))?;
    let _ = f__need({ let t3872 = v_curvature.clone(); Py::Bool({ let t3873 = Py::tuple(vec![Py::str("none"), Py::str("earth_refraction")]); if py::contains(&t3873, &t3872)? { true } else { false } }) }, Py::str("curvature must be none or earth_refraction"))?;
    v_zc = { let mut out = vec![]; for t3874 in py::iter(&(py::zip(&[v_z.clone(), v_d.clone()])?))? { let mut v_t: Py; let mut v_v: Py; 
let t3875 = (t3874).items()?;
if t3875.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3875[0].clone();
v_t = t3875[1].clone();
out.push(py::sub(&v_v.clone(), &(if ({ let t3876 = v_curvature.clone(); Py::Bool({ let t3877 = Py::str("earth_refraction"); if py::eq(&t3876, &t3877) { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&v_t.clone(), &v_t.clone())?, &py::sub(&Py::Int(1i64), &c_K_REFRACTION()?)?)?, &py::mul(&Py::Int(2i64), &c_R_EARTH_LOS()?)?)? } else { Py::Float(0.0) }))?); } Py::list(out) };
    v_eye = py::add(&py::getitem(&v_zc.clone(), &Py::Int(0i64))?, &v_observer_height.clone())?;
    v_vis = Py::list(vec![Py::Float(1.0)]);
    for t3878 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_i = t3878;
        v_tip = py::add(&py::getitem(&v_zc.clone(), &v_i.clone())?, &v_target_height.clone())?;
        v_ok = Py::Bool(true);
        for t3879 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_i.clone()).i()?]) {
            v_j = t3879;
            if ({ let t3880 = py::getitem(&v_zc.clone(), &v_j.clone())?; Py::Bool({ let t3881 = py::add(&v_eye.clone(), &py::truediv(&py::mul(&py::sub(&v_tip.clone(), &v_eye.clone())?, &py::getitem(&v_d.clone(), &v_j.clone())?)?, &py::getitem(&v_d.clone(), &v_i.clone())?)?)?; if py::ge(&t3880, &t3881)? { true } else { false } }) }).truthy() {
                v_ok = Py::Bool(false);
                break;
            }
        }
        let _ = py::method(&v_vis.clone(), "append", &[(if (v_ok.clone()).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })])?;
    }
    return Ok(Py::dict(vec![(Py::str("visible"), v_vis.clone()), (Py::str("n_visible"), py::float(&py::sub(&py::sum(&v_vis.clone(), None)?, &Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_local_g(mut v_y: Py, mut v_W: Py, mut v_statistic: Py) -> R {
    let mut v_Gs: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_Wt: Py = Py::None;
    let mut v_Zs: Py = Py::None;
    let mut v_card: Py = Py::None;
    let mut v_eg: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_star: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_sy2: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_vg: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__y_ok(v_y.clone(), Py::Int(4i64))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3932 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t3932;
out.push({ let t3933 = v_v.clone(); Py::Bool({ let t3934 = Py::Int(0i64); if py::gt(&t3933, &t3934)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the values must be positive"))?;
    let _ = f__need({ let t3935 = v_statistic.clone(); Py::Bool({ let t3936 = Py::tuple(vec![Py::str("gi_star"), Py::str("gi")]); if py::contains(&t3936, &t3935)? { true } else { false } }) }, Py::str("the statistic must be gi_star (location included) or gi"))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), Py::str("none"), Py::Bool(true), Py::Bool(false), Py::Bool(false))?;
    v_star = { let t3963 = v_statistic.clone(); Py::Bool({ let t3964 = Py::str("gi_star"); if py::eq(&t3963, &t3964) { true } else { false } }) };
    if (v_star.clone()).truthy() {
        v_Wt = { let mut out = vec![]; for t3965 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3965;
out.push({ let mut out = vec![]; for t3966 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3966;
out.push((if ({ let t3967 = v_i.clone(); Py::Bool({ let t3968 = v_j.clone(); if py::eq(&t3967, &t3968) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())? })); } Py::list(out) }); } Py::list(out) };
    }
    let t3970 = (Py::tuple(vec![py::fsum(&v_y.clone())?, py::fsum(&{ let mut out = vec![]; for t3969 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t3969;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?])).items()?;
    if t3970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sy = t3970[0].clone();
    v_sy2 = t3970[1].clone();
    v_N = (if (v_star.clone()).truthy() { v_n.clone() } else { py::sub(&v_n.clone(), &Py::Int(1i64))? });
    let t3971 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t3971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Gs = t3971[0].clone();
    v_Zs = t3971[1].clone();
    for t3972 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3972;
        v_tot = (if (v_star.clone()).truthy() { v_sy.clone() } else { py::sub(&v_sy.clone(), &py::getitem(&v_y.clone(), &v_i.clone())?)? });
        v_g = py::truediv(&py::fsum(&{ let mut out = vec![]; for t3973 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3973;
out.push(py::mul(&py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?); } Py::list(out) })?, &v_tot.clone())?;
        v_mean = py::truediv(&v_tot.clone(), &v_N.clone())?;
        v_var = py::sub(&py::truediv(&(if (v_star.clone()).truthy() { v_sy2.clone() } else { py::sub(&v_sy2.clone(), &py::mul(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_i.clone())?)?)? }), &v_N.clone())?, &py::mul(&v_mean.clone(), &v_mean.clone())?)?;
        v_card = py::fsum(&py::getitem(&v_Wt.clone(), &v_i.clone())?)?;
        v_eg = py::truediv(&v_card.clone(), &v_N.clone())?;
        v_vg = py::truediv(&py::mul(&py::truediv(&py::truediv(&py::mul(&v_card.clone(), &py::sub(&v_N.clone(), &v_card.clone())?)?, &py::sub(&v_N.clone(), &Py::Int(1i64))?)?, &py::pow(&v_N.clone(), &Py::Int(2i64))?)?, &v_var.clone())?, &py::pow(&v_mean.clone(), &Py::Int(2i64))?)?;
        let _ = f__need({ let t3974 = v_vg.clone(); Py::Bool({ let t3975 = Py::Int(0i64); if py::gt(&t3974, &t3975)? { true } else { false } }) }, Py::str("a location whose neighbourhood covers every observation has no variance (z undefined)"))?;
        let _ = py::method(&v_Gs.clone(), "append", &[v_g.clone()])?;
        let _ = py::method(&v_Zs.clone(), "append", &[py::truediv(&py::sub(&v_g.clone(), &v_eg.clone())?, &py::math1("sqrt", &v_vg.clone())?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("Gs"), f__r(v_Gs.clone())?), (Py::str("Zs"), f__r(v_Zs.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_local_morans_i(mut v_y: Py, mut v_W: Py) -> R {
    let mut v_Is: Py = Py::None;
    let mut v_Wt: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_lag: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__y_ok(v_y.clone(), Py::Int(4i64))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), Py::str("row"), Py::Bool(false), Py::Bool(false), Py::Bool(false))?;
    v_m = py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?;
    v_z = { let mut out = vec![]; for t4062 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t4062;
out.push(py::sub(&v_v.clone(), &v_m.clone())?); } Py::list(out) };
    v_den = py::fsum(&{ let mut out = vec![]; for t4063 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t4063;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    v_lag = { let mut out = vec![]; for t4064 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4064;
out.push(py::fsum(&{ let mut out = vec![]; for t4065 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4065;
out.push(py::mul(&py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_z.clone(), &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    v_Is = { let mut out = vec![]; for t4066 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4066;
out.push(py::truediv(&py::mul(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &py::getitem(&v_z.clone(), &v_i.clone())?)?, &py::getitem(&v_lag.clone(), &v_i.clone())?)?, &v_den.clone())?); } Py::list(out) };
    v_q = { let mut out = vec![]; for t4067 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4067;
out.push((if ({ let t4072 = { let t4068 = py::getitem(&v_z.clone(), &v_i.clone())?; Py::Bool({ let t4069 = Py::Int(0i64); if py::gt(&t4068, &t4069)? { true } else { false } }) }; if !t4072.truthy() { t4072 } else { { let t4070 = py::getitem(&v_lag.clone(), &v_i.clone())?; Py::Bool({ let t4071 = Py::Int(0i64); if py::gt(&t4070, &t4071)? { true } else { false } }) } } }).truthy() { Py::Float(1.0) } else { (if ({ let t4077 = { let t4073 = py::getitem(&v_z.clone(), &v_i.clone())?; Py::Bool({ let t4074 = Py::Int(0i64); if py::lt(&t4073, &t4074)? { true } else { false } }) }; if !t4077.truthy() { t4077 } else { { let t4075 = py::getitem(&v_lag.clone(), &v_i.clone())?; Py::Bool({ let t4076 = Py::Int(0i64); if py::gt(&t4075, &t4076)? { true } else { false } }) } } }).truthy() { Py::Float(2.0) } else { (if ({ let t4082 = { let t4078 = py::getitem(&v_z.clone(), &v_i.clone())?; Py::Bool({ let t4079 = Py::Int(0i64); if py::lt(&t4078, &t4079)? { true } else { false } }) }; if !t4082.truthy() { t4082 } else { { let t4080 = py::getitem(&v_lag.clone(), &v_i.clone())?; Py::Bool({ let t4081 = Py::Int(0i64); if py::lt(&t4080, &t4081)? { true } else { false } }) } } }).truthy() { Py::Float(3.0) } else { (if ({ let t4087 = { let t4083 = py::getitem(&v_z.clone(), &v_i.clone())?; Py::Bool({ let t4084 = Py::Int(0i64); if py::gt(&t4083, &t4084)? { true } else { false } }) }; if !t4087.truthy() { t4087 } else { { let t4085 = py::getitem(&v_lag.clone(), &v_i.clone())?; Py::Bool({ let t4086 = Py::Int(0i64); if py::lt(&t4085, &t4086)? { true } else { false } }) } } }).truthy() { Py::Float(4.0) } else { Py::Float(0.0) }) }) }) })); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("Is"), f__r(v_Is.clone())?), (Py::str("quadrant"), v_q.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_look_angles(mut v_lat0: Py, mut v_lon0: Py, mut v_h0: Py, mut v_lat: Py, mut v_lon: Py, mut v_h: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_r = f_geodetic_to_enu(v_lat0.clone(), v_lon0.clone(), v_h0.clone(), v_lat.clone(), v_lon.clone(), v_h.clone(), v_ellipsoid.clone())?;
    let _ = f__need({ let t4180 = py::getitem(&v_r.clone(), &Py::str("slant_range"))?; Py::Bool({ let t4181 = Py::Int(0i64); if py::gt(&t4180, &t4181)? { true } else { false } }) }, Py::str("the target must differ from the observer"))?;
    let t4182 = (Py::tuple(vec![py::getitem(&v_r.clone(), &Py::str("e"))?, py::getitem(&v_r.clone(), &Py::str("n"))?, py::getitem(&v_r.clone(), &Py::str("u"))?])).items()?;
    if t4182.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t4182[0].clone();
    v_n = t4182[1].clone();
    v_u = t4182[2].clone();
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    v_o = f__ecef(v_E.clone(), v_lat0.clone(), v_lon0.clone(), v_h0.clone())?;
    v_t = f__ecef(v_E.clone(), v_lat.clone(), v_lon.clone(), v_h.clone())?;
    v_d = { let mut out = vec![]; for t4183 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t4183;
out.push(py::sub(&py::getitem(&v_t.clone(), &v_i.clone())?, &py::getitem(&v_o.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_R = f__enu_rot(v_lat0.clone(), v_lon0.clone())?;
    let t4186 = ({ let mut out = vec![]; for t4184 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t4184;
out.push(py::fsum(&{ let mut out = vec![]; for t4185 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t4185;
out.push(py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_d.clone(), &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }).items()?;
    if t4186.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t4186[0].clone();
    v_n = t4186[1].clone();
    v_u = t4186[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("azimuth"), f__az(py::truediv(&py::atan2(&v_e.clone(), &v_n.clone())?, &c_DEG()?)?)?), (Py::str("elevation"), py::truediv(&py::atan2(&v_u.clone(), &py::hypot(&[v_e.clone(), v_n.clone()])?)?, &c_DEG()?)?), (Py::str("slant_range"), py::getitem(&v_r.clone(), &Py::str("slant_range"))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_map_scale_from_distances(mut v_map_distance: Py, mut v_ground_distance: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("the map distance"), Py::tuple(vec![v_map_distance.clone()]))?;
    let _ = f__pos(Py::str("the ground distance"), Py::tuple(vec![v_ground_distance.clone()]))?;
    let _ = f__need({ let t4238 = v_ground_distance.clone(); Py::Bool({ let t4239 = v_map_distance.clone(); if py::gt(&t4238, &t4239)? { true } else { false } }) }, Py::str("the ground distance must exceed the map distance"))?;
    v_s = py::truediv(&py::fraction(&[Py::str(&py::repr(&v_ground_distance.clone()))])?, &py::fraction(&[Py::str(&py::repr(&v_map_distance.clone()))])?)?;
    return Ok(Py::dict(vec![(Py::str("scale"), f__r(py::float(&v_s.clone())?)?), (Py::str("rf"), f__r(py::float(&py::truediv(&Py::Int(1i64), &v_s.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_map_scale_ground_distance(mut v_map_distance: Py, mut v_scale: Py) -> R {
    let _ = f__pos(Py::str("the map distance"), Py::tuple(vec![v_map_distance.clone()]))?;
    let _ = f__pos(Py::str("the scale denominator"), Py::tuple(vec![v_scale.clone()]))?;
    return Ok(Py::dict(vec![(Py::str("ground_distance"), py::float(&py::mul(&py::fraction(&[Py::str(&py::repr(&v_map_distance.clone()))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&v_scale.clone())?))])?)?)?), (Py::str("ground_km"), py::float(&py::truediv(&py::mul(&py::fraction(&[Py::str(&py::repr(&v_map_distance.clone()))])?, &py::fraction(&[Py::str(&py::repr(&py::float(&v_scale.clone())?))])?)?, &Py::Int(1000i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_covering(mut v_edges: Py, mut v_demand: Py, mut v_candidates: Py, mut v_p: Py, mut v_coverage: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_demand.clone())?;
    v_adj = f__net(v_edges.clone(), v_n.clone(), Py::Bool(true))?;
    for t4364 in py::iter(&(v_demand.clone()))? {
        v_v = t4364;
        let _ = f__nonneg(Py::str("each demand"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_C = { let mut out = vec![]; for t4371 in py::iter(&(v_candidates.clone()))? { let mut v_c: Py; 
v_c = t4371;
out.push(py::int(&v_c.clone())?); } Py::list(out) };
    let _ = f__need({ let t4382 = { let t4372 = py::len(&v_C.clone())?; Py::Bool({ let t4373 = Py::Int(1i64); if py::ge(&t4372, &t4373)? { true } else { false } }) }; if !t4382.truthy() { t4382 } else { Py::Bool(({ let mut out = vec![]; for t4374 in py::iter(&(py::zip(&[v_candidates.clone(), v_C.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4375 = (t4374).items()?;
if t4375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4375[0].clone();
v_b = t4375[1].clone();
out.push({ let t4381 = { let t4376 = py::float(&v_a.clone())?; Py::Bool({ let t4377 = v_b.clone(); if py::eq(&t4376, &t4377) { true } else { false } }) }; if !t4381.truthy() { t4381 } else { { let t4378 = Py::Int(0i64); Py::Bool({ let t4379 = v_b.clone(); if py::le(&t4378, &t4379)? { { let t4380 = v_n.clone(); if py::lt(&t4379, &t4380)? { true } else { false } } } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the candidates must be nodes of the network"))?;
    let _ = f__need({ let t4383 = py::len(&py::set(&v_C.clone())?)?; Py::Bool({ let t4384 = py::len(&v_C.clone())?; if py::eq(&t4383, &t4384) { true } else { false } }) }, Py::str("each candidate may be listed once"))?;
    let _ = f__need({ let t4388 = Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4388.truthy() { t4388 } else { { let t4385 = Py::Int(1i64); Py::Bool({ let t4386 = v_p.clone(); if py::le(&t4385, &t4386)? { { let t4387 = py::len(&v_C.clone())?; if py::le(&t4386, &t4387)? { true } else { false } } } else { false } }) } } }, Py::str("p must be a whole number from 1 to the number of candidates"))?;
    let _ = f__pos(Py::str("the coverage distance"), Py::tuple(vec![v_coverage.clone()]))?;
    let _ = f__need({ let t4395 = py::comb(&py::len(&v_C.clone())?, &v_p.clone())?; Py::Bool({ let t4396 = Py::Int(20000i64); if py::le(&t4395, &t4396)? { true } else { false } }) }, Py::str("too many candidate combinations (at most 20000)"))?;
    v_D = { let mut out = vec![]; for t4397 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t4397;
out.push((v_c.clone(), f__dijkstra(v_adj.clone(), v_c.clone())?)); } Py::dict(out) };
    v_best = py::minmax(&[{ let mut out = vec![]; for t4405 in py::iter(&(py::itertools("combinations", &[v_C.clone(), v_p.clone()])?))? { let mut v_S: Py; 
v_S = t4405;
out.push(py::fsum(&{ let mut out = vec![]; for t4406 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4406;
if (Py::Bool(({ let mut out = vec![]; for t4407 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t4407;
out.push({ let t4408 = py::getitem(&py::getitem(&v_D.clone(), &v_s.clone())?, &v_i.clone())?; Py::Bool({ let t4409 = v_coverage.clone(); if py::le(&t4408, &t4409)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() { out.push(py::getitem(&v_demand.clone(), &v_i.clone())?); }} Py::list(out) })?); } Py::list(out) }], true)?;
    v_tot = py::fsum(&v_demand.clone())?;
    return Ok(Py::dict(vec![(Py::str("covered"), f__r(v_best.clone())?), (Py::str("fraction"), (if ({ let t4420 = v_tot.clone(); Py::Bool({ let t4421 = Py::Int(0i64); if py::gt(&t4420, &t4421)? { true } else { false } }) }).truthy() { f__r(py::truediv(&v_best.clone(), &v_tot.clone())?)? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_center(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sd: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t4467 = { let t4463 = v_n.clone(); Py::Bool({ let t4464 = py::len(&v_y.clone())?; if py::eq(&t4463, &t4464) { true } else { false } }) }; if !t4467.truthy() { t4467 } else { { let t4465 = v_n.clone(); Py::Bool({ let t4466 = Py::Int(1i64); if py::ge(&t4465, &t4466)? { true } else { false } }) } } }, Py::str("give one y per x"))?;
    let t4468 = (Py::tuple(vec![py::truediv(&py::fsum(&v_x.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?])).items()?;
    if t4468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t4468[0].clone();
    v_my = t4468[1].clone();
    v_sd = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t4469 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4470 = (t4469).items()?;
if t4470.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4470[0].clone();
v_b = t4470[1].clone();
out.push(py::add(&py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?, &v_n.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("mean_x"), v_mx.clone()), (Py::str("mean_y"), v_my.clone()), (Py::str("std_distance"), v_sd.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mercator_forward(mut v_lat: Py, mut v_lon: Py, mut v_lon0: Py, mut v_lat_ts: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Float(89.5))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__lat(Py::str("lat_ts"), v_lat_ts.clone(), Py::Bool(false))?;
    let _ = f__need({ let t4541 = py::abs(&v_lat_ts.clone())?; Py::Bool({ let t4542 = Py::Int(85i64); if py::le(&t4541, &t4542)? { true } else { false } }) }, Py::str("the standard parallel must lie within 85 degrees of the equator"))?;
    v_k0 = f__merc_k0(v_E.clone(), v_lat_ts.clone())?;
    v_p = py::mul(&v_lat.clone(), &c_DEG()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::mul(&py::mul(&py::mul(&v_k0.clone(), &py::getitem(&v_E.clone(), &Py::str("a"))?)?, &f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?)?, &c_DEG()?)?), (Py::str("y"), py::mul(&py::mul(&v_k0.clone(), &py::getitem(&v_E.clone(), &Py::str("a"))?)?, &f__psi(v_E.clone(), v_p.clone())?)?), (Py::str("k"), py::truediv(&py::mul(&v_k0.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &v_p.clone())?, &Py::Int(2i64))?)?)?)?)?, &py::math1("cos", &v_p.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mercator_inverse(mut v_x: Py, mut v_y: Py, mut v_lon0: Py, mut v_lat_ts: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_psi: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__fin(Py::str("x and y"), Py::tuple(vec![v_x.clone(), v_y.clone()]))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__lat(Py::str("lat_ts"), v_lat_ts.clone(), Py::Bool(false))?;
    let _ = f__need({ let t4615 = py::abs(&v_lat_ts.clone())?; Py::Bool({ let t4616 = Py::Int(85i64); if py::le(&t4615, &t4616)? { true } else { false } }) }, Py::str("the standard parallel must lie within 85 degrees of the equator"))?;
    v_k0 = f__merc_k0(v_E.clone(), v_lat_ts.clone())?;
    v_psi = py::truediv(&v_y.clone(), &py::mul(&v_k0.clone(), &py::getitem(&v_E.clone(), &Py::str("a"))?)?)?;
    let _ = f__need({ let t4617 = py::abs(&v_psi.clone())?; Py::Bool({ let t4618 = Py::Int(6i64); if py::le(&t4617, &t4618)? { true } else { false } }) }, Py::str("the northing lies too far from the equator"))?;
    v_lam = py::truediv(&py::truediv(&v_x.clone(), &py::mul(&v_k0.clone(), &py::getitem(&v_E.clone(), &Py::str("a"))?)?)?, &c_DEG()?)?;
    let _ = f__need({ let t4619 = py::abs(&v_lam.clone())?; Py::Bool({ let t4620 = Py::Int(540i64); if py::le(&t4619, &t4620)? { true } else { false } }) }, Py::str("the easting lies outside the projection"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("lat"), py::truediv(&py::math1("atan", &f__tau_of_taup(v_E.clone(), py::math1("sinh", &v_psi.clone())?)?)?, &c_DEG()?)?), (Py::str("lon"), f__nlon(py::add(&v_lon0.clone(), &v_lam.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_meridian_arc(mut v_lat1: Py, mut v_lat2: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat1"), v_lat1.clone(), Py::Bool(true))?;
    let _ = f__lat(Py::str("lat2"), v_lat2.clone(), Py::Bool(true))?;
    let _ = f__need({ let t4693 = v_lat1.clone(); Py::Bool({ let t4694 = v_lat2.clone(); if py::ne(&t4693, &t4694) { true } else { false } }) }, Py::str("the two latitudes must differ"))?;
    let t4696 = (Py::tuple(vec![f__merid(v_E.clone(), py::mul(&v_lat1.clone(), &c_DEG()?)?)?, f__merid(v_E.clone(), py::mul(&v_lat2.clone(), &c_DEG()?)?)?])).items()?;
    if t4696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m1 = t4696[0].clone();
    v_m2 = t4696[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("distance"), py::abs(&py::sub(&v_m2.clone(), &v_m1.clone())?)?), (Py::str("m1"), v_m1.clone()), (Py::str("m2"), v_m2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mgrs_encode(mut v_lat: Py, mut v_lon: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_ee: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_letter_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zone: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    v_z = f_utm_zone(v_lat.clone(), v_lon.clone())?;
    v_zone = py::getitem(&v_z.clone(), &Py::str("zone"))?;
    let t4805 = (f__tm_fwd(v_E.clone(), py::mul(&v_lat.clone(), &c_DEG()?)?, py::mul(&f__nlon(py::sub(&v_lon.clone(), &py::getitem(&v_z.clone(), &Py::str("central_meridian"))?)?)?, &c_DEG()?)?)?).items()?;
    if t4805.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_X = t4805[0].clone();
    v_Y = t4805[1].clone();
    v_g = t4805[2].clone();
    v_k = t4805[3].clone();
    v_e = py::add(&Py::Float(500000.0), &py::mul(&Py::Float(0.9996), &v_Y.clone())?)?;
    v_n = py::add(&(if ({ let t4806 = v_lat.clone(); Py::Bool({ let t4807 = Py::Int(0i64); if py::ge(&t4806, &t4807)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(10000000.0) }), &py::mul(&Py::Float(0.9996), &v_X.clone())?)?;
    v_col = py::int(&py::math1("floor", &py::truediv(&v_e.clone(), &Py::Float(100000.0))?)?)?;
    let _ = f__need({ let t4808 = Py::Int(1i64); Py::Bool({ let t4809 = v_col.clone(); if py::le(&t4808, &t4809)? { { let t4810 = Py::Int(8i64); if py::le(&t4809, &t4810)? { true } else { false } } } else { false } }) }, Py::str("the point lies outside its zone's grid squares"))?;
    v_letter_e = py::getitem(&py::getitem(&c__MGRS_COLS()?, &py::modulo(&py::sub(&v_zone.clone(), &Py::Int(1i64))?, &Py::Int(3i64))?)?, &py::sub(&v_col.clone(), &Py::Int(1i64))?)?;
    v_row = py::modulo(&py::int(&py::math1("floor", &py::truediv(&v_n.clone(), &Py::Float(100000.0))?)?)?, &Py::Int(20i64))?;
    if ({ let t4811 = py::modulo(&v_zone.clone(), &Py::Int(2i64))?; Py::Bool({ let t4812 = Py::Int(0i64); if py::eq(&t4811, &t4812) { true } else { false } }) }).truthy() {
        v_row = py::modulo(&py::add(&v_row.clone(), &Py::Int(5i64))?, &Py::Int(20i64))?;
    }
    v_ee = py::modulo(&py::int(&py::math1("floor", &v_e.clone())?)?, &Py::Int(100000i64))?;
    v_nn = py::modulo(&py::int(&py::math1("floor", &v_n.clone())?)?, &Py::Int(100000i64))?;
    return Ok(Py::dict(vec![(Py::str("mgrs"), Py::str(&[py::format(&v_zone.clone(), "02d")?, py::format(&py::getitem(&v_z.clone(), &Py::str("band"))?, "")?, py::format(&v_letter_e.clone(), "")?, py::format(&py::getitem(&c__MGRS_ROWS()?, &v_row.clone())?, "")?, py::format(&v_ee.clone(), "05d")?, py::format(&v_nn.clone(), "05d")?].concat()))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mollweide_forward(mut v_lat: Py, mut v_lon: Py, mut v_lon0: Py, mut v_R: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Int(90i64))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__sphere_R(v_R.clone())?;
    v_p = py::mul(&v_lat.clone(), &c_DEG()?)?;
    if ({ let t4874 = py::abs(&v_lat.clone())?; Py::Bool({ let t4875 = Py::Int(90i64); if py::eq(&t4874, &t4875) { true } else { false } }) }).truthy() {
        v_th = py::copysign(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_lat.clone())?;
    } else {
        v_th = v_p.clone();
        for t4876 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
            v__ = t4876;
            v_d = py::truediv(&py::sub(&py::add(&py::mul(&Py::Int(2i64), &v_th.clone())?, &py::math1("sin", &py::mul(&Py::Int(2i64), &v_th.clone())?)?)?, &py::mul(&Py::Float(std::f64::consts::PI), &py::math1("sin", &v_p.clone())?)?)?, &py::add(&Py::Int(2i64), &py::mul(&Py::Int(2i64), &py::math1("cos", &py::mul(&Py::Int(2i64), &v_th.clone())?)?)?)?)?;
            v_th = py::sub(&v_th, &v_d.clone())?;
            if ({ let t4877 = py::abs(&v_d.clone())?; Py::Bool({ let t4878 = Py::Float(1e-15); if py::lt(&t4877, &t4878)? { true } else { false } }) }).truthy() {
                break;
            }
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::mul(&py::mul(&py::mul(&py::truediv(&py::mul(&py::mul(&v_R.clone(), &Py::Int(2i64))?, &py::math1("sqrt", &Py::Int(2i64))?)?, &Py::Float(std::f64::consts::PI))?, &f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?)?, &c_DEG()?)?, &py::math1("cos", &v_th.clone())?)?), (Py::str("y"), py::mul(&py::mul(&v_R.clone(), &py::math1("sqrt", &Py::Int(2i64))?)?, &py::math1("sin", &v_th.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_morans_i(mut v_y: Py, mut v_W: Py, mut v_transform: Py) -> R {
    let mut v_EI: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_VIn: Py = Py::None;
    let mut v_VIr: Py = Py::None;
    let mut v_Wt: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zn: Py = Py::None;
    let mut v_zr: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__y_ok(v_y.clone(), Py::Int(4i64))?;
    let _ = f__need({ let t4945 = v_transform.clone(); Py::Bool({ let t4946 = Py::tuple(vec![Py::str("row"), Py::str("none")]); if py::contains(&t4946, &t4945)? { true } else { false } }) }, Py::str("the transform must be row or none"))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), v_transform.clone(), Py::Bool(false), Py::Bool(false), Py::Bool(false))?;
    v_m = py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?;
    v_z = { let mut out = vec![]; for t4973 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t4973;
out.push(py::sub(&v_v.clone(), &v_m.clone())?); } Py::list(out) };
    let t4979 = (f__s012(v_Wt.clone())?).items()?;
    if t4979.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s0 = t4979[0].clone();
    v_s1 = t4979[1].clone();
    v_s2 = t4979[2].clone();
    v_m2 = py::fsum(&{ let mut out = vec![]; for t4980 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t4980;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    v_I = py::truediv(&py::mul(&py::truediv(&v_n.clone(), &v_s0.clone())?, &py::fsum(&{ let mut out = vec![]; for t4981 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4981;
for t4982 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4982;
out.push(py::mul(&py::mul(&py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_z.clone(), &v_i.clone())?)?, &py::getitem(&v_z.clone(), &v_j.clone())?)?); }} Py::list(out) })?)?, &v_m2.clone())?;
    v_EI = py::truediv(&py::neg(&Py::Float(1.0))?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?;
    v_VIn = py::sub(&py::truediv(&py::add(&py::sub(&py::mul(&py::mul(&v_n.clone(), &v_n.clone())?, &v_s1.clone())?, &py::mul(&v_n.clone(), &v_s2.clone())?)?, &py::mul(&py::mul(&Py::Int(3i64), &v_s0.clone())?, &v_s0.clone())?)?, &py::mul(&py::mul(&py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &Py::Int(1i64))?, &v_s0.clone())?, &v_s0.clone())?)?, &py::mul(&v_EI.clone(), &v_EI.clone())?)?;
    v_k = py::truediv(&py::truediv(&py::fsum(&{ let mut out = vec![]; for t4983 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t4983;
out.push(py::pow(&v_v.clone(), &Py::Int(4i64))?); } Py::list(out) })?, &v_n.clone())?, &py::pow(&py::truediv(&v_m2.clone(), &v_n.clone())?, &Py::Int(2i64))?)?;
    v_VIr = py::sub(&py::truediv(&py::sub(&py::mul(&v_n.clone(), &py::add(&py::sub(&py::mul(&py::add(&py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &py::mul(&Py::Int(3i64), &v_n.clone())?)?, &Py::Int(3i64))?, &v_s1.clone())?, &py::mul(&v_n.clone(), &v_s2.clone())?)?, &py::mul(&py::mul(&Py::Int(3i64), &v_s0.clone())?, &v_s0.clone())?)?)?, &py::mul(&v_k.clone(), &py::add(&py::sub(&py::mul(&py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &v_s1.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_n.clone())?, &v_s2.clone())?)?, &py::mul(&py::mul(&Py::Int(6i64), &v_s0.clone())?, &v_s0.clone())?)?)?)?, &py::mul(&py::mul(&py::mul(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &py::sub(&v_n.clone(), &Py::Int(2i64))?)?, &py::sub(&v_n.clone(), &Py::Int(3i64))?)?, &v_s0.clone())?, &v_s0.clone())?)?, &py::mul(&v_EI.clone(), &v_EI.clone())?)?;
    let t4984 = (Py::tuple(vec![py::truediv(&py::sub(&v_I.clone(), &v_EI.clone())?, &py::math1("sqrt", &v_VIn.clone())?)?, py::truediv(&py::sub(&v_I.clone(), &v_EI.clone())?, &py::math1("sqrt", &v_VIr.clone())?)?])).items()?;
    if t4984.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_zn = t4984[0].clone();
    v_zr = t4984[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("I"), v_I.clone()), (Py::str("EI"), v_EI.clone()), (Py::str("VI_norm"), v_VIn.clone()), (Py::str("z_norm"), v_zn.clone()), (Py::str("p_norm"), f__p_two(v_zn.clone())?), (Py::str("VI_rand"), v_VIr.clone()), (Py::str("z_rand"), v_zr.clone()), (Py::str("p_rand"), f__p_two(v_zr.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nearest_neighbour_index(mut v_x: Py, mut v_y: Py, mut v_area: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_dexp: Py = Py::None;
    let mut v_dobs: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t5038 = v_n.clone(); Py::Bool({ let t5039 = py::len(&v_y.clone())?; if py::eq(&t5038, &t5039) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t5040 = v_n.clone(); Py::Bool({ let t5041 = Py::Int(3i64); if py::ge(&t5040, &t5041)? { true } else { false } }) }, Py::str("give at least 3 points"))?;
    let _ = f__pos(Py::str("the study area"), Py::tuple(vec![v_area.clone()]))?;
    let _ = f__need({ let t5048 = py::len(&py::set(&py::zip(&[v_x.clone(), v_y.clone()])?)?)?; Py::Bool({ let t5049 = v_n.clone(); if py::eq(&t5048, &t5049) { true } else { false } }) }, Py::str("the points must be distinct"))?;
    v_d = { let mut out = vec![]; for t5050 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5050;
out.push(py::minmax(&[{ let mut out = vec![]; for t5051 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5051;
if ({ let t5052 = v_j.clone(); Py::Bool({ let t5053 = v_i.clone(); if py::ne(&t5052, &t5053) { true } else { false } }) }).truthy() { out.push(py::hypot(&[py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?])?); }} Py::list(out) }], false)?); } Py::list(out) };
    v_dobs = py::truediv(&py::fsum(&v_d.clone())?, &v_n.clone())?;
    v_dexp = py::truediv(&Py::Float(0.5), &py::math1("sqrt", &py::truediv(&v_n.clone(), &v_area.clone())?)?)?;
    v_se = py::truediv(&Py::Float(0.26136), &py::math1("sqrt", &py::truediv(&py::mul(&v_n.clone(), &v_n.clone())?, &v_area.clone())?)?)?;
    v_z = py::truediv(&py::sub(&v_dobs.clone(), &v_dexp.clone())?, &v_se.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("d_obs"), v_dobs.clone()), (Py::str("d_exp"), v_dexp.clone()), (Py::str("R"), py::truediv(&v_dobs.clone(), &v_dexp.clone())?), (Py::str("z"), v_z.clone()), (Py::str("p_value"), f__p_two(v_z.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_network_indices(mut v_edges: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_eN: Py = Py::None;
    let mut v_find: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_parent: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vN: Py = Py::None;
    let t5107 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, py::set(&Py::list(vec![]))?])).items()?;
    if t5107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t5107[0].clone();
    v_E = t5107[1].clone();
    for t5108 in py::iter(&(v_edges.clone()))? {
        v_e = t5108;
        let t5111 = (Py::tuple(vec![f__node(py::getitem(&v_e.clone(), &Py::Int(0i64))?, Py::str("a node"))?, f__node(py::getitem(&v_e.clone(), &Py::Int(1i64))?, Py::str("a node"))?])).items()?;
        if t5111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t5111[0].clone();
        v_v = t5111[1].clone();
        let _ = f__need({ let t5112 = v_u.clone(); Py::Bool({ let t5113 = v_v.clone(); if py::ne(&t5112, &t5113) { true } else { false } }) }, Py::str("an edge must join two different nodes"))?;
        v_nodes = py::bitor(&v_nodes, &py::set(&Py::list(vec![v_u.clone(), v_v.clone()]))?)?;
        let _ = py::method(&v_E.clone(), "add", &[Py::tuple(vec![py::minmax(&[v_u.clone(), v_v.clone()], false)?, py::minmax(&[v_u.clone(), v_v.clone()], true)?])])?;
    }
    let t5114 = (Py::tuple(vec![py::len(&v_nodes.clone())?, py::len(&v_E.clone())?])).items()?;
    if t5114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vN = t5114[0].clone();
    v_eN = t5114[1].clone();
    let _ = f__need({ let t5115 = v_vN.clone(); Py::Bool({ let t5116 = Py::Int(3i64); if py::ge(&t5115, &t5116)? { true } else { false } }) }, Py::str("the network needs at least 3 nodes"))?;
    v_parent = { let mut out = vec![]; for t5117 in py::iter(&(v_nodes.clone()))? { let mut v_x: Py; 
v_x = t5117;
out.push((v_x.clone(), v_x.clone())); } Py::dict(out) };
    v_find = { let v_E = v_E.clone(); let v_e = v_e.clone(); let v_eN = v_eN.clone(); let v_edges = v_edges.clone(); let v_find = v_find.clone(); let v_nodes = v_nodes.clone(); let v_parent = v_parent.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); let v_vN = v_vN.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            while ({ let t5118 = py::getitem(&v_parent.clone(), &v_x.clone())?; Py::Bool({ let t5119 = v_x.clone(); if py::ne(&t5118, &t5119) { true } else { false } }) }).truthy() {
                py::setitem(&v_parent.clone(), &v_x.clone(), py::getitem(&v_parent.clone(), &py::getitem(&v_parent.clone(), &v_x.clone())?)?)?;
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t5120 in py::iter(&(v_E.clone()))? {
        let t5121 = (t5120).items()?;
        if t5121.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t5121[0].clone();
        v_v = t5121[1].clone();
        py::setitem(&v_parent.clone(), &py::call(&v_find.clone(), &[v_u.clone()])?, py::call(&v_find.clone(), &[v_v.clone()])?)?;
    }
    v_p = py::len(&py::set(&{ let mut out = vec![]; for t5122 in py::iter(&(v_nodes.clone()))? { let mut v_x: Py; 
v_x = t5122;
out.push(py::call(&v_find.clone(), &[v_x.clone()])?); } Py::list(out) })?)?;
    v_mu = py::add(&py::sub(&v_eN.clone(), &v_vN.clone())?, &v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("vertices"), py::float(&v_vN.clone())?), (Py::str("edges"), py::float(&v_eN.clone())?), (Py::str("components"), py::float(&v_p.clone())?), (Py::str("cyclomatic"), py::float(&v_mu.clone())?), (Py::str("alpha"), f__r(py::truediv(&v_mu.clone(), &py::sub(&py::mul(&Py::Int(2i64), &v_vN.clone())?, &Py::Int(5i64))?)?)?), (Py::str("beta"), f__r(py::truediv(&v_eN.clone(), &v_vN.clone())?)?), (Py::str("gamma"), f__r(py::truediv(&v_eN.clone(), &py::mul(&Py::Int(3i64), &py::sub(&v_vN.clone(), &Py::Int(2i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ordinary_kriging(mut v_x: Py, mut v_y: Py, mut v_z: Py, mut v_px: Py, mut v_py: Py, mut v_model: Py, mut v_nugget: Py, mut v_sill: Py, mut v_range: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_sol: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t5179 = { let t5174 = v_n.clone(); Py::Bool({ let t5175 = py::len(&v_y.clone())?; if py::eq(&t5174, &t5175) { { let t5176 = py::len(&v_z.clone())?; if py::eq(&t5175, &t5176) { true } else { false } } } else { false } }) }; if !t5179.truthy() { t5179 } else { { let t5177 = v_n.clone(); Py::Bool({ let t5178 = Py::Int(3i64); if py::ge(&t5177, &t5178)? { true } else { false } }) } } }, Py::str("give one y and one value per x (at least 3 samples)"))?;
    let _ = f__need({ let t5180 = v_model.clone(); Py::Bool({ let t5181 = Py::tuple(vec![Py::str("exponential"), Py::str("gaussian"), Py::str("spherical")]); if py::contains(&t5181, &t5180)? { true } else { false } }) }, Py::str("the model must be exponential, gaussian or spherical"))?;
    let _ = f__nonneg(Py::str("the nugget"), Py::tuple(vec![v_nugget.clone()]))?;
    let _ = f__pos(Py::str("the range"), Py::tuple(vec![v_range.clone()]))?;
    let _ = f__need({ let t5194 = v_sill.clone(); Py::Bool({ let t5195 = v_nugget.clone(); if py::gt(&t5194, &t5195)? { true } else { false } }) }, Py::str("the sill must exceed the nugget"))?;
    let _ = f__fin(Py::str("the location"), Py::tuple(vec![v_px.clone(), v_py.clone()]))?;
    let _ = f__need({ let t5199 = py::len(&py::set(&py::zip(&[v_x.clone(), v_y.clone()])?)?)?; Py::Bool({ let t5200 = v_n.clone(); if py::eq(&t5199, &t5200) { true } else { false } }) }, Py::str("the sample locations must be distinct"))?;
    for t5201 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5201;
        if ({ let t5206 = { let t5202 = py::getitem(&v_x.clone(), &v_i.clone())?; Py::Bool({ let t5203 = v_px.clone(); if py::eq(&t5202, &t5203) { true } else { false } }) }; if !t5206.truthy() { t5206 } else { { let t5204 = py::getitem(&v_y.clone(), &v_i.clone())?; Py::Bool({ let t5205 = v_py.clone(); if py::eq(&t5204, &t5205) { true } else { false } }) } } }).truthy() {
            return Ok(f__out(Py::dict(vec![(Py::str("estimate"), py::getitem(&v_z.clone(), &v_i.clone())?), (Py::str("variance"), Py::Float(0.0))]))?);
        }
    }
    v_ps = py::sub(&v_sill.clone(), &v_nugget.clone())?;
    v_g = { let v_i = v_i.clone(); let v_model = v_model.clone(); let v_n = v_n.clone(); let v_nugget = v_nugget.clone(); let v_ps = v_ps.clone(); let v_px = v_px.clone(); let v_py = v_py.clone(); let v_range = v_range.clone(); let v_sill = v_sill.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_d: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t5219 = v_d.clone(); Py::Bool({ let t5220 = Py::Int(0i64); if py::eq(&t5219, &t5220) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::add(&v_nugget.clone(), &py::mul(&v_ps.clone(), &f__vmodel(v_model.clone(), v_d.clone(), v_range.clone())?)?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_A = py::add(&{ let mut out = vec![]; for t5227 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5227;
out.push(py::add(&{ let mut out = vec![]; for t5228 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5228;
out.push(py::call(&v_g.clone(), &[py::hypot(&[py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?])?])?); } Py::list(out) }, &Py::list(vec![Py::Float(1.0)]))?); } Py::list(out) }, &Py::list(vec![py::add(&py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?, &Py::list(vec![Py::Float(0.0)]))?]))?;
    v_b = py::add(&{ let mut out = vec![]; for t5229 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5229;
out.push(py::call(&v_g.clone(), &[py::hypot(&[py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &v_px.clone())?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &v_py.clone())?])?])?); } Py::list(out) }, &Py::list(vec![Py::Float(1.0)]))?;
    v_sol = f__solve_lin(v_A.clone(), v_b.clone())?;
    let t5247 = (Py::tuple(vec![py::slice(&v_sol.clone(), None, Some((v_n.clone()).i()?), None)?, py::getitem(&v_sol.clone(), &v_n.clone())?])).items()?;
    if t5247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t5247[0].clone();
    v_mu = t5247[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("estimate"), py::fsum(&{ let mut out = vec![]; for t5248 in py::iter(&(py::zip(&[v_lam.clone(), v_z.clone()])?))? { let mut v_l: Py; let mut v_v: Py; 
let t5249 = (t5248).items()?;
if t5249.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_l = t5249[0].clone();
v_v = t5249[1].clone();
out.push(py::mul(&v_l.clone(), &v_v.clone())?); } Py::list(out) })?), (Py::str("variance"), py::add(&py::fsum(&{ let mut out = vec![]; for t5250 in py::iter(&(py::zip(&[v_lam.clone(), py::slice(&v_b.clone(), None, Some((v_n.clone()).i()?), None)?])?))? { let mut v_l: Py; let mut v_v: Py; 
let t5251 = (t5250).items()?;
if t5251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_l = t5251[0].clone();
v_v = t5251[1].clone();
out.push(py::mul(&v_l.clone(), &v_v.clone())?); } Py::list(out) })?, &v_mu.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_orthographic_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_R: Py) -> R {
    let mut v_cc: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Int(90i64))?;
    let _ = f__lat(Py::str("lat0"), v_lat0.clone(), Py::Bool(true))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__sphere_R(v_R.clone())?;
    let t5317 = (Py::tuple(vec![py::mul(&v_lat.clone(), &c_DEG()?)?, py::mul(&v_lat0.clone(), &c_DEG()?)?, py::mul(&f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?, &c_DEG()?)?])).items()?;
    if t5317.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5317[0].clone();
    v_p0 = t5317[1].clone();
    v_dl = t5317[2].clone();
    v_cc = py::add(&py::mul(&py::math1("sin", &v_p0.clone())?, &py::math1("sin", &v_p.clone())?)?, &py::mul(&py::mul(&py::math1("cos", &v_p0.clone())?, &py::math1("cos", &v_p.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?;
    let _ = f__need({ let t5318 = v_cc.clone(); Py::Bool({ let t5319 = Py::Int(0i64); if py::ge(&t5318, &t5319)? { true } else { false } }) }, Py::str("the point lies on the far side of the globe (not visible in the orthographic view)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::mul(&py::mul(&v_R.clone(), &py::math1("cos", &v_p.clone())?)?, &py::math1("sin", &v_dl.clone())?)?), (Py::str("y"), py::mul(&v_R.clone(), &py::sub(&py::mul(&py::math1("cos", &v_p0.clone())?, &py::math1("sin", &v_p.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_p0.clone())?, &py::math1("cos", &v_p.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_p_median(mut v_edges: Py, mut v_demand: Py, mut v_candidates: Py, mut v_p: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_obj: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_demand.clone())?;
    v_adj = f__net(v_edges.clone(), v_n.clone(), Py::Bool(true))?;
    for t5399 in py::iter(&(v_demand.clone()))? {
        v_v = t5399;
        let _ = f__nonneg(Py::str("each demand"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_C = { let mut out = vec![]; for t5406 in py::iter(&(v_candidates.clone()))? { let mut v_c: Py; 
v_c = t5406;
out.push(py::int(&v_c.clone())?); } Py::list(out) };
    let _ = f__need({ let t5417 = { let t5407 = py::len(&v_C.clone())?; Py::Bool({ let t5408 = Py::Int(1i64); if py::ge(&t5407, &t5408)? { true } else { false } }) }; if !t5417.truthy() { t5417 } else { Py::Bool(({ let mut out = vec![]; for t5409 in py::iter(&(py::zip(&[v_candidates.clone(), v_C.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5410 = (t5409).items()?;
if t5410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5410[0].clone();
v_b = t5410[1].clone();
out.push({ let t5416 = { let t5411 = py::float(&v_a.clone())?; Py::Bool({ let t5412 = v_b.clone(); if py::eq(&t5411, &t5412) { true } else { false } }) }; if !t5416.truthy() { t5416 } else { { let t5413 = Py::Int(0i64); Py::Bool({ let t5414 = v_b.clone(); if py::le(&t5413, &t5414)? { { let t5415 = v_n.clone(); if py::lt(&t5414, &t5415)? { true } else { false } } } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the candidates must be nodes of the network"))?;
    let _ = f__need({ let t5418 = py::len(&py::set(&v_C.clone())?)?; Py::Bool({ let t5419 = py::len(&v_C.clone())?; if py::eq(&t5418, &t5419) { true } else { false } }) }, Py::str("each candidate may be listed once"))?;
    let _ = f__need({ let t5423 = Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5423.truthy() { t5423 } else { { let t5420 = Py::Int(1i64); Py::Bool({ let t5421 = v_p.clone(); if py::le(&t5420, &t5421)? { { let t5422 = py::len(&v_C.clone())?; if py::le(&t5421, &t5422)? { true } else { false } } } else { false } }) } } }, Py::str("p must be a whole number from 1 to the number of candidates"))?;
    let _ = f__need({ let t5424 = py::comb(&py::len(&v_C.clone())?, &v_p.clone())?; Py::Bool({ let t5425 = Py::Int(20000i64); if py::le(&t5424, &t5425)? { true } else { false } }) }, Py::str("too many candidate combinations (at most 20000)"))?;
    v_D = f__all_dist(v_adj.clone())?;
    for t5434 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5434;
        let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5435 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t5435;
out.push(py::math1("isfinite", &py::getitem(&py::getitem(&v_D.clone(), &v_c.clone())?, &v_i.clone())?)?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the network must be connected"))?;
    }
    v_best = Py::list(vec![]);
    for t5436 in py::iter(&(py::itertools("combinations", &[py::sorted(&v_C.clone(), (Py::Bool(false)).truthy())?, v_p.clone()])?))? {
        v_S = t5436;
        v_obj = py::fsum(&{ let mut out = vec![]; for t5437 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5437;
out.push(py::mul(&py::getitem(&v_demand.clone(), &v_i.clone())?, &py::minmax(&[{ let mut out = vec![]; for t5438 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t5438;
out.push(py::getitem(&py::getitem(&v_D.clone(), &v_s.clone())?, &v_i.clone())?); } Py::list(out) }], false)?)?); } Py::list(out) })?;
        let _ = py::method(&v_best.clone(), "append", &[Py::tuple(vec![v_obj.clone(), v_S.clone()])])?;
    }
    let _ = py::method_sort(&v_best.clone(), None, false)?;
    let _ = f__need({ let t5443 = { let t5439 = py::len(&v_best.clone())?; Py::Bool({ let t5440 = Py::Int(1i64); if py::eq(&t5439, &t5440) { true } else { false } }) }; if t5443.truthy() { t5443 } else { { let t5441 = py::sub(&py::getitem(&py::getitem(&v_best.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?; Py::Bool({ let t5442 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::getitem(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?], true)?)?; if py::gt(&t5441, &t5442)? { true } else { false } }) } } }, Py::str("two facility sets are equally good: the p-median is not unique"))?;
    let t5444 = (py::getitem(&v_best.clone(), &Py::Int(0i64))?).items()?;
    if t5444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_obj = t5444[0].clone();
    v_S = t5444[1].clone();
    return Ok(Py::dict(vec![(Py::str("objective"), f__r(v_obj.clone())?), (Py::str("facilities"), { let mut out = vec![]; for t5455 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t5455;
out.push(py::float(&v_s.clone())?); } Py::list(out) }), (Py::str("mean_distance"), (if ({ let t5456 = py::fsum(&v_demand.clone())?; Py::Bool({ let t5457 = Py::Int(0i64); if py::gt(&t5456, &t5457)? { true } else { false } }) }).truthy() { f__r(py::truediv(&v_obj.clone(), &py::fsum(&v_demand.clone())?)?)? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_arc(mut v_lat: Py, mut v_lon1: Py, mut v_lon2: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon1"), v_lon1.clone())?;
    let _ = f__lon(Py::str("lon2"), v_lon2.clone())?;
    v_dl = py::abs(&f__nlon(py::sub(&v_lon2.clone(), &v_lon1.clone())?)?)?;
    let _ = f__need({ let t5518 = v_dl.clone(); Py::Bool({ let t5519 = Py::Int(0i64); if py::gt(&t5518, &t5519)? { true } else { false } }) }, Py::str("the two longitudes must differ"))?;
    v_p = py::mul(&f__N(v_E.clone(), py::mul(&v_lat.clone(), &c_DEG()?)?)?, &py::math1("cos", &py::mul(&v_lat.clone(), &c_DEG()?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("distance"), py::mul(&py::mul(&v_p.clone(), &v_dl.clone())?, &c_DEG()?)?), (Py::str("radius"), v_p.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polar_stereographic_forward(mut v_lat: Py, mut v_lon: Py, mut v_pole: Py, mut v_lat_ts: Py, mut v_lon0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon"), v_lon.clone())?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let _ = f__need({ let t5588 = v_pole.clone(); Py::Bool({ let t5589 = Py::tuple(vec![Py::str("north"), Py::str("south")]); if py::contains(&t5589, &t5588)? { true } else { false } }) }, Py::str("the pole must be north or south"))?;
    v_s = (if ({ let t5590 = v_pole.clone(); Py::Bool({ let t5591 = Py::str("north"); if py::eq(&t5590, &t5591) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    let _ = f__need({ let t5592 = Py::Int(0i64); Py::Bool({ let t5593 = py::mul(&v_s.clone(), &v_lat.clone())?; if py::lt(&t5592, &t5593)? { { let t5594 = Py::Int(90i64); if py::lt(&t5593, &t5594)? { true } else { false } } } else { false } }) }, Py::str("the point must lie in the hemisphere of the pole"))?;
    let _ = f__lat(Py::str("lat_ts"), v_lat_ts.clone(), Py::Bool(false))?;
    let _ = f__need({ let t5595 = Py::Int(30i64); Py::Bool({ let t5596 = py::mul(&v_s.clone(), &v_lat_ts.clone())?; if py::le(&t5595, &t5596)? { { let t5597 = Py::Float(89.99); if py::le(&t5596, &t5597)? { true } else { false } } } else { false } }) }, Py::str("the standard parallel must lie 30 to 89.99 degrees from the equator in the pole's hemisphere"))?;
    v_m = { let v_E = v_E.clone(); let v_FE = v_FE.clone(); let v_FN = v_FN.clone(); let v_ellipsoid = v_ellipsoid.clone(); let v_lat = v_lat.clone(); let v_lat_ts = v_lat_ts.clone(); let v_lon = v_lon.clone(); let v_lon0 = v_lon0.clone(); let v_pole = v_pole.clone(); let v_s = v_s.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::math1("cos", &v_p.clone())?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &v_p.clone())?, &Py::Int(2i64))?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t5598 = (Py::tuple(vec![py::mul(&py::mul(&v_s.clone(), &v_lat_ts.clone())?, &c_DEG()?)?, py::mul(&py::mul(&v_s.clone(), &v_lat.clone())?, &c_DEG()?)?])).items()?;
    if t5598.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pc = t5598[0].clone();
    v_p = t5598[1].clone();
    v_rho = py::truediv(&py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::call(&v_m.clone(), &[v_pc.clone()])?)?, &py::math1("exp", &py::neg(&f__psi(v_E.clone(), v_p.clone())?)?)?)?, &py::math1("exp", &py::neg(&f__psi(v_E.clone(), v_pc.clone())?)?)?)?;
    v_dl = py::mul(&f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?, &c_DEG()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::add(&v_FE.clone(), &py::mul(&v_rho.clone(), &py::math1("sin", &v_dl.clone())?)?)?), (Py::str("y"), py::sub(&v_FN.clone(), &py::mul(&py::mul(&v_s.clone(), &v_rho.clone())?, &py::math1("cos", &v_dl.clone())?)?)?), (Py::str("k"), py::truediv(&v_rho.clone(), &py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &py::call(&v_m.clone(), &[v_p.clone()])?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quadrat_test(mut v_x: Py, mut v_y: Py, mut v_nx: Py, mut v_ny: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_chi2: Py = Py::None;
    let mut v_counts: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_ix: Py = Py::None;
    let mut v_iy: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let mut v_y1: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t5777 = v_n.clone(); Py::Bool({ let t5778 = py::len(&v_y.clone())?; if py::eq(&t5777, &t5778) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t5779 = v_n.clone(); Py::Bool({ let t5780 = Py::Int(2i64); if py::ge(&t5779, &t5780)? { true } else { false } }) }, Py::str("give at least 2 points"))?;
    let _ = f__need({ let t5790 = Py::Bool(matches!(v_nx.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5790.truthy() { t5790 } else { { let t5789 = Py::Bool(matches!(v_ny.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5789.truthy() { t5789 } else { { let t5788 = { let t5781 = v_nx.clone(); Py::Bool({ let t5782 = Py::Int(1i64); if py::ge(&t5781, &t5782)? { true } else { false } }) }; if !t5788.truthy() { t5788 } else { { let t5787 = { let t5783 = v_ny.clone(); Py::Bool({ let t5784 = Py::Int(1i64); if py::ge(&t5783, &t5784)? { true } else { false } }) }; if !t5787.truthy() { t5787 } else { { let t5785 = py::mul(&v_nx.clone(), &v_ny.clone())?; Py::Bool({ let t5786 = Py::Int(2i64); if py::ge(&t5785, &t5786)? { true } else { false } }) } } } } } } } } }, Py::str("the grid needs at least 2 cells"))?;
    let t5791 = (Py::tuple(vec![py::minmax(&[v_x.clone()], false)?, py::minmax(&[v_x.clone()], true)?, py::minmax(&[v_y.clone()], false)?, py::minmax(&[v_y.clone()], true)?])).items()?;
    if t5791.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t5791[0].clone();
    v_x1 = t5791[1].clone();
    v_y0 = t5791[2].clone();
    v_y1 = t5791[3].clone();
    let _ = f__need({ let t5796 = { let t5792 = v_x1.clone(); Py::Bool({ let t5793 = v_x0.clone(); if py::gt(&t5792, &t5793)? { true } else { false } }) }; if !t5796.truthy() { t5796 } else { { let t5794 = v_y1.clone(); Py::Bool({ let t5795 = v_y0.clone(); if py::gt(&t5794, &t5795)? { true } else { false } }) } } }, Py::str("the points must span a rectangle"))?;
    let t5797 = (Py::tuple(vec![py::truediv(&py::sub(&v_x1.clone(), &v_x0.clone())?, &v_nx.clone())?, py::truediv(&py::sub(&v_y1.clone(), &v_y0.clone())?, &v_ny.clone())?])).items()?;
    if t5797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w = t5797[0].clone();
    v_h = t5797[1].clone();
    v_counts = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::mul(&v_nx.clone(), &v_ny.clone())?)?;
    for t5798 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? {
        let t5803 = (t5798).items()?;
        if t5803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t5803[0].clone();
        v_b = t5803[1].clone();
        let t5799 = (Py::tuple(vec![py::int(&py::floordiv(&py::sub(&v_a.clone(), &v_x0.clone())?, &v_w.clone())?)?, py::int(&py::floordiv(&py::sub(&v_b.clone(), &v_y0.clone())?, &v_h.clone())?)?])).items()?;
        if t5799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ix = t5799[0].clone();
        v_iy = t5799[1].clone();
        let t5800 = (Py::tuple(vec![py::minmax(&[v_ix.clone(), py::sub(&v_nx.clone(), &Py::Int(1i64))?], false)?, py::minmax(&[v_iy.clone(), py::sub(&v_ny.clone(), &Py::Int(1i64))?], false)?])).items()?;
        if t5800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ix = t5800[0].clone();
        v_iy = t5800[1].clone();
        let t5801 = v_counts.clone(); let t5802 = py::add(&py::mul(&v_iy.clone(), &v_nx.clone())?, &v_ix.clone())?;
        py::setitem(&t5801, &t5802, py::iadd(&py::getitem(&t5801, &t5802)?, &Py::Int(1i64))?)?;
    }
    v_k = py::mul(&v_nx.clone(), &v_ny.clone())?;
    v_e = py::truediv(&v_n.clone(), &v_k.clone())?;
    v_chi2 = py::truediv(&py::fsum(&{ let mut out = vec![]; for t5804 in py::iter(&(v_counts.clone()))? { let mut v_c: Py; 
v_c = t5804;
out.push(py::pow(&py::sub(&v_c.clone(), &v_e.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_e.clone())?;
    return Ok(Py::dict(vec![(Py::str("chi2"), f__r(v_chi2.clone())?), (Py::str("df"), py::float(&py::sub(&v_k.clone(), &Py::Int(1i64))?)?), (Py::str("p_value"), f__r(f__chi2_sf(v_chi2.clone(), py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?), (Py::str("vmr"), f__r(py::truediv(&v_chi2.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?), (Py::str("counts"), { let mut out = vec![]; for t5815 in py::iter(&(v_counts.clone()))? { let mut v_c: Py; 
v_c = t5815;
out.push(py::float(&v_c.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radii_of_curvature(mut v_lat: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_N: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(true))?;
    let t5869 = (Py::tuple(vec![f__M(v_E.clone(), py::mul(&v_lat.clone(), &c_DEG()?)?)?, f__N(v_E.clone(), py::mul(&v_lat.clone(), &c_DEG()?)?)?])).items()?;
    if t5869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_M = t5869[0].clone();
    v_N = t5869[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("N"), v_N.clone()), (Py::str("R_gauss"), py::math1("sqrt", &py::mul(&v_M.clone(), &v_N.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rank_size(mut v_populations: Py) -> R {
    let mut v_c0: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_lx: Py = Py::None;
    let mut v_ly: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_populations.clone())?;
    let _ = f__need({ let t5923 = v_n.clone(); Py::Bool({ let t5924 = Py::Int(3i64); if py::ge(&t5923, &t5924)? { true } else { false } }) }, Py::str("give at least 3 settlement populations"))?;
    for t5925 in py::iter(&(v_populations.clone()))? {
        v_v = t5925;
        let _ = f__pos(Py::str("each population"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_p = py::sorted(&v_populations.clone(), (Py::Bool(true)).truthy())?;
    v_lx = { let mut out = vec![]; for t5932 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5932;
out.push(py::math1("log", &py::add(&v_i.clone(), &Py::Int(1i64))?)?); } Py::list(out) };
    v_ly = { let mut out = vec![]; for t5933 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t5933;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) };
    let t5943 = (f__linfit(v_lx.clone(), v_ly.clone())?).items()?;
    if t5943.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c0 = t5943[0].clone();
    v_c1 = t5943[1].clone();
    v_r2 = t5943[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("q"), py::neg(&v_c1.clone())?), (Py::str("P1_fit"), py::math1("exp", &v_c0.clone())?), (Py::str("r_squared"), v_r2.clone()), (Py::str("primacy"), py::truediv(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rhumb_destination(mut v_lat1: Py, mut v_lon1: Py, mut v_bearing: Py, mut v_distance: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_ca: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat1"), v_lat1.clone(), Py::Bool(false))?;
    let _ = f__need({ let t6009 = py::abs(&v_lat1.clone())?; Py::Bool({ let t6010 = Py::Int(89i64); if py::le(&t6009, &t6010)? { true } else { false } }) }, Py::str("the start latitude must lie within 89 degrees of the equator"))?;
    let _ = f__lon(Py::str("lon1"), v_lon1.clone())?;
    let _ = f__fin(Py::str("bearing"), Py::tuple(vec![v_bearing.clone()]))?;
    let _ = f__pos(Py::str("the distance"), Py::tuple(vec![v_distance.clone()]))?;
    v_a = py::mul(&v_bearing.clone(), &c_DEG()?)?;
    v_p1 = py::mul(&v_lat1.clone(), &c_DEG()?)?;
    v_ca = py::math1("cos", &v_a.clone())?;
    if ({ let t6020 = py::abs(&v_ca.clone())?; Py::Bool({ let t6021 = Py::Float(1e-12); if py::lt(&t6020, &t6021)? { true } else { false } }) }).truthy() {
        v_dl = py::truediv(&py::mul(&v_distance.clone(), &py::math1("sin", &v_a.clone())?)?, &py::mul(&f__N(v_E.clone(), v_p1.clone())?, &py::math1("cos", &v_p1.clone())?)?)?;
        return Ok(f__out(Py::dict(vec![(Py::str("lat2"), v_lat1.clone()), (Py::str("lon2"), f__nlon(py::add(&v_lon1.clone(), &py::truediv(&v_dl.clone(), &c_DEG()?)?)?)?)]))?);
    }
    v_m2 = py::add(&f__merid(v_E.clone(), v_p1.clone())?, &py::mul(&v_distance.clone(), &v_ca.clone())?)?;
    let _ = f__need({ let t6039 = py::abs(&v_m2.clone())?; Py::Bool({ let t6040 = f__merid(v_E.clone(), py::mul(&Py::Int(89i64), &c_DEG()?)?)?; if py::lt(&t6039, &t6040)? { true } else { false } }) }, Py::str("the rhumb line would reach within 1 degree of a pole"))?;
    v_p2 = f__merid_inv(v_E.clone(), v_m2.clone())?;
    v_dl = py::mul(&py::math1("tan", &v_a.clone())?, &py::sub(&f__psi(v_E.clone(), v_p2.clone())?, &f__psi(v_E.clone(), v_p1.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lat2"), py::truediv(&v_p2.clone(), &c_DEG()?)?), (Py::str("lon2"), f__nlon(py::add(&v_lon1.clone(), &py::truediv(&v_dl.clone(), &c_DEG()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rhumb_inverse(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_brg: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_dm: Py = Py::None;
    let mut v_dpsi: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat1"), v_lat1.clone(), Py::Bool(false))?;
    let _ = f__lat(Py::str("lat2"), v_lat2.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon1"), v_lon1.clone())?;
    let _ = f__lon(Py::str("lon2"), v_lon2.clone())?;
    let _ = f__need({ let t6104 = { let t6100 = py::abs(&v_lat1.clone())?; Py::Bool({ let t6101 = Py::Int(89i64); if py::le(&t6100, &t6101)? { true } else { false } }) }; if !t6104.truthy() { t6104 } else { { let t6102 = py::abs(&v_lat2.clone())?; Py::Bool({ let t6103 = Py::Int(89i64); if py::le(&t6102, &t6103)? { true } else { false } }) } } }, Py::str("latitudes must lie within 89 degrees of the equator"))?;
    v_dl = py::mul(&f__nlon(py::sub(&v_lon2.clone(), &v_lon1.clone())?)?, &c_DEG()?)?;
    let t6109 = (Py::tuple(vec![py::mul(&v_lat1.clone(), &c_DEG()?)?, py::mul(&v_lat2.clone(), &c_DEG()?)?])).items()?;
    if t6109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t6109[0].clone();
    v_p2 = t6109[1].clone();
    if ({ let t6110 = v_lat1.clone(); Py::Bool({ let t6111 = v_lat2.clone(); if py::eq(&t6110, &t6111) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t6112 = v_dl.clone(); Py::Bool({ let t6113 = Py::Int(0i64); if py::ne(&t6112, &t6113) { true } else { false } }) }, Py::str("the two points must be distinct"))?;
        v_s = py::mul(&py::mul(&py::abs(&v_dl.clone())?, &f__N(v_E.clone(), v_p1.clone())?)?, &py::math1("cos", &v_p1.clone())?)?;
        v_brg = (if ({ let t6114 = v_dl.clone(); Py::Bool({ let t6115 = Py::Int(0i64); if py::gt(&t6114, &t6115)? { true } else { false } }) }).truthy() { Py::Float(90.0) } else { Py::Float(270.0) });
        return Ok(f__out(Py::dict(vec![(Py::str("distance"), v_s.clone()), (Py::str("bearing"), v_brg.clone())]))?);
    }
    v_dpsi = py::sub(&f__psi(v_E.clone(), v_p2.clone())?, &f__psi(v_E.clone(), v_p1.clone())?)?;
    v_dm = py::sub(&f__merid(v_E.clone(), v_p2.clone())?, &f__merid(v_E.clone(), v_p1.clone())?)?;
    v_brg = f__az(py::truediv(&py::atan2(&v_dl.clone(), &v_dpsi.clone())?, &c_DEG()?)?)?;
    v_s = py::truediv(&py::mul(&py::hypot(&[v_dl.clone(), v_dpsi.clone()])?, &v_dm.clone())?, &v_dpsi.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("distance"), v_s.clone()), (Py::str("bearing"), v_brg.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ripley_k(mut v_x: Py, mut v_y: Py, mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pd: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t6174 = v_n.clone(); Py::Bool({ let t6175 = py::len(&v_y.clone())?; if py::eq(&t6174, &t6175) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t6176 = v_n.clone(); Py::Bool({ let t6177 = Py::Int(3i64); if py::ge(&t6176, &t6177)? { true } else { false } }) }, Py::str("give at least 3 points"))?;
    let _ = f__need({ let t6178 = py::len(&v_d.clone())?; Py::Bool({ let t6179 = Py::Int(1i64); if py::ge(&t6178, &t6179)? { true } else { false } }) }, Py::str("give at least one distance"))?;
    for t6180 in py::iter(&(v_d.clone()))? {
        v_v = t6180;
        let _ = f__pos(Py::str("each distance"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_A = py::mul(&py::sub(&py::minmax(&[v_x.clone()], true)?, &py::minmax(&[v_x.clone()], false)?)?, &py::sub(&py::minmax(&[v_y.clone()], true)?, &py::minmax(&[v_y.clone()], false)?)?)?;
    let _ = f__need({ let t6187 = v_A.clone(); Py::Bool({ let t6188 = Py::Int(0i64); if py::gt(&t6187, &t6188)? { true } else { false } }) }, Py::str("the points must span a rectangle"))?;
    v_pd = { let mut out = vec![]; for t6189 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6189;
for t6190 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6190;
out.push(py::hypot(&[py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?])?); }} Py::list(out) };
    v_K = { let mut out = vec![]; for t6191 in py::iter(&(v_d.clone()))? { let mut v_t: Py; 
v_t = t6191;
out.push(py::truediv(&py::mul(&py::mul(&v_A.clone(), &Py::Int(2i64))?, &py::sum(&{ let mut out = vec![]; for t6192 in py::iter(&(v_pd.clone()))? { let mut v_p: Py; 
v_p = t6192;
if ({ let t6193 = v_p.clone(); Py::Bool({ let t6194 = v_t.clone(); if py::lt(&t6193, &t6194)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?, &py::mul(&v_n.clone(), &v_n.clone())?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("K"), f__r(v_K.clone())?), (Py::str("L"), f__r({ let mut out = vec![]; for t6205 in py::iter(&(v_K.clone()))? { let mut v_k: Py; 
v_k = t6205;
out.push(py::math1("sqrt", &py::truediv(&v_k.clone(), &Py::Float(std::f64::consts::PI))?)?); } Py::list(out) })?), (Py::str("area"), f__r(v_A.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_segregation_evenness(mut v_x: Py, mut v_t: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_ent: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let t6272 = (f__seg_in(v_x.clone(), v_t.clone())?).items()?;
    if t6272.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t6272[0].clone();
    v_T = t6272[1].clone();
    v_P = t6272[2].clone();
    v_pi = { let mut out = vec![]; for t6273 in py::iter(&(py::zip(&[v_x.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6274 = (t6273).items()?;
if t6274.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6274[0].clone();
v_b = t6274[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    v_D = py::truediv(&py::fsum(&{ let mut out = vec![]; for t6275 in py::iter(&(py::zip(&[v_t.clone(), v_pi.clone()])?))? { let mut v_b: Py; let mut v_p: Py; 
let t6276 = (t6275).items()?;
if t6276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_b = t6276[0].clone();
v_p = t6276[1].clone();
out.push(py::mul(&v_b.clone(), &py::abs(&py::sub(&v_p.clone(), &v_P.clone())?)?)?); } Py::list(out) })?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_T.clone())?, &v_P.clone())?, &py::sub(&Py::Int(1i64), &v_P.clone())?)?)?;
    v_G = py::truediv(&py::fsum(&{ let mut out = vec![]; for t6277 in py::range_iter(&[(py::len(&v_t.clone())?).i()?]) { let mut v_i: Py; 
v_i = t6277;
for t6278 in py::range_iter(&[(py::len(&v_t.clone())?).i()?]) { let mut v_j: Py; 
v_j = t6278;
out.push(py::mul(&py::mul(&py::getitem(&v_t.clone(), &v_i.clone())?, &py::getitem(&v_t.clone(), &v_j.clone())?)?, &py::abs(&py::sub(&py::getitem(&v_pi.clone(), &v_i.clone())?, &py::getitem(&v_pi.clone(), &v_j.clone())?)?)?)?); }} Py::list(out) })?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_T.clone())?, &v_T.clone())?, &v_P.clone())?, &py::sub(&Py::Int(1i64), &v_P.clone())?)?)?;
    v_ent = { let v_D = v_D.clone(); let v_G = v_G.clone(); let v_P = v_P.clone(); let v_T = v_T.clone(); let v_X = v_X.clone(); let v_pi = v_pi.clone(); let v_t = v_t.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t6279 = v_p.clone(); Py::Bool({ let t6280 = Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]); if py::contains(&t6280, &t6279)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::neg(&py::add(&py::mul(&v_p.clone(), &py::math1("log", &v_p.clone())?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &py::math1("log", &py::sub(&Py::Int(1i64), &v_p.clone())?)?)?)?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_E = py::call(&v_ent.clone(), &[v_P.clone()])?;
    v_H = py::truediv(&py::fsum(&{ let mut out = vec![]; for t6281 in py::iter(&(py::zip(&[v_t.clone(), v_pi.clone()])?))? { let mut v_b: Py; let mut v_p: Py; 
let t6282 = (t6281).items()?;
if t6282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_b = t6282[0].clone();
v_p = t6282[1].clone();
out.push(py::mul(&v_b.clone(), &py::sub(&v_E.clone(), &py::call(&v_ent.clone(), &[v_p.clone()])?)?)?); } Py::list(out) })?, &py::mul(&v_E.clone(), &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dissimilarity"), v_D.clone()), (Py::str("gini"), v_G.clone()), (Py::str("theil_h"), v_H.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_segregation_exposure(mut v_x: Py, mut v_t: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_xPx: Py = Py::None;
    let mut v_xPy: Py = Py::None;
    let t6361 = (f__seg_in(v_x.clone(), v_t.clone())?).items()?;
    if t6361.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t6361[0].clone();
    v_T = t6361[1].clone();
    v_P = t6361[2].clone();
    v_xPx = py::fsum(&{ let mut out = vec![]; for t6362 in py::iter(&(py::zip(&[v_x.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6363 = (t6362).items()?;
if t6363.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6363[0].clone();
v_b = t6363[1].clone();
out.push(py::mul(&py::truediv(&v_a.clone(), &v_X.clone())?, &py::truediv(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) })?;
    v_xPy = py::fsum(&{ let mut out = vec![]; for t6364 in py::iter(&(py::zip(&[v_x.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6365 = (t6364).items()?;
if t6365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6365[0].clone();
v_b = t6365[1].clone();
out.push(py::mul(&py::truediv(&v_a.clone(), &v_X.clone())?, &py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &v_b.clone())?)?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("isolation"), v_xPx.clone()), (Py::str("exposure"), v_xPy.clone()), (Py::str("correlation_ratio"), py::truediv(&py::sub(&v_xPx.clone(), &v_P.clone())?, &py::sub(&Py::Int(1i64), &v_P.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_service_area(mut v_edges: Py, mut v_source: Py, mut v_cutoff: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_adj = f__net(v_edges.clone(), Py::None, Py::Bool(true))?;
    v_s = py::int(&v_source.clone())?;
    let _ = f__need({ let t6450 = { let t6445 = py::float(&v_source.clone())?; Py::Bool({ let t6446 = v_s.clone(); if py::eq(&t6445, &t6446) { true } else { false } }) }; if !t6450.truthy() { t6450 } else { { let t6447 = Py::Int(0i64); Py::Bool({ let t6448 = v_s.clone(); if py::le(&t6447, &t6448)? { { let t6449 = py::len(&v_adj.clone())?; if py::lt(&t6448, &t6449)? { true } else { false } } } else { false } }) } } }, Py::str("the source must be a node of the network"))?;
    let _ = f__pos(Py::str("the cutoff"), Py::tuple(vec![v_cutoff.clone()]))?;
    v_d = f__dijkstra(v_adj.clone(), v_s.clone())?;
    v_R = { let mut out = vec![]; for t6464 in py::range_iter(&[(py::len(&v_adj.clone())?).i()?]) { let mut v_i: Py; 
v_i = t6464;
if ({ let t6465 = py::getitem(&v_d.clone(), &v_i.clone())?; Py::Bool({ let t6466 = v_cutoff.clone(); if py::le(&t6465, &t6466)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("reachable"), { let mut out = vec![]; for t6467 in py::iter(&(v_R.clone()))? { let mut v_i: Py; 
v_i = t6467;
out.push(py::float(&v_i.clone())?); } Py::list(out) }), (Py::str("count"), py::float(&py::len(&v_R.clone())?)?), (Py::str("farthest"), f__r(py::minmax(&[{ let mut out = vec![]; for t6478 in py::iter(&(v_R.clone()))? { let mut v_i: Py; 
v_i = t6478;
out.push(py::getitem(&v_d.clone(), &v_i.clone())?); } Py::list(out) }], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shimbel_index(mut v_edges: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_adj = f__net(v_edges.clone(), Py::None, Py::Bool(true))?;
    v_n = py::len(&v_adj.clone())?;
    v_D = f__all_dist(v_adj.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t6554 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t6554;
for t6555 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t6555;
out.push(py::math1("isfinite", &v_v.clone())?); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the network must be connected (every node numbered 0 .. n-1 must be reachable)"))?;
    v_S = { let mut out = vec![]; for t6556 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t6556;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("shimbel"), f__r(v_S.clone())?), (Py::str("total"), f__r(py::fsum(&v_S.clone())?)?), (Py::str("most_accessible"), py::float(&py::minmax_key(&[Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_D = v_D.clone(); let v_S = v_S.clone(); let v_adj = v_adj.clone(); let v_edges = v_edges.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_S.clone(), &v_i.clone())?, v_i.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sinusoidal_forward(mut v_lat: Py, mut v_lon: Py, mut v_lon0: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Float(89.9))?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    v_p = py::mul(&v_lat.clone(), &c_DEG()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::truediv(&py::mul(&py::mul(&py::mul(&py::getitem(&v_E.clone(), &Py::str("a"))?, &f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?)?, &c_DEG()?)?, &py::math1("cos", &v_p.clone())?)?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::getitem(&v_E.clone(), &Py::str("e2"))?, &py::pow(&py::math1("sin", &v_p.clone())?, &Py::Int(2i64))?)?)?)?)?), (Py::str("y"), f__merid(v_E.clone(), v_p.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_slope_aspect(mut v_window: Py, mut v_cell_size: Py) -> R {
    let mut v_aspect: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__window(v_window.clone())?;
    let _ = f__pos(Py::str("the cell size"), Py::tuple(vec![v_cell_size.clone()]))?;
    let t6700 = (f__horn(v_window.clone(), v_cell_size.clone())?).items()?;
    if t6700.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t6700[0].clone();
    v_q = t6700[1].clone();
    v_g = py::hypot(&[v_p.clone(), v_q.clone()])?;
    v_aspect = (if ({ let t6701 = v_g.clone(); Py::Bool({ let t6702 = Py::Int(0i64); if py::eq(&t6701, &t6702) { true } else { false } }) }).truthy() { Py::None } else { f__az(py::truediv(&py::atan2(&py::neg(&v_p.clone())?, &py::neg(&v_q.clone())?)?, &c_DEG()?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("slope_deg"), py::truediv(&py::math1("atan", &v_g.clone())?, &c_DEG()?)?), (Py::str("slope_percent"), py::mul(&Py::Int(100i64), &v_g.clone())?), (Py::str("aspect_deg"), v_aspect.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spatial_lag(mut v_y: Py, mut v_W: Py, mut v_transform: Py) -> R {
    let mut v_Wt: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_y.clone())?;
    let _ = f__need({ let t6760 = v_n.clone(); Py::Bool({ let t6761 = Py::Int(2i64); if py::ge(&t6760, &t6761)? { true } else { false } }) }, Py::str("give at least 2 observations"))?;
    for t6762 in py::iter(&(v_y.clone()))? {
        v_v = t6762;
        let _ = f__fin(Py::str("each value"), Py::tuple(vec![v_v.clone()]))?;
    }
    let _ = f__need({ let t6766 = v_transform.clone(); Py::Bool({ let t6767 = Py::tuple(vec![Py::str("row"), Py::str("none")]); if py::contains(&t6767, &t6766)? { true } else { false } }) }, Py::str("the transform must be row or none"))?;
    v_Wt = f__weights(v_W.clone(), v_n.clone(), v_transform.clone(), Py::Bool(false), Py::Bool(false), Py::Bool(false))?;
    return Ok(Py::dict(vec![(Py::str("lag"), f__r({ let mut out = vec![]; for t6804 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6804;
out.push(py::fsum(&{ let mut out = vec![]; for t6805 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6805;
out.push(py::mul(&py::getitem(&py::getitem(&v_Wt.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_std_ellipse(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_ang: Py = Py::None;
    let mut v_bearing: Py = Py::None;
    let mut v_corr: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let mut v_tr: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t6847 = v_n.clone(); Py::Bool({ let t6848 = py::len(&v_y.clone())?; if py::eq(&t6847, &t6848) { true } else { false } }) }, Py::str("give one y per x"))?;
    let _ = f__need({ let t6849 = v_n.clone(); Py::Bool({ let t6850 = Py::Int(3i64); if py::ge(&t6849, &t6850)? { true } else { false } }) }, Py::str("give at least 3 points"))?;
    let t6851 = (Py::tuple(vec![py::truediv(&py::fsum(&v_x.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_y.clone())?, &v_n.clone())?])).items()?;
    if t6851.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t6851[0].clone();
    v_my = t6851[1].clone();
    v_sxx = py::truediv(&py::fsum(&{ let mut out = vec![]; for t6852 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t6852;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_n.clone())?;
    v_syy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t6853 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t6853;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_n.clone())?;
    v_sxy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t6854 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6855 = (t6854).items()?;
if t6855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6855[0].clone();
v_b = t6855[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) })?, &v_n.clone())?;
    let t6856 = (Py::tuple(vec![py::add(&v_sxx.clone(), &v_syy.clone())?, py::sub(&py::mul(&v_sxx.clone(), &v_syy.clone())?, &py::mul(&v_sxy.clone(), &v_sxy.clone())?)?])).items()?;
    if t6856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tr = t6856[0].clone();
    v_det = t6856[1].clone();
    v_disc = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::add(&py::truediv(&py::pow(&py::sub(&v_sxx.clone(), &v_syy.clone())?, &Py::Int(2i64))?, &Py::Int(4i64))?, &py::mul(&v_sxy.clone(), &v_sxy.clone())?)?], true)?)?;
    let t6857 = (Py::tuple(vec![py::add(&py::truediv(&v_tr.clone(), &Py::Int(2i64))?, &v_disc.clone())?, py::sub(&py::truediv(&v_tr.clone(), &Py::Int(2i64))?, &v_disc.clone())?])).items()?;
    if t6857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l1 = t6857[0].clone();
    v_l2 = t6857[1].clone();
    let _ = f__need({ let t6862 = { let t6858 = v_l2.clone(); Py::Bool({ let t6859 = py::mul(&Py::Float(1e-12), &v_l1.clone())?; if py::gt(&t6858, &t6859)? { true } else { false } }) }; if !t6862.truthy() { t6862 } else { { let t6860 = v_disc.clone(); Py::Bool({ let t6861 = py::mul(&Py::Float(1e-09), &v_tr.clone())?; if py::gt(&t6860, &t6861)? { true } else { false } }) } } }, Py::str("the points must not be collinear or circularly symmetric (the axes are undefined)"))?;
    v_corr = py::truediv(&py::mul(&Py::Int(2i64), &v_n.clone())?, &py::sub(&v_n.clone(), &Py::Int(2i64))?)?;
    v_ang = (if ({ let t6863 = v_sxy.clone(); Py::Bool({ let t6864 = Py::Int(0i64); if py::ne(&t6863, &t6864) { true } else { false } }) }).truthy() { py::atan2(&py::sub(&v_l1.clone(), &v_sxx.clone())?, &v_sxy.clone())? } else { (if ({ let t6865 = v_syy.clone(); Py::Bool({ let t6866 = v_sxx.clone(); if py::gt(&t6865, &t6866)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))? }) });
    v_bearing = py::fmod(&py::add(&py::sub(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_ang.clone())?, &py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?)?, &Py::Float(std::f64::consts::PI))?;
    return Ok(f__out(Py::dict(vec![(Py::str("sx"), py::math1("sqrt", &py::mul(&v_l1.clone(), &v_corr.clone())?)?), (Py::str("sy"), py::math1("sqrt", &py::mul(&v_l2.clone(), &v_corr.clone())?)?), (Py::str("orientation"), py::truediv(&v_bearing.clone(), &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_terrain_ruggedness(mut v_window: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let _ = f__window(v_window.clone())?;
    v_c = py::getitem(&py::getitem(&v_window.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?;
    v_nb = { let mut out = vec![]; for t7005 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_r: Py; 
v_r = t7005;
for t7006 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t7006;
if ({ let t7007 = Py::tuple(vec![v_r.clone(), v_k.clone()]); Py::Bool({ let t7008 = Py::tuple(vec![Py::Int(1i64), Py::Int(1i64)]); if py::ne(&t7007, &t7008) { true } else { false } }) }).truthy() { out.push(py::getitem(&py::getitem(&v_window.clone(), &v_r.clone())?, &v_k.clone())?); }}} Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("TRI"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t7021 in py::iter(&(v_nb.clone()))? { let mut v_v: Py; 
v_v = t7021;
out.push(py::pow(&py::sub(&v_v.clone(), &v_c.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?), (Py::str("TPI"), py::sub(&v_c.clone(), &py::truediv(&py::fsum(&v_nb.clone())?, &Py::Int(8i64))?)?), (Py::str("relief"), py::sub(&py::minmax(&[py::add(&v_nb.clone(), &Py::list(vec![v_c.clone()]))?], true)?, &py::minmax(&[py::add(&v_nb.clone(), &Py::list(vec![v_c.clone()]))?], false)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tissot_indicatrix(mut v_projection: Py, mut v_lat: Py, mut v_lon: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_ap: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bp: Py = Py::None;
    let mut v_cross: Py = Py::None;
    let mut v_ct: Py = Py::None;
    let mut v_dot: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_omega: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_xl: Py = Py::None;
    let mut v_xp: Py = Py::None;
    let mut v_yl: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let _ = f__need({ let t7063 = v_projection.clone(); Py::Bool({ let t7064 = c__TISSOT()?; if py::contains(&t7064, &t7063)? { true } else { false } }) }, py::add(&Py::str("the projection must be one of "), &py::method(&Py::str(", "), "join", &[c__TISSOT()?])?)?)?;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(false))?;
    let _ = f__need({ let t7074 = py::abs(&v_lat.clone())?; Py::Bool({ let t7075 = Py::Int(80i64); if py::le(&t7074, &t7075)? { true } else { false } }) }, Py::str("the latitude must lie within 80 degrees of the equator"))?;
    let _ = f__fin(Py::str("lon"), Py::tuple(vec![v_lon.clone()]))?;
    let _ = f__need({ let t7076 = py::neg(&Py::Int(180i64))?; Py::Bool({ let t7077 = v_lon.clone(); if py::le(&t7076, &t7077)? { { let t7078 = Py::Int(180i64); if py::le(&t7077, &t7078)? { true } else { false } } } else { false } }) }, Py::str("the longitude from the central meridian must lie between -180 and 180"))?;
    let t7079 = (Py::tuple(vec![py::mul(&v_lat.clone(), &c_DEG()?)?, py::mul(&v_lon.clone(), &c_DEG()?)?])).items()?;
    if t7079.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t7079[0].clone();
    v_l = t7079[1].clone();
    let t7091 = (f__tissot_derivs(v_projection.clone(), v_p.clone(), v_l.clone())?).items()?;
    if t7091.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_xl = t7091[0].clone();
    v_yl = t7091[1].clone();
    v_xp = t7091[2].clone();
    v_yp = t7091[3].clone();
    v_h = py::hypot(&[v_xp.clone(), v_yp.clone()])?;
    v_k = py::truediv(&py::hypot(&[v_xl.clone(), v_yl.clone()])?, &py::math1("cos", &v_p.clone())?)?;
    v_cross = py::truediv(&py::abs(&py::sub(&py::mul(&v_yp.clone(), &v_xl.clone())?, &py::mul(&v_xp.clone(), &v_yl.clone())?)?)?, &py::mul(&py::mul(&v_h.clone(), &v_k.clone())?, &py::math1("cos", &v_p.clone())?)?)?;
    v_dot = py::truediv(&py::add(&py::mul(&v_xp.clone(), &v_xl.clone())?, &py::mul(&v_yp.clone(), &v_yl.clone())?)?, &py::mul(&py::mul(&v_h.clone(), &v_k.clone())?, &py::math1("cos", &v_p.clone())?)?)?;
    v_th = py::atan2(&v_cross.clone(), &py::abs(&v_dot.clone())?)?;
    let t7092 = (Py::tuple(vec![py::math1("sin", &v_th.clone())?, py::math1("cos", &v_th.clone())?])).items()?;
    if t7092.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_st = t7092[0].clone();
    v_ct = t7092[1].clone();
    v_ap = py::math1("sqrt", &py::add(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_k.clone(), &v_k.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_h.clone())?, &v_k.clone())?, &v_st.clone())?)?)?;
    v_bp = py::math1("sqrt", &py::add(&py::pow(&py::sub(&v_h.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_h.clone())?, &v_k.clone())?, &v_ct.clone())?, &v_ct.clone())?, &py::add(&Py::Int(1i64), &v_st.clone())?)?)?)?;
    let t7093 = (Py::tuple(vec![py::truediv(&py::add(&v_ap.clone(), &v_bp.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_ap.clone(), &v_bp.clone())?, &Py::Int(2i64))?])).items()?;
    if t7093.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t7093[0].clone();
    v_b = t7093[1].clone();
    v_omega = py::mul(&Py::Int(2i64), &py::math1("asin", &py::truediv(&py::sub(&v_a.clone(), &v_b.clone())?, &py::add(&v_a.clone(), &v_b.clone())?)?)?)?;
    if ({ let t7094 = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?; Py::Bool({ let t7095 = py::mul(&Py::Float(1e-12), &v_a.clone())?; if py::le(&t7094, &t7095)? { true } else { false } }) }).truthy() {
        let t7096 = py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?;
        v_a = t7096.clone();
        v_b = t7096.clone();
        v_omega = Py::Float(0.0);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("k"), v_k.clone()), (Py::str("theta_prime"), py::truediv(&v_th.clone(), &c_DEG()?)?), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("omega"), py::truediv(&v_omega.clone(), &c_DEG()?)?), (Py::str("s"), py::mul(&py::mul(&v_h.clone(), &v_k.clone())?, &v_st.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tm_forward(mut v_lat: Py, mut v_lon: Py, mut v_lat0: Py, mut v_lon0: Py, mut v_k0: Py, mut v_ellipsoid: Py, mut v_FE: Py, mut v_FN: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_X0: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(false))?;
    let _ = f__lat(Py::str("lat0"), v_lat0.clone(), Py::Bool(false))?;
    let _ = f__lon(Py::str("lon"), v_lon.clone())?;
    let _ = f__lon(Py::str("lon0"), v_lon0.clone())?;
    let _ = f__pos(Py::str("k0"), Py::tuple(vec![v_k0.clone()]))?;
    let _ = f__fin(Py::str("the false easting and northing"), Py::tuple(vec![v_FE.clone(), v_FN.clone()]))?;
    let _ = f__need({ let t7246 = py::abs(&v_lat.clone())?; Py::Bool({ let t7247 = Py::Int(89i64); if py::le(&t7246, &t7247)? { true } else { false } }) }, Py::str("the latitude must lie within 89 degrees of the equator"))?;
    v_dl = f__nlon(py::sub(&v_lon.clone(), &v_lon0.clone())?)?;
    let _ = f__need({ let t7252 = py::abs(&v_dl.clone())?; Py::Bool({ let t7253 = Py::Int(10i64); if py::le(&t7252, &t7253)? { true } else { false } }) }, Py::str("the point must lie within 10 degrees of longitude of the central meridian"))?;
    let t7262 = (f__tm_fwd(v_E.clone(), py::mul(&v_lat.clone(), &c_DEG()?)?, py::mul(&v_dl.clone(), &c_DEG()?)?)?).items()?;
    if t7262.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_X = t7262[0].clone();
    v_Y = t7262[1].clone();
    v_g = t7262[2].clone();
    v_k = t7262[3].clone();
    v_X0 = py::getitem(&f__tm_fwd(v_E.clone(), py::mul(&v_lat0.clone(), &c_DEG()?)?, Py::Float(0.0))?, &Py::Int(0i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::add(&v_FE.clone(), &py::mul(&v_k0.clone(), &v_Y.clone())?)?), (Py::str("y"), py::add(&v_FN.clone(), &py::mul(&v_k0.clone(), &py::sub(&v_X.clone(), &v_X0.clone())?)?)?), (Py::str("k"), py::mul(&v_k0.clone(), &v_k.clone())?), (Py::str("gamma"), py::truediv(&v_g.clone(), &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_toa_reflectance(mut v_radiance: Py, mut v_esun: Py, mut v_doy: Py, mut v_sun_elevation: Py) -> R {
    let mut v_d2: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__nonneg(Py::str("the radiance"), Py::tuple(vec![v_radiance.clone()]))?;
    let _ = f__pos(Py::str("ESUN"), Py::tuple(vec![v_esun.clone()]))?;
    let _ = f__need({ let t7165 = Py::Bool(matches!(v_doy.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t7165.truthy() { t7165 } else { { let t7162 = Py::Int(1i64); Py::Bool({ let t7163 = v_doy.clone(); if py::le(&t7162, &t7163)? { { let t7164 = Py::Int(366i64); if py::le(&t7163, &t7164)? { true } else { false } } } else { false } }) } } }, Py::str("the day of year must be a whole number from 1 to 366"))?;
    let _ = f__fin(Py::str("the sun elevation"), Py::tuple(vec![v_sun_elevation.clone()]))?;
    let _ = f__need({ let t7169 = Py::Int(0i64); Py::Bool({ let t7170 = v_sun_elevation.clone(); if py::lt(&t7169, &t7170)? { { let t7171 = Py::Int(90i64); if py::le(&t7170, &t7171)? { true } else { false } } } else { false } }) }, Py::str("the sun must be above the horizon (elevation 0-90 degrees)"))?;
    v_d2 = py::truediv(&Py::Float(1.0), &f__spencer(v_doy.clone())?)?;
    v_rho = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_radiance.clone())?, &v_d2.clone())?, &py::mul(&v_esun.clone(), &py::math1("sin", &py::mul(&v_sun_elevation.clone(), &c_DEG()?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("earth_sun_distance"), py::math1("sqrt", &v_d2.clone())?), (Py::str("reflectance"), v_rho.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trend_surface(mut v_x: Py, mut v_y: Py, mut v_z: Py, mut v_order: Py) -> R {
    let mut v_beta: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_fit: Py = Py::None;
    let mut v_mz: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_sst: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t7316 = v_n.clone(); Py::Bool({ let t7317 = py::len(&v_y.clone())?; if py::eq(&t7316, &t7317) { { let t7318 = py::len(&v_z.clone())?; if py::eq(&t7317, &t7318) { true } else { false } } } else { false } }) }, Py::str("give one y and one value per x"))?;
    let _ = f__need({ let t7321 = Py::Bool(matches!(v_order.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t7321.truthy() { t7321 } else { { let t7319 = v_order.clone(); Py::Bool({ let t7320 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)]); if py::contains(&t7320, &t7319)? { true } else { false } }) } } }, Py::str("the order must be 1 or 2"))?;
    v_p = (if ({ let t7322 = v_order.clone(); Py::Bool({ let t7323 = Py::Int(1i64); if py::eq(&t7322, &t7323) { true } else { false } }) }).truthy() { Py::Int(3i64) } else { Py::Int(6i64) });
    let _ = f__need({ let t7324 = v_n.clone(); Py::Bool({ let t7325 = v_p.clone(); if py::gt(&t7324, &t7325)? { true } else { false } }) }, Py::str(&["give more than ".to_string(), py::format(&v_p.clone(), "")?, " samples for an order-".to_string(), py::format(&v_order.clone(), "")?, " surface".to_string()].concat()))?;
    v_cols = Py::list(vec![py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?, Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_x.clone())?).items()?), Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_y.clone())?).items()?)]);
    if ({ let t7326 = v_order.clone(); Py::Bool({ let t7327 = Py::Int(2i64); if py::eq(&t7326, &t7327) { true } else { false } }) }).truthy() {
        v_cols = py::iadd(&v_cols, &Py::list(vec![{ let mut out = vec![]; for t7328 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t7328;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) }, { let mut out = vec![]; for t7329 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t7330 = (t7329).items()?;
if t7330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t7330[0].clone();
v_b = t7330[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, { let mut out = vec![]; for t7331 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t7331;
out.push(py::mul(&v_b.clone(), &v_b.clone())?); } Py::list(out) }]))?;
    }
    v_beta = f__ols(v_cols.clone(), v_z.clone())?;
    v_fit = { let mut out = vec![]; for t7354 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t7354;
out.push(py::fsum(&{ let mut out = vec![]; for t7355 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_k: Py; 
v_k = t7355;
out.push(py::mul(&py::getitem(&v_beta.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_cols.clone(), &v_k.clone())?, &v_i.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    v_mz = py::truediv(&py::fsum(&v_z.clone())?, &v_n.clone())?;
    v_sst = py::fsum(&{ let mut out = vec![]; for t7356 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t7356;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mz.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    let _ = f__need({ let t7357 = v_sst.clone(); Py::Bool({ let t7358 = Py::Int(0i64); if py::gt(&t7357, &t7358)? { true } else { false } }) }, Py::str("the values must not all be equal"))?;
    v_sse = py::fsum(&{ let mut out = vec![]; for t7359 in py::iter(&(py::zip(&[v_z.clone(), v_fit.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t7360 = (t7359).items()?;
if t7360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t7360[0].clone();
v_b = t7360[1].clone();
out.push(py::pow(&py::sub(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("coefficients"), f__r(v_beta.clone())?), (Py::str("r_squared"), f__r(py::sub(&Py::Int(1i64), &py::truediv(&v_sse.clone(), &v_sst.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_step_fca(mut v_edges: Py, mut v_population: Py, mut v_supply_nodes: Py, mut v_capacity: Py, mut v_threshold: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_acc: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_dsup: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pop: Py = Py::None;
    let mut v_ratios: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_population.clone())?;
    v_adj = f__net(v_edges.clone(), v_n.clone(), Py::Bool(true))?;
    for t7438 in py::iter(&(v_population.clone()))? {
        v_v = t7438;
        let _ = f__nonneg(Py::str("each population"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_S = { let mut out = vec![]; for t7445 in py::iter(&(v_supply_nodes.clone()))? { let mut v_s: Py; 
v_s = t7445;
out.push(py::int(&v_s.clone())?); } Py::list(out) };
    let _ = f__need({ let t7456 = { let t7446 = py::len(&v_S.clone())?; Py::Bool({ let t7447 = Py::Int(1i64); if py::ge(&t7446, &t7447)? { true } else { false } }) }; if !t7456.truthy() { t7456 } else { Py::Bool(({ let mut out = vec![]; for t7448 in py::iter(&(py::zip(&[v_supply_nodes.clone(), v_S.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t7449 = (t7448).items()?;
if t7449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t7449[0].clone();
v_b = t7449[1].clone();
out.push({ let t7455 = { let t7450 = py::float(&v_a.clone())?; Py::Bool({ let t7451 = v_b.clone(); if py::eq(&t7450, &t7451) { true } else { false } }) }; if !t7455.truthy() { t7455 } else { { let t7452 = Py::Int(0i64); Py::Bool({ let t7453 = v_b.clone(); if py::le(&t7452, &t7453)? { { let t7454 = v_n.clone(); if py::lt(&t7453, &t7454)? { true } else { false } } } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the supply sites must be nodes of the network"))?;
    let _ = f__need({ let t7457 = py::len(&py::set(&v_S.clone())?)?; Py::Bool({ let t7458 = py::len(&v_S.clone())?; if py::eq(&t7457, &t7458) { true } else { false } }) }, Py::str("each supply node may be listed once"))?;
    let _ = f__need({ let t7459 = py::len(&v_capacity.clone())?; Py::Bool({ let t7460 = py::len(&v_S.clone())?; if py::eq(&t7459, &t7460) { true } else { false } }) }, Py::str("give one capacity per supply site"))?;
    for t7461 in py::iter(&(v_capacity.clone()))? {
        v_v = t7461;
        let _ = f__nonneg(Py::str("each capacity"), Py::tuple(vec![v_v.clone()]))?;
    }
    let _ = f__pos(Py::str("the catchment threshold"), Py::tuple(vec![v_threshold.clone()]))?;
    v_ratios = Py::list(vec![]);
    v_dsup = { let mut out = vec![]; for t7468 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t7468;
out.push(f__dijkstra(v_adj.clone(), v_s.clone())?); } Py::list(out) };
    for t7476 in py::iter(&(py::enumerate(&v_S.clone(), 0)?))? {
        let t7482 = (t7476).items()?;
        if t7482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7482[0].clone();
        v_s = t7482[1].clone();
        v_pop = py::fsum(&{ let mut out = vec![]; for t7477 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t7477;
if ({ let t7478 = py::getitem(&py::getitem(&v_dsup.clone(), &v_k.clone())?, &v_i.clone())?; Py::Bool({ let t7479 = v_threshold.clone(); if py::le(&t7478, &t7479)? { true } else { false } }) }).truthy() { out.push(py::getitem(&v_population.clone(), &v_i.clone())?); }} Py::list(out) })?;
        let _ = py::method(&v_ratios.clone(), "append", &[(if ({ let t7480 = v_pop.clone(); Py::Bool({ let t7481 = Py::Int(0i64); if py::gt(&t7480, &t7481)? { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_capacity.clone(), &v_k.clone())?, &v_pop.clone())? } else { Py::Float(0.0) })])?;
    }
    v_acc = { let mut out = vec![]; for t7483 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t7483;
out.push(py::fsum(&{ let mut out = vec![]; for t7484 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) { let mut v_k: Py; 
v_k = t7484;
if ({ let t7485 = py::getitem(&py::getitem(&v_dsup.clone(), &v_k.clone())?, &v_i.clone())?; Py::Bool({ let t7486 = v_threshold.clone(); if py::le(&t7485, &t7486)? { true } else { false } }) }).truthy() { out.push(py::getitem(&v_ratios.clone(), &v_k.clone())?); }} Py::list(out) })?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("accessibility"), f__r(v_acc.clone())?), (Py::str("ratios"), f__r(v_ratios.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_utm_forward(mut v_lat: Py, mut v_lon: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_FN: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    v_z = f_utm_zone(v_lat.clone(), v_lon.clone())?;
    let _ = f__need({ let t7585 = py::abs(&f__nlon(py::sub(&v_lon.clone(), &py::getitem(&v_z.clone(), &Py::str("central_meridian"))?)?)?)?; Py::Bool({ let t7586 = Py::Int(10i64); if py::le(&t7585, &t7586)? { true } else { false } }) }, Py::str("the point lies too far from its zone's central meridian"))?;
    let t7595 = (f__tm_fwd(v_E.clone(), py::mul(&v_lat.clone(), &c_DEG()?)?, py::mul(&f__nlon(py::sub(&v_lon.clone(), &py::getitem(&v_z.clone(), &Py::str("central_meridian"))?)?)?, &c_DEG()?)?)?).items()?;
    if t7595.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_X = t7595[0].clone();
    v_Y = t7595[1].clone();
    v_g = t7595[2].clone();
    v_k = t7595[3].clone();
    v_FN = (if ({ let t7596 = v_lat.clone(); Py::Bool({ let t7597 = Py::Int(0i64); if py::ge(&t7596, &t7597)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(10000000.0) });
    return Ok(py::dict_from(&f__out(Py::dict(vec![(Py::str("easting"), py::add(&Py::Float(500000.0), &py::mul(&Py::Float(0.9996), &v_Y.clone())?)?), (Py::str("northing"), py::add(&v_FN.clone(), &py::mul(&Py::Float(0.9996), &v_X.clone())?)?), (Py::str("k"), py::mul(&Py::Float(0.9996), &v_k.clone())?), (Py::str("gamma"), py::truediv(&v_g.clone(), &c_DEG()?)?)]))?, vec![(Py::str("zone"), py::getitem(&v_z.clone(), &Py::str("zone"))?), (Py::str("hemisphere"), py::getitem(&v_z.clone(), &Py::str("hemisphere"))?)])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_utm_inverse(mut v_easting: Py, mut v_northing: Py, mut v_zone: Py, mut v_hemisphere: Py, mut v_ellipsoid: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_FN: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_phi: Py = Py::None;
    v_E = f__ellipsoid(v_ellipsoid.clone())?;
    let _ = f__fin(Py::str("easting and northing"), Py::tuple(vec![v_easting.clone(), v_northing.clone()]))?;
    let _ = f__need({ let t7660 = Py::Bool(matches!(v_zone.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t7660.truthy() { t7660 } else { { let t7657 = Py::Int(1i64); Py::Bool({ let t7658 = v_zone.clone(); if py::le(&t7657, &t7658)? { { let t7659 = Py::Int(60i64); if py::le(&t7658, &t7659)? { true } else { false } } } else { false } }) } } }, Py::str("the zone must be a whole number from 1 to 60"))?;
    let _ = f__need({ let t7661 = v_hemisphere.clone(); Py::Bool({ let t7662 = Py::tuple(vec![Py::str("N"), Py::str("S")]); if py::contains(&t7662, &t7661)? { true } else { false } }) }, Py::str("the hemisphere must be N or S"))?;
    let _ = f__need({ let t7663 = Py::Float(100000.0); Py::Bool({ let t7664 = v_easting.clone(); if py::le(&t7663, &t7664)? { { let t7665 = Py::Float(900000.0); if py::le(&t7664, &t7665)? { true } else { false } } } else { false } }) }, Py::str("the easting must lie between 100 km and 900 km"))?;
    let _ = f__need({ let t7666 = Py::Int(0i64); Py::Bool({ let t7667 = v_northing.clone(); if py::le(&t7666, &t7667)? { { let t7668 = Py::Float(10000000.0); if py::le(&t7667, &t7668)? { true } else { false } } } else { false } }) }, Py::str("the northing must lie between 0 and 10000 km"))?;
    v_FN = (if ({ let t7669 = v_hemisphere.clone(); Py::Bool({ let t7670 = Py::str("N"); if py::eq(&t7669, &t7670) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(10000000.0) });
    let t7680 = (f__tm_inv(v_E.clone(), py::truediv(&py::sub(&v_northing.clone(), &v_FN.clone())?, &Py::Float(0.9996))?, py::truediv(&py::sub(&v_easting.clone(), &Py::Float(500000.0))?, &Py::Float(0.9996))?)?).items()?;
    if t7680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t7680[0].clone();
    v_lam = t7680[1].clone();
    let _ = f__need({ let t7681 = py::abs(&py::truediv(&v_phi.clone(), &c_DEG()?)?)?; Py::Bool({ let t7682 = Py::Int(85i64); if py::le(&t7681, &t7682)? { true } else { false } }) }, Py::str("the coordinates lie outside the UTM latitude range"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("lat"), py::truediv(&v_phi.clone(), &c_DEG()?)?), (Py::str("lon"), f__nlon(py::add(&py::sub(&py::mul(&Py::Int(6i64), &v_zone.clone())?, &Py::Int(183i64))?, &py::truediv(&v_lam.clone(), &c_DEG()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_utm_zone(mut v_lat: Py, mut v_lon: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__lat(Py::str("lat"), v_lat.clone(), Py::Bool(true))?;
    let _ = f__lon(Py::str("lon"), v_lon.clone())?;
    let _ = f__need({ let t7752 = py::neg(&Py::Int(80i64))?; Py::Bool({ let t7753 = v_lat.clone(); if py::le(&t7752, &t7753)? { { let t7754 = Py::Int(84i64); if py::le(&t7753, &t7754)? { true } else { false } } } else { false } }) }, Py::str("UTM covers latitudes from 80 S to 84 N (polar regions use UPS)"))?;
    v_z = f__utm_zone(v_lat.clone(), v_lon.clone())?;
    return Ok(Py::dict(vec![(Py::str("zone"), v_z.clone()), (Py::str("band"), py::getitem(&c__BANDS()?, &py::int(&py::floordiv(&py::add(&v_lat.clone(), &Py::Int(80i64))?, &Py::Int(8i64))?)?)?), (Py::str("hemisphere"), (if ({ let t7782 = v_lat.clone(); Py::Bool({ let t7783 = Py::Int(0i64); if py::ge(&t7782, &t7783)? { true } else { false } }) }).truthy() { Py::str("N") } else { Py::str("S") })), (Py::str("central_meridian"), py::float(&py::sub(&py::mul(&Py::Int(6i64), &v_z.clone())?, &Py::Int(183i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_variogram_fit(mut v_h: Py, mut v_gamma: Py, mut v_model: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_h.clone())?;
    let _ = f__need({ let t7829 = { let t7825 = v_n.clone(); Py::Bool({ let t7826 = py::len(&v_gamma.clone())?; if py::eq(&t7825, &t7826) { true } else { false } }) }; if !t7829.truthy() { t7829 } else { { let t7827 = v_n.clone(); Py::Bool({ let t7828 = Py::Int(4i64); if py::ge(&t7827, &t7828)? { true } else { false } }) } } }, Py::str("give one semivariance per lag (at least 4)"))?;
    let _ = f__need({ let t7830 = v_model.clone(); Py::Bool({ let t7831 = Py::tuple(vec![Py::str("exponential"), Py::str("gaussian")]); if py::contains(&t7831, &t7830)? { true } else { false } }) }, Py::str("the model must be exponential or gaussian"))?;
    for t7832 in py::iter(&(v_h.clone()))? {
        v_v = t7832;
        let _ = f__pos(Py::str("each lag"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t7839 in py::iter(&(v_gamma.clone()))? {
        v_v = t7839;
        let _ = f__fin(Py::str("each semivariance"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_r = f__vfit(v_h.clone(), v_gamma.clone(), v_model.clone(), Py::Bool(false))?;
    if ({ let t7930 = py::getitem(&v_r.clone(), &Py::str("nugget"))?; Py::Bool({ let t7931 = Py::Int(0i64); if py::lt(&t7930, &t7931)? { true } else { false } }) }).truthy() {
        v_r = f__vfit(v_h.clone(), v_gamma.clone(), v_model.clone(), Py::Bool(true))?;
    }
    let _ = f__need({ let t7932 = py::getitem(&v_r.clone(), &Py::str("psill"))?; Py::Bool({ let t7933 = Py::Int(0i64); if py::gt(&t7932, &t7933)? { true } else { false } }) }, Py::str("the fit gives no positive partial sill (the semivariances do not rise with the lag)"))?;
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vegetation_indices(mut v_red: Py, mut v_nir: Py, mut v_blue: Py, mut v_L: Py) -> R {
    let mut v_evi_den: Py = Py::None;
    let mut v_msavi_disc: Py = Py::None;
    let _ = f__refl(Py::str("each band"), Py::tuple(vec![v_red.clone(), v_nir.clone(), v_blue.clone()]))?;
    let _ = f__fin(Py::str("L"), Py::tuple(vec![v_L.clone()]))?;
    let _ = f__need({ let t7982 = Py::Int(0i64); Py::Bool({ let t7983 = v_L.clone(); if py::le(&t7982, &t7983)? { { let t7984 = Py::Int(1i64); if py::le(&t7983, &t7984)? { true } else { false } } } else { false } }) }, Py::str("the soil adjustment factor L must lie between 0 and 1"))?;
    v_evi_den = py::add(&py::sub(&py::add(&v_nir.clone(), &py::mul(&Py::Int(6i64), &v_red.clone())?)?, &py::mul(&Py::Float(7.5), &v_blue.clone())?)?, &Py::Int(1i64))?;
    let _ = f__need({ let t7985 = v_evi_den.clone(); Py::Bool({ let t7986 = Py::Int(0i64); if py::gt(&t7985, &t7986)? { true } else { false } }) }, Py::str("the EVI denominator must be positive"))?;
    let _ = f__need({ let t7991 = { let t7987 = py::add(&py::add(&v_nir.clone(), &v_red.clone())?, &v_L.clone())?; Py::Bool({ let t7988 = Py::Int(0i64); if py::gt(&t7987, &t7988)? { true } else { false } }) }; if !t7991.truthy() { t7991 } else { { let t7989 = py::add(&py::add(&v_nir.clone(), &v_red.clone())?, &Py::Float(0.16))?; Py::Bool({ let t7990 = Py::Int(0i64); if py::gt(&t7989, &t7990)? { true } else { false } }) } } }, Py::str("the band sums must be positive"))?;
    v_msavi_disc = py::sub(&py::pow(&py::add(&py::mul(&Py::Int(2i64), &v_nir.clone())?, &Py::Int(1i64))?, &Py::Int(2i64))?, &py::mul(&Py::Int(8i64), &py::sub(&v_nir.clone(), &v_red.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("NDVI"), f__nd(v_nir.clone(), v_red.clone())?), (Py::str("EVI"), py::truediv(&py::mul(&Py::Float(2.5), &py::sub(&v_nir.clone(), &v_red.clone())?)?, &v_evi_den.clone())?), (Py::str("SAVI"), py::truediv(&py::mul(&py::add(&Py::Int(1i64), &v_L.clone())?, &py::sub(&v_nir.clone(), &v_red.clone())?)?, &py::add(&py::add(&v_nir.clone(), &v_red.clone())?, &v_L.clone())?)?), (Py::str("OSAVI"), py::truediv(&py::sub(&v_nir.clone(), &v_red.clone())?, &py::add(&py::add(&v_nir.clone(), &v_red.clone())?, &Py::Float(0.16))?)?), (Py::str("MSAVI"), py::mul(&Py::Float(0.5), &py::sub(&py::add(&py::mul(&Py::Int(2i64), &v_nir.clone())?, &Py::Int(1i64))?, &py::math1("sqrt", &v_msavi_disc.clone())?)?)?), (Py::str("EVI2"), py::truediv(&py::mul(&Py::Float(2.5), &py::sub(&v_nir.clone(), &v_red.clone())?)?, &py::add(&py::add(&v_nir.clone(), &py::mul(&Py::Float(2.4), &v_red.clone())?)?, &Py::Int(1i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_water_indices(mut v_green: Py, mut v_nir: Py, mut v_swir1: Py, mut v_swir2: Py) -> R {
    let _ = f__refl(Py::str("each band"), Py::tuple(vec![v_green.clone(), v_nir.clone(), v_swir1.clone(), v_swir2.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("NDWI"), f__nd(v_green.clone(), v_nir.clone())?), (Py::str("MNDWI"), f__nd(v_green.clone(), v_swir1.clone())?), (Py::str("NDMI"), f__nd(v_nir.clone(), v_swir1.clone())?), (Py::str("NBR"), f__nd(v_nir.clone(), v_swir2.clone())?), (Py::str("NDBI"), f__nd(v_swir1.clone(), v_nir.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_watershed(mut v_dem: Py, mut v_cell_size: Py, mut v_row: Py, mut v_col: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_dirs: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mask: Py = Py::None;
    let mut v_rec: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_up: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_dirs = f__d8(v_dem.clone(), v_cell_size.clone())?;
    let t8150 = (Py::tuple(vec![py::len(&v_dem.clone())?, py::len(&py::getitem(&v_dem.clone(), &Py::Int(0i64))?)?])).items()?;
    if t8150.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t8150[0].clone();
    v_C = t8150[1].clone();
    let _ = f__need({ let t8159 = Py::Bool(matches!(v_row.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t8159.truthy() { t8159 } else { { let t8158 = Py::Bool(matches!(v_col.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t8158.truthy() { t8158 } else { { let t8157 = { let t8151 = Py::Int(0i64); Py::Bool({ let t8152 = v_row.clone(); if py::le(&t8151, &t8152)? { { let t8153 = v_R.clone(); if py::lt(&t8152, &t8153)? { true } else { false } } } else { false } }) }; if !t8157.truthy() { t8157 } else { { let t8154 = Py::Int(0i64); Py::Bool({ let t8155 = v_col.clone(); if py::le(&t8154, &t8155)? { { let t8156 = v_C.clone(); if py::lt(&t8155, &t8156)? { true } else { false } } } else { false } }) } } } } } } }, Py::str("the outlet must be a cell of the raster (row and column from 0)"))?;
    v_rec = f__receivers(v_dirs.clone())?;
    v_up = Py::dict(vec![]);
    for t8166 in py::iter(&(py::method(&v_rec.clone(), "items", &[])?))? {
        let t8167 = (t8166).items()?;
        if t8167.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8167[0].clone();
        v_v = t8167[1].clone();
        let _ = py::method(&py::method(&v_up.clone(), "setdefault", &[v_v.clone(), Py::list(vec![])])?, "append", &[v_k.clone()])?;
    }
    let t8168 = (Py::tuple(vec![py::set(&Py::list(vec![Py::tuple(vec![v_row.clone(), v_col.clone()])]))?, Py::list(vec![Py::tuple(vec![v_row.clone(), v_col.clone()])])])).items()?;
    if t8168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_seen = t8168[0].clone();
    v_stack = t8168[1].clone();
    while (v_stack.clone()).truthy() {
        v_k = py::method(&v_stack.clone(), "pop", &[])?;
        for t8169 in py::iter(&(py::method(&v_up.clone(), "get", &[v_k.clone(), Py::list(vec![])])?))? {
            v_u = t8169;
            if ({ let t8170 = v_u.clone(); Py::Bool({ let t8171 = v_seen.clone(); if !py::contains(&t8171, &t8170)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_seen.clone(), "add", &[v_u.clone()])?;
                let _ = py::method(&v_stack.clone(), "append", &[v_u.clone()])?;
            }
        }
    }
    v_mask = { let mut out = vec![]; for t8172 in py::range_iter(&[(v_R.clone()).i()?]) { let mut v_i: Py; 
v_i = t8172;
out.push({ let mut out = vec![]; for t8173 in py::range_iter(&[(v_C.clone()).i()?]) { let mut v_j: Py; 
v_j = t8173;
out.push((if ({ let t8174 = Py::tuple(vec![v_i.clone(), v_j.clone()]); Py::Bool({ let t8175 = v_seen.clone(); if py::contains(&t8175, &t8174)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("cells"), py::float(&py::len(&v_seen.clone())?)?), (Py::str("area"), f__r(py::mul(&py::mul(&py::len(&v_seen.clone())?, &v_cell_size.clone())?, &v_cell_size.clone())?)?), (Py::str("mask"), v_mask.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_web_map_tile(mut v_lat: Py, mut v_lon: Py, mut v_zoom: Py) -> R {
    let mut v_Z: Py = Py::None;
    let mut v_qk: Py = Py::None;
    let mut v_tx: Py = Py::None;
    let mut v_ty: Py = Py::None;
    let mut v_xf: Py = Py::None;
    let mut v_yf: Py = Py::None;
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Float(85.0511))?;
    let _ = f__need({ let t8244 = Py::Bool(matches!(v_zoom.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t8244.truthy() { t8244 } else { { let t8241 = Py::Int(0i64); Py::Bool({ let t8242 = v_zoom.clone(); if py::le(&t8241, &t8242)? { { let t8243 = Py::Int(24i64); if py::le(&t8242, &t8243)? { true } else { false } } } else { false } }) } } }, Py::str("the zoom must be a whole number from 0 to 24"))?;
    v_Z = py::pow(&Py::Int(2i64), &v_zoom.clone())?;
    v_xf = py::truediv(&py::add(&f__nlon(v_lon.clone())?, &Py::Float(180.0))?, &Py::Float(360.0))?;
    v_yf = py::sub(&Py::Float(0.5), &py::truediv(&py::math1("asinh", &py::math1("tan", &py::mul(&v_lat.clone(), &c_DEG()?)?)?)?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?;
    v_tx = py::minmax(&[py::sub(&v_Z.clone(), &Py::Int(1i64))?, py::minmax(&[Py::Int(0i64), py::int(&py::math1("floor", &py::mul(&v_xf.clone(), &v_Z.clone())?)?)?], true)?], false)?;
    v_ty = py::minmax(&[py::sub(&v_Z.clone(), &Py::Int(1i64))?, py::minmax(&[Py::Int(0i64), py::int(&py::math1("floor", &py::mul(&v_yf.clone(), &v_Z.clone())?)?)?], true)?], false)?;
    v_qk = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t8249 in py::range_iter(&[(v_zoom.clone()).i()?, (Py::Int(0i64)).i()?, (py::neg(&Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t8249;
out.push(Py::str(&py::to_str(&py::add(&(if (py::bitand(&v_tx.clone(), &py::lshift(&Py::Int(1i64), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }), &(if (py::bitand(&v_ty.clone(), &py::lshift(&Py::Int(1i64), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?).truthy() { Py::Int(2i64) } else { Py::Int(0i64) }))?))); } Py::list(out) }])?;
    return Ok(Py::dict(vec![(Py::str("tile_x"), v_tx.clone()), (Py::str("tile_y"), v_ty.clone()), (Py::str("quadkey"), v_qk.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_web_mercator_forward(mut v_lat: Py, mut v_lon: Py) -> R {
    let _ = f__pt(v_lat.clone(), v_lon.clone(), Py::Float(85.06))?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), py::mul(&py::mul(&c_WEB_MERCATOR_A()?, &f__nlon(v_lon.clone())?)?, &c_DEG()?)?), (Py::str("y"), py::mul(&c_WEB_MERCATOR_A()?, &py::math1("asinh", &py::math1("tan", &py::mul(&v_lat.clone(), &c_DEG()?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_web_mercator_inverse(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_lim: Py = Py::None;
    let _ = f__fin(Py::str("x and y"), Py::tuple(vec![v_x.clone(), v_y.clone()]))?;
    v_lim = py::mul(&c_WEB_MERCATOR_A()?, &Py::Float(std::f64::consts::PI))?;
    let _ = f__need({ let t8369 = { let t8365 = py::abs(&v_x.clone())?; Py::Bool({ let t8366 = py::mul(&v_lim.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t8365, &t8366)? { true } else { false } }) }; if !t8369.truthy() { t8369 } else { { let t8367 = py::abs(&v_y.clone())?; Py::Bool({ let t8368 = py::mul(&v_lim.clone(), &Py::Float(1.0001))?; if py::le(&t8367, &t8368)? { true } else { false } }) } } }, Py::str("x and y must lie within the web-map square (about 20037.5 km)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("lat"), py::truediv(&py::math1("atan", &py::math1("sinh", &py::truediv(&v_y.clone(), &c_WEB_MERCATOR_A()?)?)?)?, &c_DEG()?)?), (Py::str("lon"), py::truediv(&py::truediv(&v_x.clone(), &c_WEB_MERCATOR_A()?)?, &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weighted_mean_center(mut v_x: Py, mut v_y: Py, mut v_w: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t8428 = { let t8423 = v_n.clone(); Py::Bool({ let t8424 = py::len(&v_y.clone())?; if py::eq(&t8423, &t8424) { { let t8425 = py::len(&v_w.clone())?; if py::eq(&t8424, &t8425) { true } else { false } } } else { false } }) }; if !t8428.truthy() { t8428 } else { { let t8426 = v_n.clone(); Py::Bool({ let t8427 = Py::Int(1i64); if py::ge(&t8426, &t8427)? { true } else { false } }) } } }, Py::str("give one y and one weight per x"))?;
    for t8429 in py::iter(&(v_w.clone()))? {
        v_v = t8429;
        let _ = f__nonneg(Py::str("each weight"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_W = py::fsum(&v_w.clone())?;
    let _ = f__need({ let t8436 = v_W.clone(); Py::Bool({ let t8437 = Py::Int(0i64); if py::gt(&t8436, &t8437)? { true } else { false } }) }, Py::str("the weights must not all be zero"))?;
    let t8442 = (Py::tuple(vec![py::truediv(&py::fsum(&{ let mut out = vec![]; for t8438 in py::iter(&(py::zip(&[v_w.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t8439 = (t8438).items()?;
if t8439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t8439[0].clone();
v_b = t8439[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_W.clone())?, py::truediv(&py::fsum(&{ let mut out = vec![]; for t8440 in py::iter(&(py::zip(&[v_w.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t8441 = (t8440).items()?;
if t8441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t8441[0].clone();
v_b = t8441[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_W.clone())?])).items()?;
    if t8442.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t8442[0].clone();
    v_my = t8442[1].clone();
    v_sd = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t8443 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t8444 = (t8443).items()?;
if t8444.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t8444[0].clone();
v_b = t8444[1].clone();
v_c = t8444[2].clone();
out.push(py::mul(&v_c.clone(), &py::add(&py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?)?)?); } Py::list(out) })?, &v_W.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("wmean_x"), v_mx.clone()), (Py::str("wmean_y"), v_my.clone()), (Py::str("weighted_std_distance"), v_sd.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1074(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1075 = v_key.clone(); Py::Bool({ let t1076 = v_memo.clone(); if !py::contains(&t1076, &t1075)? { true } else { false } }) }).truthy() {
        let t1078: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1077 = v_inputs.clone(); f_density_gradient(py::getitem(&t1077, &Py::str("r"))?, py::getitem(&t1077, &Py::str("D"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1078 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1079 = py::len(&v_memo.clone())?; Py::Bool({ let t1080 = Py::Int(64i64); if py::gt(&t1079, &t1080)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1081 = v_want.clone(); Py::Bool({ let t1082 = Py::None; if py::is(&t1081, &t1082) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1083 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1114 = (t1083).items()?;
        if t1114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1114[0].clone();
        v_v = t1114[1].clone();
        if ({ let t1084 = v_k.clone(); Py::Bool({ let t1085 = v_outputs.clone(); if !py::contains(&t1085, &t1084)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1199(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1200 = v_key.clone(); Py::Bool({ let t1201 = v_memo.clone(); if !py::contains(&t1201, &t1200)? { true } else { false } }) }).truthy() {
        let t1203: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1202 = v_inputs.clone(); f_detour_index(py::getitem(&t1202, &Py::str("coords"))?, py::getitem(&t1202, &Py::str("edges"))?, py::getitem(&t1202, &Py::str("origin"))?, py::getitem(&t1202, &Py::str("destination"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1203 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1204 = py::len(&v_memo.clone())?; Py::Bool({ let t1205 = Py::Int(64i64); if py::gt(&t1204, &t1205)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1206 = v_want.clone(); Py::Bool({ let t1207 = Py::None; if py::is(&t1206, &t1207) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1208 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1239 = (t1208).items()?;
        if t1239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1239[0].clone();
        v_v = t1239[1].clone();
        if ({ let t1209 = v_k.clone(); Py::Bool({ let t1210 = v_outputs.clone(); if !py::contains(&t1210, &t1209)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1263(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1264 = v_key.clone(); Py::Bool({ let t1265 = v_memo.clone(); if !py::contains(&t1265, &t1264)? { true } else { false } }) }).truthy() {
        let t1267: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1266 = v_inputs.clone(); f_frechet_distance(py::getitem(&t1266, &Py::str("x1"))?, py::getitem(&t1266, &Py::str("y1"))?, py::getitem(&t1266, &Py::str("x2"))?, py::getitem(&t1266, &Py::str("y2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1267 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1268 = py::len(&v_memo.clone())?; Py::Bool({ let t1269 = Py::Int(64i64); if py::gt(&t1268, &t1269)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1270 = v_want.clone(); Py::Bool({ let t1271 = Py::None; if py::is(&t1270, &t1271) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1272 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1303 = (t1272).items()?;
        if t1303.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1303[0].clone();
        v_v = t1303[1].clone();
        if ({ let t1273 = v_k.clone(); Py::Bool({ let t1274 = v_outputs.clone(); if !py::contains(&t1274, &t1273)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1325(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1326 = v_key.clone(); Py::Bool({ let t1327 = v_memo.clone(); if !py::contains(&t1327, &t1326)? { true } else { false } }) }).truthy() {
        let t1329: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1328 = v_inputs.clone(); f_distance_band_weights(py::getitem(&t1328, &Py::str("x"))?, py::getitem(&t1328, &Py::str("y"))?, py::getitem(&t1328, &Py::str("threshold"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1329 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1330 = py::len(&v_memo.clone())?; Py::Bool({ let t1331 = Py::Int(64i64); if py::gt(&t1330, &t1331)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1332 = v_want.clone(); Py::Bool({ let t1333 = Py::None; if py::is(&t1332, &t1333) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1334 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1365 = (t1334).items()?;
        if t1365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1365[0].clone();
        v_v = t1365[1].clone();
        if ({ let t1335 = v_k.clone(); Py::Bool({ let t1336 = v_outputs.clone(); if !py::contains(&t1336, &t1335)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_137(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t138 = v_key.clone(); Py::Bool({ let t139 = v_memo.clone(); if !py::contains(&t139, &t138)? { true } else { false } }) }).truthy() {
        let t141: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t140 = v_inputs.clone(); f_albers_inverse(py::getitem(&t140, &Py::str("x"))?, py::getitem(&t140, &Py::str("y"))?, py::getitem(&t140, &Py::str("lat1"))?, py::getitem(&t140, &Py::str("lat2"))?, py::getitem(&t140, &Py::str("lat0"))?, py::getitem(&t140, &Py::str("lon0"))?, py::getitem(&t140, &Py::str("ellipsoid"))?, py::method(&t140, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t140, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t141 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t142 = py::len(&v_memo.clone())?; Py::Bool({ let t143 = Py::Int(64i64); if py::gt(&t142, &t143)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t144 = v_want.clone(); Py::Bool({ let t145 = Py::None; if py::is(&t144, &t145) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t146 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t177 = (t146).items()?;
        if t177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t177[0].clone();
        v_v = t177[1].clone();
        if ({ let t147 = v_k.clone(); Py::Bool({ let t148 = v_outputs.clone(); if !py::contains(&t148, &t147)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1397(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1398 = v_key.clone(); Py::Bool({ let t1399 = v_memo.clone(); if !py::contains(&t1399, &t1398)? { true } else { false } }) }).truthy() {
        let t1401: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1400 = v_inputs.clone(); f_dms_to_decimal(py::getitem(&t1400, &Py::str("degrees"))?, py::getitem(&t1400, &Py::str("minutes"))?, py::getitem(&t1400, &Py::str("seconds"))?, py::getitem(&t1400, &Py::str("hemisphere"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1401 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1402 = py::len(&v_memo.clone())?; Py::Bool({ let t1403 = Py::Int(64i64); if py::gt(&t1402, &t1403)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1404 = v_want.clone(); Py::Bool({ let t1405 = Py::None; if py::is(&t1404, &t1405) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1406 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1437 = (t1406).items()?;
        if t1437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1437[0].clone();
        v_v = t1437[1].clone();
        if ({ let t1407 = v_k.clone(); Py::Bool({ let t1408 = v_outputs.clone(); if !py::contains(&t1408, &t1407)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1466(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1467 = v_key.clone(); Py::Bool({ let t1468 = v_memo.clone(); if !py::contains(&t1468, &t1467)? { true } else { false } }) }).truthy() {
        let t1470: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1469 = v_inputs.clone(); f_douglas_peucker(py::getitem(&t1469, &Py::str("x"))?, py::getitem(&t1469, &Py::str("y"))?, py::getitem(&t1469, &Py::str("tolerance"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1470 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1471 = py::len(&v_memo.clone())?; Py::Bool({ let t1472 = Py::Int(64i64); if py::gt(&t1471, &t1472)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1473 = v_want.clone(); Py::Bool({ let t1474 = Py::None; if py::is(&t1473, &t1474) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1475 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1506 = (t1475).items()?;
        if t1506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1506[0].clone();
        v_v = t1506[1].clone();
        if ({ let t1476 = v_k.clone(); Py::Bool({ let t1477 = v_outputs.clone(); if !py::contains(&t1477, &t1476)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1534(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1535 = v_key.clone(); Py::Bool({ let t1536 = v_memo.clone(); if !py::contains(&t1536, &t1535)? { true } else { false } }) }).truthy() {
        let t1538: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1537 = v_inputs.clone(); f_ecef_to_geodetic(py::getitem(&t1537, &Py::str("X"))?, py::getitem(&t1537, &Py::str("Y"))?, py::getitem(&t1537, &Py::str("Z"))?, py::getitem(&t1537, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1538 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1539 = py::len(&v_memo.clone())?; Py::Bool({ let t1540 = Py::Int(64i64); if py::gt(&t1539, &t1540)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1541 = v_want.clone(); Py::Bool({ let t1542 = Py::None; if py::is(&t1541, &t1542) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1543 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1574 = (t1543).items()?;
        if t1574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1574[0].clone();
        v_v = t1574[1].clone();
        if ({ let t1544 = v_k.clone(); Py::Bool({ let t1545 = v_outputs.clone(); if !py::contains(&t1545, &t1544)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1617(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1618 = v_key.clone(); Py::Bool({ let t1619 = v_memo.clone(); if !py::contains(&t1619, &t1618)? { true } else { false } }) }).truthy() {
        let t1621: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1620 = v_inputs.clone(); f_empirical_semivariogram(py::getitem(&t1620, &Py::str("x"))?, py::getitem(&t1620, &Py::str("y"))?, py::getitem(&t1620, &Py::str("z"))?, py::getitem(&t1620, &Py::str("bin_edges"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1621 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1622 = py::len(&v_memo.clone())?; Py::Bool({ let t1623 = Py::Int(64i64); if py::gt(&t1622, &t1623)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1624 = v_want.clone(); Py::Bool({ let t1625 = Py::None; if py::is(&t1624, &t1625) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1626 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1657 = (t1626).items()?;
        if t1657.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1657[0].clone();
        v_v = t1657[1].clone();
        if ({ let t1627 = v_k.clone(); Py::Bool({ let t1628 = v_outputs.clone(); if !py::contains(&t1628, &t1627)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1707(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1708 = v_key.clone(); Py::Bool({ let t1709 = v_memo.clone(); if !py::contains(&t1709, &t1708)? { true } else { false } }) }).truthy() {
        let t1711: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1710 = v_inputs.clone(); f_enu_to_geodetic(py::getitem(&t1710, &Py::str("lat0"))?, py::getitem(&t1710, &Py::str("lon0"))?, py::getitem(&t1710, &Py::str("h0"))?, py::getitem(&t1710, &Py::str("e"))?, py::getitem(&t1710, &Py::str("n"))?, py::getitem(&t1710, &Py::str("u"))?, py::getitem(&t1710, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1711 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1712 = py::len(&v_memo.clone())?; Py::Bool({ let t1713 = Py::Int(64i64); if py::gt(&t1712, &t1713)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1714 = v_want.clone(); Py::Bool({ let t1715 = Py::None; if py::is(&t1714, &t1715) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1716 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1747 = (t1716).items()?;
        if t1747.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1747[0].clone();
        v_v = t1747[1].clone();
        if ({ let t1717 = v_k.clone(); Py::Bool({ let t1718 = v_outputs.clone(); if !py::contains(&t1718, &t1717)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1784(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1785 = v_key.clone(); Py::Bool({ let t1786 = v_memo.clone(); if !py::contains(&t1786, &t1785)? { true } else { false } }) }).truthy() {
        let t1788: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1787 = v_inputs.clone(); f_equirectangular_forward(py::getitem(&t1787, &Py::str("lat"))?, py::getitem(&t1787, &Py::str("lon"))?, py::getitem(&t1787, &Py::str("lat_ts"))?, py::getitem(&t1787, &Py::str("lat0"))?, py::getitem(&t1787, &Py::str("lon0"))?, py::getitem(&t1787, &Py::str("R"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1788 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1789 = py::len(&v_memo.clone())?; Py::Bool({ let t1790 = Py::Int(64i64); if py::gt(&t1789, &t1790)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1791 = v_want.clone(); Py::Bool({ let t1792 = Py::None; if py::is(&t1791, &t1792) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1793 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1824 = (t1793).items()?;
        if t1824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1824[0].clone();
        v_v = t1824[1].clone();
        if ({ let t1794 = v_k.clone(); Py::Bool({ let t1795 = v_outputs.clone(); if !py::contains(&t1795, &t1794)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1885(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1886 = v_key.clone(); Py::Bool({ let t1887 = v_memo.clone(); if !py::contains(&t1887, &t1886)? { true } else { false } }) }).truthy() {
        let t1889: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1888 = v_inputs.clone(); f_gearys_c(py::getitem(&t1888, &Py::str("y"))?, py::getitem(&t1888, &Py::str("W"))?, py::getitem(&t1888, &Py::str("transform"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1889 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1890 = py::len(&v_memo.clone())?; Py::Bool({ let t1891 = Py::Int(64i64); if py::gt(&t1890, &t1891)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1892 = v_want.clone(); Py::Bool({ let t1893 = Py::None; if py::is(&t1892, &t1893) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1894 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1925 = (t1894).items()?;
        if t1925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1925[0].clone();
        v_v = t1925[1].clone();
        if ({ let t1895 = v_k.clone(); Py::Bool({ let t1896 = v_outputs.clone(); if !py::contains(&t1896, &t1895)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1986 = v_key.clone(); Py::Bool({ let t1987 = v_memo.clone(); if !py::contains(&t1987, &t1986)? { true } else { false } }) }).truthy() {
        let t1989: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1988 = v_inputs.clone(); f_geodesic_destination(py::getitem(&t1988, &Py::str("lat1"))?, py::getitem(&t1988, &Py::str("lon1"))?, py::getitem(&t1988, &Py::str("azimuth"))?, py::getitem(&t1988, &Py::str("distance"))?, py::getitem(&t1988, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1989 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1990 = py::len(&v_memo.clone())?; Py::Bool({ let t1991 = Py::Int(64i64); if py::gt(&t1990, &t1991)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1992 = v_want.clone(); Py::Bool({ let t1993 = Py::None; if py::is(&t1992, &t1993) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1994 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2025 = (t1994).items()?;
        if t2025.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2025[0].clone();
        v_v = t2025[1].clone();
        if ({ let t1995 = v_k.clone(); Py::Bool({ let t1996 = v_outputs.clone(); if !py::contains(&t1996, &t1995)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_203(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t204 = v_key.clone(); Py::Bool({ let t205 = v_memo.clone(); if !py::contains(&t205, &t204)? { true } else { false } }) }).truthy() {
        let t207: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t206 = v_inputs.clone(); f_auxiliary_latitudes(py::getitem(&t206, &Py::str("lat"))?, py::getitem(&t206, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t207 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t208 = py::len(&v_memo.clone())?; Py::Bool({ let t209 = Py::Int(64i64); if py::gt(&t208, &t209)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t210 = v_want.clone(); Py::Bool({ let t211 = Py::None; if py::is(&t210, &t211) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t212 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t243 = (t212).items()?;
        if t243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t243[0].clone();
        v_v = t243[1].clone();
        if ({ let t213 = v_k.clone(); Py::Bool({ let t214 = v_outputs.clone(); if !py::contains(&t214, &t213)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2094(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2095 = v_key.clone(); Py::Bool({ let t2096 = v_memo.clone(); if !py::contains(&t2096, &t2095)? { true } else { false } }) }).truthy() {
        let t2098: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2097 = v_inputs.clone(); f_geodesic_midpoint(py::getitem(&t2097, &Py::str("lat1"))?, py::getitem(&t2097, &Py::str("lon1"))?, py::getitem(&t2097, &Py::str("lat2"))?, py::getitem(&t2097, &Py::str("lon2"))?, py::getitem(&t2097, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2098 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2099 = py::len(&v_memo.clone())?; Py::Bool({ let t2100 = Py::Int(64i64); if py::gt(&t2099, &t2100)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2101 = v_want.clone(); Py::Bool({ let t2102 = Py::None; if py::is(&t2101, &t2102) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2103 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2134 = (t2103).items()?;
        if t2134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2134[0].clone();
        v_v = t2134[1].clone();
        if ({ let t2104 = v_k.clone(); Py::Bool({ let t2105 = v_outputs.clone(); if !py::contains(&t2105, &t2104)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2211(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2212 = v_key.clone(); Py::Bool({ let t2213 = v_memo.clone(); if !py::contains(&t2213, &t2212)? { true } else { false } }) }).truthy() {
        let t2215: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2214 = v_inputs.clone(); f_geodesic_polygon_area(py::getitem(&t2214, &Py::str("lats"))?, py::getitem(&t2214, &Py::str("lons"))?, py::getitem(&t2214, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2215 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2216 = py::len(&v_memo.clone())?; Py::Bool({ let t2217 = Py::Int(64i64); if py::gt(&t2216, &t2217)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2218 = v_want.clone(); Py::Bool({ let t2219 = Py::None; if py::is(&t2218, &t2219) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2220 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2251 = (t2220).items()?;
        if t2251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2251[0].clone();
        v_v = t2251[1].clone();
        if ({ let t2221 = v_k.clone(); Py::Bool({ let t2222 = v_outputs.clone(); if !py::contains(&t2222, &t2221)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2285(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2286 = v_key.clone(); Py::Bool({ let t2287 = v_memo.clone(); if !py::contains(&t2287, &t2286)? { true } else { false } }) }).truthy() {
        let t2289: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2288 = v_inputs.clone(); f_geodetic_to_ecef(py::getitem(&t2288, &Py::str("lat"))?, py::getitem(&t2288, &Py::str("lon"))?, py::getitem(&t2288, &Py::str("h"))?, py::getitem(&t2288, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2289 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2290 = py::len(&v_memo.clone())?; Py::Bool({ let t2291 = Py::Int(64i64); if py::gt(&t2290, &t2291)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2292 = v_want.clone(); Py::Bool({ let t2293 = Py::None; if py::is(&t2292, &t2293) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2294 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2325 = (t2294).items()?;
        if t2325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2325[0].clone();
        v_v = t2325[1].clone();
        if ({ let t2295 = v_k.clone(); Py::Bool({ let t2296 = v_outputs.clone(); if !py::contains(&t2296, &t2295)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2367(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2368 = v_key.clone(); Py::Bool({ let t2369 = v_memo.clone(); if !py::contains(&t2369, &t2368)? { true } else { false } }) }).truthy() {
        let t2371: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2370 = v_inputs.clone(); f_geodetic_to_enu(py::getitem(&t2370, &Py::str("lat0"))?, py::getitem(&t2370, &Py::str("lon0"))?, py::getitem(&t2370, &Py::str("h0"))?, py::getitem(&t2370, &Py::str("lat"))?, py::getitem(&t2370, &Py::str("lon"))?, py::getitem(&t2370, &Py::str("h"))?, py::getitem(&t2370, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2371 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2372 = py::len(&v_memo.clone())?; Py::Bool({ let t2373 = Py::Int(64i64); if py::gt(&t2372, &t2373)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2374 = v_want.clone(); Py::Bool({ let t2375 = Py::None; if py::is(&t2374, &t2375) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2376 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2407 = (t2376).items()?;
        if t2407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2407[0].clone();
        v_v = t2407[1].clone();
        if ({ let t2377 = v_k.clone(); Py::Bool({ let t2378 = v_outputs.clone(); if !py::contains(&t2378, &t2377)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2418(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2419 = v_key.clone(); Py::Bool({ let t2420 = v_memo.clone(); if !py::contains(&t2420, &t2419)? { true } else { false } }) }).truthy() {
        let t2422: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2421 = v_inputs.clone(); f_geohash_decode(py::getitem(&t2421, &Py::str("geohash"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2422 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2423 = py::len(&v_memo.clone())?; Py::Bool({ let t2424 = Py::Int(64i64); if py::gt(&t2423, &t2424)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2425 = v_want.clone(); Py::Bool({ let t2426 = Py::None; if py::is(&t2425, &t2426) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2427 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2458 = (t2427).items()?;
        if t2458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2458[0].clone();
        v_v = t2458[1].clone();
        if ({ let t2428 = v_k.clone(); Py::Bool({ let t2429 = v_outputs.clone(); if !py::contains(&t2429, &t2428)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2491(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2492 = v_key.clone(); Py::Bool({ let t2493 = v_memo.clone(); if !py::contains(&t2493, &t2492)? { true } else { false } }) }).truthy() {
        let t2495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2494 = v_inputs.clone(); f_geohash_encode(py::getitem(&t2494, &Py::str("lat"))?, py::getitem(&t2494, &Py::str("lon"))?, py::getitem(&t2494, &Py::str("precision"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2495 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2496 = py::len(&v_memo.clone())?; Py::Bool({ let t2497 = Py::Int(64i64); if py::gt(&t2496, &t2497)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2498 = v_want.clone(); Py::Bool({ let t2499 = Py::None; if py::is(&t2498, &t2499) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2500 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2531 = (t2500).items()?;
        if t2531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2531[0].clone();
        v_v = t2531[1].clone();
        if ({ let t2501 = v_k.clone(); Py::Bool({ let t2502 = v_outputs.clone(); if !py::contains(&t2502, &t2501)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2598(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2599 = v_key.clone(); Py::Bool({ let t2600 = v_memo.clone(); if !py::contains(&t2600, &t2599)? { true } else { false } }) }).truthy() {
        let t2602: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2601 = v_inputs.clone(); f_general_g(py::getitem(&t2601, &Py::str("y"))?, py::getitem(&t2601, &Py::str("W"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2602 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2603 = py::len(&v_memo.clone())?; Py::Bool({ let t2604 = Py::Int(64i64); if py::gt(&t2603, &t2604)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2605 = v_want.clone(); Py::Bool({ let t2606 = Py::None; if py::is(&t2605, &t2606) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2607 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2638 = (t2607).items()?;
        if t2638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2638[0].clone();
        v_v = t2638[1].clone();
        if ({ let t2608 = v_k.clone(); Py::Bool({ let t2609 = v_outputs.clone(); if !py::contains(&t2609, &t2608)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2678(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2679 = v_key.clone(); Py::Bool({ let t2680 = v_memo.clone(); if !py::contains(&t2680, &t2679)? { true } else { false } }) }).truthy() {
        let t2682: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2681 = v_inputs.clone(); f_gnomonic_forward(py::getitem(&t2681, &Py::str("lat"))?, py::getitem(&t2681, &Py::str("lon"))?, py::getitem(&t2681, &Py::str("lat0"))?, py::getitem(&t2681, &Py::str("lon0"))?, py::getitem(&t2681, &Py::str("R"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2682 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2683 = py::len(&v_memo.clone())?; Py::Bool({ let t2684 = Py::Int(64i64); if py::gt(&t2683, &t2684)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2685 = v_want.clone(); Py::Bool({ let t2686 = Py::None; if py::is(&t2685, &t2686) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2687 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2718 = (t2687).items()?;
        if t2718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2718[0].clone();
        v_v = t2718[1].clone();
        if ({ let t2688 = v_k.clone(); Py::Bool({ let t2689 = v_outputs.clone(); if !py::contains(&t2689, &t2688)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2751(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2752 = v_key.clone(); Py::Bool({ let t2753 = v_memo.clone(); if !py::contains(&t2753, &t2752)? { true } else { false } }) }).truthy() {
        let t2755: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2754 = v_inputs.clone(); f_graticule_cell_area(py::getitem(&t2754, &Py::str("lat1"))?, py::getitem(&t2754, &Py::str("lat2"))?, py::getitem(&t2754, &Py::str("lon1"))?, py::getitem(&t2754, &Py::str("lon2"))?, py::getitem(&t2754, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2755 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2756 = py::len(&v_memo.clone())?; Py::Bool({ let t2757 = Py::Int(64i64); if py::gt(&t2756, &t2757)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2758 = v_want.clone(); Py::Bool({ let t2759 = Py::None; if py::is(&t2758, &t2759) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2760 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2791 = (t2760).items()?;
        if t2791.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2791[0].clone();
        v_v = t2791[1].clone();
        if ({ let t2761 = v_k.clone(); Py::Bool({ let t2762 = v_outputs.clone(); if !py::contains(&t2762, &t2761)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2857(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2858 = v_key.clone(); Py::Bool({ let t2859 = v_memo.clone(); if !py::contains(&t2859, &t2858)? { true } else { false } }) }).truthy() {
        let t2861: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2860 = v_inputs.clone(); f_gravity_accessibility(py::getitem(&t2860, &Py::str("edges"))?, py::getitem(&t2860, &Py::str("opportunities"))?, py::getitem(&t2860, &Py::str("beta"))?, py::getitem(&t2860, &Py::str("impedance"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2861 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2862 = py::len(&v_memo.clone())?; Py::Bool({ let t2863 = Py::Int(64i64); if py::gt(&t2862, &t2863)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2864 = v_want.clone(); Py::Bool({ let t2865 = Py::None; if py::is(&t2864, &t2865) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2866 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2897 = (t2866).items()?;
        if t2897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2897[0].clone();
        v_v = t2897[1].clone();
        if ({ let t2867 = v_k.clone(); Py::Bool({ let t2868 = v_outputs.clone(); if !py::contains(&t2868, &t2867)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2974(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2975 = v_key.clone(); Py::Bool({ let t2976 = v_memo.clone(); if !py::contains(&t2976, &t2975)? { true } else { false } }) }).truthy() {
        let t2978: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2977 = v_inputs.clone(); f_gravity_poisson(py::getitem(&t2977, &Py::str("T"))?, py::getitem(&t2977, &Py::str("O"))?, py::getitem(&t2977, &Py::str("D"))?, py::getitem(&t2977, &Py::str("d"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2978 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2979 = py::len(&v_memo.clone())?; Py::Bool({ let t2980 = Py::Int(64i64); if py::gt(&t2979, &t2980)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2981 = v_want.clone(); Py::Bool({ let t2982 = Py::None; if py::is(&t2981, &t2982) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2983 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3014 = (t2983).items()?;
        if t3014.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3014[0].clone();
        v_v = t3014[1].clone();
        if ({ let t2984 = v_k.clone(); Py::Bool({ let t2985 = v_outputs.clone(); if !py::contains(&t2985, &t2984)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3036(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3037 = v_key.clone(); Py::Bool({ let t3038 = v_memo.clone(); if !py::contains(&t3038, &t3037)? { true } else { false } }) }).truthy() {
        let t3040: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3039 = v_inputs.clone(); f_ground_sample_distance(py::getitem(&t3039, &Py::str("pixel_pitch"))?, py::getitem(&t3039, &Py::str("focal_length"))?, py::getitem(&t3039, &Py::str("altitude"))?, py::getitem(&t3039, &Py::str("pixels"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3040 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3041 = py::len(&v_memo.clone())?; Py::Bool({ let t3042 = Py::Int(64i64); if py::gt(&t3041, &t3042)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3043 = v_want.clone(); Py::Bool({ let t3044 = Py::None; if py::is(&t3043, &t3044) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3045 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3076 = (t3045).items()?;
        if t3076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3076[0].clone();
        v_v = t3076[1].clone();
        if ({ let t3046 = v_k.clone(); Py::Bool({ let t3047 = v_outputs.clone(); if !py::contains(&t3047, &t3046)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_306(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t307 = v_key.clone(); Py::Bool({ let t308 = v_memo.clone(); if !py::contains(&t308, &t307)? { true } else { false } }) }).truthy() {
        let t310: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t309 = v_inputs.clone(); f_aeqd_forward(py::getitem(&t309, &Py::str("lat"))?, py::getitem(&t309, &Py::str("lon"))?, py::getitem(&t309, &Py::str("lat0"))?, py::getitem(&t309, &Py::str("lon0"))?, py::getitem(&t309, &Py::str("ellipsoid"))?, py::method(&t309, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t309, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t310 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t311 = py::len(&v_memo.clone())?; Py::Bool({ let t312 = Py::Int(64i64); if py::gt(&t311, &t312)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t313 = v_want.clone(); Py::Bool({ let t314 = Py::None; if py::is(&t313, &t314) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t315 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t346 = (t315).items()?;
        if t346.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t346[0].clone();
        v_v = t346[1].clone();
        if ({ let t316 = v_k.clone(); Py::Bool({ let t317 = v_outputs.clone(); if !py::contains(&t317, &t316)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3115(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3116 = v_key.clone(); Py::Bool({ let t3117 = v_memo.clone(); if !py::contains(&t3117, &t3116)? { true } else { false } }) }).truthy() {
        let t3119: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3118 = v_inputs.clone(); f_hillshade(py::getitem(&t3118, &Py::str("window"))?, py::getitem(&t3118, &Py::str("cell_size"))?, py::getitem(&t3118, &Py::str("sun_azimuth"))?, py::getitem(&t3118, &Py::str("sun_altitude"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3119 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3120 = py::len(&v_memo.clone())?; Py::Bool({ let t3121 = Py::Int(64i64); if py::gt(&t3120, &t3121)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3122 = v_want.clone(); Py::Bool({ let t3123 = Py::None; if py::is(&t3122, &t3123) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3124 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3155 = (t3124).items()?;
        if t3155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3155[0].clone();
        v_v = t3155[1].clone();
        if ({ let t3125 = v_k.clone(); Py::Bool({ let t3126 = v_outputs.clone(); if !py::contains(&t3126, &t3125)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3241(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3242 = v_key.clone(); Py::Bool({ let t3243 = v_memo.clone(); if !py::contains(&t3243, &t3242)? { true } else { false } }) }).truthy() {
        let t3245: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3244 = v_inputs.clone(); f_huff_model(py::getitem(&t3244, &Py::str("edges"))?, py::getitem(&t3244, &Py::str("origin"))?, py::getitem(&t3244, &Py::str("stores"))?, py::getitem(&t3244, &Py::str("attractiveness"))?, py::getitem(&t3244, &Py::str("alpha"))?, py::getitem(&t3244, &Py::str("beta"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3245 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3246 = py::len(&v_memo.clone())?; Py::Bool({ let t3247 = Py::Int(64i64); if py::gt(&t3246, &t3247)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3248 = v_want.clone(); Py::Bool({ let t3249 = Py::None; if py::is(&t3248, &t3249) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3250 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3281 = (t3250).items()?;
        if t3281.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3281[0].clone();
        v_v = t3281[1].clone();
        if ({ let t3251 = v_k.clone(); Py::Bool({ let t3252 = v_outputs.clone(); if !py::contains(&t3252, &t3251)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3320(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3321 = v_key.clone(); Py::Bool({ let t3322 = v_memo.clone(); if !py::contains(&t3322, &t3321)? { true } else { false } }) }).truthy() {
        let t3324: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3323 = v_inputs.clone(); f_idw(py::getitem(&t3323, &Py::str("x"))?, py::getitem(&t3323, &Py::str("y"))?, py::getitem(&t3323, &Py::str("z"))?, py::getitem(&t3323, &Py::str("px"))?, py::getitem(&t3323, &Py::str("py"))?, py::getitem(&t3323, &Py::str("power"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3324 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3325 = py::len(&v_memo.clone())?; Py::Bool({ let t3326 = Py::Int(64i64); if py::gt(&t3325, &t3326)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3327 = v_want.clone(); Py::Bool({ let t3328 = Py::None; if py::is(&t3327, &t3328) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3329 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3360 = (t3329).items()?;
        if t3360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3360[0].clone();
        v_v = t3360[1].clone();
        if ({ let t3330 = v_k.clone(); Py::Bool({ let t3331 = v_outputs.clone(); if !py::contains(&t3331, &t3330)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3408(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3409 = v_key.clone(); Py::Bool({ let t3410 = v_memo.clone(); if !py::contains(&t3410, &t3409)? { true } else { false } }) }).truthy() {
        let t3412: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3411 = v_inputs.clone(); f_join_counts(py::getitem(&t3411, &Py::str("y"))?, py::getitem(&t3411, &Py::str("W"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3412 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3413 = py::len(&v_memo.clone())?; Py::Bool({ let t3414 = Py::Int(64i64); if py::gt(&t3413, &t3414)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3415 = v_want.clone(); Py::Bool({ let t3416 = Py::None; if py::is(&t3415, &t3416) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3417 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3448 = (t3417).items()?;
        if t3448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3448[0].clone();
        v_v = t3448[1].clone();
        if ({ let t3418 = v_k.clone(); Py::Bool({ let t3419 = v_outputs.clone(); if !py::contains(&t3419, &t3418)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3487(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3488 = v_key.clone(); Py::Bool({ let t3489 = v_memo.clone(); if !py::contains(&t3489, &t3488)? { true } else { false } }) }).truthy() {
        let t3491: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3490 = v_inputs.clone(); f_kernel_density(py::getitem(&t3490, &Py::str("x"))?, py::getitem(&t3490, &Py::str("y"))?, py::getitem(&t3490, &Py::str("px"))?, py::getitem(&t3490, &Py::str("py"))?, py::getitem(&t3490, &Py::str("bandwidth"))?, py::getitem(&t3490, &Py::str("kernel"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3491 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3492 = py::len(&v_memo.clone())?; Py::Bool({ let t3493 = Py::Int(64i64); if py::gt(&t3492, &t3493)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3494 = v_want.clone(); Py::Bool({ let t3495 = Py::None; if py::is(&t3494, &t3495) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3496 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3527 = (t3496).items()?;
        if t3527.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3527[0].clone();
        v_v = t3527[1].clone();
        if ({ let t3497 = v_k.clone(); Py::Bool({ let t3498 = v_outputs.clone(); if !py::contains(&t3498, &t3497)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3551(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3552 = v_key.clone(); Py::Bool({ let t3553 = v_memo.clone(); if !py::contains(&t3553, &t3552)? { true } else { false } }) }).truthy() {
        let t3555: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3554 = v_inputs.clone(); f_knn_weights(py::getitem(&t3554, &Py::str("x"))?, py::getitem(&t3554, &Py::str("y"))?, py::getitem(&t3554, &Py::str("k"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3555 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3556 = py::len(&v_memo.clone())?; Py::Bool({ let t3557 = Py::Int(64i64); if py::gt(&t3556, &t3557)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3558 = v_want.clone(); Py::Bool({ let t3559 = Py::None; if py::is(&t3558, &t3559) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3560 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3591 = (t3560).items()?;
        if t3591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3591[0].clone();
        v_v = t3591[1].clone();
        if ({ let t3561 = v_k.clone(); Py::Bool({ let t3562 = v_outputs.clone(); if !py::contains(&t3562, &t3561)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3630(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3631 = v_key.clone(); Py::Bool({ let t3632 = v_memo.clone(); if !py::contains(&t3632, &t3631)? { true } else { false } }) }).truthy() {
        let t3634: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3633 = v_inputs.clone(); f_laea_forward(py::getitem(&t3633, &Py::str("lat"))?, py::getitem(&t3633, &Py::str("lon"))?, py::getitem(&t3633, &Py::str("lat0"))?, py::getitem(&t3633, &Py::str("lon0"))?, py::getitem(&t3633, &Py::str("ellipsoid"))?, py::method(&t3633, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t3633, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3634 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3635 = py::len(&v_memo.clone())?; Py::Bool({ let t3636 = Py::Int(64i64); if py::gt(&t3635, &t3636)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3637 = v_want.clone(); Py::Bool({ let t3638 = Py::None; if py::is(&t3637, &t3638) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3639 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3670 = (t3639).items()?;
        if t3670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3670[0].clone();
        v_v = t3670[1].clone();
        if ({ let t3640 = v_k.clone(); Py::Bool({ let t3641 = v_outputs.clone(); if !py::contains(&t3641, &t3640)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3718 = v_key.clone(); Py::Bool({ let t3719 = v_memo.clone(); if !py::contains(&t3719, &t3718)? { true } else { false } }) }).truthy() {
        let t3721: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3720 = v_inputs.clone(); f_lcc_forward(py::getitem(&t3720, &Py::str("lat"))?, py::getitem(&t3720, &Py::str("lon"))?, py::getitem(&t3720, &Py::str("lat1"))?, py::getitem(&t3720, &Py::str("lat2"))?, py::getitem(&t3720, &Py::str("lat0"))?, py::getitem(&t3720, &Py::str("lon0"))?, py::getitem(&t3720, &Py::str("ellipsoid"))?, py::method(&t3720, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t3720, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3721 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3722 = py::len(&v_memo.clone())?; Py::Bool({ let t3723 = Py::Int(64i64); if py::gt(&t3722, &t3723)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3724 = v_want.clone(); Py::Bool({ let t3725 = Py::None; if py::is(&t3724, &t3725) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3726 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3757 = (t3726).items()?;
        if t3757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3757[0].clone();
        v_v = t3757[1].clone();
        if ({ let t3727 = v_k.clone(); Py::Bool({ let t3728 = v_outputs.clone(); if !py::contains(&t3728, &t3727)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3809(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3810 = v_key.clone(); Py::Bool({ let t3811 = v_memo.clone(); if !py::contains(&t3811, &t3810)? { true } else { false } }) }).truthy() {
        let t3813: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3812 = v_inputs.clone(); f_lcc_inverse(py::getitem(&t3812, &Py::str("x"))?, py::getitem(&t3812, &Py::str("y"))?, py::getitem(&t3812, &Py::str("lat1"))?, py::getitem(&t3812, &Py::str("lat2"))?, py::getitem(&t3812, &Py::str("lat0"))?, py::getitem(&t3812, &Py::str("lon0"))?, py::getitem(&t3812, &Py::str("ellipsoid"))?, py::method(&t3812, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t3812, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3813 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3814 = py::len(&v_memo.clone())?; Py::Bool({ let t3815 = Py::Int(64i64); if py::gt(&t3814, &t3815)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3816 = v_want.clone(); Py::Bool({ let t3817 = Py::None; if py::is(&t3816, &t3817) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3818 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3849 = (t3818).items()?;
        if t3849.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3849[0].clone();
        v_v = t3849[1].clone();
        if ({ let t3819 = v_k.clone(); Py::Bool({ let t3820 = v_outputs.clone(); if !py::contains(&t3820, &t3819)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3882(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3883 = v_key.clone(); Py::Bool({ let t3884 = v_memo.clone(); if !py::contains(&t3884, &t3883)? { true } else { false } }) }).truthy() {
        let t3886: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3885 = v_inputs.clone(); f_line_of_sight(py::getitem(&t3885, &Py::str("d"))?, py::getitem(&t3885, &Py::str("z"))?, py::getitem(&t3885, &Py::str("observer_height"))?, py::getitem(&t3885, &Py::str("target_height"))?, py::getitem(&t3885, &Py::str("curvature"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3886 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3887 = py::len(&v_memo.clone())?; Py::Bool({ let t3888 = Py::Int(64i64); if py::gt(&t3887, &t3888)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3889 = v_want.clone(); Py::Bool({ let t3890 = Py::None; if py::is(&t3889, &t3890) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3891 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3922 = (t3891).items()?;
        if t3922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3922[0].clone();
        v_v = t3922[1].clone();
        if ({ let t3892 = v_k.clone(); Py::Bool({ let t3893 = v_outputs.clone(); if !py::contains(&t3893, &t3892)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3986(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3987 = v_key.clone(); Py::Bool({ let t3988 = v_memo.clone(); if !py::contains(&t3988, &t3987)? { true } else { false } }) }).truthy() {
        let t3990: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3989 = v_inputs.clone(); f_local_g(py::getitem(&t3989, &Py::str("y"))?, py::getitem(&t3989, &Py::str("W"))?, py::getitem(&t3989, &Py::str("statistic"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3990 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3991 = py::len(&v_memo.clone())?; Py::Bool({ let t3992 = Py::Int(64i64); if py::gt(&t3991, &t3992)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3993 = v_want.clone(); Py::Bool({ let t3994 = Py::None; if py::is(&t3993, &t3994) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3995 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4026 = (t3995).items()?;
        if t4026.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4026[0].clone();
        v_v = t4026[1].clone();
        if ({ let t3996 = v_k.clone(); Py::Bool({ let t3997 = v_outputs.clone(); if !py::contains(&t3997, &t3996)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_403(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t404 = v_key.clone(); Py::Bool({ let t405 = v_memo.clone(); if !py::contains(&t405, &t404)? { true } else { false } }) }).truthy() {
        let t407: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t406 = v_inputs.clone(); f_bivariate_morans_i(py::getitem(&t406, &Py::str("x"))?, py::getitem(&t406, &Py::str("y"))?, py::getitem(&t406, &Py::str("W"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t407 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t408 = py::len(&v_memo.clone())?; Py::Bool({ let t409 = Py::Int(64i64); if py::gt(&t408, &t409)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t410 = v_want.clone(); Py::Bool({ let t411 = Py::None; if py::is(&t410, &t411) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t412 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t443 = (t412).items()?;
        if t443.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t443[0].clone();
        v_v = t443[1].clone();
        if ({ let t413 = v_k.clone(); Py::Bool({ let t414 = v_outputs.clone(); if !py::contains(&t414, &t413)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4098(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4099 = v_key.clone(); Py::Bool({ let t4100 = v_memo.clone(); if !py::contains(&t4100, &t4099)? { true } else { false } }) }).truthy() {
        let t4102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4101 = v_inputs.clone(); f_local_morans_i(py::getitem(&t4101, &Py::str("y"))?, py::getitem(&t4101, &Py::str("W"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4102 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4103 = py::len(&v_memo.clone())?; Py::Bool({ let t4104 = Py::Int(64i64); if py::gt(&t4103, &t4104)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4105 = v_want.clone(); Py::Bool({ let t4106 = Py::None; if py::is(&t4105, &t4106) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4107 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4138 = (t4107).items()?;
        if t4138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4138[0].clone();
        v_v = t4138[1].clone();
        if ({ let t4108 = v_k.clone(); Py::Bool({ let t4109 = v_outputs.clone(); if !py::contains(&t4109, &t4108)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4191(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4192 = v_key.clone(); Py::Bool({ let t4193 = v_memo.clone(); if !py::contains(&t4193, &t4192)? { true } else { false } }) }).truthy() {
        let t4195: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4194 = v_inputs.clone(); f_look_angles(py::getitem(&t4194, &Py::str("lat0"))?, py::getitem(&t4194, &Py::str("lon0"))?, py::getitem(&t4194, &Py::str("h0"))?, py::getitem(&t4194, &Py::str("lat"))?, py::getitem(&t4194, &Py::str("lon"))?, py::getitem(&t4194, &Py::str("h"))?, py::getitem(&t4194, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4195 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4196 = py::len(&v_memo.clone())?; Py::Bool({ let t4197 = Py::Int(64i64); if py::gt(&t4196, &t4197)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4198 = v_want.clone(); Py::Bool({ let t4199 = Py::None; if py::is(&t4198, &t4199) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4200 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4231 = (t4200).items()?;
        if t4231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4231[0].clone();
        v_v = t4231[1].clone();
        if ({ let t4201 = v_k.clone(); Py::Bool({ let t4202 = v_outputs.clone(); if !py::contains(&t4202, &t4201)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4250(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4251 = v_key.clone(); Py::Bool({ let t4252 = v_memo.clone(); if !py::contains(&t4252, &t4251)? { true } else { false } }) }).truthy() {
        let t4254: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4253 = v_inputs.clone(); f_map_scale_from_distances(py::getitem(&t4253, &Py::str("map_distance"))?, py::getitem(&t4253, &Py::str("ground_distance"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4254 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4255 = py::len(&v_memo.clone())?; Py::Bool({ let t4256 = Py::Int(64i64); if py::gt(&t4255, &t4256)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4257 = v_want.clone(); Py::Bool({ let t4258 = Py::None; if py::is(&t4257, &t4258) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4259 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4290 = (t4259).items()?;
        if t4290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4290[0].clone();
        v_v = t4290[1].clone();
        if ({ let t4260 = v_k.clone(); Py::Bool({ let t4261 = v_outputs.clone(); if !py::contains(&t4261, &t4260)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4297(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4298 = v_key.clone(); Py::Bool({ let t4299 = v_memo.clone(); if !py::contains(&t4299, &t4298)? { true } else { false } }) }).truthy() {
        let t4301: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4300 = v_inputs.clone(); f_map_scale_ground_distance(py::getitem(&t4300, &Py::str("map_distance"))?, py::getitem(&t4300, &Py::str("scale"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4301 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4302 = py::len(&v_memo.clone())?; Py::Bool({ let t4303 = Py::Int(64i64); if py::gt(&t4302, &t4303)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4304 = v_want.clone(); Py::Bool({ let t4305 = Py::None; if py::is(&t4304, &t4305) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4306 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4337 = (t4306).items()?;
        if t4337.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4337[0].clone();
        v_v = t4337[1].clone();
        if ({ let t4307 = v_k.clone(); Py::Bool({ let t4308 = v_outputs.clone(); if !py::contains(&t4308, &t4307)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4422(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4423 = v_key.clone(); Py::Bool({ let t4424 = v_memo.clone(); if !py::contains(&t4424, &t4423)? { true } else { false } }) }).truthy() {
        let t4426: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4425 = v_inputs.clone(); f_max_covering(py::getitem(&t4425, &Py::str("edges"))?, py::getitem(&t4425, &Py::str("demand"))?, py::getitem(&t4425, &Py::str("candidates"))?, py::getitem(&t4425, &Py::str("p"))?, py::getitem(&t4425, &Py::str("coverage"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4426 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4427 = py::len(&v_memo.clone())?; Py::Bool({ let t4428 = Py::Int(64i64); if py::gt(&t4427, &t4428)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4429 = v_want.clone(); Py::Bool({ let t4430 = Py::None; if py::is(&t4429, &t4430) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4431 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4462 = (t4431).items()?;
        if t4462.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4462[0].clone();
        v_v = t4462[1].clone();
        if ({ let t4432 = v_k.clone(); Py::Bool({ let t4433 = v_outputs.clone(); if !py::contains(&t4433, &t4432)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4484 = v_key.clone(); Py::Bool({ let t4485 = v_memo.clone(); if !py::contains(&t4485, &t4484)? { true } else { false } }) }).truthy() {
        let t4487: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4486 = v_inputs.clone(); f_mean_center(py::getitem(&t4486, &Py::str("x"))?, py::getitem(&t4486, &Py::str("y"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4487 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4488 = py::len(&v_memo.clone())?; Py::Bool({ let t4489 = Py::Int(64i64); if py::gt(&t4488, &t4489)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4490 = v_want.clone(); Py::Bool({ let t4491 = Py::None; if py::is(&t4490, &t4491) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4492 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4523 = (t4492).items()?;
        if t4523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4523[0].clone();
        v_v = t4523[1].clone();
        if ({ let t4493 = v_k.clone(); Py::Bool({ let t4494 = v_outputs.clone(); if !py::contains(&t4494, &t4493)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_45(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t46 = v_key.clone(); Py::Bool({ let t47 = v_memo.clone(); if !py::contains(&t47, &t46)? { true } else { false } }) }).truthy() {
        let t49: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t48 = v_inputs.clone(); f_albers_forward(py::getitem(&t48, &Py::str("lat"))?, py::getitem(&t48, &Py::str("lon"))?, py::getitem(&t48, &Py::str("lat1"))?, py::getitem(&t48, &Py::str("lat2"))?, py::getitem(&t48, &Py::str("lat0"))?, py::getitem(&t48, &Py::str("lon0"))?, py::getitem(&t48, &Py::str("ellipsoid"))?, py::method(&t48, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t48, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t49 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t50 = py::len(&v_memo.clone())?; Py::Bool({ let t51 = Py::Int(64i64); if py::gt(&t50, &t51)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t52 = v_want.clone(); Py::Bool({ let t53 = Py::None; if py::is(&t52, &t53) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t54 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t85 = (t54).items()?;
        if t85.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t85[0].clone();
        v_v = t85[1].clone();
        if ({ let t55 = v_k.clone(); Py::Bool({ let t56 = v_outputs.clone(); if !py::contains(&t56, &t55)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4559(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4560 = v_key.clone(); Py::Bool({ let t4561 = v_memo.clone(); if !py::contains(&t4561, &t4560)? { true } else { false } }) }).truthy() {
        let t4563: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4562 = v_inputs.clone(); f_mercator_forward(py::getitem(&t4562, &Py::str("lat"))?, py::getitem(&t4562, &Py::str("lon"))?, py::getitem(&t4562, &Py::str("lon0"))?, py::getitem(&t4562, &Py::str("lat_ts"))?, py::getitem(&t4562, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4563 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4564 = py::len(&v_memo.clone())?; Py::Bool({ let t4565 = Py::Int(64i64); if py::gt(&t4564, &t4565)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4566 = v_want.clone(); Py::Bool({ let t4567 = Py::None; if py::is(&t4566, &t4567) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4568 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4599 = (t4568).items()?;
        if t4599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4599[0].clone();
        v_v = t4599[1].clone();
        if ({ let t4569 = v_k.clone(); Py::Bool({ let t4570 = v_outputs.clone(); if !py::contains(&t4570, &t4569)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_462(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t463 = v_key.clone(); Py::Bool({ let t464 = v_memo.clone(); if !py::contains(&t464, &t463)? { true } else { false } }) }).truthy() {
        let t466: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t465 = v_inputs.clone(); f_brightness_temperature(py::getitem(&t465, &Py::str("radiance"))?, py::getitem(&t465, &Py::str("K1"))?, py::getitem(&t465, &Py::str("K2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t466 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t467 = py::len(&v_memo.clone())?; Py::Bool({ let t468 = Py::Int(64i64); if py::gt(&t467, &t468)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t469 = v_want.clone(); Py::Bool({ let t470 = Py::None; if py::is(&t469, &t470) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t471 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t502 = (t471).items()?;
        if t502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t502[0].clone();
        v_v = t502[1].clone();
        if ({ let t472 = v_k.clone(); Py::Bool({ let t473 = v_outputs.clone(); if !py::contains(&t473, &t472)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4640(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4641 = v_key.clone(); Py::Bool({ let t4642 = v_memo.clone(); if !py::contains(&t4642, &t4641)? { true } else { false } }) }).truthy() {
        let t4644: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4643 = v_inputs.clone(); f_mercator_inverse(py::getitem(&t4643, &Py::str("x"))?, py::getitem(&t4643, &Py::str("y"))?, py::getitem(&t4643, &Py::str("lon0"))?, py::getitem(&t4643, &Py::str("lat_ts"))?, py::getitem(&t4643, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4644 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4645 = py::len(&v_memo.clone())?; Py::Bool({ let t4646 = Py::Int(64i64); if py::gt(&t4645, &t4646)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4647 = v_want.clone(); Py::Bool({ let t4648 = Py::None; if py::is(&t4647, &t4648) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4649 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4680 = (t4649).items()?;
        if t4680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4680[0].clone();
        v_v = t4680[1].clone();
        if ({ let t4650 = v_k.clone(); Py::Bool({ let t4651 = v_outputs.clone(); if !py::contains(&t4651, &t4650)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4709(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4710 = v_key.clone(); Py::Bool({ let t4711 = v_memo.clone(); if !py::contains(&t4711, &t4710)? { true } else { false } }) }).truthy() {
        let t4713: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4712 = v_inputs.clone(); f_meridian_arc(py::getitem(&t4712, &Py::str("lat1"))?, py::getitem(&t4712, &Py::str("lat2"))?, py::getitem(&t4712, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4713 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4714 = py::len(&v_memo.clone())?; Py::Bool({ let t4715 = Py::Int(64i64); if py::gt(&t4714, &t4715)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4716 = v_want.clone(); Py::Bool({ let t4717 = Py::None; if py::is(&t4716, &t4717) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4718 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4749 = (t4718).items()?;
        if t4749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4749[0].clone();
        v_v = t4749[1].clone();
        if ({ let t4719 = v_k.clone(); Py::Bool({ let t4720 = v_outputs.clone(); if !py::contains(&t4720, &t4719)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4813(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4814 = v_key.clone(); Py::Bool({ let t4815 = v_memo.clone(); if !py::contains(&t4815, &t4814)? { true } else { false } }) }).truthy() {
        let t4817: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4816 = v_inputs.clone(); f_mgrs_encode(py::getitem(&t4816, &Py::str("lat"))?, py::getitem(&t4816, &Py::str("lon"))?, py::method(&t4816, "get", &[Py::str("ellipsoid"), Py::str("WGS84")])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4817 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4818 = py::len(&v_memo.clone())?; Py::Bool({ let t4819 = Py::Int(64i64); if py::gt(&t4818, &t4819)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4820 = v_want.clone(); Py::Bool({ let t4821 = Py::None; if py::is(&t4820, &t4821) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4822 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4853 = (t4822).items()?;
        if t4853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4853[0].clone();
        v_v = t4853[1].clone();
        if ({ let t4823 = v_k.clone(); Py::Bool({ let t4824 = v_outputs.clone(); if !py::contains(&t4824, &t4823)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4895(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4896 = v_key.clone(); Py::Bool({ let t4897 = v_memo.clone(); if !py::contains(&t4897, &t4896)? { true } else { false } }) }).truthy() {
        let t4899: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4898 = v_inputs.clone(); f_mollweide_forward(py::getitem(&t4898, &Py::str("lat"))?, py::getitem(&t4898, &Py::str("lon"))?, py::getitem(&t4898, &Py::str("lon0"))?, py::getitem(&t4898, &Py::str("R"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4899 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4900 = py::len(&v_memo.clone())?; Py::Bool({ let t4901 = Py::Int(64i64); if py::gt(&t4900, &t4901)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4902 = v_want.clone(); Py::Bool({ let t4903 = Py::None; if py::is(&t4902, &t4903) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4904 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4935 = (t4904).items()?;
        if t4935.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4935[0].clone();
        v_v = t4935[1].clone();
        if ({ let t4905 = v_k.clone(); Py::Bool({ let t4906 = v_outputs.clone(); if !py::contains(&t4906, &t4905)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4997(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4998 = v_key.clone(); Py::Bool({ let t4999 = v_memo.clone(); if !py::contains(&t4999, &t4998)? { true } else { false } }) }).truthy() {
        let t5001: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5000 = v_inputs.clone(); f_morans_i(py::getitem(&t5000, &Py::str("y"))?, py::getitem(&t5000, &Py::str("W"))?, py::getitem(&t5000, &Py::str("transform"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5001 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5002 = py::len(&v_memo.clone())?; Py::Bool({ let t5003 = Py::Int(64i64); if py::gt(&t5002, &t5003)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5004 = v_want.clone(); Py::Bool({ let t5005 = Py::None; if py::is(&t5004, &t5005) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5006 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5037 = (t5006).items()?;
        if t5037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5037[0].clone();
        v_v = t5037[1].clone();
        if ({ let t5007 = v_k.clone(); Py::Bool({ let t5008 = v_outputs.clone(); if !py::contains(&t5008, &t5007)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5066(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5067 = v_key.clone(); Py::Bool({ let t5068 = v_memo.clone(); if !py::contains(&t5068, &t5067)? { true } else { false } }) }).truthy() {
        let t5070: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5069 = v_inputs.clone(); f_nearest_neighbour_index(py::getitem(&t5069, &Py::str("x"))?, py::getitem(&t5069, &Py::str("y"))?, py::getitem(&t5069, &Py::str("area"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5070 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5071 = py::len(&v_memo.clone())?; Py::Bool({ let t5072 = Py::Int(64i64); if py::gt(&t5071, &t5072)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5073 = v_want.clone(); Py::Bool({ let t5074 = Py::None; if py::is(&t5073, &t5074) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5075 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5106 = (t5075).items()?;
        if t5106.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5106[0].clone();
        v_v = t5106[1].clone();
        if ({ let t5076 = v_k.clone(); Py::Bool({ let t5077 = v_outputs.clone(); if !py::contains(&t5077, &t5076)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5133(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5134 = v_key.clone(); Py::Bool({ let t5135 = v_memo.clone(); if !py::contains(&t5135, &t5134)? { true } else { false } }) }).truthy() {
        let t5137: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5136 = v_inputs.clone(); f_network_indices(py::getitem(&t5136, &Py::str("edges"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5137 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5138 = py::len(&v_memo.clone())?; Py::Bool({ let t5139 = Py::Int(64i64); if py::gt(&t5138, &t5139)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5140 = v_want.clone(); Py::Bool({ let t5141 = Py::None; if py::is(&t5140, &t5141) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5142 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5173 = (t5142).items()?;
        if t5173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5173[0].clone();
        v_v = t5173[1].clone();
        if ({ let t5143 = v_k.clone(); Py::Bool({ let t5144 = v_outputs.clone(); if !py::contains(&t5144, &t5143)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5252(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5253 = v_key.clone(); Py::Bool({ let t5254 = v_memo.clone(); if !py::contains(&t5254, &t5253)? { true } else { false } }) }).truthy() {
        let t5256: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5255 = v_inputs.clone(); f_ordinary_kriging(py::getitem(&t5255, &Py::str("x"))?, py::getitem(&t5255, &Py::str("y"))?, py::getitem(&t5255, &Py::str("z"))?, py::getitem(&t5255, &Py::str("px"))?, py::getitem(&t5255, &Py::str("py"))?, py::getitem(&t5255, &Py::str("model"))?, py::getitem(&t5255, &Py::str("nugget"))?, py::getitem(&t5255, &Py::str("sill"))?, py::getitem(&t5255, &Py::str("range"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5256 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5257 = py::len(&v_memo.clone())?; Py::Bool({ let t5258 = Py::Int(64i64); if py::gt(&t5257, &t5258)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5259 = v_want.clone(); Py::Bool({ let t5260 = Py::None; if py::is(&t5259, &t5260) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5261 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5292 = (t5261).items()?;
        if t5292.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5292[0].clone();
        v_v = t5292[1].clone();
        if ({ let t5262 = v_k.clone(); Py::Bool({ let t5263 = v_outputs.clone(); if !py::contains(&t5263, &t5262)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5332(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5333 = v_key.clone(); Py::Bool({ let t5334 = v_memo.clone(); if !py::contains(&t5334, &t5333)? { true } else { false } }) }).truthy() {
        let t5336: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5335 = v_inputs.clone(); f_orthographic_forward(py::getitem(&t5335, &Py::str("lat"))?, py::getitem(&t5335, &Py::str("lon"))?, py::getitem(&t5335, &Py::str("lat0"))?, py::getitem(&t5335, &Py::str("lon0"))?, py::getitem(&t5335, &Py::str("R"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5336 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5337 = py::len(&v_memo.clone())?; Py::Bool({ let t5338 = Py::Int(64i64); if py::gt(&t5337, &t5338)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5339 = v_want.clone(); Py::Bool({ let t5340 = Py::None; if py::is(&t5339, &t5340) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5341 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5372 = (t5341).items()?;
        if t5372.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5372[0].clone();
        v_v = t5372[1].clone();
        if ({ let t5342 = v_k.clone(); Py::Bool({ let t5343 = v_outputs.clone(); if !py::contains(&t5343, &t5342)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_545(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t546 = v_key.clone(); Py::Bool({ let t547 = v_memo.clone(); if !py::contains(&t547, &t546)? { true } else { false } }) }).truthy() {
        let t549: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t548 = v_inputs.clone(); f_classification_accuracy(py::getitem(&t548, &Py::str("matrix"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t549 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t550 = py::len(&v_memo.clone())?; Py::Bool({ let t551 = Py::Int(64i64); if py::gt(&t550, &t551)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t552 = v_want.clone(); Py::Bool({ let t553 = Py::None; if py::is(&t552, &t553) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t554 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t585 = (t554).items()?;
        if t585.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t585[0].clone();
        v_v = t585[1].clone();
        if ({ let t555 = v_k.clone(); Py::Bool({ let t556 = v_outputs.clone(); if !py::contains(&t556, &t555)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5458(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5459 = v_key.clone(); Py::Bool({ let t5460 = v_memo.clone(); if !py::contains(&t5460, &t5459)? { true } else { false } }) }).truthy() {
        let t5462: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5461 = v_inputs.clone(); f_p_median(py::getitem(&t5461, &Py::str("edges"))?, py::getitem(&t5461, &Py::str("demand"))?, py::getitem(&t5461, &Py::str("candidates"))?, py::getitem(&t5461, &Py::str("p"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5462 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5463 = py::len(&v_memo.clone())?; Py::Bool({ let t5464 = Py::Int(64i64); if py::gt(&t5463, &t5464)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5465 = v_want.clone(); Py::Bool({ let t5466 = Py::None; if py::is(&t5465, &t5466) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5467 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5498 = (t5467).items()?;
        if t5498.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5498[0].clone();
        v_v = t5498[1].clone();
        if ({ let t5468 = v_k.clone(); Py::Bool({ let t5469 = v_outputs.clone(); if !py::contains(&t5469, &t5468)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5532(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5533 = v_key.clone(); Py::Bool({ let t5534 = v_memo.clone(); if !py::contains(&t5534, &t5533)? { true } else { false } }) }).truthy() {
        let t5536: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5535 = v_inputs.clone(); f_parallel_arc(py::getitem(&t5535, &Py::str("lat"))?, py::getitem(&t5535, &Py::str("lon1"))?, py::getitem(&t5535, &Py::str("lon2"))?, py::getitem(&t5535, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5536 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5537 = py::len(&v_memo.clone())?; Py::Bool({ let t5538 = Py::Int(64i64); if py::gt(&t5537, &t5538)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5539 = v_want.clone(); Py::Bool({ let t5540 = Py::None; if py::is(&t5539, &t5540) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5541 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5572 = (t5541).items()?;
        if t5572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5572[0].clone();
        v_v = t5572[1].clone();
        if ({ let t5542 = v_k.clone(); Py::Bool({ let t5543 = v_outputs.clone(); if !py::contains(&t5543, &t5542)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5615(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5616 = v_key.clone(); Py::Bool({ let t5617 = v_memo.clone(); if !py::contains(&t5617, &t5616)? { true } else { false } }) }).truthy() {
        let t5619: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5618 = v_inputs.clone(); f_polar_stereographic_forward(py::getitem(&t5618, &Py::str("lat"))?, py::getitem(&t5618, &Py::str("lon"))?, py::getitem(&t5618, &Py::str("pole"))?, py::getitem(&t5618, &Py::str("lat_ts"))?, py::getitem(&t5618, &Py::str("lon0"))?, py::getitem(&t5618, &Py::str("ellipsoid"))?, py::method(&t5618, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t5618, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5619 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5620 = py::len(&v_memo.clone())?; Py::Bool({ let t5621 = Py::Int(64i64); if py::gt(&t5620, &t5621)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5622 = v_want.clone(); Py::Bool({ let t5623 = Py::None; if py::is(&t5622, &t5623) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5624 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5655 = (t5624).items()?;
        if t5655.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5655[0].clone();
        v_v = t5655[1].clone();
        if ({ let t5625 = v_k.clone(); Py::Bool({ let t5626 = v_outputs.clone(); if !py::contains(&t5626, &t5625)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5736(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5737 = v_key.clone(); Py::Bool({ let t5738 = v_memo.clone(); if !py::contains(&t5738, &t5737)? { true } else { false } }) }).truthy() {
        let t5740: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5739 = v_inputs.clone(); f_compactness(py::getitem(&t5739, &Py::str("x"))?, py::getitem(&t5739, &Py::str("y"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5740 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5741 = py::len(&v_memo.clone())?; Py::Bool({ let t5742 = Py::Int(64i64); if py::gt(&t5741, &t5742)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5743 = v_want.clone(); Py::Bool({ let t5744 = Py::None; if py::is(&t5743, &t5744) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5745 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5776 = (t5745).items()?;
        if t5776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5776[0].clone();
        v_v = t5776[1].clone();
        if ({ let t5746 = v_k.clone(); Py::Bool({ let t5747 = v_outputs.clone(); if !py::contains(&t5747, &t5746)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5816(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5817 = v_key.clone(); Py::Bool({ let t5818 = v_memo.clone(); if !py::contains(&t5818, &t5817)? { true } else { false } }) }).truthy() {
        let t5820: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5819 = v_inputs.clone(); f_quadrat_test(py::getitem(&t5819, &Py::str("x"))?, py::getitem(&t5819, &Py::str("y"))?, py::getitem(&t5819, &Py::str("nx"))?, py::getitem(&t5819, &Py::str("ny"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5820 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5821 = py::len(&v_memo.clone())?; Py::Bool({ let t5822 = Py::Int(64i64); if py::gt(&t5821, &t5822)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5823 = v_want.clone(); Py::Bool({ let t5824 = Py::None; if py::is(&t5823, &t5824) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5825 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5856 = (t5825).items()?;
        if t5856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5856[0].clone();
        v_v = t5856[1].clone();
        if ({ let t5826 = v_k.clone(); Py::Bool({ let t5827 = v_outputs.clone(); if !py::contains(&t5827, &t5826)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5882(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5883 = v_key.clone(); Py::Bool({ let t5884 = v_memo.clone(); if !py::contains(&t5884, &t5883)? { true } else { false } }) }).truthy() {
        let t5886: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5885 = v_inputs.clone(); f_radii_of_curvature(py::getitem(&t5885, &Py::str("lat"))?, py::getitem(&t5885, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5886 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5887 = py::len(&v_memo.clone())?; Py::Bool({ let t5888 = Py::Int(64i64); if py::gt(&t5887, &t5888)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5889 = v_want.clone(); Py::Bool({ let t5890 = Py::None; if py::is(&t5889, &t5890) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5891 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5922 = (t5891).items()?;
        if t5922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5922[0].clone();
        v_v = t5922[1].clone();
        if ({ let t5892 = v_k.clone(); Py::Bool({ let t5893 = v_outputs.clone(); if !py::contains(&t5893, &t5892)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5956(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5957 = v_key.clone(); Py::Bool({ let t5958 = v_memo.clone(); if !py::contains(&t5958, &t5957)? { true } else { false } }) }).truthy() {
        let t5960: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5959 = v_inputs.clone(); f_rank_size(py::getitem(&t5959, &Py::str("populations"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5960 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5961 = py::len(&v_memo.clone())?; Py::Bool({ let t5962 = Py::Int(64i64); if py::gt(&t5961, &t5962)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5963 = v_want.clone(); Py::Bool({ let t5964 = Py::None; if py::is(&t5963, &t5964) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5965 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5996 = (t5965).items()?;
        if t5996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5996[0].clone();
        v_v = t5996[1].clone();
        if ({ let t5966 = v_k.clone(); Py::Bool({ let t5967 = v_outputs.clone(); if !py::contains(&t5967, &t5966)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6044(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6045 = v_key.clone(); Py::Bool({ let t6046 = v_memo.clone(); if !py::contains(&t6046, &t6045)? { true } else { false } }) }).truthy() {
        let t6048: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6047 = v_inputs.clone(); f_rhumb_destination(py::getitem(&t6047, &Py::str("lat1"))?, py::getitem(&t6047, &Py::str("lon1"))?, py::getitem(&t6047, &Py::str("bearing"))?, py::getitem(&t6047, &Py::str("distance"))?, py::getitem(&t6047, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6048 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6049 = py::len(&v_memo.clone())?; Py::Bool({ let t6050 = Py::Int(64i64); if py::gt(&t6049, &t6050)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6051 = v_want.clone(); Py::Bool({ let t6052 = Py::None; if py::is(&t6051, &t6052) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6053 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6084 = (t6053).items()?;
        if t6084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6084[0].clone();
        v_v = t6084[1].clone();
        if ({ let t6054 = v_k.clone(); Py::Bool({ let t6055 = v_outputs.clone(); if !py::contains(&t6055, &t6054)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6133(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6134 = v_key.clone(); Py::Bool({ let t6135 = v_memo.clone(); if !py::contains(&t6135, &t6134)? { true } else { false } }) }).truthy() {
        let t6137: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6136 = v_inputs.clone(); f_rhumb_inverse(py::getitem(&t6136, &Py::str("lat1"))?, py::getitem(&t6136, &Py::str("lon1"))?, py::getitem(&t6136, &Py::str("lat2"))?, py::getitem(&t6136, &Py::str("lon2"))?, py::getitem(&t6136, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6137 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6138 = py::len(&v_memo.clone())?; Py::Bool({ let t6139 = Py::Int(64i64); if py::gt(&t6138, &t6139)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6140 = v_want.clone(); Py::Bool({ let t6141 = Py::None; if py::is(&t6140, &t6141) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6142 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6173 = (t6142).items()?;
        if t6173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6173[0].clone();
        v_v = t6173[1].clone();
        if ({ let t6143 = v_k.clone(); Py::Bool({ let t6144 = v_outputs.clone(); if !py::contains(&t6144, &t6143)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6206(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6207 = v_key.clone(); Py::Bool({ let t6208 = v_memo.clone(); if !py::contains(&t6208, &t6207)? { true } else { false } }) }).truthy() {
        let t6210: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6209 = v_inputs.clone(); f_ripley_k(py::getitem(&t6209, &Py::str("x"))?, py::getitem(&t6209, &Py::str("y"))?, py::getitem(&t6209, &Py::str("d"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6210 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6211 = py::len(&v_memo.clone())?; Py::Bool({ let t6212 = Py::Int(64i64); if py::gt(&t6211, &t6212)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6213 = v_want.clone(); Py::Bool({ let t6214 = Py::None; if py::is(&t6213, &t6214) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6215 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6246 = (t6215).items()?;
        if t6246.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6246[0].clone();
        v_v = t6246[1].clone();
        if ({ let t6216 = v_k.clone(); Py::Bool({ let t6217 = v_outputs.clone(); if !py::contains(&t6217, &t6216)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6295(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6296 = v_key.clone(); Py::Bool({ let t6297 = v_memo.clone(); if !py::contains(&t6297, &t6296)? { true } else { false } }) }).truthy() {
        let t6299: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6298 = v_inputs.clone(); f_segregation_evenness(py::getitem(&t6298, &Py::str("x"))?, py::getitem(&t6298, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6299 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6300 = py::len(&v_memo.clone())?; Py::Bool({ let t6301 = Py::Int(64i64); if py::gt(&t6300, &t6301)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6302 = v_want.clone(); Py::Bool({ let t6303 = Py::None; if py::is(&t6302, &t6303) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6304 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6335 = (t6304).items()?;
        if t6335.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6335[0].clone();
        v_v = t6335[1].clone();
        if ({ let t6305 = v_k.clone(); Py::Bool({ let t6306 = v_outputs.clone(); if !py::contains(&t6306, &t6305)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6378(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6379 = v_key.clone(); Py::Bool({ let t6380 = v_memo.clone(); if !py::contains(&t6380, &t6379)? { true } else { false } }) }).truthy() {
        let t6382: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6381 = v_inputs.clone(); f_segregation_exposure(py::getitem(&t6381, &Py::str("x"))?, py::getitem(&t6381, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6382 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6383 = py::len(&v_memo.clone())?; Py::Bool({ let t6384 = Py::Int(64i64); if py::gt(&t6383, &t6384)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6385 = v_want.clone(); Py::Bool({ let t6386 = Py::None; if py::is(&t6385, &t6386) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6387 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6418 = (t6387).items()?;
        if t6418.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6418[0].clone();
        v_v = t6418[1].clone();
        if ({ let t6388 = v_k.clone(); Py::Bool({ let t6389 = v_outputs.clone(); if !py::contains(&t6389, &t6388)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6479(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6480 = v_key.clone(); Py::Bool({ let t6481 = v_memo.clone(); if !py::contains(&t6481, &t6480)? { true } else { false } }) }).truthy() {
        let t6483: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6482 = v_inputs.clone(); f_service_area(py::getitem(&t6482, &Py::str("edges"))?, py::getitem(&t6482, &Py::str("source"))?, py::getitem(&t6482, &Py::str("cutoff"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6483 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6484 = py::len(&v_memo.clone())?; Py::Bool({ let t6485 = Py::Int(64i64); if py::gt(&t6484, &t6485)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6486 = v_want.clone(); Py::Bool({ let t6487 = Py::None; if py::is(&t6486, &t6487) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6488 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6519 = (t6488).items()?;
        if t6519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6519[0].clone();
        v_v = t6519[1].clone();
        if ({ let t6489 = v_k.clone(); Py::Bool({ let t6490 = v_outputs.clone(); if !py::contains(&t6490, &t6489)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_655(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t656 = v_key.clone(); Py::Bool({ let t657 = v_memo.clone(); if !py::contains(&t657, &t656)? { true } else { false } }) }).truthy() {
        let t659: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t658 = v_inputs.clone(); f_closest_facility(py::getitem(&t658, &Py::str("edges"))?, py::getitem(&t658, &Py::str("facilities"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t659 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t660 = py::len(&v_memo.clone())?; Py::Bool({ let t661 = Py::Int(64i64); if py::gt(&t660, &t661)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t662 = v_want.clone(); Py::Bool({ let t663 = Py::None; if py::is(&t662, &t663) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t664 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t695 = (t664).items()?;
        if t695.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t695[0].clone();
        v_v = t695[1].clone();
        if ({ let t665 = v_k.clone(); Py::Bool({ let t666 = v_outputs.clone(); if !py::contains(&t666, &t665)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6567(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6568 = v_key.clone(); Py::Bool({ let t6569 = v_memo.clone(); if !py::contains(&t6569, &t6568)? { true } else { false } }) }).truthy() {
        let t6571: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6570 = v_inputs.clone(); f_shimbel_index(py::getitem(&t6570, &Py::str("edges"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6571 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6572 = py::len(&v_memo.clone())?; Py::Bool({ let t6573 = Py::Int(64i64); if py::gt(&t6572, &t6573)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6574 = v_want.clone(); Py::Bool({ let t6575 = Py::None; if py::is(&t6574, &t6575) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6576 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6607 = (t6576).items()?;
        if t6607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6607[0].clone();
        v_v = t6607[1].clone();
        if ({ let t6577 = v_k.clone(); Py::Bool({ let t6578 = v_outputs.clone(); if !py::contains(&t6578, &t6577)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6642(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6643 = v_key.clone(); Py::Bool({ let t6644 = v_memo.clone(); if !py::contains(&t6644, &t6643)? { true } else { false } }) }).truthy() {
        let t6646: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6645 = v_inputs.clone(); f_sinusoidal_forward(py::getitem(&t6645, &Py::str("lat"))?, py::getitem(&t6645, &Py::str("lon"))?, py::getitem(&t6645, &Py::str("lon0"))?, py::getitem(&t6645, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6646 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6647 = py::len(&v_memo.clone())?; Py::Bool({ let t6648 = Py::Int(64i64); if py::gt(&t6647, &t6648)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6649 = v_want.clone(); Py::Bool({ let t6650 = Py::None; if py::is(&t6649, &t6650) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6651 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6682 = (t6651).items()?;
        if t6682.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6682[0].clone();
        v_v = t6682[1].clone();
        if ({ let t6652 = v_k.clone(); Py::Bool({ let t6653 = v_outputs.clone(); if !py::contains(&t6653, &t6652)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6719(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6720 = v_key.clone(); Py::Bool({ let t6721 = v_memo.clone(); if !py::contains(&t6721, &t6720)? { true } else { false } }) }).truthy() {
        let t6723: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6722 = v_inputs.clone(); f_slope_aspect(py::getitem(&t6722, &Py::str("window"))?, py::getitem(&t6722, &Py::str("cell_size"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6723 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6724 = py::len(&v_memo.clone())?; Py::Bool({ let t6725 = Py::Int(64i64); if py::gt(&t6724, &t6725)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6726 = v_want.clone(); Py::Bool({ let t6727 = Py::None; if py::is(&t6726, &t6727) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6728 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6759 = (t6728).items()?;
        if t6759.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6759[0].clone();
        v_v = t6759[1].clone();
        if ({ let t6729 = v_k.clone(); Py::Bool({ let t6730 = v_outputs.clone(); if !py::contains(&t6730, &t6729)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6806(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6807 = v_key.clone(); Py::Bool({ let t6808 = v_memo.clone(); if !py::contains(&t6808, &t6807)? { true } else { false } }) }).truthy() {
        let t6810: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6809 = v_inputs.clone(); f_spatial_lag(py::getitem(&t6809, &Py::str("y"))?, py::getitem(&t6809, &Py::str("W"))?, py::getitem(&t6809, &Py::str("transform"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6810 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6811 = py::len(&v_memo.clone())?; Py::Bool({ let t6812 = Py::Int(64i64); if py::gt(&t6811, &t6812)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6813 = v_want.clone(); Py::Bool({ let t6814 = Py::None; if py::is(&t6813, &t6814) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6815 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6846 = (t6815).items()?;
        if t6846.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6846[0].clone();
        v_v = t6846[1].clone();
        if ({ let t6816 = v_k.clone(); Py::Bool({ let t6817 = v_outputs.clone(); if !py::contains(&t6817, &t6816)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6879(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6880 = v_key.clone(); Py::Bool({ let t6881 = v_memo.clone(); if !py::contains(&t6881, &t6880)? { true } else { false } }) }).truthy() {
        let t6883: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6882 = v_inputs.clone(); f_std_ellipse(py::getitem(&t6882, &Py::str("x"))?, py::getitem(&t6882, &Py::str("y"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6883 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6884 = py::len(&v_memo.clone())?; Py::Bool({ let t6885 = Py::Int(64i64); if py::gt(&t6884, &t6885)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6886 = v_want.clone(); Py::Bool({ let t6887 = Py::None; if py::is(&t6886, &t6887) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6888 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6919 = (t6888).items()?;
        if t6919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6919[0].clone();
        v_v = t6919[1].clone();
        if ({ let t6889 = v_k.clone(); Py::Bool({ let t6890 = v_outputs.clone(); if !py::contains(&t6890, &t6889)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6953(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6954 = v_key.clone(); Py::Bool({ let t6955 = v_memo.clone(); if !py::contains(&t6955, &t6954)? { true } else { false } }) }).truthy() {
        let t6957: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6956 = v_inputs.clone(); f_curvature(py::getitem(&t6956, &Py::str("window"))?, py::getitem(&t6956, &Py::str("cell_size"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6957 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6958 = py::len(&v_memo.clone())?; Py::Bool({ let t6959 = Py::Int(64i64); if py::gt(&t6958, &t6959)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6960 = v_want.clone(); Py::Bool({ let t6961 = Py::None; if py::is(&t6960, &t6961) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6962 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6993 = (t6962).items()?;
        if t6993.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6993[0].clone();
        v_v = t6993[1].clone();
        if ({ let t6963 = v_k.clone(); Py::Bool({ let t6964 = v_outputs.clone(); if !py::contains(&t6964, &t6963)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7022(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7023 = v_key.clone(); Py::Bool({ let t7024 = v_memo.clone(); if !py::contains(&t7024, &t7023)? { true } else { false } }) }).truthy() {
        let t7026: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7025 = v_inputs.clone(); f_terrain_ruggedness(py::getitem(&t7025, &Py::str("window"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7026 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7027 = py::len(&v_memo.clone())?; Py::Bool({ let t7028 = Py::Int(64i64); if py::gt(&t7027, &t7028)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7029 = v_want.clone(); Py::Bool({ let t7030 = Py::None; if py::is(&t7029, &t7030) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7031 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7062 = (t7031).items()?;
        if t7062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7062[0].clone();
        v_v = t7062[1].clone();
        if ({ let t7032 = v_k.clone(); Py::Bool({ let t7033 = v_outputs.clone(); if !py::contains(&t7033, &t7032)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7110 = v_key.clone(); Py::Bool({ let t7111 = v_memo.clone(); if !py::contains(&t7111, &t7110)? { true } else { false } }) }).truthy() {
        let t7113: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7112 = v_inputs.clone(); f_tissot_indicatrix(py::getitem(&t7112, &Py::str("projection"))?, py::getitem(&t7112, &Py::str("lat"))?, py::getitem(&t7112, &Py::str("lon"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7113 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7114 = py::len(&v_memo.clone())?; Py::Bool({ let t7115 = Py::Int(64i64); if py::gt(&t7114, &t7115)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7116 = v_want.clone(); Py::Bool({ let t7117 = Py::None; if py::is(&t7116, &t7117) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7118 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7149 = (t7118).items()?;
        if t7149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7149[0].clone();
        v_v = t7149[1].clone();
        if ({ let t7119 = v_k.clone(); Py::Bool({ let t7120 = v_outputs.clone(); if !py::contains(&t7120, &t7119)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7184(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7185 = v_key.clone(); Py::Bool({ let t7186 = v_memo.clone(); if !py::contains(&t7186, &t7185)? { true } else { false } }) }).truthy() {
        let t7188: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7187 = v_inputs.clone(); f_toa_reflectance(py::getitem(&t7187, &Py::str("radiance"))?, py::getitem(&t7187, &Py::str("esun"))?, py::getitem(&t7187, &Py::str("doy"))?, py::getitem(&t7187, &Py::str("sun_elevation"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7188 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7189 = py::len(&v_memo.clone())?; Py::Bool({ let t7190 = Py::Int(64i64); if py::gt(&t7189, &t7190)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7191 = v_want.clone(); Py::Bool({ let t7192 = Py::None; if py::is(&t7191, &t7192) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7193 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7224 = (t7193).items()?;
        if t7224.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7224[0].clone();
        v_v = t7224[1].clone();
        if ({ let t7194 = v_k.clone(); Py::Bool({ let t7195 = v_outputs.clone(); if !py::contains(&t7195, &t7194)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7275(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7276 = v_key.clone(); Py::Bool({ let t7277 = v_memo.clone(); if !py::contains(&t7277, &t7276)? { true } else { false } }) }).truthy() {
        let t7279: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7278 = v_inputs.clone(); f_tm_forward(py::getitem(&t7278, &Py::str("lat"))?, py::getitem(&t7278, &Py::str("lon"))?, py::getitem(&t7278, &Py::str("lat0"))?, py::getitem(&t7278, &Py::str("lon0"))?, py::getitem(&t7278, &Py::str("k0"))?, py::getitem(&t7278, &Py::str("ellipsoid"))?, py::method(&t7278, "get", &[Py::str("FE"), Py::Float(0.0)])?, py::method(&t7278, "get", &[Py::str("FN"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7279 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7280 = py::len(&v_memo.clone())?; Py::Bool({ let t7281 = Py::Int(64i64); if py::gt(&t7280, &t7281)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7282 = v_want.clone(); Py::Bool({ let t7283 = Py::None; if py::is(&t7282, &t7283) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7284 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7315 = (t7284).items()?;
        if t7315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7315[0].clone();
        v_v = t7315[1].clone();
        if ({ let t7285 = v_k.clone(); Py::Bool({ let t7286 = v_outputs.clone(); if !py::contains(&t7286, &t7285)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_732(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t733 = v_key.clone(); Py::Bool({ let t734 = v_memo.clone(); if !py::contains(&t734, &t733)? { true } else { false } }) }).truthy() {
        let t736: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t735 = v_inputs.clone(); f_cea_forward(py::getitem(&t735, &Py::str("lat"))?, py::getitem(&t735, &Py::str("lon"))?, py::getitem(&t735, &Py::str("lat_ts"))?, py::getitem(&t735, &Py::str("lon0"))?, py::getitem(&t735, &Py::str("R"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t736 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t737 = py::len(&v_memo.clone())?; Py::Bool({ let t738 = Py::Int(64i64); if py::gt(&t737, &t738)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t739 = v_want.clone(); Py::Bool({ let t740 = Py::None; if py::is(&t739, &t740) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t741 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t772 = (t741).items()?;
        if t772.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t772[0].clone();
        v_v = t772[1].clone();
        if ({ let t742 = v_k.clone(); Py::Bool({ let t743 = v_outputs.clone(); if !py::contains(&t743, &t742)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7371(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7372 = v_key.clone(); Py::Bool({ let t7373 = v_memo.clone(); if !py::contains(&t7373, &t7372)? { true } else { false } }) }).truthy() {
        let t7375: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7374 = v_inputs.clone(); f_trend_surface(py::getitem(&t7374, &Py::str("x"))?, py::getitem(&t7374, &Py::str("y"))?, py::getitem(&t7374, &Py::str("z"))?, py::getitem(&t7374, &Py::str("order"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7375 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7376 = py::len(&v_memo.clone())?; Py::Bool({ let t7377 = Py::Int(64i64); if py::gt(&t7376, &t7377)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7378 = v_want.clone(); Py::Bool({ let t7379 = Py::None; if py::is(&t7378, &t7379) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7380 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7411 = (t7380).items()?;
        if t7411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7411[0].clone();
        v_v = t7411[1].clone();
        if ({ let t7381 = v_k.clone(); Py::Bool({ let t7382 = v_outputs.clone(); if !py::contains(&t7382, &t7381)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7497(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7498 = v_key.clone(); Py::Bool({ let t7499 = v_memo.clone(); if !py::contains(&t7499, &t7498)? { true } else { false } }) }).truthy() {
        let t7501: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7500 = v_inputs.clone(); f_two_step_fca(py::getitem(&t7500, &Py::str("edges"))?, py::getitem(&t7500, &Py::str("population"))?, py::getitem(&t7500, &Py::str("supply_nodes"))?, py::getitem(&t7500, &Py::str("capacity"))?, py::getitem(&t7500, &Py::str("threshold"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7501 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7502 = py::len(&v_memo.clone())?; Py::Bool({ let t7503 = Py::Int(64i64); if py::gt(&t7502, &t7503)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7504 = v_want.clone(); Py::Bool({ let t7505 = Py::None; if py::is(&t7504, &t7505) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7506 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7537 = (t7506).items()?;
        if t7537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7537[0].clone();
        v_v = t7537[1].clone();
        if ({ let t7507 = v_k.clone(); Py::Bool({ let t7508 = v_outputs.clone(); if !py::contains(&t7508, &t7507)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7610(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7611 = v_key.clone(); Py::Bool({ let t7612 = v_memo.clone(); if !py::contains(&t7612, &t7611)? { true } else { false } }) }).truthy() {
        let t7614: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7613 = v_inputs.clone(); f_utm_forward(py::getitem(&t7613, &Py::str("lat"))?, py::getitem(&t7613, &Py::str("lon"))?, py::getitem(&t7613, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7614 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7615 = py::len(&v_memo.clone())?; Py::Bool({ let t7616 = Py::Int(64i64); if py::gt(&t7615, &t7616)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7617 = v_want.clone(); Py::Bool({ let t7618 = Py::None; if py::is(&t7617, &t7618) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7619 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7650 = (t7619).items()?;
        if t7650.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7650[0].clone();
        v_v = t7650[1].clone();
        if ({ let t7620 = v_k.clone(); Py::Bool({ let t7621 = v_outputs.clone(); if !py::contains(&t7621, &t7620)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7699(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7700 = v_key.clone(); Py::Bool({ let t7701 = v_memo.clone(); if !py::contains(&t7701, &t7700)? { true } else { false } }) }).truthy() {
        let t7703: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7702 = v_inputs.clone(); f_utm_inverse(py::getitem(&t7702, &Py::str("easting"))?, py::getitem(&t7702, &Py::str("northing"))?, py::getitem(&t7702, &Py::str("zone"))?, py::getitem(&t7702, &Py::str("hemisphere"))?, py::getitem(&t7702, &Py::str("ellipsoid"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7703 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7704 = py::len(&v_memo.clone())?; Py::Bool({ let t7705 = Py::Int(64i64); if py::gt(&t7704, &t7705)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7706 = v_want.clone(); Py::Bool({ let t7707 = Py::None; if py::is(&t7706, &t7707) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7708 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7739 = (t7708).items()?;
        if t7739.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7739[0].clone();
        v_v = t7739[1].clone();
        if ({ let t7709 = v_k.clone(); Py::Bool({ let t7710 = v_outputs.clone(); if !py::contains(&t7710, &t7709)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7784(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7785 = v_key.clone(); Py::Bool({ let t7786 = v_memo.clone(); if !py::contains(&t7786, &t7785)? { true } else { false } }) }).truthy() {
        let t7788: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7787 = v_inputs.clone(); f_utm_zone(py::getitem(&t7787, &Py::str("lat"))?, py::getitem(&t7787, &Py::str("lon"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7788 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7789 = py::len(&v_memo.clone())?; Py::Bool({ let t7790 = Py::Int(64i64); if py::gt(&t7789, &t7790)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7791 = v_want.clone(); Py::Bool({ let t7792 = Py::None; if py::is(&t7791, &t7792) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7793 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7824 = (t7793).items()?;
        if t7824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7824[0].clone();
        v_v = t7824[1].clone();
        if ({ let t7794 = v_k.clone(); Py::Bool({ let t7795 = v_outputs.clone(); if !py::contains(&t7795, &t7794)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7934(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7935 = v_key.clone(); Py::Bool({ let t7936 = v_memo.clone(); if !py::contains(&t7936, &t7935)? { true } else { false } }) }).truthy() {
        let t7938: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7937 = v_inputs.clone(); f_variogram_fit(py::getitem(&t7937, &Py::str("h"))?, py::getitem(&t7937, &Py::str("gamma"))?, py::getitem(&t7937, &Py::str("model"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7938 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7939 = py::len(&v_memo.clone())?; Py::Bool({ let t7940 = Py::Int(64i64); if py::gt(&t7939, &t7940)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7941 = v_want.clone(); Py::Bool({ let t7942 = Py::None; if py::is(&t7941, &t7942) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7943 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7974 = (t7943).items()?;
        if t7974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7974[0].clone();
        v_v = t7974[1].clone();
        if ({ let t7944 = v_k.clone(); Py::Bool({ let t7945 = v_outputs.clone(); if !py::contains(&t7945, &t7944)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8006(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t8007 = v_key.clone(); Py::Bool({ let t8008 = v_memo.clone(); if !py::contains(&t8008, &t8007)? { true } else { false } }) }).truthy() {
        let t8010: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t8009 = v_inputs.clone(); f_vegetation_indices(py::getitem(&t8009, &Py::str("red"))?, py::getitem(&t8009, &Py::str("nir"))?, py::getitem(&t8009, &Py::str("blue"))?, py::getitem(&t8009, &Py::str("L"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t8010 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t8011 = py::len(&v_memo.clone())?; Py::Bool({ let t8012 = Py::Int(64i64); if py::gt(&t8011, &t8012)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t8013 = v_want.clone(); Py::Bool({ let t8014 = Py::None; if py::is(&t8013, &t8014) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t8015 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8046 = (t8015).items()?;
        if t8046.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8046[0].clone();
        v_v = t8046[1].clone();
        if ({ let t8016 = v_k.clone(); Py::Bool({ let t8017 = v_outputs.clone(); if !py::contains(&t8017, &t8016)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8068(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t8069 = v_key.clone(); Py::Bool({ let t8070 = v_memo.clone(); if !py::contains(&t8070, &t8069)? { true } else { false } }) }).truthy() {
        let t8072: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t8071 = v_inputs.clone(); f_water_indices(py::getitem(&t8071, &Py::str("green"))?, py::getitem(&t8071, &Py::str("nir"))?, py::getitem(&t8071, &Py::str("swir1"))?, py::getitem(&t8071, &Py::str("swir2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t8072 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t8073 = py::len(&v_memo.clone())?; Py::Bool({ let t8074 = Py::Int(64i64); if py::gt(&t8073, &t8074)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t8075 = v_want.clone(); Py::Bool({ let t8076 = Py::None; if py::is(&t8075, &t8076) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t8077 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8108 = (t8077).items()?;
        if t8108.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8108[0].clone();
        v_v = t8108[1].clone();
        if ({ let t8078 = v_k.clone(); Py::Bool({ let t8079 = v_outputs.clone(); if !py::contains(&t8079, &t8078)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8186(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t8187 = v_key.clone(); Py::Bool({ let t8188 = v_memo.clone(); if !py::contains(&t8188, &t8187)? { true } else { false } }) }).truthy() {
        let t8190: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t8189 = v_inputs.clone(); f_watershed(py::getitem(&t8189, &Py::str("dem"))?, py::getitem(&t8189, &Py::str("cell_size"))?, py::getitem(&t8189, &Py::str("row"))?, py::getitem(&t8189, &Py::str("col"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t8190 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t8191 = py::len(&v_memo.clone())?; Py::Bool({ let t8192 = Py::Int(64i64); if py::gt(&t8191, &t8192)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t8193 = v_want.clone(); Py::Bool({ let t8194 = Py::None; if py::is(&t8193, &t8194) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t8195 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8226 = (t8195).items()?;
        if t8226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8226[0].clone();
        v_v = t8226[1].clone();
        if ({ let t8196 = v_k.clone(); Py::Bool({ let t8197 = v_outputs.clone(); if !py::contains(&t8197, &t8196)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8250(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t8251 = v_key.clone(); Py::Bool({ let t8252 = v_memo.clone(); if !py::contains(&t8252, &t8251)? { true } else { false } }) }).truthy() {
        let t8254: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t8253 = v_inputs.clone(); f_web_map_tile(py::getitem(&t8253, &Py::str("lat"))?, py::getitem(&t8253, &Py::str("lon"))?, py::getitem(&t8253, &Py::str("zoom"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t8254 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t8255 = py::len(&v_memo.clone())?; Py::Bool({ let t8256 = Py::Int(64i64); if py::gt(&t8255, &t8256)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t8257 = v_want.clone(); Py::Bool({ let t8258 = Py::None; if py::is(&t8257, &t8258) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t8259 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8290 = (t8259).items()?;
        if t8290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8290[0].clone();
        v_v = t8290[1].clone();
        if ({ let t8260 = v_k.clone(); Py::Bool({ let t8261 = v_outputs.clone(); if !py::contains(&t8261, &t8260)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8321(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t8322 = v_key.clone(); Py::Bool({ let t8323 = v_memo.clone(); if !py::contains(&t8323, &t8322)? { true } else { false } }) }).truthy() {
        let t8325: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t8324 = v_inputs.clone(); f_web_mercator_forward(py::getitem(&t8324, &Py::str("lat"))?, py::getitem(&t8324, &Py::str("lon"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t8325 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t8326 = py::len(&v_memo.clone())?; Py::Bool({ let t8327 = Py::Int(64i64); if py::gt(&t8326, &t8327)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t8328 = v_want.clone(); Py::Bool({ let t8329 = Py::None; if py::is(&t8328, &t8329) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t8330 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8361 = (t8330).items()?;
        if t8361.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8361[0].clone();
        v_v = t8361[1].clone();
        if ({ let t8331 = v_k.clone(); Py::Bool({ let t8332 = v_outputs.clone(); if !py::contains(&t8332, &t8331)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8382(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t8383 = v_key.clone(); Py::Bool({ let t8384 = v_memo.clone(); if !py::contains(&t8384, &t8383)? { true } else { false } }) }).truthy() {
        let t8386: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t8385 = v_inputs.clone(); f_web_mercator_inverse(py::getitem(&t8385, &Py::str("x"))?, py::getitem(&t8385, &Py::str("y"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t8386 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t8387 = py::len(&v_memo.clone())?; Py::Bool({ let t8388 = Py::Int(64i64); if py::gt(&t8387, &t8388)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t8389 = v_want.clone(); Py::Bool({ let t8390 = Py::None; if py::is(&t8389, &t8390) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t8391 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8422 = (t8391).items()?;
        if t8422.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8422[0].clone();
        v_v = t8422[1].clone();
        if ({ let t8392 = v_k.clone(); Py::Bool({ let t8393 = v_outputs.clone(); if !py::contains(&t8393, &t8392)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_841(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t842 = v_key.clone(); Py::Bool({ let t843 = v_memo.clone(); if !py::contains(&t843, &t842)? { true } else { false } }) }).truthy() {
        let t845: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t844 = v_inputs.clone(); f_flow_accumulation(py::getitem(&t844, &Py::str("dem"))?, py::getitem(&t844, &Py::str("cell_size"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t845 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t846 = py::len(&v_memo.clone())?; Py::Bool({ let t847 = Py::Int(64i64); if py::gt(&t846, &t847)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t848 = v_want.clone(); Py::Bool({ let t849 = Py::None; if py::is(&t848, &t849) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t850 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t881 = (t850).items()?;
        if t881.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t881[0].clone();
        v_v = t881[1].clone();
        if ({ let t851 = v_k.clone(); Py::Bool({ let t852 = v_outputs.clone(); if !py::contains(&t852, &t851)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8457(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t8458 = v_key.clone(); Py::Bool({ let t8459 = v_memo.clone(); if !py::contains(&t8459, &t8458)? { true } else { false } }) }).truthy() {
        let t8461: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t8460 = v_inputs.clone(); f_weighted_mean_center(py::getitem(&t8460, &Py::str("x"))?, py::getitem(&t8460, &Py::str("y"))?, py::getitem(&t8460, &Py::str("w"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t8461 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t8462 = py::len(&v_memo.clone())?; Py::Bool({ let t8463 = Py::Int(64i64); if py::gt(&t8462, &t8463)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t8464 = v_want.clone(); Py::Bool({ let t8465 = Py::None; if py::is(&t8464, &t8465) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t8466 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8497 = (t8466).items()?;
        if t8497.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8497[0].clone();
        v_v = t8497[1].clone();
        if ({ let t8467 = v_k.clone(); Py::Bool({ let t8468 = v_outputs.clone(); if !py::contains(&t8468, &t8467)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_923(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t924 = v_key.clone(); Py::Bool({ let t925 = v_memo.clone(); if !py::contains(&t925, &t924)? { true } else { false } }) }).truthy() {
        let t927: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t926 = v_inputs.clone(); f_d8_flow_direction(py::getitem(&t926, &Py::str("dem"))?, py::getitem(&t926, &Py::str("cell_size"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t927 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t928 = py::len(&v_memo.clone())?; Py::Bool({ let t929 = Py::Int(64i64); if py::gt(&t928, &t929)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t930 = v_want.clone(); Py::Bool({ let t931 = Py::None; if py::is(&t930, &t931) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t932 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t963 = (t932).items()?;
        if t963.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t963[0].clone();
        v_v = t963[1].clone();
        if ({ let t933 = v_k.clone(); Py::Bool({ let t934 = v_outputs.clone(); if !py::contains(&t934, &t933)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_989(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t990 = v_key.clone(); Py::Bool({ let t991 = v_memo.clone(); if !py::contains(&t991, &t990)? { true } else { false } }) }).truthy() {
        let t993: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t992 = v_inputs.clone(); f_decimal_to_dms(py::getitem(&t992, &Py::str("decimal"))?, py::getitem(&t992, &Py::str("axis"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t993 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t994 = py::len(&v_memo.clone())?; Py::Bool({ let t995 = Py::Int(64i64); if py::gt(&t994, &t995)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t996 = v_want.clone(); Py::Bool({ let t997 = Py::None; if py::is(&t996, &t997) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t998 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1029 = (t998).items()?;
        if t1029.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1029[0].clone();
        v_v = t1029[1].clone();
        if ({ let t999 = v_k.clone(); Py::Bool({ let t1000 = v_outputs.clone(); if !py::contains(&t1000, &t999)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
