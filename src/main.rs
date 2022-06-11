use fvm_ipld_blockstore::MemoryBlockstore;
use fvm_ipld_hamt::Hamt;

fn main() {
    println!("Hello, world!");
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
