use {
    termion::{
        event::{Event, Key},
        cursor,
        clear,
        raw::IntoRawMode,
        screen::AlternateScreen,
        style,
        terminal_size,
    },
    futures::stream::StreamExt,
    linux_voice_test::event::Events,
    libqaul::{
        messages::{MsgRef, Mode},
        Qaul,
    },
    std::{
        time::Duration,
        env::args,
        io::{stdout, Write},
    },
    async_std::{
        stream::interval,
        task::block_on,
    },
    netmod_udp::Endpoint,
    ratman::{Router, Identity},
};

enum State {
    UserSelect(usize),
    MessageDisplay(MsgRef),
}

async fn run() {
    // set up our terminal
    let stdout = stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let mut stdout = cursor::HideCursor::from(stdout);

    let args = args().collect::<Vec<_>>();
    let endpoint = Endpoint::spawn(&args[1]);

    for i in 2..args.len() {
        endpoint.introduce(&args[i]).await;
    }

    let router = Router::new();
    router.add_endpoint(endpoint).await;
    let qaul = Qaul::new(router);
    qaul.services().register("HELLO").unwrap();

    let user = qaul.users().create("test").await.unwrap();

    let mut state = State::UserSelect(0);

    let mut stream = futures::stream::select(
        Events::new().map(|e| Some(e)),
        interval(Duration::from_millis(250)).map(|_| None),
    );
    while let Some(e) = stream.next().await {
        // keyboard input
        if let Some(e) = e {
            match e {
                Event::Key(Key::Char('q')) => { break; },
                Event::Key(Key::Ctrl('c')) => { break; },
                Event::Key(Key::Esc) => { break; },
                Event::Key(Key::Up) => { 
                    match state {
                        State::UserSelect(ref mut index) => {
                            *index = index.saturating_sub(1);
                        },
                        _ => {},
                    }
                },
                Event::Key(Key::Down) => {
                    match state {
                        State::UserSelect(ref mut index) => {
                            *index += 1;
                        },
                        _ => {},
                    }
                },
                Event::Key(Key::Char('\n')) => {
                    let next_state = match state {
                        State::UserSelect(index) => {
                            let dest = qaul
                                .users()
                                .list()
                                .into_iter()
                                .filter(|u| u.id != user.0)
                                .nth(index)
                                .unwrap();
                            qaul.messages().send(
                                user.clone(),
                                Mode::Std(dest.id),
                                "HELLO",
                                None,
                                Vec::new(),
                            ).await.unwrap();
                            State::UserSelect(index)
                        },
                        State::MessageDisplay(_) => State::UserSelect(0),
                    };
                    state = next_state;
                },
                _ => {},
            }
        }

        let (width, height) = terminal_size().unwrap();
        match state {
            State::UserSelect(ref mut index) => {
                let mut s = format!(" User ID: {}", user.0);
                while (s.len() as u16) < width {
                    s.push(' ');
                }
                writeln!(stdout, "{}{}{}{}", cursor::Goto(1, 1), style::Invert, s, style::Reset);

                let user_count = qaul
                    .users()
                    .list()
                    .into_iter()
                    .filter(|u| u.id != user.0)
                    .enumerate()
                    .map(|(i, user)| {;
                        if i == *index {
                            write!(stdout, "{}", style::Underline);
                        }
                        writeln!(
                            stdout, 
                            "{} {}{}", 
                            cursor::Goto(1, 2 + i as u16), 
                            user.id, 
                            clear::UntilNewline
                        );
                        if i == *index {
                            write!(stdout, "{}", style::Reset);
                        }
                    })
                    .count();
                *index = (*index).min(user_count.saturating_sub(1));

                write!(stdout, "{}{}", cursor::Goto(1, 2 + user_count as u16), clear::AfterCursor);
            },
            _ => {},
        }
        stdout.flush();
    }
}

fn main() {
    block_on(run())
}
