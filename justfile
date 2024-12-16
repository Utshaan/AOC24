run day: build
    @./target/release/aoc24 --day {{day}}

build:
    @cargo build --release

tests:
    @cargo test --workspace --quiet

test name:
    @cargo test {{name}} --quiet

clean:
    @cargo clean
