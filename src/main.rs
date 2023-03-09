use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Import sphere points

    let sp_path = "/home/christopher/git/ecosystem_dimension_reduction/input_files/10000SpherePoints.csv";
    let sp_file = std::fs::File::open(sp_path)?;
    let mut sp_csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(sp_file);

    // Populate the K-d tree

    let mut kdtree = kiddo::KdTree::new();
    for (i, row_res) in sp_csv_reader.records().enumerate() {
        let row = row_res?;
        let point = [row[0].parse::<f64>()?, row[1].parse::<f64>()?, row[2].parse::<f64>()?];
        kdtree.add(&point, i+1)?;
    }

    // Create raw data reader

    let path = "/home/christopher/Documents/eBirdData/rawData/0015292-230224095556074.zip";
    let file = std::fs::File::open(path)?;
    let mut zip_archive = zip::read::ZipArchive::new(file)?;
    let csv_file = zip_archive.by_index(0)?;
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(csv_file);


    // Iterate through raw data

    let mut cell_counts = HashMap::new();

     // 1060970490 is the number of observations in the dataset according to the GDIF website
    let bar = indicatif::ProgressBar::new(1060970490);
    bar.set_style(indicatif::ProgressStyle::with_template("[{elapsed_precise}] eta: {eta_precise} {bar:40} {percent}% {pos}/{len} {per_sec}")?);
    for row_res in csv_reader.records() {
        bar.inc(1);

        let res = (|| {
            let row = row_res?;
            let species = String::from(&row[13]);
            let lat = &row[21].parse::<f64>()?;
            let long = &row[22].parse::<f64>()?;
            let count = &row[19].parse::<u32>().unwrap_or(1);
            let lat_rad = lat.to_radians();
            let long_rad = long.to_radians();
            let xyz = [lat_rad.cos() * long_rad.cos(), lat_rad.cos() * long_rad.sin(), lat_rad.sin()];
            let nearest = kdtree.nearest_one(&xyz, &kiddo::distance::squared_euclidean)?;
            *cell_counts.entry(nearest.1).or_insert(HashMap::new()).entry(species).or_insert(0) += count;
            // *cell_counts.entry(nearest.1).or_insert(0) += count;
            Ok::<(), Box<dyn std::error::Error>>(())
        })();

        if let Err(err) = res {
            println!("Row error: {:?}", err);
        }
    }

    // Write results to file

    let output_path = "/home/christopher/git/ecosystem_dimension_reduction/output_files/10000CellSpeciesCounts.csv";
    let output_file = std::fs::File::create(output_path)?;
    let mut writer = csv::Writer::from_writer(output_file);
    for (cell, species_counts) in cell_counts.iter() {
        for (species, count) in species_counts.iter() {
            writer.serialize((cell, species, count))?;
        }
    }

    Ok(())
}
