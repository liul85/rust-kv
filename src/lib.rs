// pub trait CommandService {
//     fn execute(self, store: &impl Storage) -> CommandResponse;
// }

// pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
//     match cmd.request_data {
//         Some(RequestData::Hget(params)) => params.execute(store),
//     }
// }

// pub trait Storage {
//     fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
//     fn set(&self, table: &str, key: &str, value: Value) -> Result<Option<Value>, KvError>;
//     fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
//     fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
//     fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
//     fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
// }
mod pb;
pub use pb::abi::*;
