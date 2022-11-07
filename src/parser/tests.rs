#[allow(unused_imports)]
use super::*;

#[test]
fn simple_bin_header_line() {
    let input = "#42\t1_ASM285049v1_genomic.fna\n";

    let (rest, (id, name)) = bin_header_line(input).unwrap();

    assert!(rest.is_empty());
    assert_eq!(id, 42);
    assert_eq!(name, "1_ASM285049v1_genomic.fna");
}

#[test]
fn simple_full_header() {
    let input = r"#0	1_ASM321425v1_genomic.fna
#1	1_ASM321449v1_genomic.fna
#2	1_ASM321457v1_genomic.fna
#QUERY_NAME	USER_BINS
";

    let (rest, map) = full_header(input).unwrap();

    assert!(rest.is_empty());

    let mut expected = HashMap::new();
    expected.insert(0, "1_ASM321425v1_genomic.fna".to_string());
    expected.insert(1, "1_ASM321449v1_genomic.fna".to_string());
    expected.insert(2, "1_ASM321457v1_genomic.fna".to_string());

    assert_eq!(expected, map);
}

#[test]
fn simple_body_line() {
    let input = "1_ASM44157v1_genomic0\t124,125,126\n";

    let (rest, (read_name, bin_ids)) = body_line(input).unwrap();

    assert!(rest.is_empty());
    assert_eq!(read_name, "1_ASM44157v1_genomic0");
    assert_eq!(bin_ids, [124, 125, 126]);
}

#[test]
fn simple_full_body() {
    let input = r"1_ASM94791v1_genomic1	218,221,223,290
1_ASM94791v1_genomic2	290
1_ASM94791v1_genomic3	
";

    let (rest, map) = full_body(input).unwrap();

    assert!(rest.is_empty());

    let mut expected = HashMap::new();
    expected.insert(
        "1_ASM94791v1_genomic1".to_string(),
        [218, 221, 223, 290].to_vec(),
    );
    expected.insert("1_ASM94791v1_genomic2".to_string(), [290].to_vec());
    expected.insert("1_ASM94791v1_genomic3".to_string(), [].to_vec());

    assert_eq!(expected, map);
}

#[test]
fn simple_from_raptor_search_output() {
    let search_output = r"#12345	blabliblub
#54321	blimbibum
#QUERY_NAME	USER_BINS
wupdidup	4,5,6
brumdidum	
";

    let read_classifier = RaptorSearchOutput::from_str(search_output);

    let mut expected_bin_id_to_name = HashMap::new();
    expected_bin_id_to_name.insert(12345, "blabliblub".to_string());
    expected_bin_id_to_name.insert(54321, "blimbibum".to_string());

    assert_eq!(expected_bin_id_to_name, read_classifier.bin_id_to_name);

    let mut expected_read_name_to_bin_ids = HashMap::new();
    expected_read_name_to_bin_ids.insert("wupdidup".to_string(), [4, 5, 6].to_vec());
    expected_read_name_to_bin_ids.insert("brumdidum".to_string(), [].to_vec());

    assert_eq!(
        expected_read_name_to_bin_ids,
        read_classifier.read_name_to_bin_ids
    );
}
