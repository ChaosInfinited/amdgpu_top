use crate::{BASE, HEADING, HISTORY_LENGTH};
use eframe::egui::{self, collapsing_header::CollapsingState, FontId, util::History, Id, RichText};
use libamdgpu_top::stat::Sensors;

#[derive(Debug, Clone)]
pub struct SensorsHistory {
    pub sclk: History<u32>,
    pub mclk: History<u32>,
    pub vddgfx: History<u32>,
    pub vddnb: History<u32>,
    pub edge_temp: History<i64>,
    pub junction_temp: History<i64>,
    pub memory_temp: History<i64>,
    pub average_power: History<u32>,
    pub input_power: History<u32>,
    pub fan_rpm: History<u32>,
}

impl SensorsHistory {
    pub fn new() -> Self {
        let [sclk, mclk, vddgfx, vddnb, average_power, input_power, fan_rpm] = [0; 7]
            .map(|_| History::new(HISTORY_LENGTH, f32::INFINITY));
        let [edge_temp, junction_temp, memory_temp] = [0;3]
            .map(|_| History::new(HISTORY_LENGTH, f32::INFINITY));

        Self { sclk, mclk, vddgfx, vddnb, edge_temp, junction_temp, memory_temp, average_power, input_power, fan_rpm }
    }

    pub fn add(&mut self, sec: f64, sensors: &Sensors) {
        for (history, val) in [
            (&mut self.sclk, sensors.sclk),
            (&mut self.mclk, sensors.mclk),
            (&mut self.vddgfx, sensors.vddgfx),
            (&mut self.vddnb, sensors.vddnb),
            (&mut self.average_power, sensors.average_power.as_ref().map(|power| power.value)),
            (&mut self.input_power, sensors.input_power.as_ref().map(|power| power.value)),
            (&mut self.fan_rpm, sensors.fan_rpm),
        ] {
            let Some(val) = val else { continue };
            history.add(sec, val);
        }

        for (history, temp) in [
            (&mut self.edge_temp, &sensors.edge_temp),
            (&mut self.junction_temp, &sensors.junction_temp),
            (&mut self.memory_temp, &sensors.memory_temp),
        ] {
            let Some(temp) = temp else { continue };
            history.add(sec, temp.current);
        }
    }
}

impl Default for SensorsHistory {
    fn default() -> Self {
        Self::new()
    }
}

pub fn label(text: &str, font: FontId) -> egui::Label {
    egui::Label::new(RichText::new(text).font(font)).sense(egui::Sense::click())
}

pub fn collapsing_plot(
    ui: &mut egui::Ui,
    text: &str,
    default_open: bool,
    body: impl FnOnce(&mut egui::Ui),
) {
    let mut state = CollapsingState::load_with_default_open(ui.ctx(), Id::new(text), default_open);

    let _ = ui.horizontal(|ui| {
        let icon = {
            let text = if state.is_open() { "\u{25be}" } else { "\u{25b8}" };
            label(text, BASE)
        };
        let header = label(text, BASE);
        if ui.add(icon).clicked() || ui.add(header).clicked() {
            state.toggle(ui);
        }
    });

    state.show_body_unindented(ui, body);
}

pub fn collapsing_with_id(
    ui: &mut egui::Ui,
    text: &str,
    id: &str,
    default_open: bool,
    body: impl FnOnce(&mut egui::Ui),
) {
    let mut state = CollapsingState::load_with_default_open(ui.ctx(), Id::new(id), default_open);

    let header_res = ui.horizontal(|ui| {
        let icon = {
            let text = if state.is_open() { "\u{25be}" } else { "\u{25b8}" };
            label(text, HEADING)
        };
        let header = label(text, HEADING);
        if ui.add(icon).clicked() || ui.add(header).clicked() {
            state.toggle(ui);
        }
    });

    state.show_body_indented(&header_res.response, ui, body);
}

pub fn collapsing(
    ui: &mut egui::Ui,
    text: &str,
    default_open: bool,
    body: impl FnOnce(&mut egui::Ui),
) {
    collapsing_with_id(ui, text, text, default_open, body);
}

pub fn rt_base<T: Into<String>>(s: T) -> RichText {
    RichText::new(s.into()).font(BASE)
}
