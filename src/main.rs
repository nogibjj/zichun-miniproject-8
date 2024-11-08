use polars::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use plotters::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Ensure the output directory exists
    let output_dir = "output";
    fs::create_dir_all(output_dir)?;

    // Load the dataset
    let file_path = "data/medals_total.csv";
    let mut df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    println!("DataFrame Loaded:\n{}", df);

    // Cast 'Total' column to Float64 to avoid SchemaMismatch error and rename it
    let mut total_float = df.column("Total")?.cast(&DataType::Float64)?;
    let total_float = total_float.rename("Total").clone(); // Renaming and cloning as a new Series
    df.with_column(total_float)?; // Add renamed column back to DataFrame

    // Generate summary statistics for the 'Total' column
    let total_column = df.column("Total")?.f64()?;
    let mean = total_column.mean().unwrap_or(f64::NAN);
    let median = total_column.median().unwrap_or(f64::NAN);
    let std_dev = total_column.std().unwrap_or(f64::NAN);

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Standard Deviation: {}", std_dev);

    // Sort by 'Total' column and select the top 50 countries
    let top_countries = df
        .lazy()
        .sort("Total", SortOptions {
            descending: true,
            nulls_last: true,
        })
        .slice(0, 50)
        .collect()?;

    // Create a bar chart for the total medals by the top 50 countries
    let root = SVGBackend::new("output/total_medals_by_top_50_countries.svg", (1000, 600)).into_drawing_area();
    root.fill(&WHITE).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let max_total = top_countries.column("Total")?.max::<f64>().unwrap_or(1.0);
    let mut chart = ChartBuilder::on(&root)
        .caption("Total Medals by Top 50 Countries", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0..50, 
            0.0..max_total)?;

    chart.configure_mesh().x_labels(50).y_desc("Total Medals").draw().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let country_labels: Vec<String> = top_countries.column("country")?
        .utf8()?
        .into_iter()
        .map(|c| c.unwrap_or("").to_string())
        .collect();

    // Draw the bar chart
    chart.draw_series(
        top_countries.column("Total")?
            .f64()?
            .into_iter()
            .enumerate()
            .map(|(i, total)| {
                let x = i as i32; // Cast `usize` to `i32`
                let y = total.unwrap_or(0.0);
                Rectangle::new([(x, 0.0), (x + 1, y)], BLUE.filled())
            })
    ).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Save summary statistics to a Markdown file
    let mut file = File::create("output/summary_report.md")?;
    writeln!(file, "# Summary Statistics Report (Entire Dataset)\n")?;
    writeln!(file, "## Summary Statistics for Total Medals (All Countries)\n")?;
    writeln!(file, "- **Mean**: {:.2}", mean)?;
    writeln!(file, "- **Median**: {:.2}", median)?;
    writeln!(file, "- **Standard Deviation**: {:.2}", std_dev)?;
    writeln!(file, "![Total Medals by Top 50 Countries](total_medals_by_top_50_countries.svg)\n")?;

    Ok(())
}
