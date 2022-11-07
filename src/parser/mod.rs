mod tests;

use std::{collections::HashMap, fs, path::Path};

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, newline, not_line_ending, tab},
    multi::{fold_many0, many_till, separated_list0},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
pub struct RaptorSearchOutput {
    pub bin_id_to_name: HashMap<usize, String>,
    pub read_name_to_bin_ids: HashMap<String, Vec<usize>>,
}

impl RaptorSearchOutput {
    pub fn from_file(path: &Path) -> Self {
        let file_content =
            fs::read_to_string(path).expect("Could not read raptor search output file");

        Self::from_str(&file_content)
    }

    fn from_str(s: &str) -> Self {
        let (remaining, (bin_id_to_name, read_name_to_bin_ids)) =
            nom::sequence::pair(full_header, full_body)(s)
                .expect("Failed to parse the raptor search output");

        assert!(remaining.is_empty());

        RaptorSearchOutput {
            bin_id_to_name,
            read_name_to_bin_ids,
        }
    }
    // check whether bin ids assigned to reads actually have been mentioned in the header
    pub fn is_valid(&self) -> bool {
        self.read_name_to_bin_ids
            .values()
            .flatten()
            .all(|id| self.bin_id_to_name.contains_key(id))
    }
}

// subparser functions
fn bin_header_line(input: &str) -> IResult<&str, (usize, String)> {
    let id = preceded(char('#'), digit1);
    let name = terminated(not_line_ending, newline);
    let line = separated_pair(id, tab, name);

    line.map(|(id, name)| (parse_usize(id), name.to_owned()))
        .parse(input)
}

fn full_header(input: &str) -> IResult<&str, HashMap<usize, String>> {
    many_till(bin_header_line, tag("#QUERY_NAME\tUSER_BINS\n"))
        .map(|(mappings, _)| mappings.into_iter().collect())
        .parse(input)
}

fn body_line(input: &str) -> IResult<&str, (String, Vec<usize>)> {
    let read_name = take_while1(|c| c != '\t');
    let possible_bins = separated_list0(tag(","), digit1::<&str, _>);
    let terminated_possible_bins = terminated(possible_bins, newline);
    let line = separated_pair(read_name, tab, terminated_possible_bins);

    line.map(|(read_name, possible_bins)| (read_name.to_owned(), parse_bin_ids(possible_bins)))
        .parse(input)
}

fn full_body(input: &str) -> IResult<&str, HashMap<String, Vec<usize>>> {
    fold_many0(body_line, HashMap::new, |mut map, (read_name, bin_ids)| {
        map.insert(read_name, bin_ids);
        map
    })(input)
}

fn parse_usize(data: &str) -> usize {
    data.parse()
        .expect("Failed to parse a number where one should be")
}

fn parse_bin_ids(ids: Vec<&str>) -> Vec<usize> {
    ids.into_iter().map(parse_usize).collect()
}
