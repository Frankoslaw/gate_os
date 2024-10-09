with import <nixpkgs> {};

mkShell {
	buildInputs = [ xorriso qemu rustup python3 gdb ];
}