#![allow(clippy::all)]
use crate::commands::containers::exec_in_container;
use anyhow::Error;
use bollard::Docker;

pub async fn find_licenses(docker: Docker, container_id: &str) -> Result<Vec<String>, Error> {
    let license_output = exec_in_container(
        &docker,
        &container_id,
        vec![
            "find",
            ".",
            "-name",
            "*[.-]LICEN[CS]E*",
            "-o",
            "-name",
            "AGPL-*[0-9]*",
            "-o",
            "-name",
            "APACHE-*[0-9]*",
            "-o",
            "-name",
            "BSD-*[0-9]*",
            "-o",
            "-name",
            "CC-BY-*",
            "-o",
            "-name",
            "COPYING",
            "-o",
            "-name",
            "COPYING[.-]*",
            "-o",
            "-name",
            "COPYRIGHT",
            "-o",
            "-name",
            "COPYRIGHT[.-]*",
            "-o",
            "-name",
            "EULA",
            "-o",
            "-name",
            "EULA[.-]*",
            "-o",
            "-name",
            "GFDL-*[0-9]*",
            "-o",
            "-name",
            "GNU-*[0-9]*",
            "-o",
            "-name",
            "GPL-*[0-9]*",
            "-o",
            "-name",
            "LGPL-*[0-9]*",
            "-o",
            "-name",
            "LICEN[CS]E",
            "-o",
            "-name",
            "LICEN[CS]E[.-]*",
            "-o",
            "-name",
            "MIT-*[0-9]*",
            "-o",
            "-name",
            "MPL-*[0-9]*",
            "-o",
            "-name",
            "NOTICE",
            "-o",
            "-name",
            "NOTICE[.-]*",
            "-o",
            "-name",
            "OFL-*[0-9]*",
            "-o",
            "-name",
            "PATENTS",
            "-o",
            "-name",
            "PATENTS[.-]*",
            "-o",
            "-name",
            "UNLICEN[CS]E",
            "-o",
            "-name",
            "UNLICEN[CS]E[.-]*",
            "-o",
            "-name",
            "agpl[.-]*",
            "-o",
            "-name",
            "gpl[.-]*",
            "-o",
            "-name",
            "lgpl[.-]*",
            "-o",
            "-name",
            "licen[cs]e",
            "-o",
            "-name",
            "licen[cs]e.*",
        ],
        None,
        None,
    )
    .await?;
    let lines = license_output.lines();
    let mut license_vec: Vec<String> = Vec::new();
    for l in lines.clone() {
        license_vec.push(l.to_string());
    }
    Ok(license_vec)
}

pub async fn copy_licenses(
    licenses: Vec<String>,
    container_id: &str,
    docker: Docker,
    env: Option<Vec<&str>>,
) -> Result<(), Error> {
    for license in licenses {
        let _exec_output = exec_in_container(
            &docker,
            &container_id,
            vec![
                "cp",
                "--backup",
                "--verbose",
                license.to_string().as_str(),
                "/usr/licenses/",
            ],
            None,
            env.clone(),
        )
        .await?;
    }
    Ok(())
}
