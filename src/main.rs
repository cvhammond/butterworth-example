use butterworth::{Cutoff, Filter};
use c3dio::prelude::*;
use eframe::egui;
use egui::*;
use egui_plot::{Line, PlotPoints};

fn main() {
    let _ = eframe::run_native(
        "Test egui",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1024.0, 768.0)),
            ..Default::default()
        },
        Box::new(|_app| Box::<App>::default()),
    );
}

#[derive(Default)]
struct App {
    c3d: Option<C3d>,
    filtered: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(Button::new("Open C3D file")).clicked() {
                    self.c3d = Some(C3d::load("src/18124framesf.c3d").unwrap());
                }
                if let Some(c3d) = &mut self.c3d {
                    ui.add(Label::new("C3D file loaded"));
                    if ui.add(Button::new("Filter")).clicked() && !self.filtered {
                        filter_markers(
                            c3d,
                            Filter::new(
                                4,
                                c3d.points.frame_rate as f64
                                    * c3d.analog.samples_per_channel_per_frame as f64,
                                Cutoff::LowPass(2.0),
                            )
                            .unwrap(),
                        );
                        self.filtered = true;
                    }
                }
            });

            if let Some(c3d) = &self.c3d {
                let n = c3d.points.rows() - 2;
                let line_points: PlotPoints = (0..=n)
                    .map(|i| [i as f64, c3d.points.get(i, 2).unwrap()[2] as f64])
                    .collect();
                let line = Line::new(line_points);
                egui_plot::Plot::new("example_plot")
                    .height(256.0)
                    .show_axes(false)
                    .show(ui, |plot_ui| plot_ui.line(line));
            }
        });
    }
}

fn filter_markers(c3d: &mut C3d, filter: Filter) {
    let i = 2;
    let j = 2;
    let mut temp: Vec<f64> = c3d
        .points
        .iter_col(i)
        .map(|x| x[j] as f64)
        .collect::<Vec<f64>>();
    let filtered = match filter.bidirectional(&mut temp) {
        Ok(filtered) => filtered,
        Err(e) => {
            println!("{}: {}", "Error", e);
            return;
        }
    };
    for (k, val) in filtered.iter().enumerate() {
        c3d.points[k][i][j] = *val as f32;
    }
}
