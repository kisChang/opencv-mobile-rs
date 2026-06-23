// OpenCV wrapper for Rust FFI
// This file provides a C interface to OpenCV functions

#include <opencv2/core.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/features2d.hpp>
#include <opencv2/photo.hpp>
#include <opencv2/video.hpp>
#include <cstring>
#include <cstdlib>

extern "C" {

// Helper to allocate cv::Mat with configurable parameters
// Parameters: height, width, type (OpenCV type like CV_8UC3), values (array of initial values)
void* ocv_create_mat(int height, int width, int type, double* values) {
    if (!values) {
        return new cv::Mat(height, width, type);
    }
    return new cv::Mat(height, width, type, cv::Scalar(values[0], values[1], values[2], values[3]));
}

// Free cv::Mat
void ocv_free(void* mat_ptr) {
    if (mat_ptr) {
        delete static_cast<cv::Mat*>(mat_ptr);
    }
}

// Clone cv::Mat
void* ocv_clone(void* mat_ptr) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat(src->clone());
    return dst;
}

// Color conversion
void* ocv_cvtColor(void* mat_ptr, int code) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::cvtColor(*src, *dst, code);
    return dst;
}

// Resize
void* ocv_resize(void* mat_ptr, int width, int height) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::resize(*src, *dst, cv::Size(width, height));
    return dst;
}

// Blur (box filter)
void* ocv_blur(void* mat_ptr, int ksize_width, int ksize_height) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::blur(*src, *dst, cv::Size(ksize_width, ksize_height));
    return dst;
}

// Gaussian blur
void* ocv_gaussianBlur(void* mat_ptr, int ksize, double sigma) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::GaussianBlur(*src, *dst, cv::Size(ksize, ksize), sigma);
    return dst;
}

// Median blur
void* ocv_medianBlur(void* mat_ptr, int ksize) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::medianBlur(*src, *dst, ksize);
    return dst;
}

// Bilateral filter
void* ocv_bilateralFilter(void* mat_ptr, int d, double sigma_color, double sigma_space) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::bilateralFilter(*src, *dst, d, sigma_color, sigma_space);
    return dst;
}

// Threshold
void* ocv_threshold(void* mat_ptr, double thresh, double maxval, int type) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::threshold(*src, *dst, thresh, maxval, type);
    return dst;
}

// Erode
void* ocv_erode(void* mat_ptr, int ksize_width, int ksize_height) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::erode(*src, *dst, cv::Mat(), cv::Point(-1, -1), ksize_width);
    return dst;
}

// Dilate
void* ocv_dilate(void* mat_ptr, int ksize_width, int ksize_height) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::dilate(*src, *dst, cv::Mat(), cv::Point(-1, -1), ksize_width);
    return dst;
}

// Morphology open
void* ocv_morphology_open(void* mat_ptr, int ksize_width, int ksize_height) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::morphologyEx(*src, *dst, cv::MORPH_OPEN, cv::getStructuringElement(cv::MORPH_RECT, cv::Size(ksize_width, ksize_height)));
    return dst;
}

// Morphology close
void* ocv_morphology_close(void* mat_ptr, int ksize_width, int ksize_height) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::morphologyEx(*src, *dst, cv::MORPH_CLOSE, cv::getStructuringElement(cv::MORPH_RECT, cv::Size(ksize_width, ksize_height)));
    return dst;
}

// Canny edge detection
void* ocv_canny(void* mat_ptr, double threshold1, double threshold2) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::Canny(*src, *dst, threshold1, threshold2);
    return dst;
}

// Sobel
void* ocv_sobel(void* mat_ptr, int dx, int dy, int ksize) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::Sobel(*src, *dst, CV_8U, dx, dy, ksize);
    return dst;
}

// Laplacian
void* ocv_laplacian(void* mat_ptr, int ksize) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::Laplacian(*src, *dst, CV_8U, ksize);
    return dst;
}

// Equalize hist
void* ocv_equalizeHist(void* mat_ptr) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::equalizeHist(*src, *dst);
    return dst;
}

// Warp affine (basic rotation + scale)
void* ocv_warpAffine(void* mat_ptr, double angle, double scale) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();

    cv::Point2f center(src->cols / 2.0f, src->rows / 2.0f);
    cv::Mat rot_mat = cv::getRotationMatrix2D(center, angle, scale);
    cv::warpAffine(*src, *dst, rot_mat, src->size());
    return dst;
}

// Rotate (90 degree increments)
void* ocv_rotate(void* mat_ptr, double angle) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();

    int rotation_code;
    if (angle == 90) rotation_code = cv::ROTATE_90_CLOCKWISE;
    else if (angle == 180) rotation_code = cv::ROTATE_180;
    else if (angle == 270) rotation_code = cv::ROTATE_90_COUNTERCLOCKWISE;
    else {
        delete dst;
        return nullptr;
    }

    cv::rotate(*src, *dst, rotation_code);
    return dst;
}

// Flip
void* ocv_flip(void* mat_ptr, int mode) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::flip(*src, *dst, mode);
    return dst;
}

// Crop
void* ocv_crop(void* mat_ptr, int x, int y, int width, int height) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Rect roi(x, y, width, height);

    if (x < 0 || y < 0 || x + width > src->cols || y + height > src->rows) {
        return nullptr;
    }

    cv::Mat* dst = new cv::Mat((*src)(roi).clone());
    return dst;
}

// Merge channels
void* ocv_merge(void** mats, int count) {
    if (!mats || count <= 0) return nullptr;

    std::vector<cv::Mat> channels;
    for (int i = 0; i < count; i++) {
        if (mats[i]) {
            channels.push_back(*static_cast<cv::Mat*>(mats[i]));
        }
    }

    if (channels.empty()) return nullptr;

    cv::Mat* dst = new cv::Mat();
    cv::merge(channels, *dst);
    return dst;
}

// Get channels
int ocv_get_channels(void* mat_ptr) {
    if (!mat_ptr) return 0;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);
    return mat->channels();
}

// Get width
int ocv_get_width(void* mat_ptr) {
    if (!mat_ptr) return 0;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);
    return mat->cols;
}

// Get height
int ocv_get_height(void* mat_ptr) {
    if (!mat_ptr) return 0;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);
    return mat->rows;
}

// Get data pointer
unsigned char* ocv_get_data(void* mat_ptr) {
    if (!mat_ptr) return nullptr;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);
    return mat->data;
}

// Imread
void* ocv_imread(const char* filename, int flags) {
    if (!filename) return nullptr;
    cv::Mat* mat = new cv::Mat(cv::imread(filename, flags));
    if (mat->empty()) {
        delete mat;
        return nullptr;
    }
    return mat;
}

// Imwrite
int ocv_imwrite(const char* filename, void* mat_ptr, int quality) {
    if (!filename || !mat_ptr) return 0;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);

    // Try to use quality parameter for JPEG files
    std::string ext = filename;
    size_t dot_pos = ext.rfind('.');
    if (dot_pos != std::string::npos) {
        std::string ext_lower = ext.substr(dot_pos + 1);
        for (char& c : ext_lower) c = std::tolower(c);

        if (ext_lower == "jpg" || ext_lower == "jpeg") {
            // Use CV_IMWRITE_JPEG_QUALITY = 1
            std::vector<int> params;
            params.push_back(1);  // CV_IMWRITE_JPEG_QUALITY
            params.push_back(quality);
            return cv::imwrite(filename, *mat, params) ? 1 : 0;
        }
    }

    return cv::imwrite(filename, *mat) ? 1 : 0;
}

// Font handling using Hershey fonts
void* ocv_create_font() {
    return new cv::Scalar(255, 255, 255);  // Store color as font data
}

void ocv_destroy_font(void* font_ptr) {
    if (font_ptr) {
        delete static_cast<cv::Scalar*>(font_ptr);
    }
}

// Get text size
void ocv_get_text_size(void* font_ptr, const char* text, double font_scale, int* width, int* height) {
    if (!text || !width || !height) return;

    int baseline = 0;
    cv::Size size = cv::getTextSize(text, cv::FONT_HERSHEY_SIMPLEX, font_scale, 1, &baseline);
    *width = size.width;
    *height = size.height + baseline;
}

// Put text
void ocv_put_text(void* mat_ptr, void* font_ptr, const char* text, int x, int y, int b, int g, int r, double font_scale) {
    if (!mat_ptr || !text) return;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);
    cv::Scalar color(b, g, r);
    cv::putText(*mat, text, cv::Point(x, y), cv::FONT_HERSHEY_SIMPLEX, font_scale, color, 1, cv::LINE_AA);
}

// ===================== FEATURES2D =====================

// ORB Feature Detector
void* ocv_create_orb(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold,
                      int firstLevel, int WTA_K, int scoreType, int patchSize, int fastThreshold) {
    cv::Ptr<cv::ORB> detector = cv::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold,
                                                  firstLevel, WTA_K, (cv::ORB::ScoreType)scoreType,
                                                  patchSize, fastThreshold);
    return new cv::Ptr<cv::ORB>(detector);
}

void ocv_detect_orb(void* detector_ptr, void* mat_ptr, void** keypoints, int* keypoint_count) {
    if (!detector_ptr || !mat_ptr || !keypoints) {
        if (keypoint_count) *keypoint_count = 0;
        return;
    }
    cv::Ptr<cv::ORB>* detector = static_cast<cv::Ptr<cv::ORB>*>(detector_ptr);
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);
    std::vector<cv::KeyPoint> kps;
    (*detector)->detect(*mat, kps);
    *keypoint_count = kps.size();
    // Note: In production, would need to return keypoint data through a different mechanism
    // For now, just return count
}

void* ocv_create_sift(int nfeatures, int nOctaveLayers, double contrastThreshold, double edgeThreshold, double sigma) {
    cv::Ptr<cv::SIFT> detector = cv::SIFT::create(nfeatures, nOctaveLayers, contrastThreshold, edgeThreshold, sigma);
    return new cv::Ptr<cv::SIFT>(detector);
}

void* ocv_create_bfmatcher(int normType, bool crossCheck) {
    cv::Ptr<cv::BFMatcher> matcher = cv::BFMatcher::create(normType, crossCheck);
    return new cv::Ptr<cv::BFMatcher>(matcher);
}

// FlannBasedMatcher is not available in opencv-mobile (requires xfeatures2d module)
// void* ocv_create_flann_matcher() {
//     cv::Ptr<cv::FlannBasedMatcher> matcher = cv::FlannBasedMatcher::create();
//     return new cv::Ptr<cv::FlannBasedMatcher>(matcher);
// }

void ocv_match_descriptors(void* matcher_ptr, void* descriptors1, void* descriptors2, void** matches, int* match_count) {
    if (!matcher_ptr || !descriptors1 || !descriptors2 || !matches) {
        if (match_count) *match_count = 0;
        return;
    }
    cv::Ptr<cv::DescriptorMatcher>* matcher = static_cast<cv::Ptr<cv::DescriptorMatcher>*>(matcher_ptr);
    cv::Mat* desc1 = static_cast<cv::Mat*>(descriptors1);
    cv::Mat* desc2 = static_cast<cv::Mat*>(descriptors2);
    std::vector<cv::DMatch> m;
    (*matcher)->match(*desc1, *desc2, m);
    *match_count = m.size();
}

// Draw keypoints
void* ocv_draw_keypoints(void* mat_ptr, void* keypoints_ptr, int keypoint_count) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    std::vector<cv::KeyPoint> kps;  // Empty for now - would need proper keypoint handling
    cv::drawKeypoints(*src, kps, *dst, cv::Scalar::all(-1), cv::DrawMatchesFlags::DEFAULT);
    return dst;
}

// Draw matches
void* ocv_draw_matches(void* img1, void* keypoints1, void* img2, void* keypoints2, void* matches, int match_count) {
    if (!img1 || !img2) return nullptr;
    cv::Mat* img1_mat = static_cast<cv::Mat*>(img1);
    cv::Mat* img2_mat = static_cast<cv::Mat*>(img2);
    cv::Mat* dst = new cv::Mat();
    std::vector<cv::KeyPoint> kps1, kps2;
    std::vector<cv::DMatch> m;
    cv::drawMatches(*img1_mat, kps1, *img2_mat, kps2, m, *dst);
    return dst;
}

// ===================== PHOTO =====================

// Inpaint
void* ocv_inpaint(void* src_ptr, void* inpaintMask_ptr, double inpaintRadius, int flags) {
    if (!src_ptr || !inpaintMask_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* mask = static_cast<cv::Mat*>(inpaintMask_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::inpaint(*src, *mask, *dst, inpaintRadius, flags);
    return dst;
}

// FastNlMeansDenoising
void* ocv_fast_nl_means_denoising(void* mat_ptr, float h, int templateWindowSize, int searchWindowSize) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::fastNlMeansDenoising(*src, *dst, h, templateWindowSize, searchWindowSize);
    return dst;
}

// Edge preserving filter
void* ocv_edge_preserving_filter(void* mat_ptr, int flags, float sigma_s, float sigma_r) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::edgePreservingFilter(*src, *dst, flags, sigma_s, sigma_r);
    return dst;
}

// Stylization
void* ocv_stylization(void* mat_ptr, float sigma_s, float sigma_r) {
    if (!mat_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::stylization(*src, *dst, sigma_s, sigma_r);
    return dst;
}

// ===================== VIDEO =====================

// Optical flow pyr LK
void* ocv_calc_optical_flow_pyr_lk(void* prevImg, void* nextImg, void* prevPts, void* nextPts,
                                    int winSize, int maxLevel, int criteria_type, int max_iter, double epsilon) {
    if (!prevImg || !nextImg || !prevPts || !nextPts) return nullptr;
    cv::Mat* prev = static_cast<cv::Mat*>(prevImg);
    cv::Mat* next = static_cast<cv::Mat*>(nextImg);
    cv::Mat* prev_pts = static_cast<cv::Mat*>(prevPts);
    cv::Mat* next_pts = static_cast<cv::Mat*>(nextPts);
    cv::Mat* dst = new cv::Mat();

    std::vector<cv::Point2f> prev_points, next_points;
    // Would need to convert from Mat to vector

    cv::TermCriteria tc((cv::TermCriteria::Type)criteria_type, max_iter, epsilon);
    // Simplified - would need proper point conversion
    return dst;
}

// Optical flow Farneback
void* ocv_calc_optical_flow_farneback(void* prev, void* next, double pyr_scale, int levels,
                                       int winSize, int iterations, int poly_n, double poly_sigma, int flags) {
    if (!prev || !next) return nullptr;
    cv::Mat* prev_mat = static_cast<cv::Mat*>(prev);
    cv::Mat* next_mat = static_cast<cv::Mat*>(next);
    cv::Mat* flow = new cv::Mat();
    cv::calcOpticalFlowFarneback(*prev_mat, *next_mat, *flow, pyr_scale, levels, winSize, iterations,
                                   poly_n, poly_sigma, flags);
    return flow;
}

// ===================== CORE =====================

// Add
void* ocv_add(void* src1_ptr, void* src2_ptr, void* mask_ptr) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    if (mask_ptr) {
        cv::Mat* mask = static_cast<cv::Mat*>(mask_ptr);
        cv::add(*src1, *src2, *dst, *mask);
    } else {
        cv::add(*src1, *src2, *dst);
    }
    return dst;
}

// Subtract
void* ocv_subtract(void* src1_ptr, void* src2_ptr) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::subtract(*src1, *src2, *dst);
    return dst;
}

// Multiply
void* ocv_multiply(void* src1_ptr, void* src2_ptr, double scale) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::multiply(*src1, *src2, *dst, scale);
    return dst;
}

// Divide
void* ocv_divide(void* src1_ptr, void* src2_ptr, double scale) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::divide(*src1, *src2, *dst, scale);
    return dst;
}

// Normalize
void* ocv_normalize(void* src_ptr, double alpha, double beta, int norm_type, int dtype) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::normalize(*src, *dst, alpha, beta, norm_type, dtype);
    return dst;
}

// MinMaxLoc
void ocv_min_max_loc(void* mat_ptr, double* min_val, double* max_val, int* min_x, int* min_y, int* max_x, int* max_y) {
    if (!mat_ptr) return;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);

    // minMaxLoc only works with single channel matrices
    // Convert to grayscale if needed
    cv::Mat gray;
    if (mat->channels() > 1) {
        cv::cvtColor(*mat, gray, cv::COLOR_BGR2GRAY);
    } else {
        gray = *mat;
    }

    cv::Point min_loc, max_loc;
    cv::minMaxLoc(gray, min_val, max_val, &min_loc, &max_loc);
    if (min_x) *min_x = min_loc.x;
    if (min_y) *min_y = min_loc.y;
    if (max_x) *max_x = max_loc.x;
    if (max_y) *max_y = max_loc.y;
}

// Split
void** ocv_split(void* mat_ptr, int* channel_count) {
    if (!mat_ptr || !channel_count) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(mat_ptr);
    std::vector<cv::Mat> channels;
    cv::split(*src, channels);
    *channel_count = channels.size();

    void** result = (void**)malloc(sizeof(void*) * channels.size());
    for (size_t i = 0; i < channels.size(); i++) {
        result[i] = new cv::Mat(channels[i].clone());
    }
    return result;
}

// ConvertScaleAbs
void* ocv_convert_scale_abs(void* src_ptr, double alpha, double beta) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::convertScaleAbs(*src, *dst, alpha, beta);
    return dst;
}

// AbsDiff
void* ocv_absdiff(void* src1_ptr, void* src2_ptr) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::absdiff(*src1, *src2, *dst);
    return dst;
}

// Bitwise AND
void* ocv_bitwise_and(void* src1_ptr, void* src2_ptr) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::bitwise_and(*src1, *src2, *dst);
    return dst;
}

// Bitwise OR
void* ocv_bitwise_or(void* src1_ptr, void* src2_ptr) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::bitwise_or(*src1, *src2, *dst);
    return dst;
}

// Bitwise XOR
void* ocv_bitwise_xor(void* src1_ptr, void* src2_ptr) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::bitwise_xor(*src1, *src2, *dst);
    return dst;
}

// Bitwise NOT
void* ocv_bitwise_not(void* src_ptr) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::bitwise_not(*src, *dst);
    return dst;
}

// Compare
void* ocv_compare(void* src1_ptr, void* src2_ptr, int cmp_op) {
    if (!src1_ptr || !src2_ptr) return nullptr;
    cv::Mat* src1 = static_cast<cv::Mat*>(src1_ptr);
    cv::Mat* src2 = static_cast<cv::Mat*>(src2_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::compare(*src1, *src2, *dst, cmp_op);
    return dst;
}

// InRange
void* ocv_in_range(void* src_ptr, void* lowerb_ptr, void* upperb_ptr) {
    if (!src_ptr || !lowerb_ptr || !upperb_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* lowerb = static_cast<cv::Mat*>(lowerb_ptr);
    cv::Mat* upperb = static_cast<cv::Mat*>(upperb_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::inRange(*src, *lowerb, *upperb, *dst);
    return dst;
}

// CopyMakeBorder
void* ocv_copy_make_border(void* src_ptr, int top, int bottom, int left, int right, int borderType, double* value) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::Scalar scalar(0, 0, 0);
    if (value) scalar = cv::Scalar(value[0], value[1], value[2], value[3]);
    cv::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType, scalar);
    return dst;
}

// ===================== IMGPROC =====================

// Pyramids
void* ocv_pyr_down(void* src_ptr, int dst_width, int dst_height) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::pyrDown(*src, *dst, cv::Size(dst_width, dst_height));
    return dst;
}

void* ocv_pyr_up(void* src_ptr, int dst_width, int dst_height) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::pyrUp(*src, *dst, cv::Size(dst_width, dst_height));
    return dst;
}

// GetGaussianKernel
void* ocv_get_gaussian_kernel(int ksize, double sigma, int ktype) {
    cv::Mat* kernel = new cv::Mat(cv::getGaussianKernel(ksize, sigma, ktype));
    return kernel;
}

// GetStructuringElement
void* ocv_get_structuring_element(int shape, int ksize_width, int ksize_height, int anchor_x, int anchor_y) {
    cv::Mat* kernel = new cv::Mat(cv::getStructuringElement(shape, cv::Size(ksize_width, ksize_height), cv::Point(anchor_x, anchor_y)));
    return kernel;
}

// Filter2D
void* ocv_filter2d(void* src_ptr, int ddepth, void* kernel_ptr, double delta) {
    if (!src_ptr || !kernel_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* kernel = static_cast<cv::Mat*>(kernel_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::filter2D(*src, *dst, ddepth, *kernel, cv::Point(-1, -1), delta, cv::BORDER_DEFAULT);
    return dst;
}

// SepFilter2D
void* ocv_sep_filter2d(void* src_ptr, int ddepth, void* kernel_x_ptr, void* kernel_y_ptr, double delta) {
    if (!src_ptr || !kernel_x_ptr || !kernel_y_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* kx = static_cast<cv::Mat*>(kernel_x_ptr);
    cv::Mat* ky = static_cast<cv::Mat*>(kernel_y_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::sepFilter2D(*src, *dst, ddepth, *kx, *ky, cv::Point(-1, -1), delta, cv::BORDER_DEFAULT);
    return dst;
}

// Scharr
void* ocv_scharr(void* src_ptr, int ddepth, int dx, int dy, double scale, double delta) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::Scharr(*src, *dst, ddepth, dx, dy, scale, delta, cv::BORDER_DEFAULT);
    return dst;
}

// Corner Harris
void* ocv_corner_harris(void* src_ptr, int blockSize, int ksize, double k, int borderType) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::cornerHarris(*src, *dst, blockSize, ksize, k, borderType);
    return dst;
}

// Good features to track
void* ocv_good_features_to_track(void* src_ptr, int maxCorners, double qualityLevel, double minDistance,
                                   int blockSize, bool useHarrisDetector, double k) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* corners = new cv::Mat();
    std::vector<cv::Point2f> corners_vec;
    cv::goodFeaturesToTrack(*src, corners_vec, maxCorners, qualityLevel, minDistance,
                            cv::Mat(), blockSize, useHarrisDetector, k);
    // Convert back to Mat
    cv::Mat(corners_vec).copyTo(*corners);
    return corners;
}

// HoughLines
void* ocv_hough_lines(void* src_ptr, double rho, double theta, int threshold, double srn, double stn) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* lines = new cv::Mat();
    std::vector<cv::Vec2f> lines_vec;
    cv::HoughLines(*src, lines_vec, rho, theta, threshold, srn, stn);
    cv::Mat(lines_vec).copyTo(*lines);
    return lines;
}

// HoughCircles
void* ocv_hough_circles(void* src_ptr, int method, double dp, double minDist, double param1, double param2,
                         int minRadius, int maxRadius) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* circles = new cv::Mat();
    std::vector<cv::Vec3f> circles_vec;
    cv::HoughCircles(*src, circles_vec, method, dp, minDist, param1, param2, minRadius, maxRadius);
    cv::Mat(circles_vec).copyTo(*circles);
    return circles;
}

// MorphologyEx with custom kernel
void* ocv_morphology_ex(void* src_ptr, int op, void* kernel_ptr) {
    if (!src_ptr || !kernel_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* kernel = static_cast<cv::Mat*>(kernel_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::morphologyEx(*src, *dst, op, *kernel);
    return dst;
}

// GetRotationMatrix2D
void* ocv_get_rotation_matrix2d(double center_x, double center_y, double angle, double scale) {
    cv::Mat* rot_mat = new cv::Mat(cv::getRotationMatrix2D(cv::Point2f(center_x, center_y), angle, scale));
    return rot_mat;
}

// WarpPerspective
void* ocv_warp_perspective(void* src_ptr, void* M_ptr, int dst_width, int dst_height, int flags, int borderMode, double borderValue) {
    if (!src_ptr || !M_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* M = static_cast<cv::Mat*>(M_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::warpPerspective(*src, *dst, *M, cv::Size(dst_width, dst_height), flags, borderMode, cv::Scalar(borderValue));
    return dst;
}

// GetAffineTransform
void* ocv_get_affine_transform(void* src_ptr, void* dst_ptr) {
    if (!src_ptr || !dst_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = static_cast<cv::Mat*>(dst_ptr);
    cv::Mat* M = new cv::Mat(cv::getAffineTransform(*src, *dst));
    return M;
}

// GetPerspectiveTransform
void* ocv_get_perspective_transform(void* src_ptr, void* dst_ptr) {
    if (!src_ptr || !dst_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = static_cast<cv::Mat*>(dst_ptr);
    cv::Mat* M = new cv::Mat(cv::getPerspectiveTransform(*src, *dst));
    return M;
}

// InvertAffineTransform
void* ocv_invert_affine_transform(void* M_ptr) {
    if (!M_ptr) return nullptr;
    cv::Mat* M = static_cast<cv::Mat*>(M_ptr);
    cv::Mat* iM = new cv::Mat();
    cv::invertAffineTransform(*M, *iM);
    return iM;
}

// Integral
void* ocv_integral(void* src_ptr, int sdepth) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* sum = new cv::Mat();
    cv::integral(*src, *sum, sdepth);
    return sum;
}

// CLAHE
void* ocv_create_clahe(double clipLimit, int tileWidth, int tileHeight) {
    cv::Ptr<cv::CLAHE> clahe = cv::createCLAHE(clipLimit, cv::Size(tileWidth, tileHeight));
    return new cv::Ptr<cv::CLAHE>(clahe);
}

void* ocv_apply_clahe(void* clahe_ptr, void* src_ptr, double clipLimit) {
    if (!clahe_ptr || !src_ptr) return nullptr;
    cv::Ptr<cv::CLAHE>* clahe = static_cast<cv::Ptr<cv::CLAHE>*>(clahe_ptr);
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    (*clahe)->apply(*src, *dst);
    return dst;
}

// AdaptiveThreshold
void* ocv_adaptive_threshold(void* src_ptr, double maxValue, int adaptiveMethod, int thresholdType, int blockSize, double C) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::adaptiveThreshold(*src, *dst, maxValue, adaptiveMethod, thresholdType, blockSize, C);
    return dst;
}

// DistanceTransform
void* ocv_distance_transform(void* src_ptr, int distanceType, int maskSize) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::distanceTransform(*src, *dst, distanceType, maskSize);
    return dst;
}

// ApplyColorMap
void* ocv_apply_color_map(void* src_ptr, int colormap) {
    if (!src_ptr) return nullptr;
    cv::Mat* src = static_cast<cv::Mat*>(src_ptr);
    cv::Mat* dst = new cv::Mat();
    cv::applyColorMap(*src, *dst, colormap);
    return dst;
}

// MatchTemplate
void* ocv_match_template(void* image_ptr, void* templ_ptr, int method) {
    if (!image_ptr || !templ_ptr) return nullptr;
    cv::Mat* image = static_cast<cv::Mat*>(image_ptr);
    cv::Mat* templ = static_cast<cv::Mat*>(templ_ptr);
    cv::Mat* result = new cv::Mat();
    cv::matchTemplate(*image, *templ, *result, method);
    return result;
}

// FindContours (simplified - returns Mat of contours)
void* ocv_find_contours(void* image_ptr, int mode, int method) {
    if (!image_ptr) return nullptr;
    cv::Mat* image = static_cast<cv::Mat*>(image_ptr);
    std::vector<std::vector<cv::Point>> contours;
    std::vector<cv::Vec4i> hierarchy;
    cv::findContours(*image, contours, hierarchy, mode, method);

    // Convert to a flat Mat for returning
    // This is simplified - real implementation would be more complex
    cv::Mat* dst = new cv::Mat();
    return dst;
}

// BoundingRect
void ocv_bounding_rect(void* points_ptr, int* x, int* y, int* width, int* height) {
    if (!points_ptr || !x || !y || !width || !height) return;
    cv::Mat* points = static_cast<cv::Mat*>(points_ptr);
    cv::Rect rect = cv::boundingRect(*points);
    *x = rect.x;
    *y = rect.y;
    *width = rect.width;
    *height = rect.height;
}

// MinAreaRect
void ocv_min_area_rect(void* points_ptr, float* center_x, float* center_y, float* width, float* height, float* angle) {
    if (!points_ptr || !center_x || !center_y || !width || !height || !angle) return;
    cv::Mat* points = static_cast<cv::Mat*>(points_ptr);
    cv::RotatedRect rect = cv::minAreaRect(*points);
    center_x = &rect.center.x;
    center_y = &rect.center.y;
    *width = rect.size.width;
    *height = rect.size.height;
    *angle = rect.angle;
}

// ConvexHull
void* ocv_convex_hull(void* points_ptr, bool clockwise) {
    if (!points_ptr) return nullptr;
    cv::Mat* points = static_cast<cv::Mat*>(points_ptr);
    std::vector<cv::Point> hull;
    cv::convexHull(*points, hull, clockwise);
    cv::Mat* dst = new cv::Mat(hull);
    return dst;
}

// ApproxPolyDP
void* ocv_approx_poly_dp(void* curve_ptr, double epsilon, bool closed) {
    if (!curve_ptr) return nullptr;
    cv::Mat* curve = static_cast<cv::Mat*>(curve_ptr);
    std::vector<cv::Point> curve_pts;
    // Simplified
    cv::Mat* dst = new cv::Mat();
    return dst;
}

// ===================== HIGHGUI =====================

// Show image (available in opencv-mobile)
void ocv_imshow(const char* name, void* mat_ptr) {
    if (!name || !mat_ptr) return;
    cv::Mat* mat = static_cast<cv::Mat*>(mat_ptr);
    cv::imshow(name, *mat);
}

// Wait key (available in opencv-mobile)
int ocv_wait_key(int delay) {
    return cv::waitKey(delay);
}

// ===================== VideoCapture =====================

// Create VideoCapture
void* ocv_create_videocapture() {
    cv::VideoCapture* cap = new cv::VideoCapture();
    return cap;
}

// Open camera device (returns 1 on success, 0 on failure)
// Note: opencv-mobile only supports opening device by index, not file
int ocv_videocapture_open(void* cap_ptr, int device) {
    if (!cap_ptr) return 0;
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(cap_ptr);
    return cap->open(device) ? 1 : 0;
}

// Check if VideoCapture is opened
int ocv_videocapture_is_opened(void* cap_ptr) {
    if (!cap_ptr) return 0;
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(cap_ptr);
    return cap->isOpened() ? 1 : 0;
}

// Get VideoCapture property (double)
double ocv_videocapture_get(void* cap_ptr, int prop) {
    if (!cap_ptr) return 0.0;
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(cap_ptr);
    return cap->get(prop);
}

// Set VideoCapture property (returns 1 on success, 0 on failure)
int ocv_videocapture_set(void* cap_ptr, int prop, double value) {
    if (!cap_ptr) return 0;
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(cap_ptr);
    return cap->set(prop, value) ? 1 : 0;
}

// Read frame using operator>>
// Note: opencv-mobile doesn't have read(), grab(), retrieve() methods
int ocv_videocapture_read(void* cap_ptr, void* mat_ptr) {
    if (!cap_ptr || !mat_ptr) return 0;
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(cap_ptr);
    cv::Mat* frame = static_cast<cv::Mat*>(mat_ptr);
    *cap >> *frame;
    return frame->empty() ? 0 : 1;
}

// Release VideoCapture
void ocv_videocapture_release(void* cap_ptr) {
    if (!cap_ptr) return;
    cv::VideoCapture* cap = static_cast<cv::VideoCapture*>(cap_ptr);
    cap->release();
    delete cap;
}

// ===================== VideoWriter =====================

// Create VideoWriter
void* ocv_create_videowriter() {
    cv::VideoWriter* writer = new cv::VideoWriter();
    return writer;
}

// Open video file for writing (returns 1 on success, 0 on failure)
// Note: opencv-mobile only supports name + port, no fourcc/fps/width/height
int ocv_videowriter_open(void* writer_ptr, const char* filename, int port) {
    if (!writer_ptr || !filename) return 0;
    cv::VideoWriter* writer = static_cast<cv::VideoWriter*>(writer_ptr);
    return writer->open(filename, port) ? 1 : 0;
}

// Check if VideoWriter is opened
int ocv_videowriter_is_opened(void* writer_ptr) {
    if (!writer_ptr) return 0;
    cv::VideoWriter* writer = static_cast<cv::VideoWriter*>(writer_ptr);
    return writer->isOpened() ? 1 : 0;
}

// Write frame
void ocv_videowriter_write(void* writer_ptr, void* mat_ptr) {
    if (!writer_ptr || !mat_ptr) return;
    cv::VideoWriter* writer = static_cast<cv::VideoWriter*>(writer_ptr);
    cv::Mat* frame = static_cast<cv::Mat*>(mat_ptr);
    writer->write(*frame);
}

// Release VideoWriter
void ocv_videowriter_release(void* writer_ptr) {
    if (!writer_ptr) return;
    cv::VideoWriter* writer = static_cast<cv::VideoWriter*>(writer_ptr);
    writer->release();
    delete writer;
}

}  // extern "C"
