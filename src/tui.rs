use std::io;
use std::path::Path;

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
pub struct TreeRow<T> {
    pub depth: usize,
    pub is_folder: bool,
    pub name: String,
    pub selected: bool,
    pub partial: bool,
    pub leaf: Option<T>,
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    pub focus: Focus,
    pub skills: Vec<TreeRow<DiscoveredSkill>>,
    pub agents: Vec<TreeRow<DiscoveredAgent>>,
    pub commands: Vec<TreeRow<DiscoveredCommand>>,
    pub skill_list_state: ListState,
    pub agent_list_state: ListState,
    pub command_list_state: ListState,
    pub confirmed: bool,
    pub cancelled: bool,
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
        let skill_rows = build_tree(skills, |s| &s.source_rel_path, |s| &s.name);
        let agent_rows = build_tree(agents, |a| &a.source_rel_path, |a| &a.name);
        let command_rows = build_tree(commands, |c| &c.source_rel_path, |c| &c.name);

        let mut skill_list_state = ListState::default();
        if !skill_rows.is_empty() {
            skill_list_state.select(Some(0));
        }

        let mut agent_list_state = ListState::default();
        if !agent_rows.is_empty() {
            agent_list_state.select(Some(0));
        }

        let mut command_list_state = ListState::default();
        if !command_rows.is_empty() {
            command_list_state.select(Some(0));
        }

        Self {
            focus: Focus::Skills,
            skills: skill_rows,
            agents: agent_rows,
            commands: command_rows,
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
                    toggle_tree_row(&mut self.skills, index);
                }
            }
            Focus::Agents => {
                if let Some(index) = self.agent_list_state.selected() {
                    toggle_tree_row(&mut self.agents, index);
                }
            }
            Focus::Commands => {
                if let Some(index) = self.command_list_state.selected() {
                    toggle_tree_row(&mut self.commands, index);
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
            .filter(|row| !row.is_folder && row.selected)
            .filter_map(|row| row.leaf.as_ref().map(|s| s.name.clone()))
            .collect();

        let selected_agents = self
            .agents
            .iter()
            .filter(|row| !row.is_folder && row.selected)
            .filter_map(|row| row.leaf.as_ref().map(|a| a.name.clone()))
            .collect();

        let selected_commands = self
            .commands
            .iter()
            .filter(|row| !row.is_folder && row.selected)
            .filter_map(|row| row.leaf.as_ref().map(|c| c.name.clone()))
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

fn build_tree<T>(
    mut items: Vec<T>,
    get_path: impl Fn(&T) -> &Path,
    get_name: impl Fn(&T) -> &str,
) -> Vec<TreeRow<T>> {
    items.sort_by(|a, b| get_path(a).cmp(get_path(b)));

    let mut rows: Vec<TreeRow<T>> = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    for item in items {
        let path = get_path(&item);
        let components: Vec<_> = path
            .iter()
            .map(|c| c.to_string_lossy().to_string())
            .collect();

        if components.is_empty() {
            continue;
        }

        // Find common prefix length with current stack
        let mut common = 0;
        while common < stack.len()
            && common + 1 < components.len()
            && stack[common] == components[common]
        {
            common += 1;
        }

        // Pop back to common prefix
        stack.truncate(common);

        // Push new folder components
        for i in common..components.len().saturating_sub(1) {
            let folder_name = components[i].clone();
            rows.push(TreeRow {
                depth: i,
                is_folder: true,
                name: folder_name.clone(),
                selected: false,
                partial: false,
                leaf: None,
            });
            stack.push(folder_name);
        }

        // Push leaf
        let leaf_name = get_name(&item).to_string();
        rows.push(TreeRow {
            depth: components.len().saturating_sub(1),
            is_folder: false,
            name: leaf_name,
            selected: false,
            partial: false,
            leaf: Some(item),
        });
    }

    rows
}

fn toggle_tree_row<T>(rows: &mut [TreeRow<T>], index: usize) {
    let Some(row) = rows.get(index) else {
        return;
    };
    let folder_depth = row.depth;

    if row.is_folder {
        let select_all = !row.selected;
        for j in (index + 1)..rows.len() {
            if rows[j].depth <= folder_depth {
                break;
            }
            if !rows[j].is_folder {
                rows[j].selected = select_all;
            }
        }
    } else {
        rows[index].selected = !rows[index].selected;
    }

    recompute_tree_state(rows);
}

fn recompute_tree_state<T>(rows: &mut [TreeRow<T>]) {
    for i in 0..rows.len() {
        if !rows[i].is_folder {
            continue;
        }
        let folder_depth = rows[i].depth;
        let mut all_selected = true;
        let mut any_selected = false;
        let mut has_leaves = false;

        for j in (i + 1)..rows.len() {
            if rows[j].depth <= folder_depth {
                break;
            }
            if !rows[j].is_folder {
                has_leaves = true;
                if rows[j].selected {
                    any_selected = true;
                } else {
                    all_selected = false;
                }
            }
        }

        if !has_leaves {
            rows[i].selected = false;
            rows[i].partial = false;
        } else {
            rows[i].selected = all_selected && any_selected;
            rows[i].partial = any_selected && !all_selected;
        }
    }
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
        .map(|row| skill_row_to_list_item(row))
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(pane_block(
            format!(
                " Skills ({}/{}) ",
                count_selected_skills(state),
                state.skills.iter().filter(|r| !r.is_folder).count()
            ),
            state.focus == Focus::Skills,
            SKILLS_COLOR,
        ))
        .highlight_style(highlight_style(SKILLS_COLOR))
        .highlight_symbol("❯ ")
        .style(Style::default().bg(BG_PANE));

    frame.render_stateful_widget(list, area, &mut state.skill_list_state);
}

fn skill_row_to_list_item(row: &TreeRow<DiscoveredSkill>) -> ListItem<'static> {
    let indent = "  ".repeat(row.depth);
    let checkbox = if row.is_folder {
        if row.selected {
            "[x]"
        } else if row.partial {
            "[-]"
        } else {
            "[ ]"
        }
    } else if row.selected {
        "[x]"
    } else {
        "[ ]"
    };

    let style = if row.selected || (!row.is_folder && row.selected) {
        Style::default()
            .fg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let line = if row.is_folder {
        Line::from(vec![
            Span::styled(format!("{}{} ", indent, checkbox), style),
            Span::styled(format!("{}", row.name), style),
        ])
    } else {
        let desc = row
            .leaf
            .as_ref()
            .map(|s| s.description.as_str())
            .unwrap_or("");
        Line::from(vec![
            Span::styled(format!("{}{} ", indent, checkbox), style),
            Span::styled(row.name.clone(), style),
            Span::styled(format!(" — {}", desc), Style::default().fg(TEXT_DIM)),
        ])
    };

    ListItem::new(line).style(Style::default().bg(BG_PANE))
}

fn draw_agents_list(frame: &mut Frame, state: &mut SelectionState, area: Rect) {
    let items = state
        .agents
        .iter()
        .map(|row| agent_row_to_list_item(row))
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(pane_block(
            format!(
                " Agents ({}/{}) ",
                count_selected_agents(state),
                state.agents.iter().filter(|r| !r.is_folder).count()
            ),
            state.focus == Focus::Agents,
            AGENTS_COLOR,
        ))
        .highlight_style(highlight_style(AGENTS_COLOR))
        .highlight_symbol("❯ ")
        .style(Style::default().bg(BG_PANE));

    frame.render_stateful_widget(list, area, &mut state.agent_list_state);
}

fn agent_row_to_list_item(row: &TreeRow<DiscoveredAgent>) -> ListItem<'static> {
    let indent = "  ".repeat(row.depth);
    let checkbox = if row.is_folder {
        if row.selected {
            "[x]"
        } else if row.partial {
            "[-]"
        } else {
            "[ ]"
        }
    } else if row.selected {
        "[x]"
    } else {
        "[ ]"
    };

    let style = if row.selected || (!row.is_folder && row.selected) {
        Style::default()
            .fg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let line = if row.is_folder {
        Line::from(vec![
            Span::styled(format!("{}{} ", indent, checkbox), style),
            Span::styled(format!("{}", row.name), style),
        ])
    } else {
        let (desc, mode_str) = row.leaf.as_ref().map_or(("", String::new()), |a| {
            let mode = a.mode.map(|m| format!(" ({m})")).unwrap_or_default();
            (a.description.as_str(), mode)
        });
        Line::from(vec![
            Span::styled(format!("{}{} ", indent, checkbox), style),
            Span::styled(row.name.clone(), style),
            Span::styled(mode_str, Style::default().fg(AGENTS_COLOR)),
            Span::styled(format!(" — {}", desc), Style::default().fg(TEXT_DIM)),
        ])
    };

    ListItem::new(line).style(Style::default().bg(BG_PANE))
}

fn draw_commands_list(frame: &mut Frame, state: &mut SelectionState, area: Rect) {
    let items = state
        .commands
        .iter()
        .map(|row| command_row_to_list_item(row))
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(pane_block(
            format!(
                " Commands ({}/{}) ",
                count_selected_commands(state),
                state.commands.iter().filter(|r| !r.is_folder).count()
            ),
            state.focus == Focus::Commands,
            COMMANDS_COLOR,
        ))
        .highlight_style(highlight_style(COMMANDS_COLOR))
        .highlight_symbol("❯ ")
        .style(Style::default().bg(BG_PANE));

    frame.render_stateful_widget(list, area, &mut state.command_list_state);
}

fn command_row_to_list_item(row: &TreeRow<DiscoveredCommand>) -> ListItem<'static> {
    let indent = "  ".repeat(row.depth);
    let checkbox = if row.is_folder {
        if row.selected {
            "[x]"
        } else if row.partial {
            "[-]"
        } else {
            "[ ]"
        }
    } else if row.selected {
        "[x]"
    } else {
        "[ ]"
    };

    let style = if row.selected || (!row.is_folder && row.selected) {
        Style::default()
            .fg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let line = if row.is_folder {
        Line::from(vec![
            Span::styled(format!("{}{} ", indent, checkbox), style),
            Span::styled(format!("{}", row.name), style),
        ])
    } else {
        let desc = row
            .leaf
            .as_ref()
            .and_then(|c| c.description.as_deref())
            .unwrap_or("No description");
        Line::from(vec![
            Span::styled(format!("{}{} ", indent, checkbox), style),
            Span::styled(row.name.clone(), style),
            Span::styled(format!(" — {}", desc), Style::default().fg(TEXT_DIM)),
        ])
    };

    ListItem::new(line).style(Style::default().bg(BG_PANE))
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
    state
        .skills
        .iter()
        .filter(|row| !row.is_folder && row.selected)
        .count()
}

fn count_selected_agents(state: &SelectionState) -> usize {
    state
        .agents
        .iter()
        .filter(|row| !row.is_folder && row.selected)
        .count()
}

fn count_selected_commands(state: &SelectionState) -> usize {
    state
        .commands
        .iter()
        .filter(|row| !row.is_folder && row.selected)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn skill(name: &str, source_rel_path: &str) -> DiscoveredSkill {
        DiscoveredSkill {
            name: name.to_string(),
            description: format!("{name} description"),
            source_path: PathBuf::from("/tmp"),
            source_rel_path: PathBuf::from(source_rel_path),
        }
    }

    #[test]
    fn build_tree_flat_items() {
        let items = vec![skill("code-review", "code-review"), skill("test", "test")];
        let rows = build_tree(items, |s| &s.source_rel_path, |s| &s.name);
        assert_eq!(rows.len(), 2);
        assert!(!rows[0].is_folder);
        assert!(!rows[1].is_folder);
    }

    #[test]
    fn build_tree_grouped_items() {
        let items = vec![
            skill("code-review", "review/code-review"),
            skill("security-review", "review/security-review"),
            skill("python-testing", "testing/python-testing"),
        ];
        let rows = build_tree(items, |s| &s.source_rel_path, |s| &s.name);
        assert_eq!(rows.len(), 5);

        // review folder
        assert!(rows[0].is_folder);
        assert_eq!(rows[0].name, "review");
        assert_eq!(rows[0].depth, 0);

        // code-review leaf
        assert!(!rows[1].is_folder);
        assert_eq!(rows[1].name, "code-review");
        assert_eq!(rows[1].depth, 1);

        // security-review leaf
        assert!(!rows[2].is_folder);
        assert_eq!(rows[2].name, "security-review");
        assert_eq!(rows[2].depth, 1);

        // testing folder
        assert!(rows[3].is_folder);
        assert_eq!(rows[3].name, "testing");
        assert_eq!(rows[3].depth, 0);

        // python-testing leaf
        assert!(!rows[4].is_folder);
        assert_eq!(rows[4].name, "python-testing");
        assert_eq!(rows[4].depth, 1);
    }

    #[test]
    fn toggle_folder_selects_all_descendants() {
        let items = vec![
            skill("code-review", "review/code-review"),
            skill("security-review", "review/security-review"),
        ];
        let mut rows = build_tree(items, |s| &s.source_rel_path, |s| &s.name);

        // Toggle folder at index 0
        toggle_tree_row(&mut rows, 0);

        assert!(rows[0].selected);
        assert!(!rows[0].partial);
        assert!(rows[1].selected);
        assert!(rows[2].selected);
    }

    #[test]
    fn toggle_folder_deselects_all_when_all_selected() {
        let items = vec![skill("code-review", "review/code-review")];
        let mut rows = build_tree(items, |s| &s.source_rel_path, |s| &s.name);

        // First toggle selects all
        toggle_tree_row(&mut rows, 0);
        assert!(rows[0].selected);
        assert!(rows[1].selected);

        // Second toggle deselects all
        toggle_tree_row(&mut rows, 0);
        assert!(!rows[0].selected);
        assert!(!rows[0].partial);
        assert!(!rows[1].selected);
    }

    #[test]
    fn toggle_leaf_updates_parent_partial_state() {
        let items = vec![
            skill("code-review", "review/code-review"),
            skill("security-review", "review/security-review"),
        ];
        let mut rows = build_tree(items, |s| &s.source_rel_path, |s| &s.name);

        // Toggle first leaf
        toggle_tree_row(&mut rows, 1);

        assert!(!rows[0].selected);
        assert!(rows[0].partial);
        assert!(rows[1].selected);
        assert!(!rows[2].selected);
    }

    #[test]
    fn selection_result_includes_only_selected_leaves() {
        let skills = vec![
            skill("code-review", "review/code-review"),
            skill("test", "test"),
        ];
        let state = SelectionState::new(skills, vec![], vec![]);
        let result = state.result();
        assert!(result.is_none(), "result is none before confirmation");
    }
}
