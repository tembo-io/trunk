use assert_cmd::prelude::*; // Add methods on commands
use git2::Repository;
use predicates::prelude::*; // Used for writing assertions
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command; // Run programs
use tempfile::TempDir;

const CARGO_BIN: &str = "trunk";

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.arg("--help");
    cmd.assert().stdout(predicate::str::contains("Usage: "));

    Ok(())
}

#[test]
fn install_manifest_v1_extension() -> Result<(), Box<dyn std::error::Error>> {
    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("artifact-v1/my_extension-0.0.0.tar.gz");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--version");
    cmd.arg("0.0.0");
    cmd.arg("my_extension");
    cmd.assert().code(0);

    // Get output of 'pg_config --sharedir' and '--pkgdir'
    let sharedir = pg_config_path("sharedir")?;
    let pkglibdir = pg_config_path("pkglibdir")?;

    // Make sure files were installed.
    assert!(sharedir.join("extension/my_extension.control").exists());
    assert!(sharedir.join("extension/my_extension--0.0.0.sql").exists());
    assert!(pkglibdir.join("my_extension.so").exists());

    Ok(())
}

#[test]
fn build_and_install_extension_with_directory_field() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pljava_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("pljava-1.6.8-pg15.tar.gz");

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_pljava");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(0);

    if !cfg!(target_arch = "x86_64") {
        eprintln!(
            "TODO: Trunk currently only supports the x86_64 architecture; skipping installation tests"
        );
        return Ok(());
    }

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(tarball);
    cmd.arg("pljava");
    cmd.assert().code(0);

    // Get output of 'pg_config --sharedir' and '--pkgdir'
    let sharedir = pg_config_path("sharedir")?;
    let pkglibdir = pg_config_path("pkglibdir")?;

    // Make sure files were installed.
    assert!(sharedir.join("extension/pljava.control").exists());
    assert!(sharedir.join("pljava/pljava-1.6.8.jar").exists());
    assert!(sharedir.join("pljava/pljava-api-1.6.8.jar").exists());
    assert!(sharedir.join("pljava/pljava--1.6.8.sql").exists());
    assert!(pkglibdir.join("libpljava-so-1.6.8.so").exists());
    Ok(())
}

#[test]
fn build_pgrx_extension() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pgrx_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("test_pgrx_extension-0.0.0-pg15.tar.gz");
    let manifest_file = &output_dir.join("manifest.json");

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_pgrx_extension");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(0);
    assert!(output_dir.exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.txt"));

    // assert extension_name and loadable_libraries is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(tarball)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to run tar command");

    assert!(manifest_file.exists());
    let manifest: String = fs::read_to_string(manifest_file)?;
    assert!(manifest.contains("\"extension_name\": \"test_pgrx_extension\""));

    if !cfg!(target_arch = "x86_64") {
        eprintln!(
            "TODO: Trunk currently only supports the x86_64 architecture; skipping installation tests"
        );
        return Ok(());
    }

    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(tarball);
    cmd.arg("test_pgrx_extension");
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    cmd.assert().code(0);
    assert!(stdout.contains("CREATE EXTENSION IF NOT EXISTS test_pgrx_extension CASCADE;"));

    // Make sure the custom-named .so was installed.
    // Get output of 'pg_config '--pkgdir' and make sure the .so was installed.
    let pkglibdir = pg_config_path("pkglibdir")?;
    // Make sure the custom-named .so was installed.
    assert!(pkglibdir.join("test_pgrx_extension.so").exists());

    Ok(())
}

#[test]
fn build_pgrx_extension_bad_name() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pgrx_bad_name_")?;
    let output_dir = tmp_dir.path();

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_pgrx_extension");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--name");
    cmd.arg("bad_name");
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(1);

    Ok(())
}

#[test]
fn build_pgrx_extension_bad_version() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pgrx_bad_version_")?;
    let output_dir = tmp_dir.path();

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_pgrx_extension");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--version");
    cmd.arg("0.0.1");
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(1);

    Ok(())
}

#[test]
fn build_c_extension() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pg_tle_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("pg_tle-1.0.3-pg15.tar.gz");
    let manifest_file = &output_dir.join("manifest.json");

    // Example of a C extension
    let repo_url = "https://github.com/aws/pg_tle.git";
    // clone and checkout ref v1.0.3
    let repo_dir = &output_dir.join("pg_tle");
    let repo = Repository::clone(repo_url, repo_dir).unwrap();
    let refname = "v1.0.3";
    let (object, reference) = repo.revparse_ext(refname).expect("Object not found");
    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");
    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(repo_dir);
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.arg("--version");
    cmd.arg("1.0.3");
    cmd.arg("--name");
    cmd.arg("pg_tle");
    cmd.assert().code(0);
    assert!(output_dir.exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE"));
    assert!(stdout.contains("licenses/NOTICE"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(tarball)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to run tar command");

    assert!(manifest_file.exists());
    let manifest: String = fs::read_to_string(manifest_file)?;
    assert!(manifest.contains("\"extension_name\": \"pg_tle\""));

    Ok(())
}

#[test]
fn build_extension_custom_dockerfile() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pg_http_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("pgsql_http-1.5.0-pg15.tar.gz");
    let manifest_file = &output_dir.join("manifest.json");

    // Example of a C extension requires another build-time requirement
    let mut dockerfile_path = std::path::PathBuf::from(file!());
    dockerfile_path.pop(); // Remove the file name from the path
    dockerfile_path.push("test_builders");
    dockerfile_path.push("Dockerfile.http");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(output_dir);
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.arg("--dockerfile");
    cmd.arg(dockerfile_path.clone());
    cmd.arg("--install-command");
    cmd.arg("make -C pgsql-http install");
    cmd.arg("--version");
    cmd.arg("1.5.0");
    cmd.arg("--name");
    cmd.arg("pgsql_http");
    cmd.assert().code(0);
    assert!(output_dir.exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.md"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(tarball)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to run tar command");

    assert!(manifest_file.exists());
    let manifest: String = fs::read_to_string(manifest_file)?;
    // Note - name and extension_name are different here
    assert!(manifest.contains("\"name\": \"pgsql_http\""));
    assert!(manifest.contains("\"extension_name\": \"http\""));

    if !cfg!(target_arch = "x86_64") {
        eprintln!(
            "TODO: Trunk currently only supports the x86_64 architecture; skipping installation tests"
        );
        return Ok(());
    }

    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(tarball);
    cmd.arg("pgsql_http");
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    cmd.assert().code(0);
    assert!(!stdout.contains("CREATE EXTENSION IF NOT EXISTS pgsql_http CASCADE;"));
    assert!(stdout.contains("CREATE EXTENSION IF NOT EXISTS http CASCADE;"));

    Ok(())
}

#[test]
fn build_pg_stat_statements() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pg_stat_statements_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("pg_stat_statements-1.10-pg15.tar.gz");
    let manifest_file = &output_dir.join("manifest.json");

    // Example of a C extension requires another build-time requirement
    let mut dockerfile_path = std::path::PathBuf::from(file!());
    dockerfile_path.pop(); // Remove the file name from the path
    dockerfile_path.push("test_builders");
    dockerfile_path.push("Dockerfile.pg_stat_statements");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(output_dir);
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.arg("--dockerfile");
    cmd.arg(dockerfile_path.clone());
    cmd.arg("--install-command");
    cmd.arg("make -C postgres/contrib/pg_stat_statements USE_PGXS=1 install");
    cmd.arg("--version");
    cmd.arg("1.10");
    cmd.arg("--name");
    cmd.arg("pg_stat_statements");
    cmd.assert().code(0);
    assert!(output_dir.exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/COPYRIGHT"));
    assert!(stdout.contains("licenses/COPYRIGHT.~1~"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(tarball)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to run tar command");

    assert!(manifest_file.exists());
    let manifest: String = fs::read_to_string(manifest_file)?;
    assert!(manifest.contains("\"extension_name\": \"pg_stat_statements\""));

    Ok(())
}

#[test]
fn build_pg_cron_trunk_toml() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pg_cron_trunk_toml_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("pg_cron-1.6.4-pg15.tar.gz");
    let manifest_file = &output_dir.join("manifest.json");

    // Construct a path relative to the current file's directory
    let mut trunkfile_path = std::path::PathBuf::from(file!());
    trunkfile_path.pop(); // Remove the file name from the path
    trunkfile_path.push("test_trunk_toml_dirs");
    trunkfile_path.push("pg_cron");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(trunkfile_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(0);
    assert!(tarball.exists());

    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE"));

    // assert extension_name and loadable_libraries is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(tarball)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to run tar command");

    assert!(manifest_file.exists());
    let manifest = fs::read_to_string(manifest_file)?;
    assert!(manifest.contains("\"extension_name\": \"extension_name_from_toml\""));
    assert!(manifest.contains("\"loadable_libraries\": [\n    {\n      \"library_name\": \"shared_preload_libraries_from_toml\",\n      \"requires_restart\": true,\n      \"priority\": 1\n    },\n    {\n      \"library_name\": \"shared_preload_libraries_from_toml_2\",\n      \"requires_restart\": false,\n      \"priority\": 2\n    }\n  ]"));
    // Config name
    assert!(manifest.contains("ip_address"));
    // Is required
    assert!(manifest.contains("is_required\": true"));
    // Config name
    assert!(manifest.contains("port"));
    // Recommended default value
    assert!(manifest.contains("8801"));
    assert!(manifest.contains("\"dependencies\": {\n    \"apt\": [\n      \"libpq5\"\n    ],\n    \"dnf\": [\n      \"libpq-devel\"\n    ]\n  }") ||
            manifest.contains("\"dependencies\": {\n    \"dnf\": [\n      \"libpq-devel\"\n    ],\n    \"apt\": [\n      \"libpq5\"\n    ]\n  }"));

    if !cfg!(target_arch = "x86_64") {
        eprintln!(
            "TODO: Trunk currently only supports the x86_64 architecture; skipping installation tests"
        );
        return Ok(());
    }

    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(tarball);
    cmd.arg("pg_cron");

    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    cmd.assert().code(0);
    assert!(stdout.contains("CREATE EXTENSION IF NOT EXISTS extension_name_from_toml CASCADE;"));
    assert!(stdout.contains("Install the following system-level dependencies:"));
    assert!(stdout.contains("On systems using apt:"));
    assert!(stdout.contains("libpq5"));
    assert!(stdout.contains("On systems using dnf:"));
    assert!(stdout.contains("libpq-devel"));
    assert!(stdout.contains("Add the following to your postgresql.conf:"));
    assert!(stdout.contains("shared_preload_libraries = 'shared_preload_libraries_from_toml, shared_preload_libraries_from_toml_2'"));

    // assert that the dependencies were written to manifest
    assert!(manifest_file.exists());
    let manifest: String = fs::read_to_string(manifest_file)?;
    assert!(manifest.contains("\"extension_dependencies\": [\n    \"btree_gin\"\n  ],"));

    Ok(())
}

#[test]
fn build_pgrx_with_trunk_toml() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pgrx_trunk_toml_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("pgrx_with_trunk_toml-0.0.0-pg15.tar.gz");
    let manifest_file = &output_dir.join("manifest.json");

    // Construct a path relative to the current file's directory
    let mut trunkfile_path = std::path::PathBuf::from(file!());
    trunkfile_path.pop(); // Remove the file name from the path
    trunkfile_path.push("test_trunk_toml_dirs");
    trunkfile_path.push("pgrx_with_trunk_toml");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(trunkfile_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(0);
    assert!(tarball.exists());

    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.txt"));

    // assert extension_name and loadable_libraries is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(tarball)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to run tar command");

    assert!(manifest_file.exists());
    let manifest: String = fs::read_to_string(manifest_file)?;
    assert!(manifest.contains("\"extension_name\": \"extension_name_from_toml\""));
    assert!(manifest.contains("\"loadable_libraries\": [\n    {\n      \"library_name\": \"shared_preload_libraries_from_toml\",\n      \"requires_restart\": true,\n      \"priority\": 1\n    },\n    {\n      \"library_name\": \"another_shared_preload_library\",\n      \"requires_restart\": false,\n      \"priority\": 2\n    }\n  ]"));
    assert!(manifest.contains("\"another_shared_preload_library\""));
    assert!(manifest.contains("\"libpq5\""));

    if !cfg!(target_arch = "x86_64") {
        eprintln!(
            "TODO: Trunk currently only supports the x86_64 architecture; skipping installation tests"
        );
        return Ok(());
    }
    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(tarball);
    cmd.arg("pgrx_with_trunk_toml");
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    cmd.assert().code(0);
    assert!(!stdout.contains("CREATE EXTENSION IF NOT EXISTS pgrx_with_trunk_toml CASCADE;"));
    assert!(stdout.contains("CREATE EXTENSION IF NOT EXISTS extension_name_from_toml CASCADE;"));
    assert!(stdout.contains("Install the following system-level dependencies:"));
    assert!(stdout.contains("On systems using apt:"));
    assert!(stdout.contains("libpq5"));
    assert!(stdout.contains("Add the following to your postgresql.conf:"));
    assert!(stdout.contains("shared_preload_libraries = 'shared_preload_libraries_from_toml, another_shared_preload_library'"));

    // Get output of 'pg_config '--pkgdir' and make sure the .so was installed.
    let pkglibdir = pg_config_path("pkglibdir")?;
    // Make sure the custom-named .so was installed.
    assert!(pkglibdir.join("pgrx_with_trunk_toml.so").exists());

    Ok(())
}

#[test]
fn build_pgrx_with_trunk_toml_bad_name() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pgrx_with_trunk_toml_bad_name_")?;
    let output_dir = tmp_dir.path();

    // Construct a path relative to the current file's directory
    let mut trunkfile_path = std::path::PathBuf::from(file!());
    trunkfile_path.pop(); // Remove the file name from the path
    trunkfile_path.push("test_trunk_toml_dirs");
    trunkfile_path.push("pgrx_with_trunk_toml_bad_name");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(trunkfile_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(1);

    Ok(())
}

#[test]
fn build_pgrx_with_trunk_toml_bad_version() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = TempDir::with_prefix("test_pgrx_with_trunk_toml_bad_version_")?;
    let output_dir = tmp_dir.path();

    // Construct a path relative to the current file's directory
    let mut trunkfile_path = std::path::PathBuf::from(file!());
    trunkfile_path.pop(); // Remove the file name from the path
    trunkfile_path.push("test_trunk_toml_dirs");
    trunkfile_path.push("pgrx_with_trunk_toml_bad_version");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(trunkfile_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(1);

    Ok(())
}

// Test for extension with no control file
#[test]
fn build_auto_explain() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_auto_explain_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("auto_explain-15.3.0-pg15.tar.gz");
    let manifest_file = &output_dir.join("manifest.json");

    // Example of a C extension requires another build-time requirement
    let mut dockerfile_path = std::path::PathBuf::from(file!());
    dockerfile_path.pop(); // Remove the file name from the path
    dockerfile_path.push("test_builders");
    dockerfile_path.push("Dockerfile.auto_explain");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(output_dir);
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.arg("--dockerfile");
    cmd.arg(dockerfile_path.clone());
    cmd.arg("--install-command");
    cmd.arg("make -C postgres/contrib/auto_explain install USE_PGXS=1");
    cmd.arg("--version");
    cmd.arg("15.3.0");
    cmd.arg("--name");
    cmd.arg("auto_explain");
    cmd.assert().code(0);
    assert!(tarball.exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/COPYRIGHT"));
    assert!(stdout.contains("licenses/COPYRIGHT.~1~"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(tarball)
        .arg("-C")
        .arg(output_dir)
        .output()
        .expect("failed to run tar command");

    assert!(manifest_file.exists());
    let manifest: String = fs::read_to_string(manifest_file)?;
    assert!(manifest.contains("\"extension_name\": \"auto_explain\""));

    if !cfg!(target_arch = "x86_64") {
        eprintln!(
            "TODO: Trunk currently only supports the x86_64 architecture; skipping installation tests"
        );
        return Ok(());
    }

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(tarball);
    cmd.arg("auto_explain");

    cmd.assert().code(0);

    Ok(())
}

#[test]
fn build_pg_unit() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_pg_unit_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("postgresql_unit-7.10.0-pg15.tar.gz");

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_postgresql_unit");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(0);
    assert!(tarball.exists());

    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(tarball)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("extension/unit_prefixes.data"));
    assert!(stdout.contains("extension/unit_units.data"));

    Ok(())
}

// Build and install postgis
#[test]
fn build_install_postgis() -> Result<(), Box<dyn std::error::Error>> {
    // Set up a temporary directory that will be deleted when the test finishes.
    let tmp_dir = TempDir::with_prefix("test_postgis_")?;
    let output_dir = tmp_dir.path();
    let tarball = &output_dir.join("postgis-3.4.0-pg15.tar.gz");

    // Construct a path relative to the current file's directory
    let mut trunkfile_path = std::path::PathBuf::from(file!());
    trunkfile_path.pop(); // Remove the file name from the path
    trunkfile_path.push("test_trunk_toml_dirs");
    trunkfile_path.push("postgis");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(trunkfile_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir);
    cmd.assert().code(0);
    assert!(output_dir.exists());

    if !cfg!(target_arch = "x86_64") {
        eprintln!(
            "TODO: Trunk currently only supports the x86_64 architecture; skipping installation tests"
        );
        return Ok(());
    }

    // Get output of 'pg_config --sharedir'
    let sharedir = pg_config_path("sharedir")?;

    // Remove fuzzystrmatch if it exists
    let _ = std::fs::remove_file(sharedir.join("extension/fuzzystrmatch.control"));

    // Assert we recognize fuzzystrmatch as a dependency and install it
    // This is a dependency of postgis_tiger_geocoder, which is included in the postgis tar.gz
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--file");
    cmd.arg(tarball);
    cmd.arg("postgis");
    let output = cmd.output()?;
    let stderr = String::from_utf8(output.stderr)?;
    cmd.assert().code(0);
    assert!(stderr.contains("Dependent extensions to be installed: [\"fuzzystrmatch\"]"));
    assert!(stderr.contains("Installing fuzzystrmatch"));

    // Make sure files were installed.
    assert!(sharedir.join("extension/fuzzystrmatch.control").exists());
    assert!(sharedir.join("extension/postgis.control").exists());

    Ok(())
}

/// Install an extension with a dependency that requires extension name resolution
#[test]
fn install_extension_dependency_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let sharedir = pg_config_path("sharedir")?;
    let pkglibdir = pg_config_path("pkglibdir")?;

    // pgvector is a dependency of vectorize, so let's make sure it's uninstalled
    let pgvector_control_file = sharedir.join("extension/vector.control");
    let pgvector_sql = sharedir.join("extension/vector--0.1.8--0.2.0.sql");
    let pgvector_so = pkglibdir.join("vector.so");

    if pgvector_control_file.exists() {
        fs::remove_file(&pgvector_control_file)?;
    }

    if pgvector_sql.exists() {
        fs::remove_file(&pgvector_sql)?;
    }

    if pgvector_so.exists() {
        fs::remove_file(&pgvector_so)?;
    }

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--pg-version");
    cmd.arg("15");
    cmd.arg("--version");
    cmd.arg("0.12.1");
    cmd.arg("vectorize");
    cmd.assert().code(0);

    // Make sure vectorize was installed
    assert!(sharedir.join("extension/vectorize.control").exists());
    assert!(sharedir.join("extension/vectorize--0.12.1.sql").exists());
    assert!(pkglibdir.join("vectorize.so").exists());

    // Also make sure pgvector, one of its dependencies, was also installed
    assert!(pgvector_control_file.exists());
    assert!(pgvector_so.exists());
    assert!(pgvector_sql.exists());

    Ok(())
}

fn pg_config_path(opt: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Get output from pg_config
    let output = Command::new("pg_config")
        .arg(format!("--{opt}"))
        .output()
        .expect("Error executing; pg_config; is it in the path?");

    // Convert output to String and handle possible UTF-8 conversion error
    let cfg = String::from_utf8(output.stdout)?;

    // Trim the output string and create a Path
    Ok(Path::new(cfg.trim()).to_owned())
}
