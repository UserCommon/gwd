pub mod hook;
pub mod manage_file;

pub fn state_path() -> String {
    match std::env::var("HOME") {
        Ok(path) => {
            let mut p = path;
            p.push_str("/.gwd_state");
            p
        }
        Err(_) => "/.gwd_state".into(),
    }
}
#[cfg(test)]
mod utils_test {
    #[test]
    fn init_test() {
        assert_eq!(2 + 2, 4);
    }
}
