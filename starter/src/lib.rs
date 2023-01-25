use std::{cell::RefCell, ops::Deref};

use candid::{candid_method, CandidType, Deserialize};
use ic_cdk::{
    api::stable::{StableReader, StableWriter},
    post_upgrade, pre_upgrade, query, update,
};
use serde::Serialize;
use todo::Todo;
mod todo;

#[derive(Default, CandidType, Serialize, Deserialize)]
struct State {
    pub todos: Vec<Todo>,
}

thread_local! {
    static STATE: RefCell<State>  = RefCell::new(State::default());
}

#[update]
#[candid_method(update)]
fn create_note(text: String) -> () {
    STATE.with(|s| s.borrow_mut().todos.push(Todo { text }));
}

#[query]
#[candid_method(query)]
fn get_note(id: u32) -> Option<Todo> {
    STATE.with(|s| s.borrow().todos.get(id as usize).cloned())
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|cell| {
        ciborium::ser::into_writer(cell.borrow().deref(), StableWriter::default())
            .expect("failed to encode state")
    })
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|cell| {
        *cell.borrow_mut() =
            ciborium::de::from_reader(StableReader::default()).expect("failed to decode state");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::export_service;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        export_service!();
        write(dir.join("starter.did"), __export_service()).expect("Write failed.");
    }
}
