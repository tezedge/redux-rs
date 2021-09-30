use crate::{ActionId, ActionWithId, Middleware, Reducer, Vec};

/// Wraps around State and allows only immutable borrow,
/// Through `StateWrapper::get` method.
///
/// Mutable borrow of state can only happen in reducer.
pub struct StateWrapper<State> {
    inner: State,
}

impl<State> StateWrapper<State> {
    /// Get immutable reference to State.
    #[inline(always)]
    pub fn get(&self) -> &State {
        &self.inner
    }

    /// Get mutable reference to State.
    ///
    /// Only should be used in reducer and is not `pub`
    /// so it can't be accessed from lib users.
    #[inline(always)]
    fn get_mut(&mut self) -> &mut State {
        &mut self.inner
    }
}

impl<T: Clone> Clone for StateWrapper<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// A container holding a state and providing the possibility to dispatch actions.
///
/// A store is defined by the state is holds and the actions it can dispatch.
pub struct Store<State, Service, Action> {
    reducer: Reducer<State, Action>,
    /// Current State.
    ///
    /// Immutable access can be gained using `store.state.get()`.
    /// Mutation can only happen inside reducer.
    pub state: StateWrapper<State>,
    pub service: Service,
    middlewares: Vec<Middleware<State, Service, Action>>,
    last_action_id: ActionId,
}

impl<State, Service, Action> Store<State, Service, Action> {
    /// Creates a new store.
    pub fn new(reducer: Reducer<State, Action>, service: Service, initial_state: State) -> Self {
        Self {
            reducer,
            service,
            state: StateWrapper {
                inner: initial_state,
            },
            middlewares: Vec::new(),
            last_action_id: ActionId(0),
        }
    }

    /// Returns the current state.
    #[inline(always)]
    pub fn state(&self) -> &State {
        self.state.get()
    }

    #[inline(always)]
    pub fn service(&mut self) -> &mut Service {
        &mut self.service
    }

    /// Dispatches an action which is handles by the reducer, after the store got passed through the middleware.
    /// This can modify the state within the store.
    pub fn dispatch(&mut self, action: Action) {
        let action_with_id = ActionWithId {
            id: self.last_action_id.increment(),
            action,
        };

        self.dispatch_reducer(&action_with_id);
        for i in 0..self.middlewares.len() {
            self.middlewares[i](self, &action_with_id);
        }
    }

    /// Runs the reducer.
    #[inline(always)]
    fn dispatch_reducer(&mut self, action_with_id: &ActionWithId<Action>) {
        (&self.reducer)(self.state.get_mut(), action_with_id);
    }

    /// Adds a custom middleware to the store.
    ///
    /// Middleware provides the possibility to intercept actions dispatched before they reach the reducer.
    ///
    /// See [`Middleware`](type.Middleware.html).
    pub fn add_middleware(&mut self, middleware: Middleware<State, Service, Action>) {
        self.middlewares.push(middleware);
    }
}

impl<State, Service, Action> Clone for Store<State, Service, Action>
where
    State: Clone,
    Service: Clone,
    Action: Clone,
{
    fn clone(&self) -> Self {
        Self {
            reducer: self.reducer,
            service: self.service.clone(),
            state: self.state.clone(),
            middlewares: self.middlewares.clone(),
            last_action_id: self.last_action_id.clone(),
        }
    }
}
