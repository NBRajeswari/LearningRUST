### Name : Rajeswari NB
### Student Id : 916809371
### CS 510 -Rust programming Winter 2020 HW2

### Step 1 Project setup
Create new library project using `cargo new toy_rsa` command. The name `toy_rsa` is seen in the Cargo.toml and I added name, email and added the crate `toy_rsa_lib` in Cargo.toml.

To ensure my setup works fine I ran `Cargo build`

### Step 2 Implement and Test Algorithm
created the functions lambda, encrypt, decrypt and genkey using the given pseudocode as reference and using the utility functions from toy_rsa_lib crate. Also added unit testing and assert conditions. Since genkey function could not be unit tested due to its randomness, I used tests/random.rs file for do some black box testing to see I am able to successfully encrypt and decrypt messages.

### Step 3 Using cargo tools
I used cargo fmt and cargo clippy via terminal and formatted my code on par to Rust standards and ensured no warnings messages are seen.