#[cfg(target_os = "android")]
use android_log_sys::{__android_log_write, LogPriority};
use tracing::{
    span::{Attributes, Record}, 
    field::{Field, Visit},
    Event, Id, Level, Metadata, Subscriber, 
};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::CString,
    fmt,
    sync::{RwLock, atomic::{AtomicU64, Ordering::Relaxed}},
    thread::LocalKey,
};

pub struct AndroidWriter;

impl AndroidWriter {
    #[cfg(target_os = "android")]
    pub fn log(path: String, content: String, level: &Level) {
        let prio = match *level {
            Level::ERROR => LogPriority::ERROR,
            Level::WARN => LogPriority::WARN,
            Level::INFO => LogPriority::INFO,
            Level::DEBUG => LogPriority::DEBUG,
            Level::TRACE => LogPriority::VERBOSE,
        };
        let tag = CString::new(path).unwrap();
        let text = CString::new(content).unwrap();

        let ret = unsafe { __android_log_write(prio as _, tag.as_ptr(), text.as_ptr()) };
    }

    #[cfg(target_os = "linux")]
    pub fn log(path: String, content: String, level: &Level) {
        println!("path = {}, content = {}, level = {}", path, content, level);
    }
}

#[derive(Clone)]
pub(crate) struct CurrentSpanPerThread {
    current: &'static LocalKey<RefCell<Vec<Id>>>,
}

impl CurrentSpanPerThread {
    pub fn new() -> Self  {
        thread_local! {
            static CURRENT: RefCell<Vec<Id>> = RefCell::new(Vec::new());
        };
        Self { current: &CURRENT }
    }

    pub fn id(&self) -> Option<Id> {
        self.current
            .with(|current| current.borrow().last().cloned())
    }

    pub fn enter(&self, span: Id) {
        self.current.with(|current| {
            current.borrow_mut().push(span);
        })
    }

    pub fn exit(&self) {
        self.current.with(|current| {
            let _ = current.borrow_mut().pop();
        })
    }
}

pub struct AndroidSubscriber {
    base_path: String,
    current: CurrentSpanPerThread,
    spans: RwLock<HashMap<Id, Span>>,
    ids: AtomicU64,
}

impl AndroidSubscriber {
    pub fn new<S: Into<String>>(base_path: S) -> Self {
        let s = Self {
            base_path: base_path.into(),
            current: CurrentSpanPerThread::new(),
            spans: RwLock::new(HashMap::new()),
            ids: AtomicU64::new(1),
        };
        AndroidWriter::log(
            s.base_path.clone(), 
            "Logger started, listening to the world go by".into(), 
            &Level::INFO
        );
        s
    }
}

impl Subscriber for AndroidSubscriber {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, span: &Attributes<'_>) -> Id {
        let next = self.ids.fetch_add(1, Relaxed);
        let id = Id::from_u64(next);
        let span = Span::new(self.current.id(), span);
        self.spans.write().unwrap().insert(id.clone(), span);
        id
    }

    fn record(&self, span: &Id, values: &Record<'_>) {
        let mut spans = self.spans.write().unwrap();
        if let Some(mut span) = spans.get_mut(span) {
            values.record(&mut span.fields);
        }
    }

    fn record_follows_from(&self, _s: &Id, _f: &Id) {
        // unimplemented
    }

    fn enter(&self, span_id: &Id) {
        self.current.enter(span_id.clone());
    }

    fn event(&self, event: &Event<'_>) {
        let spans = self.spans.read().unwrap();
        let mut path = self.base_path.clone();
        path.push_str(&self.current.current.with(|current| {
            let current = current.borrow(); 
            if current.len() == 0 { return String::new(); }
            let mut s = String::new();
            for id in current.iter() {
                s.push(':');
                s.push(':');
                if let Some(span) = spans.get(&id) {
                    s.push_str(&format!("{}", span));
                }
            }
            s
        }));
        std::mem::drop(spans);

        let mut fields = FieldVisitor::new();
        event.record(&mut fields);
        let content = format!("{}", fields);

        let level = event.metadata().level();

        AndroidWriter::log(path, content, level);
    }

    fn exit(&self, _: &Id) {
        self.current.exit();
    }

    fn try_close(&self, id: Id) -> bool {
        let mut spans = self.spans.write().unwrap();
        if let Some(span) = spans.get_mut(&id) {
            if span.drop() {
                spans.remove(&id);
                true
            } else {
                false
            }
        } else { true }
    }
}

pub(crate) struct FieldVisitor(Vec<(&'static str, String)>);

impl FieldVisitor {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
}

impl fmt::Display for FieldVisitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (k, v)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: [{}]", k, v)?;
        }
        write!(f, "}}")
    }
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.0.push((field.name(), format!("{:?}", value)));
    }
}

pub(crate) struct Span {
    parent: Option<Id>,
    name: &'static str,
    copies: AtomicU64,
    fields: FieldVisitor,
}

impl Span {
    pub(crate) fn new(parent: Option<Id>, attrs: &Attributes<'_>) -> Self {
        let mut span = Self {
            parent,
            name: attrs.metadata().name(),
            copies: AtomicU64::new(1),
            fields: FieldVisitor::new(),
        };
        attrs.record(&mut span.fields);
        span
    }

    pub(crate) fn clone(&self) {
        self.copies.fetch_add(1, Relaxed);
    }

    pub(crate) fn drop(&self) -> bool {
        self.copies.fetch_sub(1, Relaxed) == 0
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if self.fields.0.len() > 0 {
            write!(f, "{}", self.fields)?;
        }
        Ok(())
    }
}
