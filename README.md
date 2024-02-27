I'm learning Rust and I want to build things that I will actually use. That's why I created **wd**. There is already a tool called wd, but it's hard to find, so I built my own version.

The purpose and functionality is to set aliases for commonly visited directories.


## Usage

`add <name>`: Add a warppoint with the name at the current working directory.

`rm <name>`: Remove a warpoint with the given name.

`list`: List all warps.

## Installation

Copy release binary to `~/.cargo/bin/wd`.

add a command to your `.zshrc` or `.bashrc` file like so:
```bash
wd () {
	dir=$(~/.cargo/bin/wd "$@")
	if [[ -n "$dir" ]]
	then
		cd "$dir"
	fi
}
```
