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
//!     .resolve_in_order(before);
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
/// # Types
/// - `System`: the type of the system under test.
/// - `Event`: the type of synthetic events.
/// - `Return`: the type returned by the `resolve` function. Can be the same as `S`, or
///     sometimes a `Result<S, _>`.
pub trait KnowledgeEngine<System, Event: Clone, Return>: Sized {
    fn queue_event(self, event: Event) -> Self;
    fn resolve_with<F: FnOnce(&mut dyn Iterator<Item = Event>) -> &mut dyn Iterator<Item = Event>>(
        self,
        system: System,
        comb: F,
    ) -> Return;

    fn queue_events(self, events: &[Event]) -> Self {
        let mut new = self;
        for event in events {
            new = new.queue_event(event.clone());
        }
        new
    }
    fn resolve_in_order(self, system: System) -> Return {
        self.resolve_with(system, |iter| iter)
    }
}

/// Create a new KnowledgeEngine implementation with the given resolver function.
/// This function should translate synthetic (test) events into actual changes in the
/// state of the system under test.
pub fn new_knowledge_engine<System, Event, F>(
    resolve: F,
) -> impl KnowledgeEngine<System, Event, System>
where
    Event: Clone,
    F: Fn(Event, System) -> System + 'static,
{
    KnowledgeEngineImpl {
        events: VecDeque::new(),
        resolve: Box::new(resolve),
    }
}

/// Create a new KnowledgeEngine implementation with the given fallible resolver function.
/// This function should translate synthetic (test) events into actual changes in the
/// state of the system under test.
///
/// If the resolver function ever returns an Err variant, the engine will cease and return
/// that Err.
pub fn new_fallible_engine<System, Event, Error, F>(
    resolve: F,
) -> impl KnowledgeEngine<System, Event, Result<System, Error>>
where
    Event: Clone,
    F: Fn(Event, System) -> Result<System, Error> + 'static,
{
    FallibleEngineImpl {
        events: VecDeque::new(),
        resolve: Box::new(resolve),
    }
}

struct KnowledgeEngineImpl<System, Event> {
    events: VecDeque<Event>,
    resolve: Box<dyn Fn(Event, System) -> System>,
}

struct FallibleEngineImpl<System, Event, Error> {
    events: VecDeque<Event>,
    resolve: Box<dyn Fn(Event, System) -> Result<System, Error>>,
}

impl<System, Event: Clone> KnowledgeEngine<System, Event, System>
    for KnowledgeEngineImpl<System, Event>
{
    fn queue_event(self, event: Event) -> Self {
        let mut new = self;
        new.events.push_back(event);
        new
    }
    fn resolve_with<
        F: FnOnce(&mut dyn Iterator<Item = Event>) -> &mut dyn Iterator<Item = Event>,
    >(
        self,
        system: System,
        comb: F,
    ) -> System {
        let mut system = system;
        let mut events_iter = self.events.into_iter();
        for event in comb(&mut events_iter) {
            system = (self.resolve)(event, system);
        }
        system
    }
}

impl<System, Event: Clone, Error> KnowledgeEngine<System, Event, Result<System, Error>>
    for FallibleEngineImpl<System, Event, Error>
{
    fn queue_event(self, event: Event) -> Self {
        let mut new = self;
        new.events.push_back(event);
        new
    }
    fn resolve_with<
        F: FnOnce(&mut dyn Iterator<Item = Event>) -> &mut dyn Iterator<Item = Event>,
    >(
        self,
        system: System,
        comb: F,
    ) -> Result<System, Error> {
        let mut system = system;
        let mut events_iter = self.events.into_iter();
        for event in comb(&mut events_iter) {
            system = (self.resolve)(event, system)?;
        }
        Ok(system)
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_knowledge_engine, new_fallible_engine, KnowledgeEngine};

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
            .resolve_in_order(SystemUnderTest::default());
        assert_eq!(system.a, "second a value".to_string());
        assert_eq!(system.b, "first b value".to_string());
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
            .resolve_in_order(SystemUnderTest::default())
            .unwrap_err();
    }
}
