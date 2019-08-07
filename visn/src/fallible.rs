use permute::permutations_of;
use crate::KnowledgeEngine;

/// Create a new KnowledgeEngine implementation with the given fallible resolver function.
/// This function should translate synthetic (test) events into actual changes in the
/// state of the system under test.
///
/// If the resolver function ever returns an Err variant, the engine will cease and return
/// that Err.
pub fn new_fallible_engine<'e, System, Event, Error, F>(
    resolve: F,
) -> impl KnowledgeEngine<'e, System, Event, Result<System, Error>>
where
    Event: Clone + 'e,
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

impl<System, Event: Clone, Error> FallibleEngineImpl<System, Event, Error> {
    fn resolve_onto_fallible(
        &self,
        mut system: System,
        events: impl Iterator<Item = Event>,
    ) -> Result<System, Error> {
        for event in events {
            system = (self.resolve)(event, system)?;
        }
        Ok(system)
    }
}

impl<'e, System, Event: Clone + 'e, Error> KnowledgeEngine<'e, System, Event, Result<System, Error>>
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

    fn resolve_all_orders<G: Fn() -> System>(self, init: G) -> Vec<Result<System, Error>> {
        let mut results = Vec::new();
        let permutations = permutations_of(&self.events);
        for events_iter in permutations {
            let mut system = init();
            let prologue_result = self.resolve_onto_fallible(system, self.prologue.iter().cloned());
            system = match prologue_result {
                Ok(s) => s,
                Err(e) => {
                    results.push(Err(e));
                    continue;
                }
            };
            let test_result = self.resolve_onto_fallible(system, events_iter.cloned());
            system = match test_result {
                Ok(s) => s,
                Err(e) => {
                    results.push(Err(e));
                    continue;
                }
            };
            let epilogue_result = self.resolve_onto_fallible(system, self.epilogue.iter().cloned());
            system = match epilogue_result {
                Ok(s) => s,
                Err(e) => {
                    results.push(Err(e));
                    continue;
                }
            };
            results.push(Ok(system));
        }
        results
    }
}
