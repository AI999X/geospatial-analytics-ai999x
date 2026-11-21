//! The ellipsoid is the one documented default convention of this model (owner, 2026-11-21): a call or question that
//! gives none is computed on WGS84, exactly as if WGS84 were given, and the answer says so in its assumptions.

use ax_engine::ask::ask_value;
use ax_engine::engine::solve_procedure;
use ax_engine::Input;
use geospatial_analytics_ai999x::pack;

fn midpoint(with_ellipsoid: Option<&str>) -> ax_engine::Answer {
    let mut given: Vec<(String, Input)> = vec![("lat1".into(), Input::Float(51.5)), ("lon1".into(), Input::Float(-0.13)),
                                              ("lat2".into(), Input::Float(40.71)), ("lon2".into(), Input::Float(-74.01))];
    if let Some(e) = with_ellipsoid {
        given.push(("ellipsoid".into(), Input::Text(e.into())));
    }
    solve_procedure(pack(), "geo_geodesic_midpoint", &given)
}

#[test]
fn solve_without_an_ellipsoid_is_wgs84_and_says_so() {
    let none = midpoint(None);
    let wgs = midpoint(Some("WGS84"));
    let grs = midpoint(Some("GRS80"));
    assert!(none.solved(), "{:?}", none.message);
    assert_eq!(format!("{:?}", none.values), format!("{:?}", wgs.values));
    assert!(none.assumptions.iter().any(|a| a == "ellipsoid: WGS84 (default)"), "{:?}", none.assumptions);
    assert!(!wgs.assumptions.iter().any(|a| a.contains("(default)")), "{:?}", wgs.assumptions);
    assert!(grs.solved());
    assert_ne!(format!("{:?}", grs.values), format!("{:?}", none.values));
}

#[test]
fn a_question_without_an_ellipsoid_is_answered_on_wgs84() {
    for q in ["What is the midpoint latitude of the geodesic from lat1 = 51.5, lon1 = -0.13 to lat2 = 40.71, lon2 = -74.01?",
              "Geodesic midpoint: lat1 = 51.5, lon1 = -0.13, lat2 = 40.71, lon2 = -74.01"] {
        let r = ask_value(pack(), q);
        assert_eq!(r["outcome"], "SOLVED", "{q}: {r}");
        let text = r.to_string();
        assert!(text.contains("52.38"), "{q}: no 52.385 in {r}");
        assert!(text.contains("ellipsoid: WGS84 (default)"), "{q}: {r}");
    }
}
