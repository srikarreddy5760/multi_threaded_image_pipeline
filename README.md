# Multi-threaded Image Processing Pipeline

## 📌 Project Overview

The **Multi-threaded Image Processing Pipeline** is a Rust-based project that processes multiple images using both sequential and parallel execution. It demonstrates how multithreading can improve processing performance by distributing image-processing tasks across multiple worker threads.

The project resizes images, converts them to grayscale, applies Gaussian blur, and compares the execution time of sequential and multithreaded processing.

## 🎯 Objectives

- Implement image processing using Rust.
- Understand multithreading and concurrency.
- Process multiple images using worker threads.
- Compare sequential and parallel execution times.
- Measure performance improvements using benchmarking.

## ✨ Features

- Supports JPG, JPEG, and PNG images.
- Resizes images to fit within **800 × 600 pixels**, preserving the aspect ratio.
- Converts images to grayscale.
- Applies Gaussian blur with a sigma value of `2.0`.
- Implements sequential and multithreaded processing.
- Uses four worker threads for parallel processing.
- Saves processed images in separate output directories.
- Measures and compares execution times.

## 🛠️ Technologies Used

- **Programming Language:** Rust
- **Image Processing Library:** `image`
- **Concurrency:** `std::thread`
- **Communication:** `std::sync::mpsc`
- **Synchronization:** `Arc` and `Mutex`
- **Build Tool:** Cargo

## 📂 Project Structure

```text
multi_threaded_image_processing_pipeline/
│
├── src/
│   ├── main.rs
│   ├── processor.rs
│   └── worker.rs
│
├── input/
│   ├── image1.jpg
│   ├── image2.jpg
│   ├── image3.jpg
│   ├── image4.jpg
│   └── image5.jpg
│
├── output/                 # Parallel processing results
├── sequential_output/      # Sequential processing results
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## ⚙️ Requirements

Before running the project, install:

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the required C++ build tools on Windows, if needed.

Verify your installation:

```powershell
rustc --version
cargo --version
```

## 🚀 How to Run

### 1. Clone the repository

```bash
git clone YOUR_GITHUB_REPOSITORY_URL
```

Replace `YOUR_GITHUB_REPOSITORY_URL` with your actual repository URL.

### 2. Navigate to the project directory

```bash
cd multi_threaded_image_processing_pipeline
```

### 3. Add input images

Place your JPG, JPEG, or PNG images inside the `input/` directory.

### 4. Run the project

```bash
cargo run
```

Cargo will compile and execute the Rust application. The program processes the images sequentially and then using multiple worker threads.

### 5. View the results

- `sequential_output/` contains the images processed sequentially.
- `output/` contains the images processed using multithreading.

## 📊 Performance Results

In one recorded test using five input images, the following execution times were observed:

| Processing Method | Execution Time |
|---|---:|
| Sequential Processing | 6.973 seconds |
| Multithreaded Processing | 2.597 seconds |
| Speedup | 2.68× |
| Execution Time Reduction | 62.8% |

These measurements demonstrate the potential performance benefits of multithreading for image-processing workloads. Actual results may vary depending on hardware, image sizes, and system load.

## 🧠 Concepts Demonstrated

- Rust programming
- Multithreading and concurrency
- Thread creation and management
- Message passing using channels
- Shared-state synchronization
- Image manipulation
- Execution-time measurement
- Sequential versus parallel performance comparison

## 🔮 Future Enhancements

- Support additional image formats.
- Allow users to configure image dimensions and blur intensity.
- Add a graphical user interface.
- Generate detailed performance reports.
- Test with larger image datasets and different worker-thread counts.
- Improve error handling and processing status reporting.

## 👨‍💻 Author

**Vura Srikar Reddy**

B.Tech – Computer Science and Engineering (AI & ML)  
Vardhaman College of Engineering, Hyderabad

## 📄 License

This project was developed for academic and educational purposes.
