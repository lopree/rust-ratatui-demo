use ratatui::{
    layout::{Constraint, Direction, Layout,Alignment},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::text::Line;
use ratatui::widgets::{List, ListItem, Wrap};
use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    // 创建基础的页面布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(frame.area());
    let down_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(chunks[1]);
    let down_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(down_layout[0]);
    let down_left_up_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(down_left_layout[0]);
    //list
    let list_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(down_left_up_layout[1]);
    // 上布局的抬头
    let title_block = Block::default()
        .title(" 工具 ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Blue).fg(Color::White));

    // 上布局的内容
    let title_paragraph = Paragraph::new(Span::styled(
        &app.title,
        Style::default().fg(Color::White),
    ))
    .block(title_block);

    // 渲染内容到上布局
    frame.render_widget(title_paragraph, chunks[0]);
    //下左上布局
    let input_block = Block::default()
        .title(" 输入人民币金额 ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let input_paragraph = Paragraph::new(app.amount_input.as_str())
        .alignment(Alignment::Center)
        .block(input_block)
        .wrap(Wrap { trim: false });
    frame.render_widget(input_paragraph, down_left_up_layout[0]);

    let converted_paragraph = Paragraph::new(Text::from(app.converted_amount.clone()))
        .alignment(Alignment::Center)
        .block(Block::default()
        .title(" 转换结果 ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL));

    frame.render_widget(converted_paragraph, down_left_layout[1]);
    //
    let number_items: Vec<ListItem> = (1..=8)
        .map(|n| ListItem::new(Line::from(format!("{}", n)).alignment(Alignment::Center)))
        .collect();

    // 创建第二个 List 小部件
    let number_list = List::new(number_items)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(number_list, list_layout[0]);
    //
    let items = vec![
        ListItem::new(Line::from("人民币 (RMB)").alignment(Alignment::Center)),
        ListItem::new(Line::from("美元 (USD)").alignment(Alignment::Center)),
        ListItem::new(Line::from("日元 (JPY)").alignment(Alignment::Center)),
        ListItem::new(Line::from("澳元 (AUD)").alignment(Alignment::Center)),
        ListItem::new(Line::from("阿根廷比索 (ARS)").alignment(Alignment::Center)),
        ListItem::new(Line::from("印度卢比 (INR)").alignment(Alignment::Center)),
        ListItem::new(Line::from("英镑 (GBP)").alignment(Alignment::Center)),
        ListItem::new(Line::from("土耳其里拉 (TRY)").alignment(Alignment::Center)),
    ];

    // 创建 List 小部件
    let list = List::new(items)
        .block(Block::default()
            .title(" 选择基准货币 ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
        );

    // 渲染 List 货币 到布局
    frame.render_widget(list, list_layout[1]);
    // 下右布局内容
    // 在 ui.rs 中，修改 footer 代码以显示汇率
    let rates_display = app
        .exchange_rates
        .iter()
        .map(|(currency, rate)| format!("{}: {:.2}", currency, rate))
        .collect::<Vec<_>>()
        .join(", ");

    let footer = Paragraph::new(Text::styled(
        format!("汇率: {}", rates_display),
        Style::default().fg(Color::Yellow),
    ))
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::ALL));

    // 渲染到下右布局
    frame.render_widget(footer, down_layout[1]);
}
