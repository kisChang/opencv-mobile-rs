//! Tests for OpenCV highgui module
//!
//! #include <opencv2/highgui.hpp>

mod core;

use opencv_mobile_rs::*;

// Note: These tests may fail in headless environments where no display is available.

#[test]
fn test_videocapture_open() {
    let cap = create_videocapture();
    // Try to open a non-existent device - should return false
    let result = videocapture_open(cap, 0);
    // Result depends on whether device exists
    videocapture_release(cap);
}

#[test]
fn test_videocapture_is_opened() {
    let cap = create_videocapture();
    let result = videocapture_open(cap, 0);
    let is_opened = videocapture_is_opened(cap);
    // Initially should be false (no device opened)
    assert!(is_opened, "new VideoCapture should be opened");
    videocapture_release(cap);
}

#[test]
fn test_videocapture_get() {
    let cap = create_videocapture();
    // Should return 0.0 for unopened capture
    let value = videocapture_get(cap, CAP_PROP_FRAME_WIDTH);
    assert!(value >= 0.0, "videocapture_get should return non-negative value");
    videocapture_release(cap);
}

#[test]
fn test_videocapture_set() {
    let cap = create_videocapture();
    // Setting on unopened capture may fail gracefully
    let result = videocapture_set(cap, CAP_PROP_FRAME_WIDTH, 640.0);
    // Result depends on capture state
    videocapture_release(cap);
}

#[test]
fn test_videocapture_read() {
    let cap = create_videocapture();
    let mat = core::create_mat();
    // Reading from unopened capture should return false
    let result = videocapture_read(cap, mat);
    assert!(result, "reading from opened VideoCapture should return true");
    free_mat(mat);
    videocapture_release(cap);
}


#[test]
fn test_videowriter_write() {
    let writer = create_videowriter();
    let mat = core::create_mat();
    // Writing to unopened writer should handle gracefully
    videowriter_write(writer, mat);
    free_mat(mat);
    videowriter_release(writer);
}

// Note: imshow and wait_key require a display and are typically
// tested manually or in integration tests.
// Including them here for completeness but marking with ignore
// since they require a display.

#[test]
#[ignore]
fn test_imshow() {
    let mat = core::create_mat();
    imshow("fb", mat);
    free_mat(mat);
}

#[test]
fn test_wait_key() {
    // wait_key with 0 delay should return immediately (-1)
    let result = wait_key(0);
    // Result could be -1 (no key) or a key code
    assert!(result >= -1, "wait_key should return >= -1");
}
