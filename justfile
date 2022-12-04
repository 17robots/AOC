set windows-shell := ["nu", "-c"]
default:
    @just --list

aoc date:
    @mkdir {{invocation_directory()}}/{{date}};
    @enter {{invocation_directory()}}/{{date}}; "fn main() {}" | save ./main.rs; cargo init . --name 'aoc{{date}}'; rm .gitignore .git/ -r; cargo run main.rs; cargo clean;

