### Name : Rajeswari NB
### Student Id : 916809371
### CS 510 -Rust programming Winter 2020 HW4

### Step 1 Project setup
Create new library project using `cargo new unique` command. The name `unique` is seen in the Cargo.toml and I added name, email in Cargo.toml.

To ensure my setup works fine I ran `Cargo build`

### Step 2 Implement and Test Algorithm
Unique implementor searches for unique element of an iterator that satisfies a given predicate. To do this the unique implementor function runs a filter and collects all the elements that satisfy the given predicate. Then it returns Some(element) if exactly one element returns true for the predicate function else None is returned. This algorithm is verified using the given test conditions.

### Step 3 Using cargo tools
I used cargo fmt and cargo clippy via terminal and formatted my code on par to Rust standards and ensured no warnings messages are seen.