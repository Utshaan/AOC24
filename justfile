run day: build
    @./target/debug/advent_of_code_23 --day {{day}}

build:
    @cargo build

tests:
    @cargo test --workspace --quiet

test day:
    @cargo test {{day}} --quiet

clean:
    @cargo clean
