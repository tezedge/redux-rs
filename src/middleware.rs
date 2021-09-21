use crate::{ActionWithId, Store};

pub type Middleware<State, Service, Action> =
    fn(&mut Store<State, Service, Action>, &ActionWithId<Action>);
