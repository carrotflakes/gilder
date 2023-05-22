# Gilder

Gilder is a Rust library for golden testing.

The golden test is a testing method commonly used to detect changes in program output. It ensures that the results remain consistent even when modifying the code.

In a golden test, the program's output is saved in a file known as the golden file to preserve it. During testing, the actual output is compared against the content of the golden file.

Gilder is designed to minimize the implementation cost and provides only one API for usage, which is the `assert_golden!` macro. It can be used similar to `assert_eq!` but without the second argument.

# Getting Started

1. Add Gilder as a dependency in your Cargo.toml file:

``` toml
[dev-dependencies]
gilder = "0.1"
```

2. Write a new test using `assert_golden!` macro:

``` rust
#[test]
fn my_test() {
    use gilder::assert_golden;

    let target = something_you_want_to_test();
    assert_golden!(target);
}
```

The argument to `assert_golden!` must implement the `ToString` trait to write to the golden file.

3. Create golden files:

Golden files are generated by running the tests for the first time.

``` shell
cargo test
```

A golden file with the `.gld` extension will be saved in the same directory as the source file containing the test code.

4. Testing:

When running the tests with the golden files present, the behavior is similar to that of a regular test. The actual output of the test is compared against the stored golden files, and if they match, the test is considered to pass.

``` shell
cargo test
```

5. Updating the golden files:

When you change the output of your function, it is necessary to update the golden files accordingly.

To update the golden files, delete the existing ones and rerun the tests.

## Copyright

Copyright (c) 2023 carrotflakes (carrotflakes@gmail.com)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
