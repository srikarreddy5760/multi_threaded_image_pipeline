mod processor;
mod worker;

use processor::{is_image_file, process_image};
use std::error::Error;
use std::fs;
use std::time::Instant;
use worker::{parallel_processing, WORKER_COUNT};

/// Finds all supported image files in the input folder.
fn get_image_files(folder: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && is_image_file(&path) {
            files.push(path.to_string_lossy().to_string());
        }
    }

    files.sort();

    Ok(files)
}

/// Processes all images sequentially.
fn sequential_processing(
    image_files: &[String],
    output_folder: &str,
) -> Result<(), Box<dyn Error>> {
    for image_path in image_files {
        process_image(image_path, output_folder)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("==============================================");
    println!("   MULTI-THREADED IMAGE PROCESSING PIPELINE");
    println!("==============================================\n");

    let input_folder = "input";
    let sequential_folder = "sequential_output";
    let parallel_folder = "output";

    // Create output folders
    fs::create_dir_all(sequential_folder)?;
    fs::create_dir_all(parallel_folder)?;

    // Find input images
    let image_files = get_image_files(input_folder)?;

    if image_files.is_empty() {
        println!("No images found in the input folder.");
        println!("Please add JPG, JPEG, or PNG images.");
        return Ok(());
    }

    println!("Input folder       : {}", input_folder);
    println!("Images found       : {}", image_files.len());
    println!("Worker threads     : {}", WORKER_COUNT);
    println!();

    // ==========================================
    // SEQUENTIAL PROCESSING
    // ==========================================

    println!("----------------------------------------------");
    println!("SEQUENTIAL PROCESSING");
    println!("----------------------------------------------");

    let sequential_start = Instant::now();

    sequential_processing(
        &image_files,
        sequential_folder,
    )?;

    let sequential_time = sequential_start.elapsed();

    println!(
        "Sequential time    : {:.3} seconds\n",
        sequential_time.as_secs_f64()
    );

    // ==========================================
    // MULTI-THREADED PROCESSING
    // ==========================================

    println!("----------------------------------------------");
    println!("MULTI-THREADED PROCESSING");
    println!("----------------------------------------------");

    let parallel_start = Instant::now();

    parallel_processing(
        &image_files,
        parallel_folder,
    )?;

    let parallel_time = parallel_start.elapsed();

    println!(
        "\nMulti-threaded time: {:.3} seconds",
        parallel_time.as_secs_f64()
    );

    // ==========================================
    // PERFORMANCE RESULTS
    // ==========================================

    println!("\n==============================================");
    println!("           PERFORMANCE RESULTS");
    println!("==============================================");

    println!(
        "Images processed   : {}",
        image_files.len()
    );

    println!(
        "Worker threads     : {}",
        WORKER_COUNT
    );

    println!(
        "Sequential time    : {:.3} seconds",
        sequential_time.as_secs_f64()
    );

    println!(
        "Multi-threaded time: {:.3} seconds",
        parallel_time.as_secs_f64()
    );

    if parallel_time.as_secs_f64() > 0.0 {
        let speedup =
            sequential_time.as_secs_f64()
            / parallel_time.as_secs_f64();

        println!("Speedup            : {:.2}x", speedup);
    }

    println!("==============================================");
    println!("Processing completed successfully!");
    println!("==============================================");

    println!("\nOutput folders:");
    println!("Sequential results : {}", sequential_folder);
    println!("Parallel results   : {}", parallel_folder);

    Ok(())
}