//! Tests for OpenCV core module
//!
//! #include <opencv2/core.hpp>

use std::ptr::null;
use opencv_mobile_rs::*;

pub fn create_mat() -> Mat {
    opencv_mobile_rs::create_mat(120, 120, opencv_mobile_rs::CV_8UC1, None)
}

#[test]
fn test_create_mat() {
    let mat = create_mat();
    assert!(!mat.is_null(), "create_mat should return non-null pointer");
    free_mat(mat);
}

#[test]
fn test_free_mat() {
    let mat = create_mat();
    free_mat(mat);
    // If we get here without panicking, the test passes
}

#[test]
fn test_clone_mat() {
    let mat = create_mat();
    let cloned = clone_mat(mat);
    assert!(!cloned.is_null(), "clone_mat should return non-null pointer");
    free_mat(mat);
    free_mat(cloned);
}

#[test]
fn test_get_channels() {
    let mat = create_mat();
    let channels = get_channels(mat);
    // Mat should have at least 1 channel
    assert!(channels >= 1, "channels should be >= 1, got {}", channels);
    free_mat(mat);
}

#[test]
fn test_get_width() {
    let mat = create_mat();
    let width = get_width(mat);
    // Width should be > 0
    assert!(width > 0, "width should be > 0, got {}", width);
    free_mat(mat);
}

#[test]
fn test_get_height() {
    let mat = create_mat();
    let height = get_height(mat);
    // Height should be > 0
    assert!(height > 0, "height should be > 0, got {}", height);
    free_mat(mat);
}

#[test]
fn test_get_data() {
    let mat = create_mat();
    let _data = get_data(mat);
    // Data pointer may be null for empty mats
    free_mat(mat);
}

#[test]
fn test_add() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = add(mat1, mat2, std::ptr::null_mut());
    assert!(!result.is_null(), "add should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_subtract() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = subtract(mat1, mat2);
    assert!(!result.is_null(), "subtract should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_multiply() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = multiply(mat1, mat2, 1.0);
    assert!(!result.is_null(), "multiply should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_divide() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = divide(mat1, mat2, 1.0);
    assert!(!result.is_null(), "divide should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_normalize() {
    let mat = create_mat();
    let result = normalize(mat, 0.0, 255.0, NORM_MINMAX, -1);
    assert!(!result.is_null(), "normalize should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_min_max_loc() {
    let mat = create_mat();
    let result = min_max_loc(mat);
    assert!(result.is_some(), "min_max_loc should return Some result");
    free_mat(mat);
}

#[test]
fn test_split() {
    let mat = create_mat();
    let (channels, count) = split(mat);
    assert!(count >= 1, "split should return at least 1 channel");
    for ch in channels {
        free_mat(ch);
    }
    free_mat(mat);
}

#[test]
fn test_convert_scale_abs() {
    let mat = create_mat();
    let result = convert_scale_abs(mat, 1.0, 0.0);
    assert!(!result.is_null(), "convert_scale_abs should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_absdiff() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = absdiff(mat1, mat2);
    assert!(!result.is_null(), "absdiff should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_bitwise_and() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = bitwise_and(mat1, mat2);
    assert!(!result.is_null(), "bitwise_and should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_bitwise_or() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = bitwise_or(mat1, mat2);
    assert!(!result.is_null(), "bitwise_or should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_bitwise_xor() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = bitwise_xor(mat1, mat2);
    assert!(!result.is_null(), "bitwise_xor should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_bitwise_not() {
    let mat = create_mat();
    let result = bitwise_not(mat);
    assert!(!result.is_null(), "bitwise_not should return non-null pointer");
    free_mat(mat);
    free_mat(result);
}

#[test]
fn test_compare() {
    let mat1 = create_mat();
    let mat2 = create_mat();
    let result = compare(mat1, mat2, CMP_EQ);
    assert!(!result.is_null(), "compare should return non-null pointer");
    free_mat(mat1);
    free_mat(mat2);
    free_mat(result);
}

#[test]
fn test_in_range() {
    let src = create_mat();
    let lowerb = create_mat();
    let upperb = create_mat();
    let result = in_range(src, lowerb, upperb);
    assert!(!result.is_null(), "in_range should return non-null pointer");
    free_mat(src);
    free_mat(lowerb);
    free_mat(upperb);
    free_mat(result);
}

#[test]
fn test_copy_make_border() {
    let src = create_mat();
    let result = copy_make_border(src, 10, 10, 10, 10, BORDER_CONSTANT, None);
    assert!(!result.is_null(), "copy_make_border should return non-null pointer");
    free_mat(src);
    free_mat(result);
}

#[test]
fn test_create_font() {
    let font = create_font();
    assert!(!font.is_null(), "create_font should return non-null pointer");
    destroy_font(font);
}

#[test]
fn test_destroy_font() {
    let font = create_font();
    destroy_font(font);
}

#[test]
fn test_get_text_size() {
    let font = create_font();
    let (width, height) = get_text_size(font, "Hello", 1.0);
    assert!(width >= 0, "text width should be >= 0");
    assert!(height >= 0, "text height should be >= 0");
    destroy_font(font);
}

#[test]
fn test_put_text() {
    let mat = create_mat();
    let font = create_font();
    put_text(mat, font, "Test", 10, 10, 255, 255, 255, 1.0);
    destroy_font(font);
    free_mat(mat);
}

#[test]
fn test_imread() {
    // Test with a non-existent file - should return None
    let result = imread("tests/nonexistent_file_xyz.jpg", IMREAD_COLOR);
    assert!(result.is_none(), "imread with invalid file should return None");

    // Test with existing file - should return Some
    let result2 = imread("tests/test.jpg", IMREAD_COLOR);
    assert!(result2.is_some(), "imread with valid file should return Some");
    if let Some(mat) = result2 {
        free_mat(mat);
    }
}

#[test]
fn test_imwrite() {
    let mat = create_mat();
    // Writing to /tmp should succeed or fail gracefully
    let _result = imwrite("/tmp/test_opencv_rs.jpg", mat, 95);
    // Result may be true or false depending on OpenCV build
    free_mat(mat);
}

#[test]
fn test_merge() {
    // Test merge with null and zero count
    let result = unsafe { ocv_merge(std::ptr::null(), 0) };
    assert!(result.is_null(), "merge with null inputs should return null");
}
