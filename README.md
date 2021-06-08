[![Latest version](https://img.shields.io/crates/v/sexually_transmitted_disease.svg)](https://crates.io/crates/sexually_transmitted_disease)

When you add this crate as a dependency, you can refer to `std` modules as `sexually_transmitted_disease`.

For example:
```rust
fn main() {
    // Disease vector
    let diseases: sexually_transmitted_disease::vec::Vec<&str> =
        ["HIV/AIDS", "HPV", "herpes", "chlamydia", "hepatitis"].into();

    let (sender, receiver) = sexually_transmitted_disease::sync::mpsc::channel();
    for disease in diseases {
        sender.send(disease);
    }

    println!("{}", receiver.recv().unwrap()); // HIV/AIDS
}

```
