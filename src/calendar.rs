use crossterm::style::Stylize;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
struct Calendar {
    counter: u32,
}

impl Widget for &Calendar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".to_string(),
            "<Left>".blue().bold().to_string(),
            " Increment ".to_string(),
            "<Right>".blue().bold().to_string(),
            " Quit ".to_string(),
            "<Q> ".blue().bold().to_string(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".to_string(),
            self.counter.to_string().yellow().to_string(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
