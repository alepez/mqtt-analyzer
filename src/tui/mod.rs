use std::collections::BTreeMap;
use std::io::{self};
use std::thread;

use circular_queue::CircularQueue;
use rumqtt::Notification;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::{Frame, Terminal};

use navigation::{BlockId, Navigation};
use retain::draw_retain_tab;
use stream::draw_stream_tab;
use style::get_color;
use subscriptions::{
    draw_subscriptions_tab, handle_input_on_subscribe_input, handle_input_on_subscriptions_list,
    handle_input_on_subscriptions_list_item,
};
use tabs::TabsState;
use utils::{Event, Events};

use super::cli::Mode;
use super::engine::Engine;
use super::format::MessageFormat;

mod navigation;
mod notification_list;
mod retain;
mod stream;
mod style;
mod subscriptions;
mod tabs;
mod utils;

type RetainedMessages = BTreeMap<String, Notification>;

pub struct App {
    engine: Engine,
    tabs: TabsState,
    subscribe_input: String,
    notifications: CircularQueue<Notification>,
    retained_messages: RetainedMessages,
    navigation: Navigation,
}

impl App {
    fn new(engine: Engine) -> App {
        App {
            engine,
            tabs: TabsState::default(),
            subscribe_input: String::new(),
            notifications: CircularQueue::with_capacity(100),
            retained_messages: RetainedMessages::default(),
            navigation: Navigation::default(),
        }
    }
}

fn draw_tab_nav<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    use BlockId::*;

    let highlight_state = (
        app.navigation.peek() == TabNav,
        app.navigation.peek() == TabNav,
    );

    let style = get_color(highlight_state);

    Tabs::default()
        .block(Block::default().borders(Borders::ALL).border_style(style))
        .titles(TabsState::TITLES)
        .select(app.tabs.index)
        .style(style)
        .highlight_style(Style::default().fg(Color::Yellow))
        .render(f, area);
}

fn handle_input_on_tabs(c: termion::event::Key, app: &mut App) {
    use BlockId::*;
    use Key::*;

    match c {
        Right => app.tabs.next(),
        Left => app.tabs.previous(),
        Down | Key::Char('j') => {
            app.navigation.modify_top(SubscribeInput);
        }
        _ => (),
    }
}

fn handle_input(input: termion::event::Key, app: &mut App) {
    use BlockId::*;
    use Key::*;

    let nav = &mut app.navigation;

    match input {
        Esc => {
            nav.pop();

            if nav.parent() == Root {
                nav.push(SubscriptionsWindow);
                nav.push(TabNav);
            }
        }
        c => match nav.peek() {
            Root => {
                nav.push(SubscriptionsWindow);
                nav.push(TabNav);
            }
            TabNav => handle_input_on_tabs(c, app),
            SubscribeInput => handle_input_on_subscribe_input(c, app),
            SubscriptionsList => handle_input_on_subscriptions_list(c, app),
            SubscriptionsListItem(index) => handle_input_on_subscriptions_list_item(c, app, index),
            _ => (),
        },
    }
}

pub fn start_tui(
    engine: Engine,
    format_options: MessageFormat,
    mode: Mode,
) -> Result<(), failure::Error> {
    use Event::*;
    use Key::*;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let events = Events::new();

    let tx = events.tx();

    let notifications = engine.notifications.clone();
    let mut app = App::new(engine);

    app.tabs.index = mode as usize;

    thread::spawn(move || {
        for notification in notifications {
            tx.send(Event::MqttNotification(notification)).unwrap();
        }
    });

    loop {
        terminal.draw(|mut f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            draw_tab_nav(&mut f, chunks[0], &app);

            match app.tabs.index {
                0 => draw_subscriptions_tab(&mut f, chunks[1], &app),
                1 => draw_stream_tab(&mut f, chunks[1], &app, format_options),
                2 => draw_retain_tab(&mut f, chunks[1], &app, format_options),
                _ => {}
            }
        })?;

        match events.next()? {
            Input(Ctrl('c')) => {
                break;
            }
            Input(input) => handle_input(input, &mut app),
            MqttNotification(notification) => {
                if let Notification::Publish(msg) = notification {
                    let msg = msg.clone();
                    app.retained_messages
                        .insert(msg.topic_name.clone(), Notification::Publish(msg.clone()));
                    app.notifications.push(Notification::Publish(msg));
                } else {
                    app.notifications.push(notification);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
