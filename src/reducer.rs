use crate::ActionWithMeta;

/// Function signature for a reducer.
pub type Reducer<State, Action> = fn(&mut State, &ActionWithMeta<Action>);

#[macro_export]
/// Combines multiple reducers into a single one.
///
/// The first one gets called first, chained into the second one and so on...
///
/// # Example
///
/// ```
/// # use redux_rs::{chain_reducers, Reducer, ActionWithMeta};
/// #
/// enum Action {
///     Increment,
///     Decrement
/// }
///
/// fn counter_reducer(state: &mut u8, action: &ActionWithMeta<Action>) {
///     *state = match &action.action {
///         Action::Increment => *state + 1,
///         Action::Decrement => *state - 1
///     }
/// }
///
/// fn add_two_reducer(state: &mut u8, _: &ActionWithMeta<Action>) {
///     *state += 2
/// }
///
/// fn reducer(state: &mut u8, action: &ActionWithMeta<Action>) {
///    chain_reducers!(
///        state,
///        action,
///        counter_reducer,
///        add_two_reducer
///    );
/// }
/// ```
macro_rules! chain_reducers {
    ($state:ident, $action:ident, $reducer:ident) => {
        $reducer($state, $action);
    };
    ($state:ident, $action:ident, $($reducer:ident),+) => {
        $( $reducer($state, $action) );+
    }
}
