pub fn redefine_cd() -> String {
    String::from(
        r#"cd() {
                    builtin cd "$@" 
                    new_dir=$(pwd)
                    gwd "$new_dir"
                }"#,
    )
}
