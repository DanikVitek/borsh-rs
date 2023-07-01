use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        hash_collections: { any(feature = "hashbrown", feature = "std") },
        derive_schema: { all(feature = "schema", feature = "derive")},
    }
}
