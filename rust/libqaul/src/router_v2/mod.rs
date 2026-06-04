/// Instance-based router state that owns all routing sub-state.
/// This is the major struct that will replace the current Router.
/// Each `RouterState` instance is fully independent, enabling multiple
/// nodes to run in the same process.
pub struct NewRouterState {}

impl NewRouterState  {
    pub fn new() -> Self {
        Self {  }
    }
}