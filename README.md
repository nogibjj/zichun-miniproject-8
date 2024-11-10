
# Rust Data Processing Script - Rewritten from Python

## Project Overview

This project is a Rust rewrite of an existing Python data processing script in [zichun-miniproject-2](https://github.com/nogibjj/zichun-miniproject-2). The goal was to replicate the functionality of the Python script in Rust while improving speed and resource usage.

### Original Python Script (zichun-miniproject-2)

The Python script performs the following tasks:
- Loads a CSV dataset.
- Calculates summary statistics for a specific column.
- Generates a bar chart for visualization.
- Saves a report and image output.

### Rewritten Rust Script (zichun-miniproject-8)

The Rust script mirrors the Python script's functionality with improvements in performance. Rust’s memory management and compiled nature make it faster and more efficient for processing data compared to Python.

### Key Improvements

- **Speed**: The Rust version processes data faster due to optimized compilation and memory handling.
- **Resource Usage**: Rust's strict memory management reduces the memory overhead, making it more efficient for large data sets.

## Performance Comparison

| Metric               | Python Script       | Rust Script       | Improvement       |
|----------------------|---------------------|-------------------|-------------------|
| **User Time**        | 1.70s              | 0.08s            | 21.25x faster     |
| **System Time**      | 1.56s              | 0.07s            | 22.29x faster     |
| **Total Time**       | 0.837s             | 1.331s           | Python faster (1.6x) |
 
## Deliverables

- **Rust and Python Scripts**: Both the rewritten Rust script (in this repository) and the original Python script (in `zichun-miniproject-2`).
- **Output Files**:
  - **Summary Report** (`output/summary_report.md`): Contains summary statistics for the dataset.
  - **Visualization Image** (`output/total_medals_by_top_50_countries.svg`): A bar chart visualization generated from the dataset.

## Repository Structure

```
zichun-miniproject-8/
├── src/
│   └── main.rs                    # Main Rust script
├── output/
│   ├── summary_report.md          # Generated summary report
│   └── total_medals_by_top_50_countries.svg  # Generated bar chart visualization
├── .github/workflows/
│   └── ci.yml                     # GitHub Actions CI/CD pipeline
├── README.md                      # Project documentation (this file)
├── Cargo.toml                     # Rust dependencies and configuration
```

## Setup and Usage

### Requirements

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Running the Rust Script

1. Clone the repository:
   ```bash
   git clone https://github.com/nogibjj/zichun-miniproject-8.git
   cd zichun-miniproject-8
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the Rust script to generate outputs:
   ```bash
   cargo run --release
   ```

## CI/CD Pipeline

The GitHub Actions CI/CD pipeline performs the following tasks:
1. Builds the Rust project.
2. Runs the main Rust script to generate outputs.
3. Commits the generated files (`summary_report.md` and `total_medals_by_top_50_countries.svg`) if there are updates.
