build:
    nix build .#zdir

run:
    cargo run -p zdir

run-daemon:
    cargo run -p zdir-librarian
