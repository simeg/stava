#![allow(non_snake_case)]
mod cli {
    use assert_cmd::prelude::*;
    use predicates::str::{contains, ends_with};
    use std::io::Write;
    use std::process::Command;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_output_ends_with_newline() -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("speling");

        cmd.assert().success().stdout(ends_with("\n"));
        Ok(())
    }

    #[test]
    fn test_returns_match__when_match_is_in_default_words() -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        // I know this word is in the default set of words (by manually checking)
        cmd.arg("speling");

        cmd.assert().success().stdout(contains("spelling"));
        Ok(())
    }

    #[test]
    fn test_returns_match__when_match_is_in_file() -> TestResult {
        let mut tmp_file = tempfile::NamedTempFile::new()?;
        tmp_file.write_all("spelling, and some other words".as_bytes())?;

        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("speling");
        cmd.arg(tmp_file.path());

        cmd.assert().success().stdout(contains("spelling"));
        Ok(())
    }

    #[test]
    fn test_returns_match__when_match_is_in_any_of_files() -> TestResult {
        let mut tmp_file1 = tempfile::NamedTempFile::new()?;
        tmp_file1.write_all("no match in this file".as_bytes())?;
        let mut tmp_file2 = tempfile::NamedTempFile::new()?;
        tmp_file2.write_all("but a match in this file - spelling".as_bytes())?;

        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("speling");
        cmd.arg(tmp_file1.path());
        cmd.arg(tmp_file2.path());

        cmd.assert().success().stdout(contains("spelling"));
        Ok(())
    }

    #[test]
    fn test_returns_input_word__when_no_match__with_file() -> TestResult {
        let mut tmp_file = tempfile::NamedTempFile::new()?;
        tmp_file.write_all("no match in this file".as_bytes())?;

        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("inputword");
        cmd.arg(tmp_file.path());

        cmd.assert().success().stdout(contains("inputword"));
        Ok(())
    }

    #[test]
    fn test_returns_input_word__when_no_match__with_default_words() -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        // I know this word is NOT in the default set of words (by manually checking)
        cmd.arg("quintessential");

        cmd.assert().success().stdout(contains("quintessential"));
        Ok(())
    }

    #[test]
    fn test_returns_match__when_uppercase_word_in_file() -> TestResult {
        let mut tmp_file = tempfile::NamedTempFile::new()?;
        tmp_file.write_all("SPELLING".as_bytes())?;

        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("speling");
        cmd.arg(tmp_file.path());

        cmd.assert().success().stdout(contains("spelling"));
        Ok(())
    }

    #[test]
    fn test_exits__when_non_existing_file() -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("speling");
        cmd.arg("some_non_existing_file");

        cmd.assert()
            .failure()
            .stderr(contains("File not found [\"some_non_existing_file\"]"));
        Ok(())
    }

    #[test]
    fn test_exits__when_missing_word_arg() -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;

        cmd.assert().failure().stderr(contains(
            "The following required arguments were not provided:\n    <WORD>",
        ));
        Ok(())
    }

    #[test]
    fn test_returns_match_found_in_default_words__with_file_and_default_flag() -> TestResult {
        let mut tmp_file = tempfile::NamedTempFile::new()?;
        tmp_file.write_all("no match in this file".as_bytes())?;

        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("--default");
        cmd.arg("speling");
        cmd.arg(tmp_file.path());

        cmd.assert().success().stdout(contains("spelling"));
        Ok(())
    }

    #[test]
    fn test_returns_match_found_in_file__with_default_flag() -> TestResult {
        let mut tmp_file = tempfile::NamedTempFile::new()?;
        tmp_file.write_all("quintessential is not included in default words".as_bytes())?;

        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("--default");
        cmd.arg("ruintessential");
        cmd.arg(tmp_file.path());

        cmd.assert().success().stdout(contains("quintessential"));
        Ok(())
    }

    #[test]
    fn test_returns_exit_code_1__when_word_was_corrected__with_exit_code_flag() -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("--exit-code");
        cmd.arg("speling");

        cmd.assert().failure().stdout(contains("spelling"));
        Ok(())
    }

    #[test]
    fn test_returns_exit_code_0__when_word_was_not_corrected__with_exit_code_flag() -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("--exit-code");
        cmd.arg("spelling");

        cmd.assert().success().stdout(contains("spelling"));
        Ok(())
    }

    #[test]
    fn test_returns_only_exit_code_1__when_word_was_corrected__with_exit_code_only_flag(
    ) -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("--exit-code-only");
        cmd.arg("speling");

        cmd.assert().failure().stdout("");
        Ok(())
    }

    #[test]
    fn test_returns_only_exit_code_0__when_word_was_not_corrected__with_exit_code_only_flag(
    ) -> TestResult {
        let mut cmd = Command::cargo_bin("stava")?;
        cmd.arg("--exit-code-only");
        cmd.arg("spelling");

        cmd.assert().success().stdout("");
        Ok(())
    }
}
