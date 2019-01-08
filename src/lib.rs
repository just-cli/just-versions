use just_core::blueprint::instruction::Versions;
use semver::{Version, VersionReq};

pub fn find_matching_version(versions: &Versions, req: Option<VersionReq>) -> Option<Version> {
    if let Some(constraint) = req {
        find_all_versions(versions)
            .iter()
            .find(|version| constraint.matches(version))
            .map(|version| version.to_owned())
    } else {
        find_all_versions(versions)
            .first()
            .map(|version| version.to_owned())
    }
}

pub fn find_all_versions(versions: &Versions) -> Vec<Version> {
    use itertools::Itertools;
    use just_core::system::Cmd;
    use regex::Regex;

    Cmd::parse(&versions.cmd)
        .and_then(|cmd| {
            let re = Regex::new(&versions.pattern).expect("Invalid Regex");
            let versions = cmd
                .read()
                .and_then(|content| {
                    let captures: Vec<&str> = re
                        .captures_iter(&content)
                        .filter_map(|caps| caps.get(1))
                        .map(|m| m.as_str())
                        .unique()
                        .collect();
                    let versions: Vec<Version> = captures
                        .iter()
                        .map(|vs| {
                            Version::parse(vs)
                                .unwrap_or_else(|e| panic!("Version {} is invalid: {:?}", vs, e))
                        })
                        .collect();

                    Ok(versions)
                })
                .expect("Could not find any versions");

            Some(versions)
        })
        .unwrap_or_default()
}
