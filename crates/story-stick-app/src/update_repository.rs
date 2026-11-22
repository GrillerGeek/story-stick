//! Build-time update source normalization, shared with the regression tests.
pub const REPOSITORY: &str = "GrillerGeek/story-stick";

pub fn url(value: &str) -> Result<String, String> {
    let value = value.trim().trim_end_matches('/').trim_end_matches(".git");
    let value = value
        .strip_prefix("https://github.com/")
        .or_else(|| value.strip_prefix("git@github.com:"))
        .or_else(|| value.strip_prefix("ssh://git@github.com/"))
        .unwrap_or(value);
    let parts: Vec<_> = value.split('/').collect();
    if parts.len() != 2
        || parts.iter().any(|part| {
            part.is_empty()
                || *part == "."
                || *part == ".."
                || !part
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
        })
    {
        return Err("Set STORY_STICK_UPDATE_REPOSITORY to a GitHub owner/repository".into());
    }
    Ok(format!("https://github.com/{value}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upstream_repositories_are_not_remapped_to_another_feed() {
        for (input, expected) in [
            ("https://github.com/BenchUp3D/app/", "https://github.com/BenchUp3D/app"),
            ("git@github.com:Ketchup3D/app.git", "https://github.com/Ketchup3D/app"),
            ("ketchup3d/editor", "https://github.com/ketchup3d/editor"),
        ] {
            assert_eq!(url(input).unwrap(), expected);
        }
        assert_eq!(url("ssh://git@github.com/someone/fork.git").unwrap(), "https://github.com/someone/fork");
    }

    #[test]
    fn custom_forks_are_preserved_and_invalid_sources_are_rejected() {
        assert_eq!(
            url("someone/editor").unwrap(),
            "https://github.com/someone/editor"
        );
        assert_eq!(
            url("StoryStick3D/other").unwrap(),
            "https://github.com/StoryStick3D/other"
        );
        for input in [
            "",
            "../editor",
            "org/repo/extra",
            "https://example.org/org/repo",
            "org/repo?token=x",
        ] {
            assert!(url(input).is_err());
        }
    }
}
