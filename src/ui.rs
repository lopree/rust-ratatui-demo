use ratatui::{
    layout::{Constraint, Direction, Layout,Alignment},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
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
            Constraint::Percentage(60),
        ])
        .split(down_left_layout[0]);
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
        .borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    frame.render_widget(converted_paragraph, down_left_layout[1]);
    //
    // 在 ui 函数中
    let down_left_up_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(down_left_layout[0]);

    // 示例数据
    let items = vec![
        ListItem::new("美元 (USD)"),
        ListItem::new("日元 (JPY)"),
        ListItem::new("澳大利亚元 (AUD)"),
        ListItem::new("阿根廷比索 (ARS)"),
        ListItem::new("印度卢比 (INR)"),
        ListItem::new("英国镑 (GBP)"),
        ListItem::new("土耳其里拉 (TRY)"),
    ];

    // 创建 List 小部件
    let list = List::new(items)
        .block(Block::default()
            .title(" 选择基准货币 ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
        );

    // 渲染 List 到布局
    frame.render_widget(list, down_left_up_layout[1]);
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
