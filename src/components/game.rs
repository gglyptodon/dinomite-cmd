use super::{Component, lib::dinomite::Dinomite};
use crate::components::lib::dinomite;
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use std::time::Instant;
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Game {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    gamestate: GameState,
    cells: Vec<Vec<char>>,
    dinomite: Dinomite,
    width: usize,
    height: usize,
    num_dinos: usize,
}
impl Game {
    pub fn new(width: usize, height: usize, num_dinos: usize) -> Self {
        Self {
            command_tx: Default::default(),
            config: Default::default(),
            gamestate: Default::default(),
            cells: Default::default(),
            width,
            height,
            num_dinos,
            dinomite: Dinomite::new(width, height, num_dinos),
        }
    }
}
#[derive(Debug)]
pub struct GameState {
    pub curpos: dinomite::Position,
    pub game_start: Option<Instant>,
    pub elapsed_seconds: u64,
    pub is_game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        let default_pos = dinomite::Position(0, 0);
        GameState {
            curpos: default_pos,
            game_start: None,
            elapsed_seconds: 0,
            is_game_over: false,
        }
    }
}
impl GameState {
    pub fn start_game(&mut self) {
        if self.game_start.is_none() {
            self.game_start = Some(Instant::now());
        }
    }
    pub fn update_timer(&mut self) {
        if let Some(start_time) = self.game_start {
            if !self.is_game_over {
                self.elapsed_seconds = start_time.elapsed().as_secs();
            }
        }
    }
    fn reset(&mut self) {
        self.game_start = None;
        self.elapsed_seconds = 0;
        self.is_game_over = false;
    }
}

impl Game {
    fn align(c: char) -> String {
        let res = match c {
            '1' => "１",
            '2' => "２",
            '3' => "３",
            '4' => "４",
            '5' => "５",
            '6' => "６",
            '7' => "７",
            '8' => "８",
            _ => &format!("{}", c),
        };
        res.to_string()
    }
    fn create_styled_row(&mut self, row_idx: usize) -> Line<'static> {
        let spans: Vec<Span> = self.cells[row_idx]
            .iter()
            .enumerate()
            .map(|(col_idx, &ch)| {
                if !self.dinomite.is_game_over()
                    && self.gamestate.curpos.0 == col_idx
                    && self.gamestate.curpos.1 == row_idx
                {
                    Span::styled(
                        format!("{:*^3}", Self::align(ch)),
                        Style::default()
                            .fg(Color::Red)
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::UNDERLINED),
                    )
                } else {
                    Span::raw(format!("{: ^3}", Self::align(ch)))
                }
            })
            .collect();

        Line::from(spans)
    }
}

impl Component for Game {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            Action::Flag => {
                let pos = self.gamestate.curpos.clone(); //dinomite::Position(self.gamestate.curp, self.gamestate.cur_y);
                if self.gamestate.game_start.is_some() {
                    self.dinomite.toggle_flag(&pos); //todo something is not right w/ positions...
                }
            }
            Action::Look => {
                let pos = self.gamestate.curpos.clone(); //dinomite::Position(self.gamestate.cur_y, self.gamestate.cur_x);
                if self.gamestate.game_start.is_some() {
                    self.dinomite.check_position(&pos);
                }
                //println!("{:?} -  {:?}", self.dinomite.is_game_over(), &pos)
            }
            Action::MoveDown => {
                if self.gamestate.curpos.1 <= (self.dinomite.height - 2) {
                    self.gamestate.curpos.1 += 1;
                }
            }
            Action::MoveUp => {
                if self.gamestate.curpos.1 >= 1 {
                    self.gamestate.curpos.1 -= 1;
                }
            }
            Action::MoveLeft => {
                if self.gamestate.curpos.0 >= 1 {
                    self.gamestate.curpos.0 -= 1;
                }
            }
            Action::MoveRight => {
                if self.gamestate.curpos.0 <= (self.dinomite.width - 2) {
                    self.gamestate.curpos.0 += 1;
                }
            }
            Action::StartGame => {
                self.gamestate.start_game();
            }
            Action::RestartGame => {
                if self.dinomite.is_game_over() {
                    // allow restart
                    self.gamestate.reset();
                    self.dinomite = Dinomite::new(self.width, self.height, self.num_dinos);
                }
            }

            _ => {}
        }
        self.gamestate.update_timer();
        if self.dinomite.is_game_over() {
            self.gamestate.is_game_over = true;
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let info_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("dinomite-cmd")
            .bold();

        // Calculate inner area for content
        let inner_area = block.inner(layout[1]);

        // Render the block
        frame.render_widget(block, layout[1]);

        // Convert grid to displayable string with spaces between characters
        self.cells = self
            .dinomite
            .to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Create styled lines for each row
        let mut text: Vec<Line> = (0..self.cells.len())
            .map(|row_idx| self.create_styled_row(row_idx))
            .collect();
        // add won/lost message to bottom
        if self.dinomite.is_won() {
            //let won_message = won_message();
            text.push(won_message().into());
        }
        if !self.dinomite.is_won() && self.dinomite.is_game_over() {
            text.push(lost_message().into());
        }

        //
        let mut timer_text = if self.gamestate.game_start.is_some() {
            format!("Time: {}s", self.gamestate.elapsed_seconds)
        } else {
            "Controls:\nstart: 's'\nquit: 'q'\nflag: <space>\nuncover: <enter>".to_string()
        };

        if self.dinomite.is_game_over() && self.dinomite.is_won() {
            timer_text.push_str("\n\n😎 YOU WON!!! 😎");
        }
        if self.dinomite.is_game_over() && !self.dinomite.is_won() {
            timer_text.push_str("\n\n💀 GAME OVER 💀");
        }
        if self.dinomite.is_game_over() {
            timer_text.push_str("\n\nPress 'r' to reset\n\nPress 'q' to quit");
        }
        let timer = Paragraph::new(timer_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::bordered());
        frame.render_widget(timer, info_layout[0]);
        //

        let info_text = if self.gamestate.game_start.is_some() {
            [
                format!("🦖: {}", self.dinomite.get_num_dinos()),
                format!("Width: {}", self.dinomite.get_width()),
                format!("Height: {}", self.dinomite.get_height()),
            ]
            .join("\n")
        } else {
            [
                "Not started".to_string(),
                format!("🦖: {}", self.dinomite.get_num_dinos()),
                format!("Width: {}", self.dinomite.get_width()),
                format!("Height: {}", self.dinomite.get_height()),
            ]
            .join("\n")
        };
        let info = Paragraph::new(info_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::bordered());
        frame.render_widget(info, info_layout[1]);

        let paragraph = Paragraph::new(text).alignment(Alignment::Center);
        if self.gamestate.game_start.is_some() {
            frame.render_widget(paragraph, inner_area);
        }

        Ok(())
    }
}

fn won_message() -> Span<'static> {
    let won_message = Span::styled(
        format!("{: ^3}", "\n😎 YOU WON!!! 😎"),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::RAPID_BLINK),
    );
    won_message
}
fn lost_message() -> Span<'static> {
    let lost_message = Span::styled(
        format!("{: ^3}", "💀 GAME OVER 💀"),
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::SLOW_BLINK),
    );
    lost_message
}
