use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "query-distributor",
    author,
    version,
    about = "Write .fastq files of reads that might be in a genome bin according to raptor search."
)]
pub struct Cli {
    /// The file that contains the output of raptor search, reads classified into genome bins
    #[arg(short, long)]
    pub raptor_search_output: PathBuf,

    /// The .fastq file with the queries that were input to raptor search and will be distributed
    #[arg(short, long)]
    pub queries: PathBuf,

    /// The folder where a .fastq file for every genome bin will be created
    #[arg(short, long)]
    pub output_folder: PathBuf,
}
