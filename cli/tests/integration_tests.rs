use assert_cmd::prelude::*; // Add methods on commands
use git2::Repository;
use predicates::prelude::*; // Used for writing assertions
use rand::Rng;
use std::fs;
use std::path::Path;
use std::process::Command; // Run programs

const CARGO_BIN: &str = "trunk";

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

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
fn build_and_install_extension_with_directory_field() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/test_pgrx_{}", rng.gen_range(0..1000000));

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_pljava");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(&output_dir);
    cmd.assert().code(0);

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--file");
    cmd.arg(Path::new(&output_dir).join("pljava-1.6.5-pg15.tar.gz"));
    cmd.arg("pljava");
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

    assert!(file_exists(format!("{sharedir}/pljava/pljava.control")));

    assert!(file_exists(format!("{sharedir}/pljava/pljava-1.6.5.jar")));

    assert!(file_exists(format!(
        "{sharedir}/pljava/pljava-api-1.6.5.jar"
    )));

    assert!(file_exists(format!("{sharedir}/pljava/pljava--1.6.5.sql")));

    assert!(file_exists(format!("{pkglibdir}/libpljava-so-1.6.5.so")));
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
        format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str()
    )
    .exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.txt"));

    // assert extension_name and loadable_libraries is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str())
        .arg("-C")
        .arg(format!("{output_dir}").as_str())
        .output()
        .expect("failed to run tar command");

    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");
    let stdout = String::from_utf8(manifest.stdout).unwrap();
    assert!(stdout.contains("\"extension_name\": \"test_pgrx_extension\""));

    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--file");
    cmd.arg(format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str());
    cmd.arg("test_pgrx_extension");
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    cmd.assert().code(0);
    assert!(stdout.contains("CREATE EXTENSION IF NOT EXISTS test_pgrx_extension CASCADE;"));

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
    cmd.assert().code(1);

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
    cmd.assert().code(1);

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
    assert!(
        std::path::Path::new(format!("{output_dir}/pg_tle-1.0.3-pg15.tar.gz").as_str()).exists()
    );
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/pg_tle-1.0.3-pg15.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE"));
    assert!(stdout.contains("licenses/NOTICE"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(format!("{output_dir}/pg_tle-1.0.3-pg15.tar.gz").as_str())
        .arg("-C")
        .arg(format!("{output_dir}").as_str())
        .output()
        .expect("failed to run tar command");

    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");
    let stdout = String::from_utf8(manifest.stdout).unwrap();
    assert!(stdout.contains("\"extension_name\": \"pg_tle\""));

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
    cmd.arg("pgsql_http");
    cmd.assert().code(0);
    assert!(
        std::path::Path::new(format!("{output_dir}/pgsql_http-1.5.0-pg15.tar.gz").as_str())
            .exists()
    );
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/pgsql_http-1.5.0-pg15.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.md"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(format!("{output_dir}/pgsql_http-1.5.0-pg15.tar.gz").as_str())
        .arg("-C")
        .arg(format!("{output_dir}").as_str())
        .output()
        .expect("failed to run tar command");

    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");
    let stdout = String::from_utf8(manifest.stdout).unwrap();
    // Note - name and extension_name are different here
    assert!(stdout.contains("\"name\": \"pgsql_http\""));
    assert!(stdout.contains("\"extension_name\": \"http\""));

    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--file");
    cmd.arg(format!("{output_dir}/pgsql_http-1.5.0-pg15.tar.gz").as_str());
    cmd.arg("pgsql_http");
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    cmd.assert().code(0);
    assert!(!stdout.contains("CREATE EXTENSION IF NOT EXISTS pgsql_http CASCADE;"));
    assert!(stdout.contains("CREATE EXTENSION IF NOT EXISTS http CASCADE;"));

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
    let repo_dir_path = current_file_path
        .parent()
        .unwrap()
        .join("postgres_pg_stat_statements");
    let repo_dir = repo_dir_path;
    if repo_dir.exists() {
        println!(
            "Repo directory {:?} already exists. Deleting.",
            repo_dir.to_str()
        );
        fs::remove_dir_all(repo_dir.clone())?;
    }
    let repo = Repository::clone(repo_url, &repo_dir).unwrap();
    let refname = "REL_15_3";
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
    extension_path.push("postgres_pg_stat_statements");

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
    assert!(std::path::Path::new(
        format!("{output_dir}/pg_stat_statements-1.10-pg15.tar.gz").as_str()
    )
    .exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/pg_stat_statements-1.10-pg15.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/COPYRIGHT"));
    assert!(stdout.contains("licenses/COPYRIGHT.~1~"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(format!("{output_dir}/pg_stat_statements-1.10-pg15.tar.gz").as_str())
        .arg("-C")
        .arg(format!("{output_dir}").as_str())
        .output()
        .expect("failed to run tar command");

    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");
    let stdout = String::from_utf8(manifest.stdout).unwrap();
    assert!(stdout.contains("\"extension_name\": \"pg_stat_statements\""));
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
    assert!(
        std::path::Path::new(format!("{output_dir}/pg_cron-1.5.2-pg15.tar.gz").as_str()).exists()
    );
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/pg_cron-1.5.2-pg15.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE"));

    // assert extension_name and loadable_libraries is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(format!("{output_dir}/pg_cron-1.5.2-pg15.tar.gz").as_str())
        .arg("-C")
        .arg(format!("{output_dir}").as_str())
        .output()
        .expect("failed to run tar command");

    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");
    let stdout = String::from_utf8(manifest.stdout).unwrap();
    assert!(stdout.contains("\"extension_name\": \"extension_name_from_toml\""));
    assert!(stdout.contains("\"loadable_libraries\": [\n    {\n      \"library_name\": \"shared_preload_libraries_from_toml\",\n      \"requires_restart\": true,\n      \"priority\": 1\n    },\n    {\n      \"library_name\": \"shared_preload_libraries_from_toml_2\",\n      \"requires_restart\": false,\n      \"priority\": 2\n    }\n  ]"));
    // Config name
    assert!(stdout.contains("ip_address"));
    // Is required
    assert!(stdout.contains("is_required\": true"));
    // Config name
    assert!(stdout.contains("port"));
    // Recommended default value
    assert!(stdout.contains("8801"));
    assert!(stdout.contains("\"dependencies\": {\n    \"apt\": [\n      \"libpq5\"\n    ],\n    \"dnf\": [\n      \"libpq-devel\"\n    ]\n  }") ||
            stdout.contains("\"dependencies\": {\n    \"dnf\": [\n      \"libpq-devel\"\n    ],\n    \"apt\": [\n      \"libpq5\"\n    ]\n  }"));
    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--file");
    cmd.arg(format!("{output_dir}/pg_cron-1.5.2-pg15.tar.gz").as_str());
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
    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");

    let stdout = String::from_utf8(manifest.stdout).unwrap();
    assert!(stdout.contains("\"extension_dependencies\": [\n    \"btree_gin\"\n  ],"));

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
        format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str()
    )
    .exists());
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/LICENSE.txt"));

    // assert extension_name and loadable_libraries is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str())
        .arg("-C")
        .arg(format!("{output_dir}").as_str())
        .output()
        .expect("failed to run tar command");

    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");
    let stdout = String::from_utf8(manifest.stdout).unwrap();
    assert!(stdout.contains("\"extension_name\": \"extension_name_from_toml\""));
    assert!(stdout.contains("\"loadable_libraries\": [\n    {\n      \"library_name\": \"shared_preload_libraries_from_toml\",\n      \"requires_restart\": true,\n      \"priority\": 1\n    },\n    {\n      \"library_name\": \"another_shared_preload_library\",\n      \"requires_restart\": false,\n      \"priority\": 2\n    }\n  ]"));
    assert!(stdout.contains("\"another_shared_preload_library\""));
    assert!(stdout.contains("\"libpq5\""));

    // Get output of 'pg_config --pkglibdir'
    let output = Command::new("pg_config")
        .arg("--pkglibdir")
        .output()
        .expect("failed to find pkglibdir, is pg_config in path?");
    let pkglibdir = String::from_utf8(output.stdout)?;
    let pkglibdir = pkglibdir.trim();

    // Remove .so if it exists
    let _rm_so = Command::new("rm")
        .arg(format!("{pkglibdir}/test_pgrx_extension.so").as_str())
        .output()
        .expect("failed to remove .so file");

    // assert post installation steps contain correct CREATE EXTENSION command
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--file");
    cmd.arg(format!("{output_dir}/test_pgrx_extension-0.0.0-pg15.tar.gz").as_str());
    cmd.arg("test_pgrx_extension");
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    cmd.assert().code(0);
    assert!(!stdout.contains("CREATE EXTENSION IF NOT EXISTS test_pgrx_extension CASCADE;"));
    assert!(stdout.contains("CREATE EXTENSION IF NOT EXISTS extension_name_from_toml CASCADE;"));
    assert!(stdout.contains("Install the following system-level dependencies:"));
    assert!(stdout.contains("On systems using apt:"));
    assert!(stdout.contains("libpq5"));
    assert!(stdout.contains("Add the following to your postgresql.conf:"));
    assert!(stdout.contains("shared_preload_libraries = 'shared_preload_libraries_from_toml, another_shared_preload_library'"));

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
    cmd.assert().code(1);

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
    cmd.assert().code(1);

    Ok(())
}

// Test for extension with no control file
#[test]
fn build_auto_explain() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/auto_explain_test_{}", rng.gen_range(0..1000000));

    let current_file_path = Path::new(file!()).canonicalize().unwrap();
    // Example of a C extension requires another build-time requirement
    let repo_url = "https://github.com/postgres/postgres.git";
    let repo_dir_path = current_file_path
        .parent()
        .unwrap()
        .join("postgres_auto_explain");
    let repo_dir = repo_dir_path;
    if repo_dir.exists() {
        println!(
            "Repo directory {:?} already exists. Deleting.",
            repo_dir.to_str()
        );
        fs::remove_dir_all(repo_dir.clone())?;
    }
    let repo = Repository::clone(repo_url, &repo_dir).unwrap();
    let refname = "REL_15_3";
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
    extension_path.push("postgres_auto_explain");

    let mut dockerfile_path = std::path::PathBuf::from(file!());
    dockerfile_path.pop(); // Remove the file name from the path
    dockerfile_path.push("test_builders");
    dockerfile_path.push("Dockerfile.auto_explain");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(output_dir.clone());
    cmd.arg("--dockerfile");
    cmd.arg(dockerfile_path.clone());
    cmd.arg("--install-command");
    cmd.arg("cd contrib/auto_explain && make install && set -x && mv /usr/local/pgsql/share/extension/* /usr/share/postgresql/15/extension && mv /usr/local/pgsql/lib/* /usr/lib/postgresql/15/lib");
    cmd.arg("--version");
    cmd.arg("15.3.0");
    cmd.arg("--name");
    cmd.arg("auto_explain");
    cmd.assert().code(0);
    assert!(
        std::path::Path::new(format!("{output_dir}/auto_explain-15.3.0-pg15.tar.gz").as_str())
            .exists()
    );
    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(format!("{output_dir}/auto_explain-15.3.0-pg15.tar.gz").as_str())
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("licenses/COPYRIGHT"));
    assert!(stdout.contains("licenses/COPYRIGHT.~1~"));

    // assert extension_name is in manifest.json
    let _extract = Command::new("tar")
        .arg("-xvf")
        .arg(format!("{output_dir}/auto_explain-15.3.0-pg15.tar.gz").as_str())
        .arg("-C")
        .arg(format!("{output_dir}").as_str())
        .output()
        .expect("failed to run tar command");

    let manifest = Command::new("cat")
        .arg(format!("{output_dir}/manifest.json").as_str())
        .output()
        .expect("failed to run cat command");
    let stdout = String::from_utf8(manifest.stdout).unwrap();

    assert!(stdout.contains("\"extension_name\": \"auto_explain\""));

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--file");
    cmd.arg(format!("{output_dir}/auto_explain-15.3.0-pg15.tar.gz").as_str());
    cmd.arg("auto_explain");

    cmd.assert().code(0);
    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

#[test]
fn build_pg_unit() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/postgresql_unit_test_{}", rng.gen_range(0..1000000));
    let package_name = "postgresql_unit-7.0.0-pg15.tar.gz";

    // Construct a path relative to the current file's directory
    let mut extension_path = std::path::PathBuf::from(file!());
    extension_path.pop(); // Remove the file name from the path
    extension_path.push("test_postgresql_unit");

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("build");
    cmd.arg("--path");
    cmd.arg(extension_path.as_os_str());
    cmd.arg("--output-path");
    cmd.arg(&output_dir);
    cmd.assert().code(0);

    let package_location = format!("{output_dir}/{package_name}");

    assert!(file_exists(&package_location));

    // assert any license files are included
    let output = Command::new("tar")
        .arg("-tvf")
        .arg(package_location)
        .output()
        .expect("failed to run tar command");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("extension/unit_prefixes.data"));
    assert!(stdout.contains("extension/unit_units.data"));

    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}

// Build and install postgis
#[test]
fn build_install_postgis() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let output_dir = format!("/tmp/postgis_test_{}", rng.gen_range(0..1000000));

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
    cmd.arg(output_dir.clone());
    cmd.assert().code(0);
    assert!(file_exists(format!(
        "{output_dir}/postgis-3.4.0-pg15.tar.gz"
    )));

    // Get output of 'pg_config --sharedir'
    let output = Command::new("pg_config")
        .arg("--sharedir")
        .output()
        .expect("failed to find sharedir, is pg_config in path?");
    let sharedir = String::from_utf8(output.stdout)?;
    let sharedir = sharedir.trim();

    // Remove fuzzystrmatch if it exists
    let _rm_fuzzystrmatch = Command::new("rm")
        .arg(format!("{sharedir}/extension/fuzzystrmatch.control").as_str())
        .output()
        .expect("failed to remove fuzzystrmatch");

    // Assert we recognize fuzzystrmatch as a dependency and install it
    // This is a dependency of postgis_tiger_geocoder, which is included in the postgis tar.gz
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    cmd.arg("install");
    cmd.arg("--file");
    cmd.arg(format!("{output_dir}/postgis-3.4.0-pg15.tar.gz").as_str());
    cmd.arg("postgis");
    let output = cmd.output()?;
    let stderr = String::from_utf8(output.stderr)?;
    cmd.assert().code(0);
    assert!(stderr.contains("Dependent extensions to be installed: [\"fuzzystrmatch\"]"));
    assert!(stderr.contains("Installing fuzzystrmatch"));

    assert!(file_exists(format!(
        "{sharedir}/extension/fuzzystrmatch.control"
    )));
    assert!(file_exists(format!("{sharedir}/extension/postgis.control")));

    // delete the temporary file
    std::fs::remove_dir_all(output_dir)?;

    Ok(())
}
