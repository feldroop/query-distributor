mod cli;
mod parser;

use clap::Parser;
use cli::Cli;

use parser::RaptorSearchOutput;

use fastq::Record;

use std::collections::HashMap;
use std::io::BufWriter;
use std::path::PathBuf;
use std::{
    fs::{self, File},
    path::Path,
};

fn main() {
    let cli = Cli::parse();

    let search_output = parser::RaptorSearchOutput::from_file(&cli.raptor_search_output);

    if !search_output.is_valid() {
        println!("Search output is not valid");
        std::process::exit(-1);
    }

    let mut output_writers = setup_output_folder_and_files(&search_output, &cli.output_folder);

    let queries_file = File::open(&cli.queries).expect("Could not open query file");
    let queries = fastq::Parser::new(queries_file);

    queries
        .each(|record| {
            let name =
                std::str::from_utf8(record.head()).expect("Query name surprisingly was not UTF-8");

            let bin_ids = search_output.read_name_to_bin_ids.get(name).expect(
                "Query file contains a read that is not mentioned in the raptor output file",
            );

            for bin_id in bin_ids {
                let writer = output_writers.get_mut(bin_id).unwrap();
                record
                    .write(writer)
                    .expect("Failed to read to output .fastq");
            }

            true
        })
        .expect("Failed to parse through the .fastq file");
}

fn setup_output_folder_and_files(
    search_output: &RaptorSearchOutput,
    output_folder: &Path,
) -> HashMap<usize, BufWriter<File>> {
    let mut output_writers = HashMap::new();

    if !output_folder.exists() {
        fs::create_dir_all(output_folder).expect("Could not create output folder");
    }

    for (bin_id, bin_name) in &search_output.bin_id_to_name {
        // I assume the bin name is a file path. I want to name the output
        // as the filename + .fastq (including any previous extension of the filename)
        let file_path = PathBuf::from(bin_name);
        let mut filename_string = file_path.file_name().unwrap().to_str().unwrap().to_owned();
        filename_string += ".fastq";

        let mut output_path = output_folder.to_owned();
        output_path.push(filename_string);

        let f = File::create(output_path).expect("Could not create the output file for a bin");
        let writer = BufWriter::new(f);
        output_writers.insert(*bin_id, writer);
    }

    output_writers
}
