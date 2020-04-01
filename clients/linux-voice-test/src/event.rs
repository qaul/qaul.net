///! An async stream that produces input events
// copied shamelessly from an old project of mine

use {
    async_std::io::{
        Read,
        stdin,
        Stdin,
    },
    futures::{
        stream::Stream,
    },
    termion::event::{
        Event,
        Key,
        parse_event,
    },
    std::{
        collections::VecDeque,
        io::Error,
        task::{Context, Poll},
        pin::Pin,
    },
};

/// Stream providing events from stdin
pub struct Events {
    input: Stdin,
    buffer: VecDeque<u8>,
    events: VecDeque<Event>,
}

impl Events {
    pub fn new() -> Self {
        Self {
            input: stdin(),
            buffer: VecDeque::new(),
            events: VecDeque::new(),
        }
    }
}

impl Stream for Events {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        // if we have ready events just serve one of those
        if let Some(e) = self.events.pop_front() {
            return Poll::Ready(Some(e));
        }

        // otherwise check if we've got waiting bytes
        let mut buffer = [0; 16];
        let n = match Read::poll_read(Pin::new(&mut self.input), cx, &mut buffer[..]) {
            Poll::Pending => { return Poll::Pending; },
            Poll::Ready(Ok(n)) => n,
            Poll::Ready(Err(_)) => { return Poll::Ready(None); },
        };

        // if the buffer is empty and we've read an escape and only an escape
        // assume that the input actually contains an escape instead of an escape
        // code
        // 
        // it'd be great if termion would deal with this on it's own but `parse_event`
        // literally can't output `Key::Esc` so...
        if self.buffer.len() == 0 && n == 1 && buffer[0] == 27 {
            self.events.push_front(Event::Key(Key::Esc));
            return Self::poll_next(self, cx);
        }

        // copy the read data into the buffer
        for b in &buffer[..n] {
            self.buffer.push_back(*b);
        }


        // check to see if there's any events in our newly aquired data
        let mut iter = PointerIter::new(&self.buffer);
        let mut events = Vec::new();
        while let Some(event) = iter.next().and_then(|b| parse_event(b.unwrap(), &mut iter).ok()) {
            iter.set_checkpoint();
            events.push(event);
        }

        // consume the bytes we used from the buffer
        for _ in 0..iter.checkpoint {
            self.buffer.pop_front();
        }

        // add the new events into the event queue
        self.events.extend(events.into_iter());

        // then recuse
        // if we did find events, this will immediately return them
        // if we didn't, we'll either read more data or set the reader to wake us
        Self::poll_next(self, cx)
    }
}

/// this struct does two things
/// - gives us an iterator without having to clone the buffer
/// - keeps track of what bytes we've used so we can take them off the buffer
///
/// it does the second by using a checkpoint system. when you've decided you'll
/// want to remove the bytes that were take out of the iterator set the checkpoint.
/// then you can reference the checkpoint when you drain the buffer, or reset the pointer
/// after the iterator moves forward.
struct PointerIter<'a> {
    pointer: usize,
    checkpoint: usize,
    data: &'a VecDeque<u8>,
}

impl<'a> PointerIter<'a> {
    pub fn new(data: &'a VecDeque<u8>) -> Self {
        Self {
            pointer: 0,
            checkpoint: 0,
            data,
        }
    }

    /// reset the pointer to the checkpoint
    pub fn reset_pointer(&mut self) {
        self.pointer = self.checkpoint;
    }

    /// set the checkpoint to the current pointer location
    pub fn set_checkpoint(&mut self) {
        self.checkpoint = self.pointer;
    }
}

impl<'a> Iterator for PointerIter<'a> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer == self.data.len() { 
            return None;
        }

        self.pointer += 1;
        Some(Ok(self.data[self.pointer - 1]))
    }
}
