use assert_cmd::cargo::*;
use assert_fs::fixture::*;
use predicates::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("grab");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));

    Ok(())
}

#[test]
fn pattern_empty_string() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("grab");

    cmd.arg("").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No pattern provided"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("grab");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
