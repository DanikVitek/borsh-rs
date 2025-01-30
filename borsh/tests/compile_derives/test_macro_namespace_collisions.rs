// Borsh macros should not collide with the local modules:
// https://github.com/near/borsh-rs/issues/11
mod std {}
mod core {}

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
struct A;

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
enum B {
    C,
    D,
}

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
struct C {
    x: u64,
    #[allow(unused)]
    #[borsh(skip)]
    y: String,
}
