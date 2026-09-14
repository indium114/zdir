build:
    nix build .#zdir

run:
    cargo run -p zdir

run-daemon:
    cargo run -p zdir-librarian

publish:
    echo "=== Publishing zdir-librarian"
    cargo publish -p zdir-librarian
    echo "=== Publishing zdir"
    cargo publish -p zdir
