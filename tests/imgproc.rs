//! Tests for OpenCV imgproc module
//!
//! #include <opencv2/imgproc.hpp>

mod core;
use opencv_mobile_rs::*;

#[test]
fn test_cvt_color() {
    let mat = core::create_mat();
    let result = cvt_color(mat, COLOR_BGR2GRAY);
    assert!(!result.is_null(), "cvt_color should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_resize() {
    let mat = core::create_mat();
    let result = resize(mat, 100, 100);
    assert!(!result.is_null(), "resize should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_blur() {
    let mat = core::create_mat();
    let result = blur(mat, 5, 5);
    assert!(!result.is_null(), "blur should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_gaussian_blur() {
    let mat = core::create_mat();
    let result = gaussian_blur(mat, 5, 1.0);
    assert!(!result.is_null(), "gaussian_blur should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_median_blur() {
    let mat = core::create_mat();
    let result = median_blur(mat, 5);
    assert!(!result.is_null(), "median_blur should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_bilateral_filter() {
    let mat = core::create_mat();
    let result = bilateral_filter(mat, 9, 75.0, 75.0);
    assert!(!result.is_null(), "bilateral_filter should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_threshold() {
    let mat = core::create_mat();
    let result = threshold(mat, 128.0, 255.0, THRESH_BINARY);
    assert!(!result.is_null(), "threshold should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_erode() {
    let mat = core::create_mat();
    let result = erode(mat, 3);
    assert!(!result.is_null(), "erode should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_dilate() {
    let mat = core::create_mat();
    let result = dilate(mat, 3);
    assert!(!result.is_null(), "dilate should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_morphology_open() {
    let mat = core::create_mat();
    let result = morphology_open(mat, 3);
    assert!(!result.is_null(), "morphology_open should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_morphology_close() {
    let mat = core::create_mat();
    let result = morphology_close(mat, 3);
    assert!(!result.is_null(), "morphology_close should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_canny() {
    let mat = core::create_mat();
    let result = canny(mat, 50.0, 150.0);
    assert!(!result.is_null(), "canny should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_sobel() {
    let mat = core::create_mat();
    let result = sobel(mat, 1, 0, 3);
    assert!(!result.is_null(), "sobel should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_laplacian() {
    let mat = core::create_mat();
    let result = laplacian(mat, 3);
    assert!(!result.is_null(), "laplacian should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_equalize_hist() {
    let mat = core::create_mat();
    let result = equalize_hist(mat);
    assert!(!result.is_null(), "equalize_hist should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_warp_affine() {
    let mat = core::create_mat();
    let result = warp_affine(mat, 45.0, 1.0);
    assert!(!result.is_null(), "warp_affine should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_rotate() {
    let mat = core::create_mat();
    let result = rotate(mat, 90.0);
    assert!(!result.is_null(), "rotate should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_flip() {
    let mat = core::create_mat();
    let result = flip(mat, 0);
    assert!(!result.is_null(), "flip should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_crop() {
    let mat = core::create_mat();
    let result = crop(mat, 0, 0, 100, 100);
    assert!(!result.is_null(), "crop should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_pyr_down() {
    let mat = core::create_mat();
    let result = pyr_down(mat, 100, 100);
    assert!(!result.is_null(), "pyr_down should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_pyr_up() {
    let mat = core::create_mat();
    let result = pyr_up(mat, 200, 200);
    assert!(!result.is_null(), "pyr_up should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_get_gaussian_kernel() {
    let kernel = get_gaussian_kernel(5, 1.0, -1);
    assert!(!kernel.is_null(), "get_gaussian_kernel should return non-null pointer");
    free_mat(kernel);
}

#[test]
fn test_get_structuring_element() {
    let kernel = get_structuring_element(0, 3, 3, -1, -1);
    assert!(!kernel.is_null(), "get_structuring_element should return non-null pointer");
    free_mat(kernel);
}

#[test]
fn test_filter2d() {
    let src = core::create_mat();
    let kernel = get_gaussian_kernel(3, 1.0, -1);
    let result = filter2d(src, -1, kernel, 0.0);
    assert!(!result.is_null(), "filter2d should return non-null pointer");
    free_mat(src);
    free_mat(kernel);
    free_mat(result);
}

#[test]
fn test_sep_filter2d() {
    let src = core::create_mat();
    let kernel_x = get_gaussian_kernel(3, 1.0, -1);
    let kernel_y = get_gaussian_kernel(3, 1.0, -1);
    let result = sep_filter2d(src, -1, kernel_x, kernel_y, 0.0);
    assert!(!result.is_null(), "sep_filter2d should return non-null pointer");
    free_mat(src);
    free_mat(kernel_x);
    free_mat(kernel_y);
    free_mat(result);
}

#[test]
fn test_scharr() {
    let src = core::create_mat();
    let result = scharr(src, -1, 1, 0, 1.0, 0.0);
    assert!(!result.is_null(), "scharr should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_corner_harris() {
    let src = core::create_mat();
    let result = corner_harris(src, 2, 3, 0.04, BORDER_REFLECT);
    assert!(!result.is_null(), "corner_harris should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_good_features_to_track() {
    let src = core::create_mat();
    let result = good_features_to_track(src, 100, 0.01, 10.0, 3, false, 0.04);
    assert!(!result.is_null(), "good_features_to_track should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_hough_lines() {
    let src = core::create_mat();
    let edges = canny(src, 50.0, 150.0);
    let result = hough_lines(edges, 1.0, 3.14159 / 180.0, 100, 0.0, 0.0);
    assert!(!result.is_null(), "hough_lines should return non-null pointer");
    free_mat(src);
    free_mat(edges);
    free_mat(result);
}

#[test]
fn test_hough_circles() {
    let src = core::create_mat();
    let result = hough_circles(src, 3, 1.0, 10.0, 100.0, 100.0, 0, 30);
    assert!(!result.is_null(), "hough_circles should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_morphology_ex() {
    let src = core::create_mat();
    let kernel = get_structuring_element(0, 3, 3, -1, -1);
    let result = morphology_ex(src, MORPH_GRADIENT, kernel);
    assert!(!result.is_null(), "morphology_ex should return non-null pointer");
    free_mat(src);
    free_mat(kernel);
    free_mat(result);
}

#[test]
fn test_get_rotation_matrix2d() {
    let result = get_rotation_matrix2d(100.0, 100.0, 45.0, 1.0);
    assert!(!result.is_null(), "get_rotation_matrix2d should return non-null pointer");
    free_mat(result);
}

#[test]
fn test_warp_perspective() {
    let src = core::create_mat();
    let m = get_rotation_matrix2d(100.0, 100.0, 45.0, 1.0);
    let result = warp_perspective(src, m, 200, 200, 0, BORDER_CONSTANT, 0.0);
    assert!(!result.is_null(), "warp_perspective should return non-null pointer");
    free_mat(src);
    free_mat(m);
    free_mat(result);
}

#[test]
fn test_get_affine_transform() {
    // This test requires valid point mats, which we don't have in unit tests
    // Just verify the function signature works (would return null with invalid input)
    let src = core::create_mat();
    let dst = core::create_mat();
    let result = get_affine_transform(src, dst);
    // May return null with invalid input, but shouldn't panic
    free_mat(src);
    free_mat(dst);
    if !result.is_null() {
        free_mat(result);
    }
}

#[test]
fn test_get_perspective_transform() {
    let src = core::create_mat();
    let dst = core::create_mat();
    let result = get_perspective_transform(src, dst);
    // May return null with invalid input
    free_mat(src);
    free_mat(dst);
    if !result.is_null() {
        free_mat(result);
    }
}

#[test]
fn test_invert_affine_transform() {
    let m = get_rotation_matrix2d(100.0, 100.0, 45.0, 1.0);
    let result = invert_affine_transform(m);
    assert!(!result.is_null(), "invert_affine_transform should return non-null pointer");
    free_mat(m);
    free_mat(result);
}

#[test]
fn test_integral() {
    let src = core::create_mat();
    let result = integral(src, -1);
    assert!(!result.is_null(), "integral should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_create_clahe() {
    let clahe = create_clahe(2.0, 8, 8);
    assert!(!clahe.is_null(), "create_clahe should return non-null pointer");
}

#[test]
fn test_apply_clahe() {
    let clahe = create_clahe(2.0, 8, 8);
    let src = core::create_mat();
    let result = apply_clahe(clahe, src, 2.0);
    assert!(!result.is_null(), "apply_clahe should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_adaptive_threshold() {
    let src = core::create_mat();
    let result = adaptive_threshold(src, 255.0, 0, THRESH_BINARY, 11, 2.0);
    assert!(!result.is_null(), "adaptive_threshold should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_distance_transform() {
    let src = core::create_mat();
    let result = distance_transform(src, 2, 3);
    assert!(!result.is_null(), "distance_transform should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_apply_color_map() {
    let src = core::create_mat();
    let result = apply_color_map(src, COLORMAP_JET);
    assert!(!result.is_null(), "apply_color_map should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_match_template() {
    let image = core::create_mat();
    let templ = core::create_mat();
    let result = match_template(image, templ, TM_SQDIFF);
    assert!(!result.is_null(), "match_template should return non-null pointer");
    free_mat(image);
    free_mat(templ);
    free_mat(result);
}

#[test]
fn test_find_contours() {
    let image = core::create_mat();
    let edges = canny(image, 50.0, 150.0);
    let result = find_contours(edges, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE);
    assert!(!result.is_null(), "find_contours should return non-null pointer");
    free_mat(image);
    free_mat(edges);
    free_mat(result);
}

#[test]
fn test_bounding_rect() {
    let points = core::create_mat();
    let result = bounding_rect(points);
    assert!(result.is_some(), "bounding_rect should return Some result");
    free_mat(points);
}

#[test]
fn test_min_area_rect() {
    let points = core::create_mat();
    let result = min_area_rect(points);
    assert!(result.is_some(), "min_area_rect should return Some result");
    free_mat(points);
}

#[test]
fn test_convex_hull() {
    let points = core::create_mat();
    let result = convex_hull(points, false);
    assert!(!result.is_null(), "convex_hull should return non-null pointer");
    free_mat(points);
    free_mat(result);
}

#[test]
fn test_approx_poly_dp() {
    let curve = core::create_mat();
    let result = approx_poly_dp(curve, 0.01, true);
    assert!(!result.is_null(), "approx_poly_dp should return non-null pointer");
    free_mat(curve);
    free_mat(result);
}
