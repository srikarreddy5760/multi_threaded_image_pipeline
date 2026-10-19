use crate::processor::process_image;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub const WORKER_COUNT: usize = 4;

/// Processes images using multiple worker threads.
pub fn parallel_processing(
    image_files: &[String],
    output_folder: &str,
) -> Result<(), Box<dyn Error>> {
    // Channel for sending image-processing jobs
    let (sender, receiver) = mpsc::channel::<String>();

    // Allow multiple workers to share the receiver
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::new();

    // Create worker threads
    for worker_id in 1..=WORKER_COUNT {
        let receiver = Arc::clone(&receiver);
        let output_folder = output_folder.to_string();

        let worker = thread::spawn(move || {
            loop {
                // Get the next image job
                let image_path = {
                    let receiver = receiver.lock().unwrap();

                    match receiver.recv() {
                        Ok(path) => path,
                        Err(_) => break,
                    }
                };

                let file_name = Path::new(&image_path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown");

                println!(
                    "[Worker {}] Processing {}",
                    worker_id, file_name
                );

                match process_image(&image_path, &output_folder) {
                    Ok(_) => {
                        println!(
                            "[Worker {}] Completed {}",
                            worker_id, file_name
                        );
                    }

                    Err(error) => {
                        println!(
                            "[Worker {}] Failed {}: {}",
                            worker_id, file_name, error
                        );
                    }
                }
            }
        });

        workers.push(worker);
    }

    // Send image jobs
    for image_path in image_files {
        sender.send(image_path.clone())?;
    }

    // Close channel
    drop(sender);

    // Wait for all workers
    for worker in workers {
        worker
            .join()
            .map_err(|_| "A worker thread panicked")?;
    }

    Ok(())
}