//! Geospatial Analytics: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p geospatial-analytics-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&geospatial_analytics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "geo_map_scale_ground_distance", "inputs": {"map_distance": "4.2 cm", "scale": "50000"}}));
    println!();
    show(json!({"key": "geo_decimal_to_dms", "inputs": {"decimal": "51.477222", "axis": "latitude"}}));
    println!();
    show(json!({"key": "geo_auxiliary_latitudes", "inputs": {"lat": "45", "ellipsoid": "WGS84"}}));
    println!();
    show(json!({"key": "geo_parallel_arc", "inputs": {"lat": "60", "lon1": "0", "lon2": "1", "ellipsoid": "WGS84"}}));
    println!();
    show(json!({"key": "geo_closest_facility", "inputs": {"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1; 3-4 4", "facilities": "0, 3"}}));
    println!();
    show(json!({"key": "geo_empirical_semivariogram", "inputs": {"x": [0, 1, 2, 3, 4], "y": [0, 0, 0, 0, 0], "z": [1, 3, 2, 5, 4], "bin_edges": [0, 1.5, 2.5, 4.5]}}));
    println!();
    show(json!({"key": "geo_albers_equal_area_forward", "inputs": {"lat": "40", "lon": "-100", "lat1": "29.5", "lat2": "45.5", "lat0": "23", "lon0": "-96", "ellipsoid": "GRS80"}}));
    println!();
    show(json!({"key": "geo_gravity_accessibility", "inputs": {"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1", "opportunities": "100, 50, 80, 20", "beta": "1", "impedance": "power"}}));
    println!();
    show(json!({"key": "geo_discrete_frechet_distance", "inputs": {"x1": [0, 1, 2], "y1": [0, 0, 0], "x2": [0, 2], "y2": [1, 1]}}));
    println!();
    show(json!({"key": "geo_kernel_density", "inputs": {"x": [0, 1, 2, 5], "y": [0, 1, 0, 5], "px": "1", "py": "0.5", "bandwidth": "2", "kernel": "gaussian"}}));
    println!();
    show(json!({"key": "geo_brightness_temperature", "inputs": {"radiance": "10.5", "K1": "774.8853", "K2": "1321.0789"}}));
    println!();
    show(json!({"key": "geo_bivariate_morans_i", "inputs": {"x": [8, 6, 6, 3, 2, 1], "y": [7, 7, 5, 4, 1, 2], "W": [[0, 1, 0, 0, 0, 0], [1, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0], [0, 0, 1, 0, 1, 0], [0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 1, 0]]}}));
    println!();
    show(json!({"key": "geo_d8_flow_accumulation", "inputs": {"dem": [[78, 72, 69, 71], [74, 67, 56, 49], [69, 53, 44, 37], [64, 58, 55, 22]], "cell_size": "10 m"}}));
    println!();
    show(json!({"key": "geo_huff_model", "inputs": {"edges": "0-1 2; 1-2 3; 0-2 6; 2-3 1; 1-3 5", "origin": "0", "stores": "1, 2, 3", "attractiveness": "5000, 12000, 8000", "alpha": "1", "beta": "2"}}));
    println!();
    show(json!({"key": "geo_map_scale_from_distances", "inputs": {"map_distance": "5 cm", "ground_distance": "2.5 km"}}));
}
