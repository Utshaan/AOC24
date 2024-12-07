run day: build
    @./target/debug/aoc24 --day {{day}}

build:
    @cargo build

tests:
    @cargo test --workspace --quiet

test name:
    @cargo test {{name}} --quiet

clean:
    @cargo clean
