#!/bin/bash
cd() {
	builtin cd "$@"

	current_dir=$(pwd)

	target/debug/gwd "$current_dir"
}
