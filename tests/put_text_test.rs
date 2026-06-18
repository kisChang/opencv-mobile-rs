use opencv_mobile_rs::{
    create_font, destroy_font, get_text_size, imread, imwrite, put_text,
};

#[test]
fn test_put_text() {
    // 读取测试图片
    let mat = imread("tests/test.jpg", 1).expect("Failed to read test image");
    assert!(!mat.is_null());

    // 创建字体
    let font = create_font();
    assert!(!font.is_null());

    // 获取文本大小
    let (width, height) = get_text_size(font, "Hello OpenCV", 1.0);
    println!("Text size: {}x{}", width, height);
    assert!(width > 0);
    assert!(height > 0);

    // 绘制文本 (白色文字)
    put_text(mat, font, "Hello OpenCV", 50, 100, 255, 255, 255, 1.0);

    // 保存输出图片
    let result = imwrite("tests/test_output.jpg", mat, 95);
    assert!(result, "Failed to write output image");

    // 清理资源
    destroy_font(font);
}
