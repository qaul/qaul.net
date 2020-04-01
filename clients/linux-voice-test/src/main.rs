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
    libqaul::Qaul,
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

    let user = qaul.users().create("test").await.unwrap();

    let mut stream = futures::stream::select(
        Events::new().map(|e| Some(e)),
        interval(Duration::from_millis(250)).map(|_| None),
    );
    while let Some(e) = stream.next().await {
        if let Some(e) = e {
            match e {
                Event::Key(Key::Char('q')) => { break; },
                Event::Key(Key::Ctrl('c')) => { break; },
                Event::Key(Key::Esc) => { break; },
                _ => {},
            }
        } else {
            let (width, height) = terminal_size().unwrap();
            let mut s = format!(" User ID: {}", user.0);
            while (s.len() as u16) < width {
                s.push(' ');
            }
            write!(stdout, "{}{}{}{}", cursor::Goto(1, 1), style::Invert, s, style::Reset);

            for other_user in qaul.users().list().into_iter() {
                if other_user.id == user.0 {
                    continue;
                }
                write!(stdout, " {}{}", other_user.id, clear::UntilNewline);
            }

            write!(stdout, "{}", clear::AfterCursor);
            stdout.flush();
        }
    }
}

fn main() {
    block_on(run())
}
