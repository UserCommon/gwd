# GWD (Global Work Directory)

gwd is simple utility to provide auto - entering in
previous directory in new terminal(for bash/zsh)

## how to use?
compile and move to /usr/bin:
```bash
git clone https://github.com/UserCommon/gwd
cd gwd
cargo build --release
mv ./target/release/gwd /usr/bin/
```

add following line into your .zshrc or .bashrc:
```bash
eval "$(gwd)"
```

now it will create .gwd_state in your home directory and 
will write/read path to your last `cd`. it is also overriding
cd in `utils/hook.rs` to execute gwd with path
