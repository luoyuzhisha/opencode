use codex_utils_absolute_path::AbsolutePathBuf;
use dirs::home_dir;
use std::path::PathBuf;

const OPENCODE_HOME_ENV: &str = "OPENCODE_HOME";
const LEGACY_CODEX_HOME_ENV: &str = "CODEX_HOME";
const DEFAULT_HOME_DIRNAME: &str = ".opencode";

/// Returns the path to the OpenCode configuration directory, which can be
/// specified by the `OPENCODE_HOME` environment variable (or legacy
/// `CODEX_HOME`). If neither is set, defaults to `~/.opencode`.
///
/// - If `OPENCODE_HOME`/`CODEX_HOME` is set, the value must exist and be a
///   directory. The
///   value will be canonicalized and this function will Err otherwise.
/// - If neither env var is set, this function does not verify that the
///   directory exists.
pub fn find_codex_home() -> std::io::Result<AbsolutePathBuf> {
    let codex_home_env = std::env::var(OPENCODE_HOME_ENV)
        .ok()
        .filter(|val| !val.is_empty())
        .or_else(|| {
            std::env::var(LEGACY_CODEX_HOME_ENV)
                .ok()
                .filter(|val| !val.is_empty())
        });
    find_codex_home_from_env(codex_home_env.as_deref())
}

fn home_env_error_label() -> String {
    format!("{OPENCODE_HOME_ENV}/{LEGACY_CODEX_HOME_ENV}")
}

fn find_codex_home_from_env(codex_home_env: Option<&str>) -> std::io::Result<AbsolutePathBuf> {
    // Honor OPENCODE_HOME/CODEX_HOME when set to allow users (and tests) to
    // override the default location.
    match codex_home_env {
        Some(val) => {
            let path = PathBuf::from(val);
            let env_label = home_env_error_label();
            let metadata = std::fs::metadata(&path).map_err(|err| match err.kind() {
                std::io::ErrorKind::NotFound => std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!(
                        "{env_label} points to {val:?}, but that path does not exist"
                    ),
                ),
                _ => std::io::Error::new(
                    err.kind(),
                    format!("failed to read {env_label} {val:?}: {err}"),
                ),
            })?;

            if !metadata.is_dir() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("{env_label} points to {val:?}, but that path is not a directory"),
                ))
            } else {
                let canonical = path.canonicalize().map_err(|err| {
                    std::io::Error::new(
                        err.kind(),
                        format!("failed to canonicalize {env_label} {val:?}: {err}"),
                    )
                })?;
                AbsolutePathBuf::from_absolute_path(canonical)
            }
        }
        None => {
            let mut p = home_dir().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find home directory",
                )
            })?;
            p.push(DEFAULT_HOME_DIRNAME);
            AbsolutePathBuf::from_absolute_path(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_codex_home_from_env;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use dirs::home_dir;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::ErrorKind;
    use tempfile::TempDir;

    #[test]
    fn find_codex_home_env_missing_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let missing = temp_home.path().join("missing-opencode-home");
        let missing_str = missing
            .to_str()
            .expect("missing opencode home path should be valid utf-8");

        let err = find_codex_home_from_env(Some(missing_str))
            .expect_err("missing OPENCODE_HOME/CODEX_HOME");
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(
            err.to_string().contains("OPENCODE_HOME/CODEX_HOME"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_env_file_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let file_path = temp_home.path().join("opencode-home.txt");
        fs::write(&file_path, "not a directory").expect("write temp file");
        let file_str = file_path
            .to_str()
            .expect("file opencode home path should be valid utf-8");

        let err = find_codex_home_from_env(Some(file_str))
            .expect_err("file OPENCODE_HOME/CODEX_HOME");
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("not a directory"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_env_valid_directory_canonicalizes() {
        let temp_home = TempDir::new().expect("temp home");
        let temp_str = temp_home
            .path()
            .to_str()
            .expect("temp opencode home path should be valid utf-8");

        let resolved =
            find_codex_home_from_env(Some(temp_str)).expect("valid OPENCODE_HOME/CODEX_HOME");
        let expected = temp_home
            .path()
            .canonicalize()
            .expect("canonicalize temp home");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_without_env_uses_default_home_dir() {
        let resolved = find_codex_home_from_env(/*codex_home_env*/ None)
            .expect("default OPENCODE_HOME/CODEX_HOME");
        let mut expected = home_dir().expect("home dir");
        expected.push(".opencode");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }
}
