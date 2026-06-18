# opencv-mobile-rs

[English](./README.md) | [中文](./README.zh-CN.md)

Rust 绑定库，用于 [opencv-mobile](https://github.com/nihui/opencv-mobile) https://github.com/nihui/opencv-mobile ，这是一个针对移动和嵌入式设备优化的轻量级 OpenCV 构建版本。

## 功能特性

- **基础操作**：Mat 创建、克隆、属性获取（宽度、高度、通道、数据）
- **图像处理**：resize、blur、gaussian blur、median blur、bilateral filter、threshold、形态学操作
- **颜色转换**：BGR2GRAY、BGR2RGB、BGR2HSV、BGR2YUV 等
- **特征检测**：ORB、SIFT、BFMatcher、绘制关键点/匹配
- **计算机视觉**：Canny、Sobel、Laplacian、equalizeHist、warpAffine、rotate、flip、crop
- **照片模块**：inpaint、fastNlMeansDenoising、edgePreservingFilter、stylization
- **视频处理**：光流（Lucas-Kanade、Farneback）、VideoCapture、VideoWriter
- **HighGUI**：imshow、waitKey
- **其他功能**：CLAHE、自适应阈值、距离变换、颜色映射、模板匹配、轮廓检测、Hough 变换

## 环境要求

- Rust 1.56+
- C++ 编译器（gcc/clang）
- CMake（用于构建 opencv-mobile）

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
opencv-mobile-rs = "0.1"
```

## 使用示例

```rust
use opencv_mobile_rs::*;

// 读取图像
let img = imread("test.jpg", IMREAD_COLOR).expect("Failed to read image");

// 获取图像属性
let width = get_width(img);
let height = get_height(img);
let channels = get_channels(img);
println!("图像: {}x{} {} 通道", width, height, channels);

// 调整图像大小
let resized = resize(img, 200, 200);

// 转换为灰度图
let gray = cvt_color(img, COLOR_BGR2GRAY);

// 高斯模糊
let blurred = gaussian_blur(gray, 5, 1.0);

// 保存结果
imwrite("output.jpg", blurred, 95);

// 释放内存
free_mat(img);
free_mat(resized);
free_mat(gray);
free_mat(blurred);
```

## 构建方式

默认情况下，库会静态构建 opencv-mobile。也可以使用动态链接：

```toml
[dependencies]
opencv-mobile-rs = { version = "0.1", default-features = false, features = ["dynamic"] }
```

## 许可证

Apache License 2.0 - 参见 [LICENSE](LICENSE) 文件。
