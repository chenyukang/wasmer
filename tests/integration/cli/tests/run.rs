//! Basic tests for the `run` subcommand

use anyhow::bail;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use wasmer_integration_tests_cli::{ASSET_PATH, C_ASSET_PATH, WASMER_PATH};

fn wasi_test_wasm_path() -> String {
    format!("{}/{}", C_ASSET_PATH, "qjs.wasm")
}

fn test_no_imports_wat_path() -> String {
    format!("{}/{}", ASSET_PATH, "fib.wat")
}

fn test_no_start_wat_path() -> String {
    format!("{}/{}", ASSET_PATH, "no_start.wat")
}

#[test]
fn run_wasi_works() -> anyhow::Result<()> {
    let output = Command::new(WASMER_PATH)
        .arg("run")
        .arg(wasi_test_wasm_path())
        .arg("--")
        .arg("-e")
        .arg("print(3 * (4 + 5))")
        .output()?;

    if !output.status.success() {
        bail!(
            "linking failed with: stdout: {}\n\nstderr: {}",
            std::str::from_utf8(&output.stdout)
                .expect("stdout is not utf8! need to handle arbitrary bytes"),
            std::str::from_utf8(&output.stderr)
                .expect("stderr is not utf8! need to handle arbitrary bytes")
        );
    }

    let stdout_output = std::str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout_output, "27\n");

    Ok(())
}

#[test]

fn run_no_imports_wasm_works() -> anyhow::Result<()> {
    let output = Command::new(WASMER_PATH)
        .arg("run")
        .arg(test_no_imports_wat_path())
        .output()?;

    if !output.status.success() {
        bail!(
            "linking failed with: stdout: {}\n\nstderr: {}",
            std::str::from_utf8(&output.stdout)
                .expect("stdout is not utf8! need to handle arbitrary bytes"),
            std::str::from_utf8(&output.stderr)
                .expect("stderr is not utf8! need to handle arbitrary bytes")
        );
    }

    Ok(())
}

#[test]
fn run_no_start_wasm_report_error() -> anyhow::Result<()> {
    let bytes = vec![
        0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 124, 1, 127, 2, 1, 0, 3, 3, 2, 0, 0, 10, 20,
        2, 3, 0, 0, 11, 14, 0, 65, 255, 135, 140, 120, 183, 155, 155, 32, 0, 166, 0, 11,
    ];

    let mut file = File::create("binary_bug.wasm")?;
    // Write a slice of bytes to the file
    file.write_all(&bytes)?;

    let output = Command::new(WASMER_PATH)
        .arg("run")
        .arg(test_no_start_wat_path())
        .output()?;

    assert_eq!(output.status.success(), false);
    let result = std::str::from_utf8(&output.stderr).unwrap().to_string();
    assert_eq!(result.contains("Can not find any export functions."), true);
    Ok(())
}
