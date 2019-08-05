//! # visn - "knowledge"
//! ## A simulation engine for eventually consistent systems
//!
//! `visn` provides a simple framework for testing eventually-consistent systems.
//! Users provide a list of synthetic events, of the kind they would like to specify in
//! test code, and a function to map those to real changes in the state of a system.
//!
//! `visn` then provides utility functions for applying those in order; in any order; or
//! in other combinations.
//!
//! # Example
//! ```
//! use visn::{KnowledgeEngine, new_knowledge_engine};
//!
//! // This is a simplistic example of a system being tested
//! #[derive(Debug, Default)]
//! struct SystemUnderTest {
//!     a: String,
//!     b: String
//! }
//!
//! // The two possible changes to the system are setting string A or setting string B
//! #[derive(Debug, Clone)]
//! enum SyntheticEvent {
//!     SetA(&'static str),
//!     SetB(&'static str),
//! }
//!
//! // This function maps SyntheticEvent variants to real changes in the system
//! fn resolve(event: SyntheticEvent, system: SystemUnderTest) -> SystemUnderTest {
//!     let mut system = system;
//!     match event {
//!         SyntheticEvent::SetA(s) => system.a = s.into(),
//!         SyntheticEvent::SetB(s) => system.b = s.into()
//!     };
//!     system
//! }
//!
//! use SyntheticEvent::*;
//! // Create a new knowledge engine
//! let result = new_knowledge_engine(resolve)
//!     // Queue up some events for the engine to execute
//!     .queue_events(&[SetA("a1"), SetB("b1"), SetA("a2")])
//!     // Resolve these events in order, starting from the default state and returning
//!     // the final state of the system.
//!     .resolve_in_order(SystemUnderTest::default);
//!
//! assert_eq!(result.a, "a2".to_string());
//! assert_eq!(result.b, "b1".to_string());
//! ```
mod arbitrary_tandem_control_iter;
mod fallible;
mod infallible;

pub use fallible::new_fallible_engine;
pub use infallible::new_knowledge_engine;

/// The KnowledgeEngine provides a framework for testing the consequences of messages
/// in an eventually consistent system arriving in various orders.
///
/// Synthetic events are queued up and transformed by a given function into
/// actual changes to the state of the system under test. Once this function is defined,
/// each test needs only define a starting state, a list of events to execute, and
/// an expected ending condition.
///
/// # Types
/// - `System`: the type of the system under test.
/// - `Event`: the type of synthetic events.
/// - `Return`: the type returned by the `resolve` function. Can be the same as `System`,
/// or sometimes a `Result<System, _>`.
pub trait KnowledgeEngine<System, Event: Clone, Return>: Sized {
    /// Add a single event to a queue of events to run before permutation.
    fn queue_prologue(self, event: Event) -> Self;

    /// Add a single event to a queue of events to be permuted.
    fn queue_event(self, event: Event) -> Self;

    /// Add a single event to a queue of events to be permuted.
    fn queue_epilogue(self, event: Event) -> Self;

    /// Resolve the queue of events using the given iterator combinator (a function taking
    /// an iterator over events and returning another iterator over events)
    fn resolve_with<
        F: FnOnce(&mut dyn Iterator<Item = Event>) -> &mut dyn Iterator<Item = Event>,
        G: Fn() -> System,
    >(
        self,
        init: G,
        comb: F,
    ) -> Return;

    /// Queue multiple events from a slice.
    fn queue_events(self, events: &[Event]) -> Self {
        let mut new = self;
        for event in events {
            new = new.queue_event(event.clone());
        }
        new
    }

    /// The simplest resolution function - resolves the events on the queue in the order
    /// in which they were added.
    fn resolve_in_order<G: Fn() -> System>(self, init: G) -> Return {
        self.resolve_with(init, |iter| iter)
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_fallible_engine, new_knowledge_engine, KnowledgeEngine};

    #[derive(Debug, Default, PartialEq, Eq)]
    struct SystemUnderTest {
        a: String,
        b: String,
        c: String,
    }

    #[derive(Clone, Debug)]
    enum SyntheticEvent {
        SetA(&'static str),
        SetB(&'static str),
        SetC(&'static str),
    }

    fn resolve(event: SyntheticEvent, system: SystemUnderTest) -> SystemUnderTest {
        use SyntheticEvent::*;
        let mut system = system;
        match event {
            SetA(s) => system.a = s.into(),
            SetB(s) => system.b = s.into(),
            SetC(s) => system.c = s.into(),
        };
        system
    }

    fn fallible_resolve(
        event: SyntheticEvent,
        system: SystemUnderTest,
    ) -> Result<SystemUnderTest, String> {
        use SyntheticEvent::*;
        let mut system = system;
        match event {
            SetA(s) => system.a = s.into(),
            SetB(s) => system.b = s.into(),
            SetC(s) => {
                return Err(format!("Could not set C to {}.", s));
            }
        };
        Ok(system)
    }

    #[test]
    fn knowledge_engine_example() {
        use SyntheticEvent::*;
        let system = new_knowledge_engine::<SystemUnderTest, SyntheticEvent, _>(resolve)
            .queue_events(&[
                SetA("first a value"),
                SetB("first b value"),
                SetA("second a value"),
            ])
            .resolve_in_order(SystemUnderTest::default);
        assert_eq!(system.a, "second a value".to_string());
        assert_eq!(system.b, "first b value".to_string());
    }

    #[test]
    fn inorder_vs_prologue_epilogue() {
        use SyntheticEvent::*;
        let system_using_events = new_knowledge_engine(resolve)
            .queue_event(SetA("first a value"))
            .queue_event(SetB("first b value"))
            .queue_event(SetA("second a value"))
            .resolve_in_order(SystemUnderTest::default);
        let system_using_prologue = new_knowledge_engine(resolve)
            .queue_prologue(SetA("first a value"))
            .queue_prologue(SetB("first b value"))
            .queue_prologue(SetA("second a value"))
            .resolve_in_order(SystemUnderTest::default);
        let system_using_epilogue = new_knowledge_engine(resolve)
            .queue_epilogue(SetA("first a value"))
            .queue_epilogue(SetB("first b value"))
            .queue_epilogue(SetA("second a value"))
            .resolve_in_order(SystemUnderTest::default);
        assert_eq!(system_using_events.a, "second a value".to_string());
        assert_eq!(system_using_events.b, "first b value".to_string());
        assert_eq!(system_using_events, system_using_prologue);
        assert_eq!(system_using_events, system_using_epilogue);
    }

    #[test]
    fn fallible_engine_example() {
        use SyntheticEvent::*;
        new_fallible_engine(fallible_resolve)
            .queue_events(&[
                SetA("first a value"),
                SetB("first b value"),
                SetC("this will error"),
                SetA("second a value"),
            ])
            .resolve_in_order(SystemUnderTest::default)
            .unwrap_err();
    }
}
