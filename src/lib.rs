//! OpenCV Rust bindings using C wrapper
//!
//! This crate provides bindings to OpenCV through a C wrapper (wrapper.cpp).

use std::os::raw::c_char;

// Re-export the Mat type (opaque pointer)
pub type Mat = *mut std::ffi::c_void;
pub type Font = *mut std::ffi::c_void;
pub type VideoCapture = *mut std::ffi::c_void;
pub type VideoWriter = *mut std::ffi::c_void;

// Common OpenCV image types
pub const CV_8UC1: i32 = 0;  // 8-bit unsigned, 1 channel
pub const CV_8UC3: i32 = 16; // 8-bit unsigned, 3 channels
pub const CV_8UC4: i32 = 24; // 8-bit unsigned, 4 channels
pub const CV_32FC1: i32 = 5; // 32-bit float, 1 channel
pub const CV_32FC3: i32 = 21; // 32-bit float, 3 channels

// Mat 基础操作
#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_create_mat(height: i32, width: i32, type_: i32, values: *mut f64) -> Mat;
    fn ocv_free(mat: Mat);
    fn ocv_clone(mat: Mat) -> Mat;
    fn ocv_cvtColor(mat: Mat, code: i32) -> Mat;
    fn ocv_resize(mat: Mat, width: i32, height: i32) -> Mat;
    fn ocv_blur(mat: Mat, ksize_width: i32, ksize_height: i32) -> Mat;
    fn ocv_gaussianBlur(mat: Mat, ksize: i32, sigma: f64) -> Mat;
    fn ocv_medianBlur(mat: Mat, ksize: i32) -> Mat;
    fn ocv_bilateralFilter(mat: Mat, d: i32, sigma_color: f64, sigma_space: f64) -> Mat;
    fn ocv_threshold(mat: Mat, thresh: f64, maxval: f64, type_: i32) -> Mat;
    fn ocv_erode(mat: Mat, ksize_width: i32, ksize_height: i32) -> Mat;
    fn ocv_dilate(mat: Mat, ksize_width: i32, ksize_height: i32) -> Mat;
    fn ocv_morphology_open(mat: Mat, ksize_width: i32, ksize_height: i32) -> Mat;
    fn ocv_morphology_close(mat: Mat, ksize_width: i32, ksize_height: i32) -> Mat;
    fn ocv_canny(mat: Mat, threshold1: f64, threshold2: f64) -> Mat;
    fn ocv_sobel(mat: Mat, dx: i32, dy: i32, ksize: i32) -> Mat;
    fn ocv_laplacian(mat: Mat, ksize: i32) -> Mat;
    fn ocv_equalizeHist(mat: Mat) -> Mat;
    fn ocv_warpAffine(mat: Mat, angle: f64, scale: f64) -> Mat;
    fn ocv_rotate(mat: Mat, angle: f64) -> Mat;
    fn ocv_flip(mat: Mat, mode: i32) -> Mat;
    fn ocv_crop(mat: Mat, x: i32, y: i32, width: i32, height: i32) -> Mat;
    fn ocv_merge(mats: *const *const std::ffi::c_void, count: i32) -> Mat;
}

// 属性获取
#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_get_channels(mat: Mat) -> i32;
    fn ocv_get_width(mat: Mat) -> i32;
    fn ocv_get_height(mat: Mat) -> i32;
    fn ocv_get_data(mat: Mat) -> *mut u8;
}

// 图像读写
#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_imread(filename: *const c_char, flags: i32) -> Mat;
    fn ocv_imwrite(filename: *const c_char, mat: Mat, quality: i32) -> i32;
}

// 文本绘制
#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_create_font() -> Font;
    fn ocv_destroy_font(font: Font);
    fn ocv_get_text_size(
        font: Font,
        text: *const c_char,
        font_scale: f64,
        width: *mut i32,
        height: *mut i32,
    );
    fn ocv_put_text(
        mat: Mat,
        font: Font,
        text: *const c_char,
        x: i32,
        y: i32,
        b: i32,
        g: i32,
        r: i32,
        font_scale: f64,
    );
}

// 辅助函数 - 使用 FFI 调用
pub fn create_mat(height: i32, width: i32, type_: i32, values: Option<&[f64; 4]>) -> Mat {
    unsafe {
        match values {
            Some(v) => ocv_create_mat(height, width, type_, v.as_ptr() as *mut f64),
            None => ocv_create_mat(height, width, type_, std::ptr::null_mut()),
        }
    }
}

pub fn free_mat(mat: Mat) {
    unsafe { ocv_free(mat) }
}

pub fn clone_mat(mat: Mat) -> Mat {
    unsafe { ocv_clone(mat) }
}

pub fn get_channels(mat: Mat) -> i32 {
    unsafe { ocv_get_channels(mat) }
}

pub fn get_width(mat: Mat) -> i32 {
    unsafe { ocv_get_width(mat) }
}

pub fn get_height(mat: Mat) -> i32 {
    unsafe { ocv_get_height(mat) }
}

pub fn get_data(mat: Mat) -> *mut u8 {
    unsafe { ocv_get_data(mat) }
}

pub fn cvt_color(mat: Mat, code: i32) -> Mat {
    unsafe { ocv_cvtColor(mat, code) }
}

pub fn resize(mat: Mat, width: i32, height: i32) -> Mat {
    unsafe { ocv_resize(mat, width, height) }
}

pub fn blur(mat: Mat, ksize_width: i32, ksize_height: i32) -> Mat {
    unsafe { ocv_blur(mat, ksize_width, ksize_height) }
}

pub fn gaussian_blur(mat: Mat, ksize: i32, sigma: f64) -> Mat {
    unsafe { ocv_gaussianBlur(mat, ksize, sigma) }
}

pub fn median_blur(mat: Mat, ksize: i32) -> Mat {
    unsafe { ocv_medianBlur(mat, ksize) }
}

pub fn bilateral_filter(mat: Mat, d: i32, sigma_color: f64, sigma_space: f64) -> Mat {
    unsafe { ocv_bilateralFilter(mat, d, sigma_color, sigma_space) }
}

pub fn threshold(mat: Mat, thresh: f64, maxval: f64, type_: i32) -> Mat {
    unsafe { ocv_threshold(mat, thresh, maxval, type_) }
}

pub fn erode(mat: Mat, ksize: i32) -> Mat {
    unsafe { ocv_erode(mat, ksize, ksize) }
}

pub fn dilate(mat: Mat, ksize: i32) -> Mat {
    unsafe { ocv_dilate(mat, ksize, ksize) }
}

pub fn morphology_open(mat: Mat, ksize: i32) -> Mat {
    unsafe { ocv_morphology_open(mat, ksize, ksize) }
}

pub fn morphology_close(mat: Mat, ksize: i32) -> Mat {
    unsafe { ocv_morphology_close(mat, ksize, ksize) }
}

pub fn canny(mat: Mat, threshold1: f64, threshold2: f64) -> Mat {
    unsafe { ocv_canny(mat, threshold1, threshold2) }
}

pub fn sobel(mat: Mat, dx: i32, dy: i32, ksize: i32) -> Mat {
    unsafe { ocv_sobel(mat, dx, dy, ksize) }
}

pub fn laplacian(mat: Mat, ksize: i32) -> Mat {
    unsafe { ocv_laplacian(mat, ksize) }
}

pub fn equalize_hist(mat: Mat) -> Mat {
    unsafe { ocv_equalizeHist(mat) }
}

pub fn warp_affine(mat: Mat, angle: f64, scale: f64) -> Mat {
    unsafe { ocv_warpAffine(mat, angle, scale) }
}

pub fn rotate(mat: Mat, angle: f64) -> Mat {
    unsafe { ocv_rotate(mat, angle) }
}

pub fn flip(mat: Mat, mode: i32) -> Mat {
    unsafe { ocv_flip(mat, mode) }
}

pub fn crop(mat: Mat, x: i32, y: i32, width: i32, height: i32) -> Mat {
    unsafe { ocv_crop(mat, x, y, width, height) }
}

pub fn merge(mats: *const *const std::ffi::c_void, count: i32) -> Mat {
    unsafe { ocv_merge(mats, count) }
}

pub fn imread(filename: &str, flags: i32) -> Option<Mat> {
    unsafe {
        let c_str = std::ffi::CString::new(filename).ok()?;
        let mat = ocv_imread(c_str.as_ptr(), flags);
        if mat.is_null() {
            None
        } else {
            Some(mat)
        }
    }
}

pub fn imwrite(filename: &str, mat: Mat, quality: i32) -> bool {
    unsafe {
        if let Ok(c_str) = std::ffi::CString::new(filename) {
            ocv_imwrite(c_str.as_ptr(), mat, quality) != 0
        } else {
            false
        }
    }
}

pub fn create_font() -> Font {
    unsafe { ocv_create_font() }
}

pub fn destroy_font(font: Font) {
    unsafe { ocv_destroy_font(font) }
}

pub fn get_text_size(font: Font, text: &str, font_scale: f64) -> (i32, i32) {
    unsafe {
        let c_str = std::ffi::CString::new(text).unwrap_or_default();
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        ocv_get_text_size(font, c_str.as_ptr(), font_scale, &mut width, &mut height);
        (width, height)
    }
}

pub fn put_text(
    mat: Mat,
    font: Font,
    text: &str,
    x: i32,
    y: i32,
    b: i32,
    g: i32,
    r: i32,
    font_scale: f64,
) {
    unsafe {
        let c_str = std::ffi::CString::new(text).unwrap_or_default();
        ocv_put_text(mat, font, c_str.as_ptr(), x, y, b, g, r, font_scale);
    }
}

// ===================== FEATURES2D =====================

pub type FeatureDetector = *mut std::ffi::c_void;
pub type DescriptorMatcher = *mut std::ffi::c_void;

#[link(name = "opencv_wrapper")]
extern "C" {
    // ORB
    fn ocv_create_orb(
        nfeatures: i32,
        scale_factor: f32,
        nlevels: i32,
        edge_threshold: i32,
        first_level: i32,
        wta_k: i32,
        score_type: i32,
        patch_size: i32,
        fast_threshold: i32,
    ) -> FeatureDetector;

    fn ocv_detect_orb(
        detector: FeatureDetector,
        mat: Mat,
        keypoints: *mut std::ffi::c_void,
        keypoint_count: *mut i32,
    );

    fn ocv_create_sift(
        nfeatures: i32,
        n_octave_layers: i32,
        contrast_threshold: f64,
        edge_threshold: f64,
        sigma: f64,
    ) -> FeatureDetector;

    fn ocv_create_bfmatcher(norm_type: i32, cross_check: bool) -> DescriptorMatcher;

    fn ocv_match_descriptors(
        matcher: DescriptorMatcher,
        descriptors1: Mat,
        descriptors2: Mat,
        matches: *mut std::ffi::c_void,
        match_count: *mut i32,
    );

    fn ocv_draw_keypoints(mat: Mat, keypoints: *const std::ffi::c_void, keypoint_count: i32) -> Mat;
    fn ocv_draw_matches(
        img1: Mat,
        keypoints1: *const std::ffi::c_void,
        img2: Mat,
        keypoints2: *const std::ffi::c_void,
        matches: *const std::ffi::c_void,
        match_count: i32,
    ) -> Mat;
}

// ===================== PHOTO =====================

#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_inpaint(src: Mat, inpaint_mask: Mat, inpaint_radius: f64, flags: i32) -> Mat;
    fn ocv_fast_nl_means_denoising(
        src: Mat,
        h: f32,
        template_window_size: i32,
        search_window_size: i32,
    ) -> Mat;
    fn ocv_edge_preserving_filter(src: Mat, flags: i32, sigma_s: f32, sigma_r: f32) -> Mat;
    fn ocv_stylization(src: Mat, sigma_s: f32, sigma_r: f32) -> Mat;
}

// ===================== VIDEO =====================

#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_calc_optical_flow_pyr_lk(
        prev_img: Mat,
        next_img: Mat,
        prev_pts: Mat,
        next_pts: Mat,
        win_size: i32,
        max_level: i32,
        criteria_type: i32,
        max_iter: i32,
        epsilon: f64,
    ) -> Mat;

    fn ocv_calc_optical_flow_farneback(
        prev: Mat,
        next: Mat,
        pyr_scale: f64,
        levels: i32,
        win_size: i32,
        iterations: i32,
        poly_n: i32,
        poly_sigma: f64,
        flags: i32,
    ) -> Mat;
}

// ===================== CORE =====================

#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_add(src1: Mat, src2: Mat, mask: Mat) -> Mat;
    fn ocv_subtract(src1: Mat, src2: Mat) -> Mat;
    fn ocv_multiply(src1: Mat, src2: Mat, scale: f64) -> Mat;
    fn ocv_divide(src1: Mat, src2: Mat, scale: f64) -> Mat;
    fn ocv_normalize(src: Mat, alpha: f64, beta: f64, norm_type: i32, dtype: i32) -> Mat;

    fn ocv_min_max_loc(
        mat: Mat,
        min_val: *mut f64,
        max_val: *mut f64,
        min_x: *mut i32,
        min_y: *mut i32,
        max_x: *mut i32,
        max_y: *mut i32,
    );

    fn ocv_split(mat: Mat, channel_count: *mut i32) -> *mut *mut std::ffi::c_void;
    fn ocv_convert_scale_abs(src: Mat, alpha: f64, beta: f64) -> Mat;
    fn ocv_absdiff(src1: Mat, src2: Mat) -> Mat;
    fn ocv_bitwise_and(src1: Mat, src2: Mat) -> Mat;
    fn ocv_bitwise_or(src1: Mat, src2: Mat) -> Mat;
    fn ocv_bitwise_xor(src1: Mat, src2: Mat) -> Mat;
    fn ocv_bitwise_not(src: Mat) -> Mat;
    fn ocv_compare(src1: Mat, src2: Mat, cmp_op: i32) -> Mat;
    fn ocv_in_range(src: Mat, lowerb: Mat, upperb: Mat) -> Mat;
    fn ocv_copy_make_border(
        src: Mat,
        top: i32,
        bottom: i32,
        left: i32,
        right: i32,
        border_type: i32,
        value: *const f64,
    ) -> Mat;
}

// ===================== IMGPROC =====================

#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_pyr_down(src: Mat, dst_width: i32, dst_height: i32) -> Mat;
    fn ocv_pyr_up(src: Mat, dst_width: i32, dst_height: i32) -> Mat;
    fn ocv_get_gaussian_kernel(ksize: i32, sigma: f64, ktype: i32) -> Mat;
    fn ocv_get_structuring_element(
        shape: i32,
        ksize_width: i32,
        ksize_height: i32,
        anchor_x: i32,
        anchor_y: i32,
    ) -> Mat;
    fn ocv_filter2d(src: Mat, ddepth: i32, kernel: Mat, delta: f64) -> Mat;
    fn ocv_sep_filter2d(src: Mat, ddepth: i32, kernel_x: Mat, kernel_y: Mat, delta: f64) -> Mat;
    fn ocv_scharr(src: Mat, ddepth: i32, dx: i32, dy: i32, scale: f64, delta: f64) -> Mat;
    fn ocv_corner_harris(
        src: Mat,
        block_size: i32,
        ksize: i32,
        k: f64,
        border_type: i32,
    ) -> Mat;
    fn ocv_good_features_to_track(
        src: Mat,
        max_corners: i32,
        quality_level: f64,
        min_distance: f64,
        block_size: i32,
        use_harris_detector: bool,
        k: f64,
    ) -> Mat;
    fn ocv_hough_lines(
        src: Mat,
        rho: f64,
        theta: f64,
        threshold: i32,
        srn: f64,
        stn: f64,
    ) -> Mat;
    fn ocv_hough_circles(
        src: Mat,
        method: i32,
        dp: f64,
        min_dist: f64,
        param1: f64,
        param2: f64,
        min_radius: i32,
        max_radius: i32,
    ) -> Mat;
    fn ocv_morphology_ex(src: Mat, op: i32, kernel: Mat) -> Mat;
    fn ocv_get_rotation_matrix2d(
        center_x: f64,
        center_y: f64,
        angle: f64,
        scale: f64,
    ) -> Mat;
    fn ocv_warp_perspective(
        src: Mat,
        m: Mat,
        dst_width: i32,
        dst_height: i32,
        flags: i32,
        border_mode: i32,
        border_value: f64,
    ) -> Mat;
    fn ocv_get_affine_transform(src: Mat, dst: Mat) -> Mat;
    fn ocv_get_perspective_transform(src: Mat, dst: Mat) -> Mat;
    fn ocv_invert_affine_transform(m: Mat) -> Mat;
    fn ocv_integral(src: Mat, sdepth: i32) -> Mat;

    // CLAHE
    fn ocv_create_clahe(clip_limit: f64, tile_width: i32, tile_height: i32) -> *mut std::ffi::c_void;
    fn ocv_apply_clahe(clahe: *mut std::ffi::c_void, src: Mat, clip_limit: f64) -> Mat;

    fn ocv_adaptive_threshold(
        src: Mat,
        max_value: f64,
        adaptive_method: i32,
        threshold_type: i32,
        block_size: i32,
        c: f64,
    ) -> Mat;
    fn ocv_distance_transform(src: Mat, distance_type: i32, mask_size: i32) -> Mat;
    fn ocv_apply_color_map(src: Mat, colormap: i32) -> Mat;
    fn ocv_match_template(image: Mat, templ: Mat, method: i32) -> Mat;
    fn ocv_find_contours(image: Mat, mode: i32, method: i32) -> Mat;

    fn ocv_bounding_rect(points: Mat, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32);
    fn ocv_min_area_rect(
        points: Mat,
        center_x: *mut f32,
        center_y: *mut f32,
        width: *mut f32,
        height: *mut f32,
        angle: *mut f32,
    );
    fn ocv_convex_hull(points: Mat, clockwise: bool) -> Mat;
    fn ocv_approx_poly_dp(curve: Mat, epsilon: f64, closed: bool) -> Mat;
}

// ===================== HIGHGUI =====================

#[link(name = "opencv_wrapper")]
extern "C" {
    fn ocv_imshow(name: *const c_char, mat: Mat);
    fn ocv_wait_key(delay: i32) -> i32;

    // VideoCapture
    fn ocv_create_videocapture() -> VideoCapture;
    fn ocv_videocapture_open(cap: VideoCapture, device: i32) -> i32;
    fn ocv_videocapture_is_opened(cap: VideoCapture) -> i32;
    fn ocv_videocapture_get(cap: VideoCapture, prop: i32) -> f64;
    fn ocv_videocapture_set(cap: VideoCapture, prop: i32, value: f64) -> i32;
    fn ocv_videocapture_read(cap: VideoCapture, mat: Mat) -> i32;
    fn ocv_videocapture_release(cap: VideoCapture);

    // VideoWriter
    fn ocv_create_videowriter() -> VideoWriter;
    fn ocv_videowriter_open(writer: VideoWriter, filename: *const c_char, port: i32) -> i32;
    fn ocv_videowriter_is_opened(writer: VideoWriter) -> i32;
    fn ocv_videowriter_write(writer: VideoWriter, mat: Mat);
    fn ocv_videowriter_release(writer: VideoWriter);
}

// ===================== High-level Rust API =====================

// Feature2D wrappers
pub fn create_orb(
    nfeatures: i32,
    scale_factor: f32,
    nlevels: i32,
    edge_threshold: i32,
    first_level: i32,
    wta_k: i32,
    score_type: i32,
    patch_size: i32,
    fast_threshold: i32,
) -> FeatureDetector {
    unsafe {
        ocv_create_orb(
            nfeatures,
            scale_factor,
            nlevels,
            edge_threshold,
            first_level,
            wta_k,
            score_type,
            patch_size,
            fast_threshold,
        )
    }
}

pub fn create_sift(
    nfeatures: i32,
    n_octave_layers: i32,
    contrast_threshold: f64,
    edge_threshold: f64,
    sigma: f64,
) -> FeatureDetector {
    unsafe {
        ocv_create_sift(
            nfeatures,
            n_octave_layers as i32,
            contrast_threshold,
            edge_threshold,
            sigma,
        )
    }
}

pub fn create_bf_matcher(norm_type: i32, cross_check: bool) -> DescriptorMatcher {
    unsafe { ocv_create_bfmatcher(norm_type, cross_check) }
}

// Photo wrappers
pub fn inpaint(src: Mat, inpaint_mask: Mat, inpaint_radius: f64, flags: i32) -> Mat {
    unsafe { ocv_inpaint(src, inpaint_mask, inpaint_radius, flags) }
}

pub fn fast_nl_means_denoising(
    src: Mat,
    h: f32,
    template_window_size: i32,
    search_window_size: i32,
) -> Mat {
    unsafe { ocv_fast_nl_means_denoising(src, h, template_window_size, search_window_size) }
}

pub fn edge_preserving_filter(src: Mat, flags: i32, sigma_s: f32, sigma_r: f32) -> Mat {
    unsafe { ocv_edge_preserving_filter(src, flags, sigma_s, sigma_r) }
}

pub fn stylization(src: Mat, sigma_s: f32, sigma_r: f32) -> Mat {
    unsafe { ocv_stylization(src, sigma_s, sigma_r) }
}

// Video wrappers
pub fn calc_optical_flow_pyr_lk(
    prev_img: Mat,
    next_img: Mat,
    prev_pts: Mat,
    next_pts: Mat,
    win_size: i32,
    max_level: i32,
    criteria_type: i32,
    max_iter: i32,
    epsilon: f64,
) -> Mat {
    unsafe {
        ocv_calc_optical_flow_pyr_lk(
            prev_img,
            next_img,
            prev_pts,
            next_pts,
            win_size,
            max_level,
            criteria_type,
            max_iter,
            epsilon,
        )
    }
}

pub fn calc_optical_flow_farneback(
    prev: Mat,
    next: Mat,
    pyr_scale: f64,
    levels: i32,
    win_size: i32,
    iterations: i32,
    poly_n: i32,
    poly_sigma: f64,
    flags: i32,
) -> Mat {
    unsafe {
        ocv_calc_optical_flow_farneback(
            prev,
            next,
            pyr_scale,
            levels,
            win_size,
            iterations,
            poly_n,
            poly_sigma,
            flags,
        )
    }
}

// Core wrappers
pub fn add(src1: Mat, src2: Mat, mask: Mat) -> Mat {
    unsafe { ocv_add(src1, src2, mask) }
}

pub fn subtract(src1: Mat, src2: Mat) -> Mat {
    unsafe { ocv_subtract(src1, src2) }
}

pub fn multiply(src1: Mat, src2: Mat, scale: f64) -> Mat {
    unsafe { ocv_multiply(src1, src2, scale) }
}

pub fn divide(src1: Mat, src2: Mat, scale: f64) -> Mat {
    unsafe { ocv_divide(src1, src2, scale) }
}

pub fn normalize(src: Mat, alpha: f64, beta: f64, norm_type: i32, dtype: i32) -> Mat {
    unsafe { ocv_normalize(src, alpha, beta, norm_type, dtype) }
}

#[repr(C)]
pub struct MinMaxLocResult {
    pub min_val: f64,
    pub max_val: f64,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

pub fn min_max_loc(mat: Mat) -> Option<MinMaxLocResult> {
    unsafe {
        let mut min_val: f64 = 0.0;
        let mut max_val: f64 = 0.0;
        let mut min_x: i32 = 0;
        let mut min_y: i32 = 0;
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;

        ocv_min_max_loc(
            mat,
            &mut min_val,
            &mut max_val,
            &mut min_x,
            &mut min_y,
            &mut max_x,
            &mut max_y,
        );

        Some(MinMaxLocResult {
            min_val,
            max_val,
            min_x,
            min_y,
            max_x,
            max_y,
        })
    }
}

pub fn split(mat: Mat) -> (Vec<Mat>, i32) {
    unsafe {
        let mut channel_count: i32 = 0;
        let result = ocv_split(mat, &mut channel_count);
        if result.is_null() {
            return (vec![], 0);
        }

        let mut channels = Vec::new();
        for i in 0..channel_count {
            let ptr = *result.offset(i as isize);
            if !ptr.is_null() {
                channels.push(ptr);
            }
        }

        libc::free(result as *mut libc::c_void);
        (channels, channel_count)
    }
}

pub fn convert_scale_abs(src: Mat, alpha: f64, beta: f64) -> Mat {
    unsafe { ocv_convert_scale_abs(src, alpha, beta) }
}

pub fn absdiff(src1: Mat, src2: Mat) -> Mat {
    unsafe { ocv_absdiff(src1, src2) }
}

pub fn bitwise_and(src1: Mat, src2: Mat) -> Mat {
    unsafe { ocv_bitwise_and(src1, src2) }
}

pub fn bitwise_or(src1: Mat, src2: Mat) -> Mat {
    unsafe { ocv_bitwise_or(src1, src2) }
}

pub fn bitwise_xor(src1: Mat, src2: Mat) -> Mat {
    unsafe { ocv_bitwise_xor(src1, src2) }
}

pub fn bitwise_not(src: Mat) -> Mat {
    unsafe { ocv_bitwise_not(src) }
}

pub fn compare(src1: Mat, src2: Mat, cmp_op: i32) -> Mat {
    unsafe { ocv_compare(src1, src2, cmp_op) }
}

pub fn in_range(src: Mat, lowerb: Mat, upperb: Mat) -> Mat {
    unsafe { ocv_in_range(src, lowerb, upperb) }
}

pub fn copy_make_border(
    src: Mat,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    border_type: i32,
    value: Option<[f64; 4]>,
) -> Mat {
    unsafe {
        match value {
            Some(v) => ocv_copy_make_border(src, top, bottom, left, right, border_type, v.as_ptr()),
            None => {
                let default_val: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
                ocv_copy_make_border(src, top, bottom, left, right, border_type, default_val.as_ptr())
            }
        }
    }
}

// Imgproc wrappers
pub fn pyr_down(src: Mat, dst_width: i32, dst_height: i32) -> Mat {
    unsafe { ocv_pyr_down(src, dst_width, dst_height) }
}

pub fn pyr_up(src: Mat, dst_width: i32, dst_height: i32) -> Mat {
    unsafe { ocv_pyr_up(src, dst_width, dst_height) }
}

pub fn get_gaussian_kernel(ksize: i32, sigma: f64, ktype: i32) -> Mat {
    unsafe { ocv_get_gaussian_kernel(ksize, sigma, ktype) }
}

pub fn get_structuring_element(
    shape: i32,
    ksize_width: i32,
    ksize_height: i32,
    anchor_x: i32,
    anchor_y: i32,
) -> Mat {
    unsafe {
        ocv_get_structuring_element(shape, ksize_width, ksize_height, anchor_x, anchor_y)
    }
}

pub fn filter2d(src: Mat, ddepth: i32, kernel: Mat, delta: f64) -> Mat {
    unsafe { ocv_filter2d(src, ddepth, kernel, delta) }
}

pub fn sep_filter2d(src: Mat, ddepth: i32, kernel_x: Mat, kernel_y: Mat, delta: f64) -> Mat {
    unsafe { ocv_sep_filter2d(src, ddepth, kernel_x, kernel_y, delta) }
}

pub fn scharr(src: Mat, ddepth: i32, dx: i32, dy: i32, scale: f64, delta: f64) -> Mat {
    unsafe { ocv_scharr(src, ddepth, dx, dy, scale, delta) }
}

pub fn corner_harris(src: Mat, block_size: i32, ksize: i32, k: f64, border_type: i32) -> Mat {
    unsafe { ocv_corner_harris(src, block_size, ksize, k, border_type) }
}

pub fn good_features_to_track(
    src: Mat,
    max_corners: i32,
    quality_level: f64,
    min_distance: f64,
    block_size: i32,
    use_harris_detector: bool,
    k: f64,
) -> Mat {
    unsafe {
        ocv_good_features_to_track(
            src,
            max_corners,
            quality_level,
            min_distance,
            block_size,
            use_harris_detector,
            k,
        )
    }
}

pub fn hough_lines(
    src: Mat,
    rho: f64,
    theta: f64,
    threshold: i32,
    srn: f64,
    stn: f64,
) -> Mat {
    unsafe { ocv_hough_lines(src, rho, theta, threshold, srn, stn) }
}

pub fn hough_circles(
    src: Mat,
    method: i32,
    dp: f64,
    min_dist: f64,
    param1: f64,
    param2: f64,
    min_radius: i32,
    max_radius: i32,
) -> Mat {
    unsafe {
        ocv_hough_circles(
            src,
            method,
            dp,
            min_dist,
            param1,
            param2,
            min_radius,
            max_radius,
        )
    }
}

pub fn morphology_ex(src: Mat, op: i32, kernel: Mat) -> Mat {
    unsafe { ocv_morphology_ex(src, op, kernel) }
}

pub fn get_rotation_matrix2d(center_x: f64, center_y: f64, angle: f64, scale: f64) -> Mat {
    unsafe { ocv_get_rotation_matrix2d(center_x, center_y, angle, scale) }
}

pub fn warp_perspective(
    src: Mat,
    m: Mat,
    dst_width: i32,
    dst_height: i32,
    flags: i32,
    border_mode: i32,
    border_value: f64,
) -> Mat {
    unsafe { ocv_warp_perspective(src, m, dst_width, dst_height, flags, border_mode, border_value) }
}

pub fn get_affine_transform(src: Mat, dst: Mat) -> Mat {
    unsafe { ocv_get_affine_transform(src, dst) }
}

pub fn get_perspective_transform(src: Mat, dst: Mat) -> Mat {
    unsafe { ocv_get_perspective_transform(src, dst) }
}

pub fn invert_affine_transform(m: Mat) -> Mat {
    unsafe { ocv_invert_affine_transform(m) }
}

pub fn integral(src: Mat, sdepth: i32) -> Mat {
    unsafe { ocv_integral(src, sdepth) }
}

pub fn create_clahe(clip_limit: f64, tile_width: i32, tile_height: i32) -> *mut std::ffi::c_void {
    unsafe { ocv_create_clahe(clip_limit, tile_width, tile_height) }
}

pub fn apply_clahe(clahe: *mut std::ffi::c_void, src: Mat, clip_limit: f64) -> Mat {
    unsafe { ocv_apply_clahe(clahe, src, clip_limit) }
}

pub fn adaptive_threshold(
    src: Mat,
    max_value: f64,
    adaptive_method: i32,
    threshold_type: i32,
    block_size: i32,
    c: f64,
) -> Mat {
    unsafe {
        ocv_adaptive_threshold(
            src,
            max_value,
            adaptive_method,
            threshold_type,
            block_size,
            c,
        )
    }
}

pub fn distance_transform(src: Mat, distance_type: i32, mask_size: i32) -> Mat {
    unsafe { ocv_distance_transform(src, distance_type, mask_size) }
}

pub fn apply_color_map(src: Mat, colormap: i32) -> Mat {
    unsafe { ocv_apply_color_map(src, colormap) }
}

pub fn match_template(image: Mat, templ: Mat, method: i32) -> Mat {
    unsafe { ocv_match_template(image, templ, method) }
}

pub fn find_contours(image: Mat, mode: i32, method: i32) -> Mat {
    unsafe { ocv_find_contours(image, mode, method) }
}

pub fn bounding_rect(points: Mat) -> Option<(i32, i32, i32, i32)> {
    unsafe {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        ocv_bounding_rect(points, &mut x, &mut y, &mut width, &mut height);
        Some((x, y, width, height))
    }
}

pub fn min_area_rect(points: Mat) -> Option<(f32, f32, f32, f32, f32)> {
    unsafe {
        let mut center_x: f32 = 0.0;
        let mut center_y: f32 = 0.0;
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        let mut angle: f32 = 0.0;
        ocv_min_area_rect(
            points,
            &mut center_x,
            &mut center_y,
            &mut width,
            &mut height,
            &mut angle,
        );
        Some((center_x, center_y, width, height, angle))
    }
}

pub fn convex_hull(points: Mat, clockwise: bool) -> Mat {
    unsafe { ocv_convex_hull(points, clockwise) }
}

pub fn approx_poly_dp(curve: Mat, epsilon: f64, closed: bool) -> Mat {
    unsafe { ocv_approx_poly_dp(curve, epsilon, closed) }
}

// HighGUI wrappers
pub fn imshow(name: &str, mat: Mat) {
    unsafe {
        if let Ok(c_str) = std::ffi::CString::new(name) {
            ocv_imshow(c_str.as_ptr(), mat);
        }
    }
}

pub fn wait_key(delay: i32) -> i32 {
    unsafe { ocv_wait_key(delay) }
}

// VideoCapture wrappers
pub fn create_videocapture() -> VideoCapture {
    unsafe { ocv_create_videocapture() }
}

/// Open video capture device by index
/// Note: opencv-mobile only supports device index, not filename
pub fn videocapture_open(cap: VideoCapture, device: i32) -> i32 {
    unsafe { ocv_videocapture_open(cap, device) }
}

pub fn videocapture_is_opened(cap: VideoCapture) -> bool {
    unsafe { ocv_videocapture_is_opened(cap) != 0 }
}

pub fn videocapture_get(cap: VideoCapture, prop: i32) -> f64 {
    unsafe { ocv_videocapture_get(cap, prop) }
}

pub fn videocapture_set(cap: VideoCapture, prop: i32, value: f64) -> bool {
    unsafe { ocv_videocapture_set(cap, prop, value) != 0 }
}

pub fn videocapture_read(cap: VideoCapture, mat: Mat) -> bool {
    unsafe { ocv_videocapture_read(cap, mat) != 0 }
}

pub fn videocapture_release(cap: VideoCapture) {
    unsafe { ocv_videocapture_release(cap) }
}

// VideoWriter wrappers
pub fn create_videowriter() -> VideoWriter {
    unsafe { ocv_create_videowriter() }
}

/// Open video writer
/// Note: opencv-mobile only supports filename and port parameters
pub fn videowriter_open(writer: VideoWriter, filename: &str, port: i32) -> bool {
    unsafe {
        if let Ok(c_str) = std::ffi::CString::new(filename) {
            ocv_videowriter_open(writer, c_str.as_ptr(), port) != 0
        } else {
            false
        }
    }
}

pub fn videowriter_is_opened(writer: VideoWriter) -> bool {
    unsafe { ocv_videowriter_is_opened(writer) != 0 }
}

pub fn videowriter_write(writer: VideoWriter, mat: Mat) {
    unsafe { ocv_videowriter_write(writer, mat) }
}

pub fn videowriter_release(writer: VideoWriter) {
    unsafe { ocv_videowriter_release(writer) }
}

// ===================== Common constants =====================

// Color conversion codes
pub const COLOR_BGR2GRAY: i32 = 6;
pub const COLOR_BGR2RGB: i32 = 4;
pub const COLOR_BGR2HSV: i32 = 40;
pub const COLOR_BGR2YUV: i32 = 82;
pub const COLOR_YUV2BGR: i32 = 84;

// Threshold types
pub const THRESH_BINARY: i32 = 0;
pub const THRESH_BINARY_INV: i32 = 1;
pub const THRESH_TRUNC: i32 = 2;
pub const THRESH_TOZERO: i32 = 3;
pub const THRESH_TOZERO_INV: i32 = 4;

// Morphological operations
pub const MORPH_ERODE: i32 = 0;
pub const MORPH_DILATE: i32 = 1;
pub const MORPH_OPEN: i32 = 2;
pub const MORPH_CLOSE: i32 = 3;
pub const MORPH_GRADIENT: i32 = 4;

// Border types
pub const BORDER_CONSTANT: i32 = 0;
pub const BORDER_REPLICATE: i32 = 1;
pub const BORDER_REFLECT: i32 = 2;
pub const BORDER_WRAP: i32 = 3;
pub const BORDER_REFLECT_101: i32 = 4;

// Norm types
pub const NORM_L1: i32 = 2;
pub const NORM_L2: i32 = 4;
pub const NORM_INF: i32 = 8;
pub const NORM_MINMAX: i32 = 32;

// Compare operators
pub const CMP_EQ: i32 = 0;
pub const CMP_GT: i32 = 1;
pub const CMP_GE: i32 = 2;
pub const CMP_LT: i32 = 3;
pub const CMP_LE: i32 = 4;
pub const CMP_NE: i32 = 5;

// Imread flags
pub const IMREAD_UNCHANGED: i32 = -1;
pub const IMREAD_GRAYSCALE: i32 = 0;
pub const IMREAD_COLOR: i32 = 1;

// TemplateMatchModes
pub const TM_SQDIFF: i32 = 0;
pub const TM_SQDIFF_NORMED: i32 = 1;
pub const TM_CCORR: i32 = 2;
pub const TM_CCORR_NORMED: i32 = 3;
pub const TM_CCOEFF: i32 = 4;
pub const TM_CCOEFF_NORMED: i32 = 5;

// Contour retrieval modes
pub const RETR_EXTERNAL: i32 = 0;
pub const RETR_LIST: i32 = 1;
pub const RETR_CCOMP: i32 = 2;
pub const RETR_TREE: i32 = 3;

// Contour approximation methods
pub const CHAIN_APPROX_NONE: i32 = 1;
pub const CHAIN_APPROX_SIMPLE: i32 = 2;

// Colormap types
pub const COLORMAP_AUTUMN: i32 = 0;
pub const COLORMAP_BONE: i32 = 1;
pub const COLORMAP_JET: i32 = 2;
pub const COLORMAP_WINTER: i32 = 3;
pub const COLORMAP_RAINBOW: i32 = 4;
pub const COLORMAP_OCEAN: i32 = 5;
pub const COLORMAP_SUMMER: i32 = 6;
pub const COLORMAP_SPRING: i32 = 7;
pub const COLORMAP_COOL: i32 = 8;
pub const COLORMAP_HSV: i32 = 9;
pub const COLORMAP_PINK: i32 = 10;
pub const COLORMAP_HOT: i32 = 11;
pub const COLORMAP_PARULA: i32 = 12;
pub const COLORMAP_MAGMA: i32 = 13;
pub const COLORMAP_INFERNO: i32 = 14;
pub const COLORMAP_PLASMA: i32 = 15;
pub const COLORMAP_VIRIDIS: i32 = 16;
pub const COLORMAP_CIVIDIS: i32 = 17;
pub const COLORMAP_TWILIGHT: i32 = 18;
pub const COLORMAP_TWILIGHT_SHIFTED: i32 = 19;
pub const COLORMAP_TURBO: i32 = 20;
pub const COLORMAP_DEEPGREEN: i32 = 21;

// VideoCapture properties (only available in opencv-mobile)
pub const CAP_PROP_FRAME_WIDTH: i32 = 3;
pub const CAP_PROP_FRAME_HEIGHT: i32 = 4;
pub const CAP_PROP_FPS: i32 = 5;
