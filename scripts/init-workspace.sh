#!/usr/bin/bash

rm -fr .git
git init
# for some strange reasons this one needa add yourself
# cargo new crates/proc-macros --lib
cargo new crates/types --lib
cargo new crates/macros --lib
cargo new crates/io --lib
cargo new crates/utils --lib
cargo new crates/tests --lib
cargo new crates/bins
cargo new crates/errors --lib

git add .
git commit -am "init-ed workspace crates"

git clone git@github.com:jonhoo/rust-ci-conf.git
mv rust-ci-conf/.github ./.github
rm -fr rust-ci-conf

if command -v python3 &>/dev/null; then
    pip install pre-commit==3.7.0
else
    echo "Python is not installed. Installing Python..."
    if [ "$(uname)" == "Darwin" ]; then
        # MacOS
        brew install python
    elif [ "$(uname -s)" == "Linux" ]; then
        # Linux
        sudo apt-get update
        sudo apt-get install -y python3 python3-pip
    elif [ "$(uname -s)" == "CYGWIN" ] || [ "$(uname -s)" == "MINGW" ]; then
        # Windows
        choco install python
    else
        echo "Unsupported OS. Please install Python manually."
        exit 1
    fi
    pip install pre-commit==3.7.0
fi
pre-commit install



# must do these two last or else error

# /rust-workspace-template# git add .
# error: 'crates/proc-macros/' does not have a commit checked out
# fatal: adding files failed

# sed -i '/\[dependencies\]/i [lib]\nproc-macro = true\n' crates/proc-macros/Cargo.toml

# sed -i 's/pub fn/fn/' crates/proc-macros/src/lib.rs
