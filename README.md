
# Data Processing Script Conversion: Python to Rust

## Project Overview
This project involves rewriting a Python data processing script in Rust to compare performance, particularly speed and resource usage. The project evaluates the efficiency gains Rust offers over Python in processing the same dataset.

### Original Python Code
The original Python script is located [here](https://github.com/nogibjj/zichun-miniproject-2), which uses Pandas for data manipulation.

### Converted Rust Code
The Rust version implements similar data processing logic, aiming to demonstrate improvements in execution time and memory efficiency.

## Project Structure
```
├── src
│   ├── main.rs              # Main Rust code implementing data processing
│   └── lib.rs               # Optional modular code for Rust functions
├── python_script.py         # Original Python code for data processing
├── performance_report.md    # Performance comparison between Python and Rust
├── README.md                # Project documentation
└── .github/workflows        # CI/CD setup for GitHub Actions
    └── ci.yml               # CI configuration file
```

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or later)
- Python 3.x with required libraries (`pandas`)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust’s package manager)
- Dataset (default: `data/titanic.csv`)

### Setup Instructions

1. **Clone the Repository**:
   ```bash
   git clone <repository_url>
   cd repository_name
   ```

2. **Install Dependencies**:
   - For Python:
     ```bash
     pip install -r requirements.txt
     ```
   - For Rust (add any dependencies if specified in `Cargo.toml`):
     ```bash
     cargo build
     ```

3. **Run the Scripts**:
   - **Python Script**:
     ```bash
     python python_script.py
     ```
   - **Rust Script**:
     ```bash
     cargo run --release
     ```

### CI/CD Pipeline
The project includes a CI/CD pipeline using GitHub Actions, configured to:
- Run tests on Rust code (`cargo test`)
- Check formatting, linting, and build errors
- Verify successful execution for both Python and Rust scripts across multiple Python versions

## Performance Comparison
The [performance comparison report](performance_report.md) documents:
- **Execution Time**: The time taken by both Python and Rust scripts
- **Memory Usage**: RAM usage for each script
- **Efficiency Gains**: Highlighting areas where Rust improves performance

## Results Summary
| Language | Execution Time | Memory Usage | Notes                 |
|----------|----------------|--------------|-----------------------|
| Python   | [x] sec        | [x] MB       | Uses Pandas library   |
| Rust     | [x] sec        | [x] MB       | Uses Polars or native functions for processing |

## Deliverables
- **Rust and Python Scripts**: Located in `src/main.rs` and `python_script.py`
- **Performance Comparison Report**: See `performance_report.md` for detailed analysis
- **CI/CD Setup**: GitHub Actions configuration in `.github/workflows/ci.yml`
