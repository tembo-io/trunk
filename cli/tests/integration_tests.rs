use assert_cmd::prelude::*; // Add methods on commands
use git2::Repository;
use predicates::prelude::*; // Used for writing assertions
use rand::Rng;
use std::fs;
use std::path::Path;
use std::process::Command; // Run programs

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
    cmd.arg("--file");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--version");
    cmd.arg("0.0.0");
    cmd.arg("my_extension");
    cmd.assert().code(0);

    // Get output of 'pg_config --sharedir'
    let output = Command::new("pg_config")
        .arg("--sharedir")
        .output()
        .expect("failed to find sharedir, is pg_config in path?");
    let sharedir = String::from_utf8(output.stdout)?;
    let sharedir = sharedir.trim();

    let output = Command::new("pg_config")
        .arg("--pkglibdir")
        .output()
        .expect("failed to find pkglibdir, is pg_config in path?");
    let pkglibdir = String::from_utf8(output.stdout)?;
    let pkglibdir = pkglibdir.trim();

    assert!(
        std::path::Path::new(format!("{sharedir}/extension/my_extension.control").as_str())
            .exists()
    );
    assert!(
        std::path::Path::new(format!("{sharedir}/extension/my_extension--0.0.0.sql").as_str())
            .exists()
    );
    assert!(std::path::Path::new(format!("{pkglibdir}/my_extension.so").as_str()).exists());
    Ok(())
}

#[test]
fn build_pgrx_extension() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/test_pgrx_{}", rng.gen_range(0..1000000));

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_pgrx_extension");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir.clone());
    cmd.assert().code(0);
    assert!(std::path::Path::new(
        format!("{output_dir}/test_pgrx_extension-0.0.0.tar.gz").as_str()
    )
    .exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/test_pgrx_extension-0.0.0.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.txt"));
    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

#[test]
fn build_pgrx_extension_bad_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/test_pgrx_{}", rng.gen_range(0..1000000));

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
    cmd.arg(output_dir.clone());
    cmd.assert().code(101);

    Ok(())
}

#[test]
fn build_pgrx_extension_bad_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/test_pgrx_{}", rng.gen_range(0..1000000));

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
    cmd.arg(output_dir.clone());
    cmd.assert().code(101);

    Ok(())
}

#[test]
fn build_c_extension() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/pg_tle_test_{}", rng.gen_range(0..1000000));

    let current_file_path = Path::new(file!()).canonicalize().unwrap();
    // Example of a C extension
    let repo_url = "https://github.com/aws/pg_tle.git";
    // clone and checkout ref v1.0.3
    let repo_dir_path = current_file_path.parent().unwrap().join("pg_tle");
    let repo_dir = repo_dir_path;
    if repo_dir.exists() {
        fs::remove_dir_all(repo_dir.clone()).unwrap();
    }
    let repo = Repository::clone(repo_url, &repo_dir).unwrap();
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

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("pg_tle");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir.clone());
    cmd.arg("--version");
    cmd.arg("1.0.3");
    cmd.arg("--name");
    cmd.arg("pg_tle");
    cmd.assert().code(0);
    assert!(std::path::Path::new(format!("{output_dir}/pg_tle-1.0.3.tar.gz").as_str()).exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/pg_tle-1.0.3.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE"));
    assert!(stdout.contains("licenses/NOTICE"));

    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

#[test]
fn build_extension_custom_dockerfile() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/pg_http_test_{}", rng.gen_range(0..1000000));

    let current_file_path = Path::new(file!()).canonicalize().unwrap();
    // Example of a C extension requires another build-time requirement
    let repo_url = "https://github.com/pramsey/pgsql-http.git";
    // clone and checkout ref v1.5.0
    let repo_dir_path = current_file_path.parent().unwrap().join("pgsql-http");
    let repo_dir = repo_dir_path;
    if repo_dir.exists() {
        fs::remove_dir_all(repo_dir.clone()).unwrap();
    }
    let repo = Repository::clone(repo_url, &repo_dir).unwrap();
    let refname = "v1.5.0";
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

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("pgsql-http");

    let mut dockerfile_path = std::path::PathBuf::from(file!());
    dockerfile_path.pop(); // Remove the file name from the path
    dockerfile_path.push("test_builders");
    dockerfile_path.push("Dockerfile.http");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir.clone());
    cmd.arg("--dockerfile");
    cmd.arg(dockerfile_path.clone());
    cmd.arg("--version");
    cmd.arg("1.5.0");
    cmd.arg("--name");
    cmd.arg("http");
    cmd.assert().code(0);
    assert!(std::path::Path::new(format!("{output_dir}/http-1.5.0.tar.gz").as_str()).exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/http-1.5.0.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.md"));
    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

#[test]
fn build_pg_stat_statements() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/pg_stat_statements_test_{}", rng.gen_range(0..1000000));

    let current_file_path = Path::new(file!()).canonicalize().unwrap();
    // Example of a C extension requires another build-time requirement
    let repo_url = "https://github.com/postgres/postgres.git";
    // clone and checkout ref v1.5.0
    let repo_dir_path = current_file_path.parent().unwrap().join("postgres");
    let repo_dir = repo_dir_path;
    if repo_dir.exists() {
        fs::remove_dir_all(repo_dir.clone()).unwrap();
    }
    let repo = Repository::clone(repo_url, &repo_dir).unwrap();
    let refname = "REL_15_2";
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

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("postgres");

    let mut dockerfile_path = std::path::PathBuf::from(file!());
    dockerfile_path.pop(); // Remove the file name from the path
    dockerfile_path.push("test_builders");
    dockerfile_path.push("Dockerfile.pg_stat_statements");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir.clone());
    cmd.arg("--dockerfile");
    cmd.arg(dockerfile_path.clone());
    cmd.arg("--install-command");
    cmd.arg("cd contrib/pg_stat_statements && make install && set -x && mv /usr/local/pgsql/share/extension/* /usr/share/postgresql/15/extension && mv /usr/local/pgsql/lib/* /usr/lib/postgresql/15/lib");
    cmd.arg("--version");
    cmd.arg("1.10");
    cmd.arg("--name");
    cmd.arg("pg_stat_statements");
    cmd.assert().code(0);
    assert!(
        std::path::Path::new(format!("{output_dir}/pg_stat_statements-1.10.tar.gz").as_str())
            .exists()
    );
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/pg_stat_statements-1.10.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/COPYRIGHT"));
    assert!(stdout.contains("licenses/COPYRIGHT.~1~"));
    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

#[test]
fn build_pg_cron_trunk_toml() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/pg_cron_test_{}", rng.gen_range(0..1000000));

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
    cmd.arg(output_dir.clone());
    cmd.assert().code(0);
    assert!(std::path::Path::new(format!("{output_dir}/pg_cron-1.5.2.tar.gz").as_str()).exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/pg_cron-1.5.2.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE"));
    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

#[test]
fn build_pgrx_with_trunk_toml() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!(
        "/tmp/test_pgrx_with_trunk_toml_{}",
        rng.gen_range(0..1000000)
    );

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
    cmd.arg(output_dir.clone());
    cmd.assert().code(0);
    assert!(std::path::Path::new(
        format!("{output_dir}/test_pgrx_extension-0.0.0.tar.gz").as_str()
    )
    .exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/test_pgrx_extension-0.0.0.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.txt"));
    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

#[test]
fn build_pgrx_with_trunk_toml_bad_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!(
        "/tmp/test_pgrx_with_trunk_toml_bad_name_{}",
        rng.gen_range(0..1000000)
    );

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
    cmd.arg(output_dir.clone());
    cmd.assert().code(101);

    Ok(())
}

#[test]
fn build_pgrx_with_trunk_toml_bad_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!(
        "/tmp/test_pgrx_with_trunk_toml_bad_version_{}",
        rng.gen_range(0..1000000)
    );

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
    cmd.arg(output_dir.clone());
    cmd.assert().code(101);

    Ok(())
}
