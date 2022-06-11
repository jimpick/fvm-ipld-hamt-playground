use fvm_ipld_blockstore::MemoryBlockstore;
use fvm_ipld_hamt::Hamt;

// https://docs.rs/fvm_ipld_hamt/0.5.1/fvm_ipld_hamt/
// https://github.com/Schwartz10/sample-erc20-fvm-actor/blob/43acb18d9509859d9adc1d50e1fed2d2cf6f023e/src/lib.rs#L76

fn main() {
    println!("Hello, world!");
    let store = MemoryBlockstore::default();
    let mut names : Hamt<_, String, usize> = Hamt::new(&store);
    names.set(1, "Jim".to_string()).unwrap();
    println!("1: {}", names.get(&1).unwrap().unwrap());
    names.set(2, "Sheldon".to_string()).unwrap();
    println!("2: {}", names.get(&2).unwrap().unwrap());
}

#[test]
fn test_basics() {
    let store = MemoryBlockstore::default();
    let mut hamt = Hamt::<_, String, _>::new(&store);
    hamt.set(1, "world".to_string()).unwrap();

    assert_eq!(hamt.get(&1).unwrap(), Some(&"world".to_string()));
    hamt.set(1, "world2".to_string()).unwrap();
    assert_eq!(hamt.get(&1).unwrap(), Some(&"world2".to_string()));
}
