//! Tests for OpenCV photo module
//!
//! #include <opencv2/photo.hpp>

mod core;
use opencv_mobile_rs::*;

#[test]
fn test_inpaint() {
    let src = core::create_mat();
    let inpaint_mask = core::create_mat();
    let result = inpaint(src, inpaint_mask, 3.0, 0);
    assert!(!result.is_null(), "inpaint should return non-null pointer");
    free_mat(src);
    free_mat(inpaint_mask);
    free_mat(result);
}

#[test]
fn test_fast_nl_means_denoising() {
    let src = core::create_mat();
    let result = fast_nl_means_denoising(src, 10.0, 7, 21);
    assert!(!result.is_null(), "fast_nl_means_denoising should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_edge_preserving_filter() {
    let src = core::create_mat();
    let result = edge_preserving_filter(src, 1, 60.0, 0.4);
    assert!(!result.is_null(), "edge_preserving_filter should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_stylization() {
    let src = core::create_mat();
    let result = stylization(src, 60.0, 0.4);
    assert!(!result.is_null(), "stylization should return non-null pointer");
    free_mat(src);
    free_mat(result);
}
