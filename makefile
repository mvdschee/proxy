dev:
	CONFIG=./example/example.toml watchexec -q -c -w src --exts rs --restart "cargo run"
