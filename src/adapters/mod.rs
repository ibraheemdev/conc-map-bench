pub use self::{dashmap::DashMapTable, flurry::FlurryTable, papaya::PapayaTable};

mod dashmap;
mod flurry;
mod papaya;
// mod std;
// mod btreemap;
// mod chashmap;
// mod contrie;
// mod crossbeam_skiplist;
// mod evmap;

type Value = u32;
