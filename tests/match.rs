use std::io::BufReader;

#[test]
fn find_a_match() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new("lorem ipsum\ndolor sit amet".as_bytes());
    let mut result = Vec::new();

    grab::find_matches(reader, &mut result, "lorem")?;
    assert_eq!(result, b"lorem ipsum\n");

    Ok(())
}
