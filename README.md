# query-distributor

Command line program to read the output of the [raptor](https://github.com/seqan/raptor) search command and a .fastq query file. This program then creates a fastq. file for every genome bin of the raptor output.This file will contain the reads from the .fastq query file that were classified (according to the raptor output) into the respective bin.

## Usage

Run `query-distributor --help` for a description of the command line interface.

## Disclaimer

Please note that this program is merely a prototype. It is not fully optimized, user-friendly and robust.
