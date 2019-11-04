use std::collections::HashMap;
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

use utils::{Event, Events};

use crate::engine::Engine;
use crate::format::MessageFormat;

use self::retain::draw_retain_tab;
use self::stream::draw_stream_tab;
use self::style::get_color;
use self::subscriptions::{
    draw_subscriptions_tab, handle_input_on_subscribe_input, handle_input_on_subscriptions_list,
    handle_input_on_subscriptions_list_item,
};
use self::tabs::TabsState;
use crate::cli::Mode;

mod notification_list;
mod retain;
mod stream;
mod style;
mod subscriptions;
mod tabs;
mod utils;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockId {
    Root,
    SubscriptionsWindow,
    TabNav,
    SubscribeInput,
    SubscriptionsList,
    SubscriptionsListItem(usize),
}

struct Navigation(Vec<BlockId>);

impl Navigation {
    fn default() -> Navigation {
        let mut nav = Vec::new();
        nav.push(BlockId::Root);
        nav.push(BlockId::SubscriptionsWindow);
        nav.push(BlockId::TabNav);
        Navigation(nav)
    }

    fn push(&mut self, block_id: BlockId) {
        self.0.push(block_id);
    }

    fn peek(&self) -> BlockId {
        self.0.last().cloned().unwrap_or(BlockId::Root)
    }

    fn parent(&self) -> BlockId {
        if self.0.len() < 2 {
            BlockId::Root
        } else {
            self.0
                .get(self.0.len() - 2)
                .cloned()
                .unwrap_or(BlockId::Root)
        }
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn modify_top(&mut self, new_value: BlockId) {
        *self.0.last_mut().unwrap() = new_value
    }
}

type RetainedMessages = HashMap<String, Notification>;

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
        let tabs = vec!["Subscriptions", "Stream", "Retain", "Statistics"]
            .iter()
            .map(|&s| String::from(s))
            .collect();

        App {
            engine,
            tabs: TabsState::new(tabs),
            subscribe_input: String::new(),
            notifications: CircularQueue::with_capacity(100),
            retained_messages: RetainedMessages::default(),
            navigation: Navigation::default(),
        }
    }
}

pub fn draw_empty_tab<B>(
    f: &mut tui::Frame<B>,
    area: tui::layout::Rect,
    _app: &App,
    _format: MessageFormat,
) where
    B: tui::backend::Backend,
{
    Block::default().borders(Borders::ALL).render(f, area)
}

fn draw_tab_nav<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let highlight_state = (
        app.navigation.peek() == BlockId::TabNav,
        app.navigation.peek() == BlockId::TabNav,
    );

    let style = get_color(highlight_state);

    Tabs::default()
        .block(Block::default().borders(Borders::ALL).border_style(style))
        .titles(&app.tabs.titles)
        .select(app.tabs.index)
        .style(style)
        .highlight_style(Style::default().fg(Color::Yellow))
        .render(f, area);
}

fn handle_input_on_tabs(c: termion::event::Key, app: &mut App) {
    match c {
        Key::Right => app.tabs.next(),
        Key::Left => app.tabs.previous(),
        Key::Down | Key::Char('j') => {
            app.navigation.modify_top(BlockId::SubscribeInput);
        }
        _ => (),
    }
}

fn handle_input(input: termion::event::Key, app: &mut App) {
    let nav = &mut app.navigation;

    match input {
        Key::Esc => {
            nav.pop();

            if nav.parent() == BlockId::Root {
                nav.push(BlockId::SubscriptionsWindow);
                nav.push(BlockId::TabNav);
            }
        }
        c => match nav.peek() {
            BlockId::Root => {
                nav.push(BlockId::SubscriptionsWindow);
                nav.push(BlockId::TabNav);
            }
            BlockId::TabNav => handle_input_on_tabs(c, app),
            BlockId::SubscribeInput => handle_input_on_subscribe_input(c, app),
            BlockId::SubscriptionsList => handle_input_on_subscriptions_list(c, app),
            BlockId::SubscriptionsListItem(index) => {
                handle_input_on_subscriptions_list_item(c, app, index)
            }
            _ => (),
        },
    }
}

pub fn start_tui(
    engine: Engine,
    format_options: MessageFormat,
    mode: Mode,
) -> Result<(), failure::Error> {
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
                3 => draw_empty_tab(&mut f, chunks[1], &app, format_options),
                _ => {}
            }
        })?;

        match events.next()? {
            Event::Input(Key::Ctrl('c')) => {
                break;
            }
            Event::Input(input) => handle_input(input, &mut app),
            Event::MqttNotification(notification) => {
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
