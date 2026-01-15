use crate::app::{App, AppState};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(frame.size());

    draw_header(frame, app, chunks[0]);
    draw_content(frame, app, chunks[1]);
    draw_status_bar(frame, app, chunks[2]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!("Hacker News - {} Stories", app.story_type_name());
    let help_text = "[j/k] scroll [Space] category [d] details [o] open [m] more [q] quit";

    let text = Line::from(vec![
        Span::styled(title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(help_text, Style::default().fg(Color::DarkGray)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_content(frame: &mut Frame, app: &App, area: Rect) {
    match &app.state {
        AppState::Loading => {
            let text = Text::from("Loading stories...").centered();
            let block = Block::default().borders(Borders::ALL);
            let paragraph = Paragraph::new(text).block(block);
            frame.render_widget(paragraph, area);
        }
        AppState::LoadingMore => {
            let text = Text::from("Loading more stories...").centered();
            let block = Block::default().borders(Borders::ALL);
            let paragraph = Paragraph::new(text).block(block);
            frame.render_widget(paragraph, area);
        }
        AppState::Error(msg) => {
            let text = Text::from(format!("Error: {}", msg)).centered().red();
            let block = Block::default().borders(Borders::ALL);
            let paragraph = Paragraph::new(text).block(block);
            frame.render_widget(paragraph, area);
        }
        AppState::Ready => {
            if app.show_details {
                draw_details_view(frame, app, area);
            } else {
                draw_story_list(frame, app, area);
            }
        }
    }
}

fn draw_story_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .stories
        .iter()
        .skip(app.scroll_offset)
        .take(area.height as usize)
        .enumerate()
        .map(|(i, story)| {
            let idx = app.scroll_offset + i;
            let is_selected = idx == app.selected_index;
            let title = story.title.clone().unwrap_or_default();
            let has_url = story.url.is_some();

            let prefix = if is_selected {
                Span::styled("▶ ", Style::default().fg(Color::Green))
            } else if has_url {
                Span::styled("🔗 ", Style::default().fg(Color::Blue))
            } else {
                Span::styled("  ", Style::default().fg(Color::DarkGray))
            };

            let title_span = if is_selected {
                Span::styled(
                    title.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::DarkGray),
                )
            } else {
                Span::styled(title, Style::default().fg(Color::White))
            };

            let meta = format!(
                " {} | {} | {} comments",
                story.score,
                story.time_ago(),
                story.descendant.unwrap_or(0)
            );
            let meta_span = Span::styled(meta, Style::default().fg(Color::Gray));

            let domain = format!(" ({})", story.domain());
            let domain_span = Span::styled(domain, Style::default().fg(Color::Blue));

            let line = Line::from(vec![prefix, title_span, meta_span, domain_span]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Stories")
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_widget(list, area);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"))
        .track_symbol(Some(" "))
        .thumb_symbol("█")
        .style(Style::default().fg(Color::Gray));

    let scrollbar_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(area)[0];

    frame.render_stateful_widget(
        scrollbar,
        scrollbar_area,
        &mut ratatui::widgets::ScrollbarState::new(app.stories.len())
            .position(app.selected_index)
            .viewport_content_length(area.height as usize),
    );
}

fn draw_details_view(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(story) = app.selected_story() {
        let title = story.title.clone().unwrap_or_default();
        let url = story.url.clone().unwrap_or_default();
        let text = story.text.clone().unwrap_or_default();
        let score = story.score.to_string();
        let time_ago = story.time_ago();
        let comments = story.descendant.unwrap_or(0).to_string();
        let by = story.by.clone();
        let domain = story.domain();
        let story_type = story.r#type.clone();
        let kids_count = story.kids.as_ref().map_or(0, |k| k.len());

        let mut content = vec![
            Line::from(Span::styled(
                title,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Type: ", Style::default().fg(Color::Gray)),
                Span::styled(story_type, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Points: ", Style::default().fg(Color::Gray)),
                Span::styled(score, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("By: ", Style::default().fg(Color::Gray)),
                Span::styled(by, Style::default().fg(Color::Blue)),
            ]),
            Line::from(vec![
                Span::styled("Time: ", Style::default().fg(Color::Gray)),
                Span::styled(time_ago, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Comments: ", Style::default().fg(Color::Gray)),
                Span::styled(comments, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Comment IDs: ", Style::default().fg(Color::Gray)),
                Span::styled(kids_count.to_string(), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
        ];

        if !url.is_empty() {
            content.push(Line::from(vec![
                Span::styled("URL: ", Style::default().fg(Color::Gray)),
                Span::styled(&url, Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED)),
            ]));
            content.push(Line::from(vec![
                Span::styled("Domain: ", Style::default().fg(Color::Gray)),
                Span::styled(domain, Style::default().fg(Color::Cyan)),
            ]));
            content.push(Line::from(""));
        }

        if !text.is_empty() {
            let stripped_text = strip_html_tags(&text);
            content.push(Line::from(Span::styled(
                "Story Text:",
                Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
            )));
            content.push(Line::from(""));
            for line in stripped_text.lines() {
                if !line.trim().is_empty() {
                    content.push(Line::from(line.to_string()));
                }
            }
            content.push(Line::from(""));
        }

        content.push(Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::DarkGray)),
            Span::styled("[d]", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(" to go back", Style::default().fg(Color::DarkGray)),
        ]));

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Story Details")
                    .border_style(Style::default().fg(Color::White)),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

fn strip_html_tags(input: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in input.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    result
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let (left_text, right_text) = match app.state {
        AppState::Ready => {
            let position = format!("{}/{}", app.selected_index + 1, app.stories.len());
            let position_info = format!("Position: {}", position);
            let has_link = if app.has_selected_story_url() {
                "[o] open"
            } else {
                "[no link]"
            };
            let more_info = if app.can_load_more() {
                "[m] more"
            } else {
                "[all loaded]"
            };
            (position_info, format!("{} | {} | 'q' quit", has_link, more_info))
        }
        AppState::Loading => ("Loading...".to_string(), "Press 'q' to quit".to_string()),
        AppState::LoadingMore => {
            let position = format!("{}/{}", app.selected_index + 1, app.stories.len());
            let position_info = format!("Position: {}", position);
            (position_info, "Loading more stories...".to_string())
        }
        AppState::Error(_) => (
            "Error loading stories".to_string(),
            "Press 'r' to retry or 'q' to quit".to_string(),
        ),
    };

    let text = Line::from(vec![
        Span::styled(left_text, Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(
            right_text,
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
