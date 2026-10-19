use image::ImageReader;
use std::error::Error;
use std::path::Path;

/// Checks whether a file is a supported image.
pub fn is_image_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(extension) => {
            let ext = extension.to_lowercase();

            ext == "jpg" || ext == "jpeg" || ext == "png"
        }
        None => false,
    }
}

/// Processes one image through the image-processing pipeline.
///
/// Pipeline:
/// 1. Load image
/// 2. Resize
/// 3. Convert to grayscale
/// 4. Apply blur
/// 5. Save result
pub fn process_image(
    input_path: &str,
    output_folder: &str,
) -> Result<(), Box<dyn Error>> {
    // Load image
    let image = ImageReader::open(input_path)?.decode()?;

    // Stage 1: Resize
    let resized = image.resize(
        800,
        600,
        image::imageops::FilterType::Lanczos3,
    );

    // Stage 2: Convert to grayscale
    let grayscale = resized.grayscale();

    // Stage 3: Apply blur
    let blurred = grayscale.blur(2.0);

    // Get original filename
    let file_name = Path::new(input_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("image.jpg");

    // Create output filename
    let output_path = format!("{}/processed_{}", output_folder, file_name);

    // Save processed image
    blurred.save(&output_path)?;

    Ok(())
}