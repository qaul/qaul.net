#[cfg(target_os = "android")]
use android_log_sys::{__android_log_write, LogPriority};
use tracing::{
    span::{Attributes, Record}, 
    field::{Field, Visit},
    Event, Id, Level, Metadata, Subscriber, 
};
use serde_json::{map::Map, json, Value};
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
    log_fields: bool,
    current: CurrentSpanPerThread,
    spans: RwLock<HashMap<Id, Span>>,
    ids: AtomicU64,
}

impl AndroidSubscriber {
    pub fn new(log_fields: bool) -> Self {
        let s = Self {
            log_fields,
            current: CurrentSpanPerThread::new(),
            spans: RwLock::new(HashMap::new()),
            ids: AtomicU64::new(1),
        };
        AndroidWriter::log(
            "android-tracing".into(),
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
        let mut span_string = String::new();
        self.current.current.with(|current| {
            for (i, id) in current.borrow().iter().enumerate() {
                if i != 0 {
                    span_string.push(':');
                    span_string.push(':');
                }
                if let Some(span) = spans.get(id) {
                    span_string.push_str(&format!("{}", span.name));
                }
            }
        });

        let metadata = event.metadata();
        let file = metadata.file().unwrap_or_else(|| metadata.target());
        let line = metadata.line().map_or_else(|| "[Unknown]".to_string(), |line| line.to_string());

        let mut fields = FieldVisitor::new();
        event.record(&mut fields);

        let mut path = format!("{} ({}:{})", span_string, file, line);

        let mut message = if let Some(msg) = &fields.message {
            msg.clone()
        } else {
            String::new()
        };

        if self.log_fields {
            let obj = self
                .current
                .current
                .with(|current| {
                    current.borrow()
                        .iter()
                        .rev()
                        .fold(fields.to_object(), |prev, id| {
                            if let Some(span) = spans.get(id) {
                                span.to_object(prev)
                            } else {
                                json!({ "id": id.into_u64(), "child": prev })
                            }
                        })
                });
            let obj = serde_json::to_string(&obj).unwrap();

            message.push('\n');
            message.push_str(&obj);
        }

        AndroidWriter::log(path, message, metadata.level());
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

pub(crate) struct FieldVisitor {
    fields: Vec<(&'static str, String)>,
    message: Option<String>,
}

impl FieldVisitor {
    pub(crate) fn new() -> Self {
        Self {
            fields: Vec::new(),
            message: None,
        }
    }

    pub(crate) fn to_object(&self) -> Value {
        let mut map = Map::new();

        if let Some(msg) = &self.message {
            map.insert("message".to_string(), Value::String(msg.clone()));
        }

        for (key, val) in self.fields.iter() {
            map.insert(key.to_string(), Value::String(val.clone()));
        }

        Value::Object(map)
    }
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{:?}", value));
        } else {
            self.fields.push((field.name(), format!("{:?}", value)));
        }
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

    pub(crate) fn to_object(&self, prev: Value) -> Value {
        let mut obj = self.fields.to_object();
        let mut map = obj.as_object_mut().unwrap();
        map.insert("name".to_string(), self.name.into());
        map.insert("child".to_string(), prev);
        obj
    }
}
