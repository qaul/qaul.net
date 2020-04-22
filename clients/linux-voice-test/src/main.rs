use {
    async_std::{
        stream::interval,
        sync::{Arc, Mutex},
        task,
    },
    futures::stream::StreamExt,
    libqaul::{
        api::TagSet,
        messages::{Mode, MsgRef},
        Qaul,
    },
    linux_voice_test::event::Events,
    netmod_udp::Endpoint,
    ratman::Router,
    std::{
        env::args,
        io::{stdout, Write},
        ops::DerefMut,
        time::Duration,
    },
    termion::{
        clear, cursor,
        event::{Event, Key},
        raw::IntoRawMode,
        screen::AlternateScreen,
        style, terminal_size,
    },
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
    let endpoint = Endpoint::spawn(args[1].parse::<u16>().unwrap());

    for i in 2..args.len() {
        endpoint.introduce(&args[i]).await.unwrap();
    }

    let router = Router::new();
    router.add_endpoint(endpoint).await;

    let dir = tempfile::tempdir().unwrap();
    let qaul = Qaul::new(router, dir.path());
    qaul.services().register("HELLO").unwrap();

    let user = qaul.users().create("test").await.unwrap();

    let state = Arc::new(Mutex::new(State::UserSelect(0)));

    let _user = user.clone();
    let _qaul = qaul.clone();
    let _state = state.clone();
    task::spawn(async {
        let user = _user;
        let state = _state;
        let qaul = _qaul;

        while let Ok(m) = qaul.messages().next(user.clone(), "HELLO", None).await {
            *(state.lock().await.deref_mut()) = State::MessageDisplay(m);
        }
    });

    let mut stream = futures::stream::select(
        Events::new().map(|e| Some(e)),
        interval(Duration::from_millis(250)).map(|_| None),
    );
    while let Some(e) = stream.next().await {
        let mut state = state.lock().await;
        let state = state.deref_mut();
        // keyboard input
        if let Some(e) = e {
            match e {
                Event::Key(Key::Char('q')) => {
                    break;
                }
                Event::Key(Key::Ctrl('c')) => {
                    break;
                }
                Event::Key(Key::Esc) => {
                    break;
                }
                Event::Key(Key::Up) => match state {
                    State::UserSelect(ref mut index) => {
                        *index = index.saturating_sub(1);
                    }
                    _ => {}
                },
                Event::Key(Key::Down) => match state {
                    State::UserSelect(ref mut index) => {
                        *index += 1;
                    }
                    _ => {}
                },
                Event::Key(Key::Char('\n')) => {
                    let next_state = match state {
                        State::UserSelect(index) => {
                            let dest = qaul
                                .users()
                                .list()
                                .await
                                .into_iter()
                                .filter(|u| u.id != user.0)
                                .nth(*index)
                                .unwrap();
                            qaul.messages()
                                .send(
                                    user.clone(),
                                    Mode::Std(dest.id),
                                    "HELLO",
                                    TagSet::empty(),
                                    Vec::new(),
                                )
                                .await
                                .unwrap();
                            State::UserSelect(*index)
                        }
                        State::MessageDisplay(_) => State::UserSelect(0),
                    };
                    *state = next_state;
                }
                _ => {}
            }
        }

        let (width, _) = terminal_size().unwrap();
        match state {
            State::UserSelect(ref mut index) => {
                let mut s = format!(" User ID: {}", user.0);
                while (s.len() as u16) < width {
                    s.push(' ');
                }
                writeln!(
                    stdout,
                    "{}{}{}{}",
                    cursor::Goto(1, 1),
                    style::Invert,
                    s,
                    style::Reset
                )
                .unwrap();

                let user_count = qaul
                    .users()
                    .list()
                    .await
                    .into_iter()
                    .filter(|u| u.id != user.0)
                    .enumerate()
                    .map(|(i, user)| {
                        if i == *index {
                            write!(stdout, "{}", style::Underline).unwrap();
                        }
                        writeln!(
                            stdout,
                            "{} {}{}",
                            cursor::Goto(1, 2 + i as u16),
                            user.id,
                            clear::UntilNewline
                        )
                        .unwrap();
                        if i == *index {
                            write!(stdout, "{}", style::Reset).unwrap();
                        }
                    })
                    .count();
                *index = (*index).min(user_count.saturating_sub(1));

                write!(
                    stdout,
                    "{}{}",
                    cursor::Goto(1, 2 + user_count as u16),
                    clear::AfterCursor
                )
                .unwrap();
            }
            State::MessageDisplay(m) => {
                writeln!(
                    stdout,
                    "{} Message from {}{}",
                    cursor::Goto(1, 1),
                    m.sender,
                    clear::AfterCursor,
                )
                .unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}

fn main() {
    use {
        std::fs::File,
        tracing_subscriber::{fmt, EnvFilter},
    };

    let logfile = File::create("/tmp/qaul.log").unwrap();
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false)
        .with_writer(move || logfile.try_clone().unwrap())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    task::block_on(run())
}
