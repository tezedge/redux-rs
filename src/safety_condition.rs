pub trait SafetyCondition {
    type Error;

    /// Safety condition for some State.
    ///
    /// Checks if the safety conditions pass for a given state. In other
    /// words it checks if the state is valid.
    fn check_safety_condition(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
