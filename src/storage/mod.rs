use crate::{KvError, Kvpair, Value};
mod memory;
pub use memory::MemTable;

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    #[test]
    #[ignore = "not implemented"]
    fn memtable_get_iter_should_work() {
        let store = MemTable::new();
        test_get_all_iter(store);
    }

    fn test_basic_interface(store: impl Storage) {
        //insert for the first time
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        //set for the same key return the previous value
        let v1 = store.set("t1", "hello".into(), "world1".into());
        assert_eq!(v1, Ok(Some("world".into())));

        //get exsiting key
        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));

        //get a key that doesn't exist
        assert!(store.get("t1", "hello1").unwrap().is_none());
        //get from a table that doesn't exist
        assert!(store.get("t2", "hello").unwrap().is_none());

        //contains
        assert_eq!(store.contains("t1", "hello"), Ok(true));

        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        //del
        assert_eq!(store.del("t1", "hello"), Ok(Some("world1".into())));
        assert_eq!(store.del("t1", "hello1"), Ok(None));
        assert_eq!(store.del("t2", "hello"), Ok(None));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t1", "k1".into(), "v1".into()).unwrap();
        store.set("t1", "k2".into(), "v2".into()).unwrap();

        let mut data = store.get_all("t1").unwrap();

        data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        )
    }

    fn test_get_all_iter(store: impl Storage) {
        store.set("t1", "k1".into(), "v1".into()).unwrap();
        store.set("t1", "k2".into(), "v2".into()).unwrap();

        let mut data: Vec<_> = store.get_iter("t1").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        )
    }
}
