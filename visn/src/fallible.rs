use crate::KnowledgeEngine;

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
        prologue: Vec::new(),
        events: Vec::new(),
        epilogue: Vec::new(),
        resolve: Box::new(resolve),
    }
}

struct FallibleEngineImpl<System, Event, Error> {
    prologue: Vec<Event>,
    events: Vec<Event>,
    epilogue: Vec<Event>,
    resolve: Box<dyn Fn(Event, System) -> Result<System, Error>>,
}

impl<System, Event: Clone, Error> KnowledgeEngine<System, Event, Result<System, Error>>
    for FallibleEngineImpl<System, Event, Error>
{
    fn queue_event(self, event: Event) -> Self {
        let mut new = self;
        new.events.push(event);
        new
    }

    fn queue_prologue(self, event: Event) -> Self {
        let mut new = self;
        new.prologue.push(event);
        new
    }

    fn queue_epilogue(self, event: Event) -> Self {
        let mut new = self;
        new.epilogue.push(event);
        new
    }
    fn resolve_with<
        F: FnOnce(&mut dyn Iterator<Item = Event>) -> &mut dyn Iterator<Item = Event>,
        G: Fn() -> System,
    >(
        self,
        init: G,
        comb: F,
    ) -> Result<System, Error> {
        let mut system = init();
        for event in self.prologue.into_iter() {
            system = (self.resolve)(event, system)?;
        }

        let mut events_iter = self.events.into_iter();
        for event in comb(&mut events_iter) {
            system = (self.resolve)(event, system)?;
        }

        for event in self.epilogue.into_iter() {
            system = (self.resolve)(event, system)?;
        }
        Ok(system)
    }
}
