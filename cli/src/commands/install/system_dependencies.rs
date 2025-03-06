use std::{
    fs,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy)]
pub enum OperatingSystem {
    MacOs,
    Debian,
    RedHat,
    Windows,
}

impl OperatingSystem {
    pub fn package_managers(&self) -> &[PackageManager] {
        match self {
            OperatingSystem::MacOs => &[PackageManager::Homebrew],
            OperatingSystem::Debian => &[PackageManager::Apt],
            OperatingSystem::RedHat => &[PackageManager::Dnf, PackageManager::Yum],
            OperatingSystem::Windows => &[PackageManager::Chocolatey],
        }
    }

    /// Detect the current operating system, if it's supported
    pub fn detect() -> Result<OperatingSystem> {
        let os = if cfg!(test) {
            // Always use Debian for testing
            Some(OperatingSystem::Debian)
        } else if cfg!(target_os = "linux") {
            Self::detect_linux_distribution()
        } else if cfg!(windows) {
            Some(OperatingSystem::Windows)
        } else if cfg!(target_os = "macos") {
            Some(OperatingSystem::MacOs)
        } else {
            None
        };

        os.with_context(|| "Current operating system is unsupported")
    }

    /// Check `os-release` to detect current Linux distro
    fn detect_linux_distribution() -> Option<OperatingSystem> {
        let os_release = fs::File::open("/etc/os-release").ok()?;
        let reader = BufReader::new(os_release);

        for maybe_line in reader.lines() {
            let Ok(line) = maybe_line else {
                continue;
            };

            match &*line {
                "ID=debian" => return Some(OperatingSystem::Debian),
                "ID=fedora" | "ID=centos" | "ID=rhel" => return Some(OperatingSystem::RedHat),
                _ => continue,
            }
        }

        None
    }
}

#[derive(Debug)]
pub enum PackageManager {
    Apt,
    Dnf,
    Yum,
    Chocolatey,
    Homebrew,
}

impl PackageManager {
    pub fn as_str(&self) -> &str {
        match self {
            PackageManager::Apt => "apt",
            PackageManager::Dnf => "dnf",
            PackageManager::Yum => "yum",
            PackageManager::Chocolatey => "chocolatey",
            PackageManager::Homebrew => "brew",
        }
    }

    pub fn install(&self, package_name: &str) -> String {
        let install_command = match self {
            PackageManager::Apt => "apt-get install -y",
            PackageManager::Dnf => "dnf install -y",
            PackageManager::Yum => "yum install -y",
            PackageManager::Homebrew => "brew install",
            PackageManager::Chocolatey => "choco install",
        };

        format!(
            "{sudo}{install_command} {package_name}",
            sudo = if self.requires_sudo() { "sudo " } else { "" }
        )
    }

    pub fn requires_sudo(&self) -> bool {
        match self {
            PackageManager::Apt | PackageManager::Dnf | PackageManager::Yum => true,
            PackageManager::Homebrew | PackageManager::Chocolatey => false,
        }
    }
}
