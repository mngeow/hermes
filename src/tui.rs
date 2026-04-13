use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::models::{DiscoveredAgent, DiscoveredCommand, DiscoveredSkill};

const SKILLS_COLOR: Color = Color::Cyan;
const AGENTS_COLOR: Color = Color::Green;
const COMMANDS_COLOR: Color = Color::Magenta;
const HELP_COLOR: Color = Color::Yellow;
const BG_DARK: Color = Color::Rgb(18, 18, 24);
const BG_PANE: Color = Color::Rgb(28, 28, 36);
const TEXT_DIM: Color = Color::Rgb(140, 140, 160);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Skills,
    Agents,
    Commands,
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    pub focus: Focus,
    pub skills: Vec<SelectableSkill>,
    pub agents: Vec<SelectableAgent>,
    pub commands: Vec<SelectableCommand>,
    pub skill_list_state: ListState,
    pub agent_list_state: ListState,
    pub command_list_state: ListState,
    pub confirmed: bool,
    pub cancelled: bool,
}

#[derive(Debug, Clone)]
pub struct SelectableSkill {
    pub skill: DiscoveredSkill,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub struct SelectableAgent {
    pub agent: DiscoveredAgent,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub struct SelectableCommand {
    pub command: DiscoveredCommand,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub struct SelectionResult {
    pub selected_skills: Vec<String>,
    pub selected_agents: Vec<String>,
    pub selected_commands: Vec<String>,
}

impl SelectionState {
    pub fn new(
        skills: Vec<DiscoveredSkill>,
        agents: Vec<DiscoveredAgent>,
        commands: Vec<DiscoveredCommand>,
    ) -> Self {
        let mut skill_list_state = ListState::default();
        if !skills.is_empty() {
            skill_list_state.select(Some(0));
        }

        let mut agent_list_state = ListState::default();
        if !agents.is_empty() {
            agent_list_state.select(Some(0));
        }

        let mut command_list_state = ListState::default();
        if !commands.is_empty() {
            command_list_state.select(Some(0));
        }

        Self {
            focus: Focus::Skills,
            skills: skills
                .into_iter()
                .map(|skill| SelectableSkill {
                    skill,
                    selected: false,
                })
                .collect(),
            agents: agents
                .into_iter()
                .map(|agent| SelectableAgent {
                    agent,
                    selected: false,
                })
                .collect(),
            commands: commands
                .into_iter()
                .map(|command| SelectableCommand {
                    command,
                    selected: false,
                })
                .collect(),
            skill_list_state,
            agent_list_state,
            command_list_state,
            confirmed: false,
            cancelled: false,
        }
    }

    pub fn toggle_current(&mut self) {
        match self.focus {
            Focus::Skills => {
                if let Some(index) = self.skill_list_state.selected() {
                    if let Some(item) = self.skills.get_mut(index) {
                        item.selected = !item.selected;
                    }
                }
            }
            Focus::Agents => {
                if let Some(index) = self.agent_list_state.selected() {
                    if let Some(item) = self.agents.get_mut(index) {
                        item.selected = !item.selected;
                    }
                }
            }
            Focus::Commands => {
                if let Some(index) = self.command_list_state.selected() {
                    if let Some(item) = self.commands.get_mut(index) {
                        item.selected = !item.selected;
                    }
                }
            }
        }
    }

    pub fn next(&mut self) {
        match self.focus {
            Focus::Skills => move_selection(&mut self.skill_list_state, self.skills.len(), true),
            Focus::Agents => move_selection(&mut self.agent_list_state, self.agents.len(), true),
            Focus::Commands => {
                move_selection(&mut self.command_list_state, self.commands.len(), true)
            }
        }
    }

    pub fn previous(&mut self) {
        match self.focus {
            Focus::Skills => move_selection(&mut self.skill_list_state, self.skills.len(), false),
            Focus::Agents => move_selection(&mut self.agent_list_state, self.agents.len(), false),
            Focus::Commands => {
                move_selection(&mut self.command_list_state, self.commands.len(), false)
            }
        }
    }

    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Skills => Focus::Agents,
            Focus::Agents => Focus::Commands,
            Focus::Commands => Focus::Skills,
        };
    }

    pub fn confirm(&mut self) {
        self.confirmed = true;
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn is_done(&self) -> bool {
        self.confirmed || self.cancelled
    }

    pub fn result(&self) -> Option<SelectionResult> {
        if self.cancelled || !self.confirmed {
            return None;
        }

        let selected_skills = self
            .skills
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.skill.name.clone())
            .collect();

        let selected_agents = self
            .agents
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.agent.name.clone())
            .collect();

        let selected_commands = self
            .commands
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.command.name.clone())
            .collect();

        Some(SelectionResult {
            selected_skills,
            selected_agents,
            selected_commands,
        })
    }
}

pub fn run_interactive_selection(
    skills: Vec<DiscoveredSkill>,
    agents: Vec<DiscoveredAgent>,
    commands: Vec<DiscoveredCommand>,
) -> io::Result<Option<SelectionResult>> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut state = SelectionState::new(skills, agents, commands);

    let result = loop {
        terminal.draw(|frame| draw(frame, &mut state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => state.cancel(),
                    KeyCode::Char(' ') | KeyCode::Enter => state.toggle_current(),
                    KeyCode::Down | KeyCode::Char('j') => state.next(),
                    KeyCode::Up | KeyCode::Char('k') => state.previous(),
                    KeyCode::Tab => state.switch_focus(),
                    KeyCode::Char('c') => state.confirm(),
                    _ => {}
                }
            }
        }

        if state.is_done() {
            break state.result();
        }
    };

    ratatui::restore();
    Ok(result)
}

fn draw(frame: &mut Frame, state: &mut SelectionState) {
    let [banner_area, skills_area, agents_area, commands_area, help_area] = Layout::vertical([
        Constraint::Length(7),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    draw_banner(frame, state, banner_area);
    draw_skills_list(frame, state, skills_area);
    draw_agents_list(frame, state, agents_area);
    draw_commands_list(frame, state, commands_area);
    draw_help(frame, help_area);
}

fn draw_banner(frame: &mut Frame, state: &SelectionState, area: Rect) {
    let art = vec![
        Line::from(vec![
            Span::styled("    ╭─────────╮     ", Style::default().fg(Color::Yellow)),
            Span::styled(
                " _   _                      ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("    │ ◠   ◠ │     ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "| | | | ___ _ __ _ __ ___   ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("    │   ω   │     ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "| |_| |/ _ \\ '__| '_ ` _ \\  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("    ╰────┬────╯     ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "|  _  |  __/ |  | | | | | | ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("      ╱│╲        ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "|_| |_|\\___|_|  |_| |_| |_| ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("     ╱ │ ╲       ", Style::default().fg(Color::Yellow)),
            Span::raw("     "),
            Span::styled(
                "Hermes",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  —  "),
            Span::styled(
                "Install Navigator",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("    🦶   🦶      ", Style::default().fg(Color::Yellow)),
            Span::raw("     "),
            Span::styled(
                format!(
                    "{} skills  •  {} agents  •  {} commands",
                    count_selected_skills(state),
                    count_selected_agents(state),
                    count_selected_commands(state)
                ),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let banner = Paragraph::new(art)
        .alignment(Alignment::Center)
        .style(Style::default().bg(BG_DARK).fg(Color::White))
        .block(
            Block::default()
                .title(" Hermes ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightBlue))
                .style(Style::default().bg(BG_DARK).fg(Color::White)),
        );

    frame.render_widget(banner, area);
}

fn draw_skills_list(frame: &mut Frame, state: &mut SelectionState, area: Rect) {
    let items = state
        .skills
        .iter()
        .map(|item| {
            let checkbox = if item.selected { "[x]" } else { "[ ]" };
            let style = if item.selected {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", checkbox), style),
                Span::styled(item.skill.name.clone(), style),
                Span::styled(
                    format!(" — {}", item.skill.description),
                    Style::default().fg(TEXT_DIM),
                ),
            ]))
            .style(Style::default().bg(BG_PANE))
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(pane_block(
            format!(
                " Skills ({}/{}) ",
                count_selected_skills(state),
                state.skills.len()
            ),
            state.focus == Focus::Skills,
            SKILLS_COLOR,
        ))
        .highlight_style(highlight_style(SKILLS_COLOR))
        .highlight_symbol("❯ ")
        .style(Style::default().bg(BG_PANE));

    frame.render_stateful_widget(list, area, &mut state.skill_list_state);
}

fn draw_agents_list(frame: &mut Frame, state: &mut SelectionState, area: Rect) {
    let items = state
        .agents
        .iter()
        .map(|item| {
            let checkbox = if item.selected { "[x]" } else { "[ ]" };
            let suffix = item
                .agent
                .mode
                .map(|mode| format!(" ({mode})"))
                .unwrap_or_default();
            let style = if item.selected {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", checkbox), style),
                Span::styled(item.agent.name.clone(), style),
                Span::styled(suffix, Style::default().fg(AGENTS_COLOR)),
                Span::styled(
                    format!(" — {}", item.agent.description),
                    Style::default().fg(TEXT_DIM),
                ),
            ]))
            .style(Style::default().bg(BG_PANE))
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(pane_block(
            format!(
                " Agents ({}/{}) ",
                count_selected_agents(state),
                state.agents.len()
            ),
            state.focus == Focus::Agents,
            AGENTS_COLOR,
        ))
        .highlight_style(highlight_style(AGENTS_COLOR))
        .highlight_symbol("❯ ")
        .style(Style::default().bg(BG_PANE));

    frame.render_stateful_widget(list, area, &mut state.agent_list_state);
}

fn draw_commands_list(frame: &mut Frame, state: &mut SelectionState, area: Rect) {
    let items = state
        .commands
        .iter()
        .map(|item| {
            let checkbox = if item.selected { "[x]" } else { "[ ]" };
            let style = if item.selected {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", checkbox), style),
                Span::styled(item.command.name.clone(), style),
                Span::styled(
                    format!(
                        " — {}",
                        item.command
                            .description
                            .as_deref()
                            .unwrap_or("No description")
                    ),
                    Style::default().fg(TEXT_DIM),
                ),
            ]))
            .style(Style::default().bg(BG_PANE))
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(pane_block(
            format!(
                " Commands ({}/{}) ",
                count_selected_commands(state),
                state.commands.len()
            ),
            state.focus == Focus::Commands,
            COMMANDS_COLOR,
        ))
        .highlight_style(highlight_style(COMMANDS_COLOR))
        .highlight_symbol("❯ ")
        .style(Style::default().bg(BG_PANE));

    frame.render_stateful_widget(list, area, &mut state.command_list_state);
}

fn draw_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(Line::from(vec![
        Span::styled(
            "Tab",
            Style::default().fg(HELP_COLOR).add_modifier(Modifier::BOLD),
        ),
        Span::styled(": switch panes", Style::default().fg(TEXT_DIM)),
        Span::raw("  |  "),
        Span::styled(
            "↑/↓ or j/k",
            Style::default().fg(HELP_COLOR).add_modifier(Modifier::BOLD),
        ),
        Span::styled(": navigate", Style::default().fg(TEXT_DIM)),
        Span::raw("  |  "),
        Span::styled(
            "Space/Enter",
            Style::default().fg(HELP_COLOR).add_modifier(Modifier::BOLD),
        ),
        Span::styled(": toggle", Style::default().fg(TEXT_DIM)),
        Span::raw("  |  "),
        Span::styled(
            "c",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(": confirm", Style::default().fg(TEXT_DIM)),
        Span::raw("  |  "),
        Span::styled(
            "q/Esc",
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(": cancel", Style::default().fg(TEXT_DIM)),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(BG_DARK).fg(Color::White))
    .block(
        Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(HELP_COLOR))
            .style(Style::default().bg(BG_DARK).fg(Color::White)),
    );

    frame.render_widget(help, area);
}

fn move_selection(list_state: &mut ListState, len: usize, forward: bool) {
    if len == 0 {
        return;
    }

    let current = list_state.selected().unwrap_or(0);
    let next = if forward {
        (current + 1) % len
    } else if current == 0 {
        len - 1
    } else {
        current - 1
    };

    list_state.select(Some(next));
}

fn pane_block(title: String, focused: bool, accent: Color) -> Block<'static> {
    let border_style = if focused {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(60, 60, 70))
    };

    let title_style = if focused {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEXT_DIM)
    };

    let border_type = if focused {
        BorderType::Thick
    } else {
        BorderType::Plain
    };

    Block::default()
        .title(Span::styled(title, title_style))
        .borders(Borders::ALL)
        .border_style(border_style)
        .border_type(border_type)
        .style(Style::default().bg(BG_PANE))
}

fn highlight_style(accent: Color) -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(accent)
        .add_modifier(Modifier::BOLD)
}

fn count_selected_skills(state: &SelectionState) -> usize {
    state.skills.iter().filter(|item| item.selected).count()
}

fn count_selected_agents(state: &SelectionState) -> usize {
    state.agents.iter().filter(|item| item.selected).count()
}

fn count_selected_commands(state: &SelectionState) -> usize {
    state.commands.iter().filter(|item| item.selected).count()
}
