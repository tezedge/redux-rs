use crate::{ActionWithId, Store};

pub type Effects<State, Service, Action> =
    fn(&mut Store<State, Service, Action>, &ActionWithId<Action>);
