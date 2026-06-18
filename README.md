# opencv-mobile-rs

[English](./README.md) | [中文](./README.zh-CN.md)
Rust bindings for [opencv-mobile](https://github.com/nihui/opencv-mobile) https://github.com/nihui/opencv-mobile , a lightweight build of OpenCV optimized for mobile and embedded devices.

## Features

- **Core Operations**: Mat creation, cloning, property access (width, height, channels, data)
- **Image Processing**: resize, blur, gaussian blur, median blur, bilateral filter, threshold, morphology operations
- **Color Conversion**: BGR2GRAY, BGR2RGB, BGR2HSV, BGR2YUV, etc.
- **Feature Detection**: ORB, SIFT, BFMatcher, draw keypoints/matches
- **Computer Vision**: Canny, Sobel, Laplacian, equalizeHist, warpAffine, rotate, flip, crop
- **Photo Module**: inpaint, fastNlMeansDenoising, edgePreservingFilter, stylization
- **Video**: Optical flow (Lucas-Kanade, Farneback), VideoCapture, VideoWriter
- **HighGUI**: imshow, waitKey
- **Additional**: CLAHE, adaptive threshold, distance transform, color maps, template matching, contours, Hough transforms

## Requirements

- Rust 1.56+
- C++ compiler (gcc/clang)
- CMake (for building opencv-mobile)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
opencv-mobile-rs = "0.1"
```

## Usage

```rust
use opencv_mobile_rs::*;

// Read an image
let img = imread("test.jpg", IMREAD_COLOR).expect("Failed to read image");

// Get image properties
let width = get_width(img);
let height = get_height(img);
let channels = get_channels(img);
println!("Image: {}x{} with {} channels", width, height, channels);

// Resize image
let resized = resize(img, 200, 200);

// Convert to grayscale
let gray = cvt_color(img, COLOR_BGR2GRAY);

// Apply Gaussian blur
let blurred = gaussian_blur(gray, 5, 1.0);

// Save result
imwrite("output.jpg", blurred, 95);

// Clean up
free_mat(img);
free_mat(resized);
free_mat(gray);
free_mat(blurred);
```

## Building

By default, the crate builds opencv-mobile statically. You can also use dynamic linking:

```toml
[dependencies]
opencv-mobile-rs = { version = "0.1", default-features = false, features = ["dynamic"] }
```

## License

Apache License 2.0 - see [LICENSE](LICENSE) file.
