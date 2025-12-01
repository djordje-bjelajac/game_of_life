use std::time::{Duration, Instant};

use eframe::egui::{
    self,
    color_picker::{self, Alpha},
    Color32, ComboBox, RichText, Sense, Slider,
};

use crate::domain::{
    next_generation, Cell, Grid, MAX_GRID_SIZE, MAX_UPS, MIN_GRID_SIZE, MIN_UPS, PATTERNS,
};

pub struct GameApp {
    grid: Grid,
    generation: u64,
    paused: bool,
    last_step: Instant,
    settings: Settings,
    selected_pattern: usize,
    alive_cells: usize,
}

struct Settings {
    grid_width: usize,
    grid_height: usize,
    updates_per_second: u32,
    alive_color: Color32,
    dead_color: Color32,
    background_color: Color32,
    grid_line_color: Color32,
}

impl Settings {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid_width: width,
            grid_height: height,
            updates_per_second: 10,
            alive_color: Color32::from_rgb(0x3b, 0xd9, 0x20),
            dead_color: Color32::from_rgb(0x24, 0x2b, 0x30),
            background_color: Color32::from_rgb(0x0f, 0x12, 0x14),
            grid_line_color: Color32::from_rgba_unmultiplied(255, 255, 255, 25),
        }
    }

    fn step_duration(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.updates_per_second as f32)
    }
}

impl GameApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let width = 80;
        let height = 60;
        let settings = Settings::new(width, height);
        let mut grid = Grid::new(width, height);
        grid.randomize();

        let mut app = Self {
            grid,
            generation: 0,
            paused: false,
            last_step: Instant::now(),
            settings,
            selected_pattern: 0,
            alive_cells: 0,
        };
        app.recount_alive();
        app
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    fn randomize(&mut self) {
        self.grid.randomize();
        self.generation = 0;
        self.recount_alive();
    }

    fn clear(&mut self) {
        self.grid.clear();
        self.generation = 0;
        self.recount_alive();
    }

    fn apply_resize(&mut self) {
        let mut new_grid = Grid::new(self.settings.grid_width, self.settings.grid_height);
        let max_y = usize::min(new_grid.height(), self.grid.height());
        let max_x = usize::min(new_grid.width(), self.grid.width());

        for y in 0..max_y {
            for x in 0..max_x {
                new_grid.set(x, y, self.grid.get(x, y));
            }
        }

        self.grid = new_grid;
        self.generation = 0;
        self.recount_alive();
    }

    fn insert_selected_pattern(&mut self) {
        let pattern = &PATTERNS[self.selected_pattern];
        let center_x = (self.grid.width() / 2) as i32;
        let center_y = (self.grid.height() / 2) as i32;

        for &(dx, dy) in pattern.cells {
            let x = center_x + dx;
            let y = center_y + dy;
            if x >= 0 && y >= 0 {
                let ux = x as usize;
                let uy = y as usize;
                if ux < self.grid.width() && uy < self.grid.height() {
                    self.grid.set(ux, uy, Cell::Alive);
                }
            }
        }

        self.generation = 0;
        self.recount_alive();
    }

    fn recount_alive(&mut self) {
        let mut count = 0usize;
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                if self.grid.get(x, y).is_alive() {
                    count += 1;
                }
            }
        }
        self.alive_cells = count;
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|input| {
            if input.key_pressed(egui::Key::Space) {
                self.toggle_pause();
            }
            if input.key_pressed(egui::Key::R) {
                self.randomize();
            }
            if input.key_pressed(egui::Key::C) {
                self.clear();
            }
        });
    }

    fn maybe_step_simulation(&mut self) {
        if self.paused {
            return;
        }

        if self.last_step.elapsed() >= self.settings.step_duration() {
            self.grid = next_generation(&self.grid);
            self.generation += 1;
            self.last_step = Instant::now();
            self.recount_alive();
        }
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.heading("Simulation");
        if ui
            .button(if self.paused {
                "Resume (Space)"
            } else {
                "Pause (Space)"
            })
            .clicked()
        {
            self.toggle_pause();
        }

        ui.horizontal(|ui| {
            if ui.button("Randomize (R)").clicked() {
                self.randomize();
            }
            if ui.button("Clear (C)").clicked() {
                self.clear();
            }
        });

        ui.separator();
        ui.heading("Speed");
        if ui
            .add(
                Slider::new(&mut self.settings.updates_per_second, MIN_UPS..=MAX_UPS)
                    .text("Updates / second"),
            )
            .changed()
            && self.settings.updates_per_second < MIN_UPS
        {
            self.settings.updates_per_second = MIN_UPS;
        }

        ui.separator();
        ui.heading("Grid Size");
        let mut resized = false;
        let width_changed = ui
            .add(
                Slider::new(&mut self.settings.grid_width, MIN_GRID_SIZE..=MAX_GRID_SIZE)
                    .text("Width"),
            )
            .changed();
        let height_changed = ui
            .add(
                Slider::new(
                    &mut self.settings.grid_height,
                    MIN_GRID_SIZE..=MAX_GRID_SIZE,
                )
                .text("Height"),
            )
            .changed();
        if width_changed || height_changed {
            resized = true;
        }
        if resized {
            self.apply_resize();
        }

        ui.separator();
        ui.heading("Colors");
        ui.horizontal(|ui| {
            ui.label("Alive");
            color_picker::color_edit_button_srgba(
                ui,
                &mut self.settings.alive_color,
                Alpha::Opaque,
            );
        });
        ui.horizontal(|ui| {
            ui.label("Dead");
            color_picker::color_edit_button_srgba(ui, &mut self.settings.dead_color, Alpha::Opaque);
        });
        ui.horizontal(|ui| {
            ui.label("Background");
            color_picker::color_edit_button_srgba(
                ui,
                &mut self.settings.background_color,
                Alpha::Opaque,
            );
        });

        ui.separator();
        ui.heading("Patterns");
        ComboBox::from_label("Pattern")
            .selected_text(PATTERNS[self.selected_pattern].name)
            .show_ui(ui, |ui| {
                for (idx, pattern) in PATTERNS.iter().enumerate() {
                    ui.selectable_value(&mut self.selected_pattern, idx, pattern.name);
                }
            });
        if ui.button("Insert Pattern").clicked() {
            self.insert_selected_pattern();
        }
    }

    fn render_status(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Generation: {}", self.generation)).strong());
            ui.separator();
            ui.label(format!(
                "Alive cells: {} / {}",
                self.alive_cells,
                self.grid.width() * self.grid.height()
            ));
            ui.separator();
            ui.label(format!(
                "Grid: {} x {} | Speed: {} UPS",
                self.grid.width(),
                self.grid.height(),
                self.settings.updates_per_second
            ));
        });
    }

    fn render_canvas(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let available_size = ui.available_size();
        let grid_width = self.grid.width() as f32;
        let grid_height = self.grid.height() as f32;
        if grid_width <= 0.0 || grid_height <= 0.0 {
            return;
        }
        let mut cell_size = (available_size.x / grid_width)
            .min(available_size.y / grid_height)
            .max(4.0);
        if !cell_size.is_finite() {
            cell_size = 4.0;
        }
        let grid_size = egui::vec2(cell_size * grid_width, cell_size * grid_height);
        let (response, painter) =
            ui.allocate_painter(grid_size, Sense::click_and_drag().union(Sense::hover()));
        let rect = response.rect;

        painter.rect_filled(rect, 0.0, self.settings.background_color);

        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let cell = self.grid.get(x, y);
                let min = egui::pos2(
                    rect.left() + x as f32 * cell_size,
                    rect.top() + y as f32 * cell_size,
                );
                let max = egui::pos2(min.x + cell_size, min.y + cell_size);
                let cell_rect = egui::Rect::from_min_max(min, max);
                let color = if cell.is_alive() {
                    self.settings.alive_color
                } else {
                    self.settings.dead_color
                };
                painter.rect_filled(cell_rect, 0.0, color);
                painter.rect_stroke(
                    cell_rect,
                    0.0,
                    egui::Stroke::new(0.5, self.settings.grid_line_color),
                );
            }
        }

        self.handle_pointer_input(&response, rect, cell_size, ctx);
    }

    fn handle_pointer_input(
        &mut self,
        response: &egui::Response,
        rect: egui::Rect,
        cell_size: f32,
        ctx: &egui::Context,
    ) {
        let (primary_down, secondary_down) =
            ctx.input(|input| (input.pointer.primary_down(), input.pointer.secondary_down()));
        if !primary_down && !secondary_down {
            return;
        }

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            if !rect.contains(pointer_pos) {
                return;
            }

            let x = ((pointer_pos.x - rect.left()) / cell_size).floor() as usize;
            let y = ((pointer_pos.y - rect.top()) / cell_size).floor() as usize;

            if x < self.grid.width() && y < self.grid.height() {
                if primary_down {
                    self.grid.set(x, y, Cell::Alive);
                } else if secondary_down {
                    self.grid.set(x, y, Cell::Dead);
                }
                self.recount_alive();
            }
        }
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_keyboard(ctx);
        self.maybe_step_simulation();

        egui::SidePanel::left("control_panel")
            .resizable(false)
            .default_width(260.0)
            .show(ctx, |ui| {
                self.render_controls(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_canvas(ui, ctx);
        });

        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            self.render_status(ui);
        });

        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
