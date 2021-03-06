use config::{Config, Environment, File};
use std::path::MAIN_SEPARATOR;

#[cfg(target_os = "windows")]
const CFG_FILE: &str = "ctehxk2";
#[cfg(not(target_os = "windows"))]
const CFG_FILE: &str = "libctehxk2";

#[derive(Debug, Deserialize)]
pub struct Settings {
    timeout: u64,
    base_url: String,
    log_level: String,
    log_path: Option<String>,
    ctn: Option<u16>,
    pn: Option<u16>,
}

impl Settings {
    fn new() -> Self {
        let mut settings = Config::new();

        // set defaults
        settings
            .set_default("base_url", "http://localhost:8080/k2/ctapi/")
            .expect("Failed to set default for base_url!")
            .set_default("log_level", "Error")
            .expect("Failed to set default for log_level!")
            .set_default("timeout", 5000)
            .expect("Failed set default for timeout!");

        // merge with optional config file and env variables
        settings
            .merge(File::with_name(CFG_FILE).required(false))
            .expect("Failed to merge config file!")
            .merge(Environment::with_prefix("k2"))
            .expect("Failed to merge env variables!");

        settings.try_into().expect("Failed to create configuration")
    }

    pub fn base_url() -> String {
        let s = Settings::new();
        let mut url = s.base_url.clone();
        if !url.trim().ends_with("/") {
            url.push_str("/");
        }
        url
    }

    pub fn ctn_or(fallback: u16) -> u16 {
        let s = Settings::new();
        match s.pn {
            Some(_pn) => match s.ctn {
                Some(ctn) => ctn,
                None => fallback,
            },
            None => fallback,
        }
    }

    pub fn pn_or(fallback: u16) -> u16 {
        let s = Settings::new();
        match s.ctn {
            Some(_ctn) => match s.pn {
                Some(pn) => pn,
                None => fallback,
            },
            None => fallback,
        }
    }

    pub fn log_level() -> String {
        let s = Settings::new();
        s.log_level.clone()
    }

    pub fn log_path() -> Option<String> {
        let s = Settings::new();
        match s.log_path {
            Some(mut path) => {
                if !path.trim().ends_with(MAIN_SEPARATOR) {
                    path.push(MAIN_SEPARATOR);
                }

                Some(path)
            }
            None => None,
        }
    }

    pub fn timeout() -> u64 {
        let s = Settings::new();
        s.timeout.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;
    use std::env;
    use std::path::MAIN_SEPARATOR;

    #[test]
    fn ctn_or_return_env_value_if_pn_is_set_too() {
        env::set_var("K2_CTN", "2");
        env::set_var("K2_PN", "4");

        let ctn = Settings::ctn_or(1);

        assert_eq!(ctn, 2);
    }

    #[test]
    fn ctn_or_return_given_value_if_pn_not_set() {
        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");

        let ctn = Settings::ctn_or(1);

        assert_eq!(ctn, 1);

        env::set_var("K2_CTN", "2");

        let ctn = Settings::ctn_or(1);

        assert_eq!(ctn, 1);
    }

    #[test]
    fn pn_or_return_env_value_if_ctn_is_set_too() {
        env::set_var("K2_CTN", "2");
        env::set_var("K2_PN", "4");

        let pn = Settings::pn_or(1);

        assert_eq!(pn, 4);
    }

    #[test]
    fn pn_or_return_given_value_if_ctn_not_set() {
        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");

        let pn = Settings::pn_or(1);

        assert_eq!(pn, 1);

        env::set_var("K2_PN", "2");

        let pn = Settings::pn_or(1);

        assert_eq!(pn, 1);
    }

    #[test]
    fn base_url_return_env_value() {
        env::set_var("K2_BASE_URL", "a/");

        let url = Settings::base_url();

        assert_eq!(url, "a/");

        env::set_var("K2_BASE_URL", "1");

        let url = Settings::base_url();

        assert_eq!(url, "1/");
    }

    #[test]
    fn base_url_return_default_value_if_no_env() {
        env::remove_var("K2_BASE_URL");

        let url = Settings::base_url();

        assert_eq!(url, "http://localhost:8080/k2/ctapi/");
    }

    #[test]
    fn log_path_returns_env_value() {
        env::set_var("K2_LOG_PATH", "a");

        let path = Settings::log_path();

        assert_eq!(path, Some(format!("a{}", MAIN_SEPARATOR)));
    }

    #[test]
    fn log_path_return_none_if_no_env() {
        env::remove_var("K2_LOG_PATH");

        let path = Settings::log_path();

        assert!(path.is_none());
    }

    #[test]
    fn timeout_returns_env_value() {
        env::set_var("K2_TIMEOUT", "42");

        let timeout = Settings::timeout();

        assert_eq!(timeout, 42);
    }

    #[test]
    fn timeout_return_default_if_no_env() {
        env::remove_var("K2_TIMEOUT");

        let timeout = Settings::timeout();

        assert_eq!(timeout, 5000);
    }
}
