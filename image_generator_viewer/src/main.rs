use std::time::Duration;

use anyhow::Result;
use nvml_wrapper::Nvml;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
};
struct GpuStats {
    name: String,
    vram_used_mb: u64,
    vram_total_mb: u64,
    temp: u32,
    gpu_util: u32,
    power: f32,
}

fn main() -> Result<()> {
    let nvml = Nvml::init()?;

    let mut terminal = ratatui::init();

    let app_result = run(&mut terminal, &nvml);

    ratatui::restore();

    app_result
}

fn run(terminal: &mut DefaultTerminal, nvml: &Nvml) -> Result<()> {
    let device = nvml.device_by_index(0)?;
    let name = device.name()?;

    loop {
        let mem = device.memory_info()?;
        let temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0);
        let util = device.utilization_rates().map(|u| u.gpu).unwrap_or(0);
        let power = device.power_usage().unwrap_or(0) as f32 / 1000.0;

        let stats = GpuStats {
            name: name.clone(),
            vram_used_mb: mem.used / 1024 / 1024,
            vram_total_mb: mem.total / 1024 / 1024,
            temp,
            gpu_util: util,
            power: power,
        };

        terminal.draw(|frame| ui(frame, &stats))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, stats: &GpuStats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let info_text = format!(
        "Видеократа: {}\n Загрузка ядра: {}% | Температура: {}°C | Питание: {:.1} W",
        stats.name, stats.gpu_util, stats.temp, stats.power
    );
    let info_widget = Paragraph::new(info_text)
        .block(Block::default().title(" Статус GPU ").borders(Borders::ALL));
    frame.render_widget(info_widget, chunks[0]);

    let ratio = if stats.vram_total_mb > 0 {
        (stats.vram_used_mb as f64 / stats.vram_total_mb as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let bar_color = if ratio > 0.85 {
        Color::Red
    } else if ratio > 0.65 {
        Color::Yellow
    } else {
        Color::Green
    };

    let label = format!(
        "{} MB / {} MB ({:.1}%)",
        stats.vram_used_mb,
        stats.vram_total_mb,
        ratio * 100.0,
    );

    let vram_gauge = Gauge::default()
        .block(
            Block::default()
                .title(" Потребление VRAM ")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(bar_color))
        .ratio(ratio)
        .label(label);
    frame.render_widget(vram_gauge, chunks[1]);
    let footer = Paragraph::new("Нажми [q] или [Esc] для выхода")
        .block(Block::default().title(" Лаунчер ").borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}
