use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::models::{DiscoveredAgent, DiscoveredSkill};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Skills,
    Agents,
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    pub focus: Focus,
    pub skills: Vec<SelectableSkill>,
    pub agents: Vec<SelectableAgent>,
    pub skill_list_state: ListState,
    pub agent_list_state: ListState,
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
pub struct SelectionResult {
    pub selected_skills: Vec<String>,
    pub selected_agents: Vec<String>,
}

impl SelectionState {
    pub fn new(skills: Vec<DiscoveredSkill>, agents: Vec<DiscoveredAgent>) -> Self {
        let mut skill_list_state = ListState::default();
        if !skills.is_empty() {
            skill_list_state.select(Some(0));
        }

        let mut agent_list_state = ListState::default();
        if !agents.is_empty() {
            agent_list_state.select(Some(0));
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
            skill_list_state,
            agent_list_state,
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
        }
    }

    pub fn next(&mut self) {
        match self.focus {
            Focus::Skills => {
                let len = self.skills.len();
                if len == 0 {
                    return;
                }
                let next = self
                    .skill_list_state
                    .selected()
                    .map(|i| (i + 1) % len)
                    .unwrap_or(0);
                self.skill_list_state.select(Some(next));
            }
            Focus::Agents => {
                let len = self.agents.len();
                if len == 0 {
                    return;
                }
                let next = self
                    .agent_list_state
                    .selected()
                    .map(|i| (i + 1) % len)
                    .unwrap_or(0);
                self.agent_list_state.select(Some(next));
            }
        }
    }

    pub fn previous(&mut self) {
        match self.focus {
            Focus::Skills => {
                let len = self.skills.len();
                if len == 0 {
                    return;
                }
                let prev = self
                    .skill_list_state
                    .selected()
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(len - 1);
                let prev = if prev == 0 && self.skill_list_state.selected() == Some(0) {
                    len - 1
                } else {
                    prev
                };
                self.skill_list_state.select(Some(prev));
            }
            Focus::Agents => {
                let len = self.agents.len();
                if len == 0 {
                    return;
                }
                let prev = self
                    .agent_list_state
                    .selected()
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(len - 1);
                let prev = if prev == 0 && self.agent_list_state.selected() == Some(0) {
                    len - 1
                } else {
                    prev
                };
                self.agent_list_state.select(Some(prev));
            }
        }
    }

    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Skills => Focus::Agents,
            Focus::Agents => Focus::Skills,
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
        if self.cancelled {
            return None;
        }
        if !self.confirmed {
            return None;
        }

        let selected_skills: Vec<String> = self
            .skills
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.skill.name.clone())
            .collect();

        let selected_agents: Vec<String> = self
            .agents
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.agent.name.clone())
            .collect();

        Some(SelectionResult {
            selected_skills,
            selected_agents,
        })
    }
}

pub fn run_interactive_selection(
    skills: Vec<DiscoveredSkill>,
    agents: Vec<DiscoveredAgent>,
) -> io::Result<Option<SelectionResult>> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut state = SelectionState::new(skills, agents);

    let result = loop {
        terminal.draw(|frame| draw(frame, &mut state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        state.cancel();
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        state.toggle_current();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        state.next();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        state.previous();
                    }
                    KeyCode::Tab => {
                        state.switch_focus();
                    }
                    KeyCode::Char('c') => {
                        state.confirm();
                    }
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
    let [top, bottom] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(frame.area());

    let [skills_area, agents_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(top);

    draw_skills_list(frame, state, skills_area);
    draw_agents_list(frame, state, agents_area);
    draw_help(frame, bottom);
}

fn draw_skills_list(frame: &mut Frame, state: &mut SelectionState, area: Rect) {
    let focus_style = if state.focus == Focus::Skills {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let block = Block::default()
        .title(" Skills ")
        .borders(Borders::ALL)
        .border_style(focus_style);

    let items: Vec<ListItem> = state
        .skills
        .iter()
        .map(|item| {
            let checkbox = if item.selected { "[x]" } else { "[ ]" };
            let content = format!("{} {}", checkbox, item.skill.name);
            let style = if item.selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state.skill_list_state);
}

fn draw_agents_list(frame: &mut Frame, state: &mut SelectionState, area: Rect) {
    let focus_style = if state.focus == Focus::Agents {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let block = Block::default()
        .title(" Agents ")
        .borders(Borders::ALL)
        .border_style(focus_style);

    let items: Vec<ListItem> = state
        .agents
        .iter()
        .map(|item| {
            let checkbox = if item.selected { "[x]" } else { "[ ]" };
            let suffix = item
                .agent
                .mode
                .map(|m| format!(" ({})", m))
                .unwrap_or_default();
            let content = format!("{} {}{}", checkbox, item.agent.name, suffix);
            let style = if item.selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state.agent_list_state);
}

fn draw_help(frame: &mut Frame, area: Rect) {
    let help_text = "Tab: switch focus | ↑/↓ or j/k: navigate | Space/Enter: toggle | c: confirm | q/Esc: cancel";
    let paragraph =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title(" Help "));
    frame.render_widget(paragraph, area);
}
