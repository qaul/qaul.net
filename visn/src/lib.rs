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
//! #[derive(Debug)]
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
//! let before = SystemUnderTest {
//!     a: "initial value".into(),
//!     b: "initial value".into()
//! };
//! let after = new_knowledge_engine::<SystemUnderTest, SyntheticEvent, _>(resolve)
//!     .queue_events(&[SetA("a1"), SetB("b1"), SetA("a2")])
//!     .resolve_all(before);
//!
//! assert_eq!(after.a, "a2".to_string());
//! assert_eq!(after.b, "b1".to_string());
//! ```
use std::collections::VecDeque;

/// The KnowledgeEngine provides a framework for testing the consequences of messages
/// in an eventually consistent system arriving in various orders.
///
/// Synthetic events are queued up and transformed by a given function into
/// actual changes to the state of the system under test. Once this function is defined,
/// each test needs only define a starting state, a list of events to execute, and
/// an expected ending condition.
///

pub trait KnowledgeEngine<S, E> {
    fn queue_event(self, event: E) -> Self;
    fn queue_events(self, events: &[E]) -> Self;
    fn resolve_all(self, system: S) -> S;
}

/// Create a new KnowledgeEngine implementation with the given resolver function.
/// This function should translate synthetic (test) events into actual changes in the
/// state of the system under test.
pub fn new_knowledge_engine<S, E, R>(resolve: R) -> impl KnowledgeEngine<S, E>
where
    E: Clone,
    R: Fn(E, S) -> S + 'static,
{
    KnowledgeEngineImpl {
        events: VecDeque::new(),
        resolve: Box::new(resolve),
    }
}

struct KnowledgeEngineImpl<E, S> {
    events: VecDeque<E>,
    resolve: Box<dyn Fn(E, S) -> S>,
}

impl<S, E: Clone> KnowledgeEngine<S, E> for KnowledgeEngineImpl<E, S> {
    fn queue_event(self, event: E) -> Self {
        let mut new = self;
        new.events.push_back(event);
        new
    }
    fn queue_events(self, events: &[E]) -> Self {
        let mut new = self;
        let events = Vec::from(events);
        for event in events.into_iter() {
            new = new.queue_event(event);
        }
        new
    }
    fn resolve_all(self, system: S) -> S {
        let mut new = self;
        let mut system = system;
        while let Some(event) = new.events.pop_front() {
            system = (new.resolve)(event, system);
        }
        system
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_knowledge_engine, KnowledgeEngine};

    #[derive(Debug, Default)]
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
        println!("{:?}", event);
        match event {
            SetA(s) => system.a = s.into(),
            SetB(s) => system.b = s.into(),
            SetC(s) => system.c = s.into(),
        };
        system
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
            .resolve_all(SystemUnderTest::default());
        assert_eq!(system.a, "second a value".to_string());
        assert_eq!(system.b, "first b value".to_string());
    }
}
