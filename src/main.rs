use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use cid::Cid;
use multihash::{Code, MultihashDigest};
use fvm_ipld_blockstore::{Blockstore, MemoryBlockstore};
use fvm_ipld_encoding::RawBytes;
use fvm_ipld_hamt::{BytesKey, Error as HamtError, Hamt};
use fvm_shared::HAMT_BIT_WIDTH;
use fvm_shared::address::Address;
use fvm_shared::bigint::bigint_ser;
use fvm_shared::econ::TokenAmount;

// https://docs.rs/fvm_ipld_hamt/0.5.1/fvm_ipld_hamt/
// https://github.com/Schwartz10/sample-erc20-fvm-actor/blob/43acb18d9509859d9adc1d50e1fed2d2cf6f023e/src/lib.rs#L76

/// Map type to be used within actors. The underlying type is a HAMT.
// From builtin-actors actors/runtime/src/lib.rs
pub type Map<'bs, BS, V> = Hamt<&'bs BS, V, BytesKey>;

/// Create a hamt with a custom bitwidth.
#[inline]
pub fn make_empty_map<BS, V>(store: &'_ BS, bitwidth: u32) -> Map<'_, BS, V>
where
    BS: Blockstore,
    V: DeserializeOwned + Serialize,
{
    Map::<_, V>::new_with_bit_width(store, bitwidth)
}

/// Create a map with a root cid.
#[inline]
pub fn make_map_with_root<'bs, BS, V>(
    root: &Cid,
    store: &'bs BS,
) -> Result<Map<'bs, BS, V>, HamtError>
where
    BS: Blockstore,
    V: DeserializeOwned + Serialize,
{
    Map::<_, V>::load_with_bit_width(root, store, HAMT_BIT_WIDTH)
}


#[derive(Serialize, Debug)]
pub struct BountyKey {
    pub piece_cid: Cid,
    pub address: Address,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BountyValue {
    #[serde(with = "bigint_ser")]
    pub amount: TokenAmount,
}

const RAW: u64 = 0x55;

fn main() {
    println!("Hello, world!");
    let store = MemoryBlockstore::default();

    let mut names : Hamt<_, String, usize> = Hamt::new(&store);
    names.set(1, "Jim".to_string()).unwrap();
    println!("1: {}", names.get(&1).unwrap().unwrap());
    names.set(2, "Sheldon".to_string()).unwrap();
    println!("2: {}", names.get(&2).unwrap().unwrap());

    let bounties_cid = make_empty_map::<_, ()>(&store, HAMT_BIT_WIDTH)
            .flush()
            .unwrap();

    let mut bounties = make_map_with_root::<_, BountyValue>(&bounties_cid, &store).unwrap();

    // https://crates.io/crates/cid
    let h = Code::Sha2_256.digest(b"beep boop");
    let cid = Cid::new_v1(RAW, h);
    // let data = cid.to_bytes();

    let key1 = BountyKey { piece_cid: cid, address: Address::new_id(100) };
    println!("key1 {:?}", &key1);
    let raw_bytes = RawBytes::serialize(&key1).unwrap();
    let bytes = raw_bytes.bytes();
    println!("key1 bytes {:?}", &bytes);
    let bounty1_value = BountyValue { amount: TokenAmount::from(10) };
    let key1 = BytesKey::from(bytes);
    let key1_clone = key1.clone();
    bounties.set(key1, bounty1_value).unwrap();
    let retrieved1_value = bounties.get(&key1_clone);
    println!("Retrieved value key1 {:?}", &retrieved1_value);

    let key2 = BountyKey { piece_cid: cid, address: Address::new_id(101) };
    println!("key2 {:?}", &key2);
    let raw_bytes = RawBytes::serialize(&key2).unwrap();
    let bytes = raw_bytes.bytes();
    println!("key2 bytes {:?}", &bytes);
    let bounty2_value = BountyValue { amount: TokenAmount::from(20) };
    let key2 = BytesKey::from(bytes);
    let key2_clone = key2.clone();
    bounties.set(key2, bounty2_value).unwrap();
    let retrieved2_value = bounties.get(&key2_clone);
    println!("Retrieved value key2 {:?}", &retrieved2_value);

    bounties.for_each(|k, v: &BountyValue| {
      println!("k {:?} v {:?}", &k, &v);
      Ok(())
    });

    list_bounties(&bounties);
}

fn list_bounties(&bounties: Map<_, _, BountyValue>) {
    bounties.for_each(|k, v: &BountyValue| {
      println!("k {:?} v {:?}", &k, &v);
      Ok(())
    });
}

// https://github.com/filecoin-project/ref-fvm/blob/29ac9a32459ac1172c69c68640182570b24562dc/ipld/hamt/tests/hamt_tests.rs

#[test]
fn test_basics() {
    let store = MemoryBlockstore::default();
    let mut hamt = Hamt::<_, String, _>::new(&store);
    hamt.set(1, "world".to_string()).unwrap();

    assert_eq!(hamt.get(&1).unwrap(), Some(&"world".to_string()));
    hamt.set(1, "world2".to_string()).unwrap();
    assert_eq!(hamt.get(&1).unwrap(), Some(&"world2".to_string()));
}
