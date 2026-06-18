//! Tests for OpenCV video module
//!
//! #include <opencv2/video.hpp>

mod core;
use opencv_mobile_rs::*;

#[test]
fn test_calc_optical_flow_pyr_lk() {
    let prev_img = core::create_mat();
    let next_img = core::create_mat();
    let prev_pts = core::create_mat();
    let next_pts = core::create_mat();

    let result = calc_optical_flow_pyr_lk(
        prev_img,
        next_img,
        prev_pts,
        next_pts,
        15,      // win_size
        3,       // max_level
        0,       // criteria_type
        30,      // max_iter
        0.01,    // epsilon
    );

    assert!(!result.is_null(), "calc_optical_flow_pyr_lk should return non-null pointer");

    free_mat(prev_img);
    free_mat(next_img);
    free_mat(prev_pts);
    free_mat(next_pts);
    free_mat(result);
}

#[test]
fn test_calc_optical_flow_farneback() {
    let prev = core::create_mat();
    let next = core::create_mat();

    let result = calc_optical_flow_farneback(
        prev,
        next,
        0.5,     // pyr_scale
        3,       // levels
        15,      // win_size
        3,       // iterations
        5,       // poly_n
        1.1,     // poly_sigma
        0,       // flags
    );

    assert!(!result.is_null(), "calc_optical_flow_farneback should return non-null pointer");

    free_mat(prev);
    free_mat(next);
    free_mat(result);
}
