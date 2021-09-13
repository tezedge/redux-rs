use crate::{Middleware, Reducer, Subscription, Vec};

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
    subscriptions: Vec<Subscription<State>>
}

impl<State, Service, Action> Store<State, Service, Action> {
    /// Creates a new store.
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// type State = i8;
    ///
    /// enum Action {
    ///     Increment,
    ///     Decrement
    /// }
    ///
    /// fn reducer(state: &State, action: &Action) -> State {
    ///     match action {
    ///         Action::Increment => state + 1,
    ///         Action::Decrement => state - 1
    ///     }
    /// }
    ///
    /// let mut store = Store::new(reducer, 0);
    /// ```
    pub fn new(reducer: Reducer<State, Action>, service: Service, initial_state: State) -> Self {
        Self {
            reducer,
            service,
            state: StateWrapper {
                inner: initial_state
            },
            middlewares: Vec::new(),
            subscriptions: Vec::new()
        }
    }

    /// Returns the current state.
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// # let store = Store::new(|&u8, ()| 0, 0);
    /// #
    /// println!("Current state: {}", store.state());
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// # type State = i8;
    /// #
    /// enum Action {
    ///     DoSomething,
    ///     DoSomethingElse
    /// }
    ///
    /// // ...
    ///
    /// # fn reducer(state: &u8, action: &Action) -> u8 {
    /// #     0
    /// # }
    /// #
    /// # let mut store = Store::new(reducer, 0);
    /// #
    /// store.dispatch(Action::DoSomething);
    /// println!("Current state: {}", store.state());
    /// ```
    pub fn dispatch(&mut self, action: Action) {
        self.dispatch_reducer(&action);
        for i in 0..self.middlewares.len() {
            self.middlewares[i](self, &action);
        }
    }

    /// Runs the reducer.
    #[inline(always)]
    fn dispatch_reducer(&mut self, action: &Action) {
        (&self.reducer)(self.state.get_mut(), action);
        self.dispatch_subscriptions();
    }

    /// Runs all subscriptions.
    fn dispatch_subscriptions(&self) {
        for subscription in &self.subscriptions {
            subscription(self.state());
        }
    }

    /// Subscribes a callback to any change of the state.
    ///
    /// Subscriptions will be called, whenever an action is dispatched.
    ///
    /// See [`Subscription`](type.Subscription.html).
    ///
    /// # Example
    ///
    /// ```
    /// use redux_rs::{Store, Subscription};
    /// #
    /// # type State = u8;
    /// # let initial_state = 0;
    /// #
    /// # fn reducer(_: &State, action: &bool) -> State {
    /// #     0
    /// # }
    ///
    /// let mut store = Store::new(reducer, initial_state);
    ///
    /// let listener: Subscription<State> = |state: &State| {
    ///     println!("Something changed! New value: {}", state);
    /// };
    ///
    /// store.subscribe(listener);
    /// ```
    pub fn subscribe(&mut self, callback: Subscription<State>) {
        self.subscriptions.push(callback);
    }

    /// Adds a custom middleware to the store.
    ///
    /// Middleware provides the possibility to intercept actions dispatched before they reach the reducer.
    ///
    /// See [`Middleware`](type.Middleware.html).
    pub fn add_middleware(&mut self, middleware: Middleware<State, Service, Action>) {
        self.middlewares.push(middleware);
    }

    /// Replaces the currently used reducer.
    ///
    /// # Example
    ///
    /// ```
    /// # use redux_rs::Store;
    /// #
    /// # pub struct State(u8);
    /// #
    /// # impl State {
    /// #     pub fn something_else() -> State {
    /// #         State(1)
    /// #     }
    /// # }
    /// #
    /// # enum Action {
    /// #     SomeAction
    /// # }
    /// #
    /// # fn reducer(state: &State, action: &Action) -> State {
    /// #     State(0)
    /// # }
    /// #
    /// # let mut store = Store::new(reducer, State(0));
    /// #
    /// store.dispatch(Action::SomeAction);
    ///
    /// store.replace_reducer(|state: &State, action: &Action| {
    ///     State::something_else()
    /// });
    ///
    /// store.dispatch(Action::SomeAction);
    /// ```
    pub fn replace_reducer(&mut self, reducer: Reducer<State, Action>) {
        self.reducer = reducer;
    }
}
