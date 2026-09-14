use ratatui::{
    Frame,
    TerminalOptions,
    Viewport,
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::{
        Block, Borders, BorderType, List, ListItem, ListState
    },
    style::{
        Color, Style
    },
    text::{
        Line, Span
    },
};
use std::process;

struct App {
    state: ListState,
    results: Vec<(f64, String)>,
    running: bool,
}

impl App {
    pub fn new(results: Vec<(f64, String)>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            state: list_state,
            results,
            running: true,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> String {
        let mut selected: String = String::new();

        while self.running {
            terminal.draw(|frame| {
                self.render(frame)
            }).unwrap();
            selected = self.keybinds();
        }

        selected
    }

    fn render(&mut self, frame: &mut Frame) {
        let list = List::new(
            self.results
                .iter()
                .map(|(s, p)| {
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            s.to_string(),
                            Style::default().fg(Color::Green),
                        ),
                        Span::raw(" "),
                        Span::raw(p),
                    ]))
                })
                .collect::<Vec<ListItem>>(),
        )
        .block(
            Block::default()
                .title(" Select a directory ")
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Rgb(203, 166, 247)))
                .border_type(BorderType::Rounded),
        )
        .highlight_symbol("▌")
        .highlight_style(Style::new().fg(Color::Rgb(203, 166, 247)));

        frame.render_stateful_widget(list, frame.area(), &mut self.state);
    }

    fn keybinds(&mut self) -> String {
        match event::read().unwrap() {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => process::exit(-1),
                    KeyCode::Up => {
                        self.state.select_previous();
                        return "".to_string();
                    },
                    KeyCode::Down => {
                        self.state.select_next();
                        return "".to_string();
                    },
                    KeyCode::Enter => {
                        if let Some(i) = self.state.selected()
                            && let Some((_, selection)) = self.results.get(i)
                        {
                            self.running = false;
                            return selection.to_string();
                        } else {
                            return "".to_string();
                        }
                    },
                    _ => return "".to_string(),
                }
            }
            _ => return "".to_string(),
        }
    }
}

pub fn tui(results: Vec<(f64, String)>) -> String {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    });
    let final_result = App::new(results).run(&mut terminal);

    ratatui::restore();

    final_result
}
