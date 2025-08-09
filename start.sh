#!/bin/zsh

cat <<'EOF' >> ~/.zshrc
alias run="clear && cargo run"
push() {
	cd ~/Desktop/piscine-rust/
	git add .
	git commit -m "$1"
	git push 
	git push github
}

lib() {
	cargo new --lib "$1"
	cd "$1"
	touch src/main.rs
	: > src/lib.rs
	code src/main.rs
	code src/lib.rs
}
EOF

source ~/.zshrc
clear