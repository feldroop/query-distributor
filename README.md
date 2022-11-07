# query-distributor

This is a command line program that reads the output of the [raptor](https://github.com/seqan/raptor) search command and a .fastq query file. It then creates a .fastq file for every genome bin of the raptor output. That file will contain the reads from the .fastq query file that were classified (according to the raptor output) its respective bin.

## Usage

Run `query-distributor --help` for a description of the command line interface.

## Disclaimer

Please note that this program is merely a prototype. It is not fully optimized, user-friendly and robust.
