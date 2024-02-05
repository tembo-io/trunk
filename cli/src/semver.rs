use std::cmp::Ordering;

pub fn compare_by_semver(a: &str, b: &str) -> Ordering {
    let a_parts: Vec<i32> = a
        .split('.')
        .map(|p| p.parse::<i32>().unwrap_or(0))
        .collect();
    let b_parts: Vec<i32> = b
        .split('.')
        .map(|p| p.parse::<i32>().unwrap_or(0))
        .collect();

    let len = std::cmp::min(a_parts.len(), b_parts.len());

    for i in 0..len {
        match a_parts[i].cmp(&b_parts[i]) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => continue,
        }
    }

    a_parts.len().cmp(&b_parts.len())
}

#[test]
fn sorts_by_semver_correctly() {
    assert_eq!(compare_by_semver("29.13.5", "29.13.5"), Ordering::Equal);
    assert_eq!(compare_by_semver("1.0.5", "1.0.0"), Ordering::Greater);
    assert_eq!(compare_by_semver("1.2.0", "1.2.3"), Ordering::Less);

    let mut versions = [
        "3.4.7", "2.2.0", "0.0.3", "0.9.5", "1.1.1", "1.1.0", "0.3.3", "1.7.8", "0.105.50",
    ];

    versions.sort_by(|a, b| compare_by_semver(a, b));

    assert_eq!(
        versions,
        ["0.0.3", "0.3.3", "0.9.5", "0.105.50", "1.1.0", "1.1.1", "1.7.8", "2.2.0", "3.4.7",]
    );
}
