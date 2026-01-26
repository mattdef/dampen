//! Image comparison utilities for visual regression testing.

use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;

/// Compares two images and returns the difference as a percentage.
///
/// # Arguments
///
/// * `baseline` - Path to the baseline (expected) image
/// * `actual` - Path to the actual (rendered) image
///
/// # Returns
///
/// Returns a value between 0.0 (identical) and 1.0 (completely different),
/// or an error if the images cannot be loaded or have different dimensions.
pub fn compare_images(
    baseline: impl AsRef<Path>,
    actual: impl AsRef<Path>,
) -> Result<f32, CompareError> {
    let baseline_img = image::open(baseline.as_ref())
        .map_err(|e| CompareError::LoadError(format!("Failed to load baseline: {}", e)))?;

    let actual_img = image::open(actual.as_ref())
        .map_err(|e| CompareError::LoadError(format!("Failed to load actual: {}", e)))?;

    compare_dynamic_images(&baseline_img, &actual_img)
}

/// Compares two DynamicImage instances.
pub fn compare_dynamic_images(
    baseline: &DynamicImage,
    actual: &DynamicImage,
) -> Result<f32, CompareError> {
    let (base_width, base_height) = baseline.dimensions();
    let (actual_width, actual_height) = actual.dimensions();

    if base_width != actual_width || base_height != actual_height {
        return Err(CompareError::DimensionMismatch {
            baseline: (base_width, base_height),
            actual: (actual_width, actual_height),
        });
    }

    let baseline_rgba = baseline.to_rgba8();
    let actual_rgba = actual.to_rgba8();

    let mut total_diff: f64 = 0.0;
    let total_pixels = (base_width * base_height) as f64;

    for y in 0..base_height {
        for x in 0..base_width {
            let base_pixel = baseline_rgba.get_pixel(x, y);
            let actual_pixel = actual_rgba.get_pixel(x, y);

            let diff = pixel_difference(base_pixel, actual_pixel);
            total_diff += diff as f64;
        }
    }

    // Normalize to 0.0-1.0 range
    let avg_diff = total_diff / total_pixels;
    let normalized_diff = (avg_diff / 255.0) as f32;

    Ok(normalized_diff)
}

/// Generates a diff image showing the differences between two images.
///
/// # Arguments
///
/// * `baseline` - The baseline image
/// * `actual` - The actual rendered image
/// * `output_path` - Where to save the diff image
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if generation fails.
pub fn generate_diff_image(
    baseline: impl AsRef<Path>,
    actual: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), CompareError> {
    let baseline_img = image::open(baseline.as_ref())
        .map_err(|e| CompareError::LoadError(format!("Failed to load baseline: {}", e)))?;

    let actual_img = image::open(actual.as_ref())
        .map_err(|e| CompareError::LoadError(format!("Failed to load actual: {}", e)))?;

    let (width, height) = baseline_img.dimensions();
    if actual_img.dimensions() != (width, height) {
        return Err(CompareError::DimensionMismatch {
            baseline: baseline_img.dimensions(),
            actual: actual_img.dimensions(),
        });
    }

    let baseline_rgba = baseline_img.to_rgba8();
    let actual_rgba = actual_img.to_rgba8();

    let mut diff_img = image::RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let base_pixel = baseline_rgba.get_pixel(x, y);
            let actual_pixel = actual_rgba.get_pixel(x, y);

            let diff = pixel_difference(base_pixel, actual_pixel);

            // Highlight differences in red
            let diff_pixel = if diff > 0 {
                Rgba([255, 0, 0, 255]) // Red for differences
            } else {
                // Show grayscale for matching pixels
                let gray = base_pixel[0];
                Rgba([gray, gray, gray, 255])
            };

            diff_img.put_pixel(x, y, diff_pixel);
        }
    }

    diff_img
        .save(output_path.as_ref())
        .map_err(|e| CompareError::SaveError(format!("Failed to save diff image: {}", e)))?;

    Ok(())
}

/// Calculates the difference between two pixels.
///
/// Returns a value representing the total color channel difference.
fn pixel_difference(a: &Rgba<u8>, b: &Rgba<u8>) -> u32 {
    let r_diff = (a[0] as i32 - b[0] as i32).unsigned_abs();
    let g_diff = (a[1] as i32 - b[1] as i32).unsigned_abs();
    let b_diff = (a[2] as i32 - b[2] as i32).unsigned_abs();
    let a_diff = (a[3] as i32 - b[3] as i32).unsigned_abs();

    r_diff + g_diff + b_diff + a_diff
}

/// Errors that can occur during image comparison.
#[derive(Debug, thiserror::Error)]
pub enum CompareError {
    /// Failed to load an image
    #[error("Failed to load image: {0}")]
    LoadError(String),

    /// Images have different dimensions
    #[error("Image dimension mismatch: baseline {baseline:?} vs actual {actual:?}")]
    DimensionMismatch {
        baseline: (u32, u32),
        actual: (u32, u32),
    },

    /// Failed to save an image
    #[error("Failed to save image: {0}")]
    SaveError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_difference_identical() {
        let pixel1 = Rgba([100, 150, 200, 255]);
        let pixel2 = Rgba([100, 150, 200, 255]);
        assert_eq!(pixel_difference(&pixel1, &pixel2), 0);
    }

    #[test]
    fn test_pixel_difference_different() {
        let pixel1 = Rgba([100, 150, 200, 255]);
        let pixel2 = Rgba([101, 151, 201, 255]);
        assert_eq!(pixel_difference(&pixel1, &pixel2), 3);
    }

    #[test]
    fn test_compare_identical_images() {
        let img1 = DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            100,
            100,
            Rgba([128, 128, 128, 255]),
        ));
        let img2 = DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            100,
            100,
            Rgba([128, 128, 128, 255]),
        ));

        let diff = compare_dynamic_images(&img1, &img2).unwrap();
        assert!(diff < 0.001, "Expected near-zero diff, got {}", diff);
    }

    #[test]
    fn test_compare_different_dimensions() {
        let img1 = DynamicImage::ImageRgba8(image::RgbaImage::new(100, 100));
        let img2 = DynamicImage::ImageRgba8(image::RgbaImage::new(200, 200));

        let result = compare_dynamic_images(&img1, &img2);
        assert!(matches!(
            result,
            Err(CompareError::DimensionMismatch { .. })
        ));
    }
}
