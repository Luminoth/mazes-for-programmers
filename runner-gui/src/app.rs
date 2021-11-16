use std::fmt;
use std::time::Instant;

use derivative::Derivative;
use derive_more::Display;
use eframe::{egui, epi};
use strum::{EnumIter, IntoEnumIterator};
use tracing::{debug, info};

use mazecore::generators::*;
use mazecore::solvers::*;
use mazecore::Grid;

// TODO: all of this would be cleaner with macros

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Derivative, Display)]
#[derivative(Default)]
pub enum GeneratorType {
    #[derivative(Default)]
    #[display(fmt = "Binary Tree")]
    BinaryTree,

    #[display(fmt = "Sidewinder")]
    Sidewinder,

    #[display(fmt = "Aldous-Broder")]
    AldousBroder,

    #[display(fmt = "Wilson's Algorithm")]
    Wilsons,

    #[display(fmt = "Hunt-and-Kill")]
    HuntAndKill,
}

impl GeneratorType {
    fn generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorType::BinaryTree => Box::new(BinaryTree::default()),
            GeneratorType::Sidewinder => Box::new(Sidewinder::default()),
            GeneratorType::AldousBroder => Box::new(AldousBroder::default()),
            GeneratorType::Wilsons => Box::new(Wilsons::default()),
            GeneratorType::HuntAndKill => Box::new(HuntAndKill::default()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Derivative, Display)]
#[derivative(Default)]
pub enum SolverType {
    #[derivative(Default)]
    #[display(fmt = "None")]
    None,

    #[display(fmt = "Djikstra")]
    Djikstra,
}

impl SolverType {
    fn solver(&self, grid: Grid, root_row: usize, root_col: usize) -> Box<dyn Solver> {
        match self {
            SolverType::None => Box::new(NoneSolver::new(grid)),
            SolverType::Djikstra => Box::new(Djikstra::new(grid, root_row, root_col)),
        }
    }
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct RunnerApp {
    #[derivative(Default(value = "20"))]
    width: usize,

    #[derivative(Default(value = "20"))]
    height: usize,

    generator_type: GeneratorType,
    solver_type: SolverType,

    maze_renderable: Option<Box<dyn Solver>>,
    maze_ascii: String,
}

impl RunnerApp {
    pub fn add_selection<T>(generator_type: T, ui: &mut egui::Ui, selection: &mut T)
    where
        T: Copy + PartialEq + fmt::Display,
    {
        ui.selectable_value(selection, generator_type, format!("{}", generator_type));
    }

    fn add_generators_select(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Maze Generator")
            .selected_text(format!("{}", self.generator_type))
            .width(130.0)
            .show_ui(ui, |ui| {
                for generator_type in GeneratorType::iter() {
                    RunnerApp::add_selection(generator_type, ui, &mut self.generator_type);
                }
            });
    }

    fn add_solvers_select(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Maze Solver")
            .selected_text(format!("{}", self.solver_type))
            .width(130.0)
            .show_ui(ui, |ui| {
                for solver_type in SolverType::iter() {
                    RunnerApp::add_selection(solver_type, ui, &mut self.solver_type);
                }
            });
    }

    fn add_generate_button(&mut self, ui: &mut egui::Ui) {
        // TODO: make this async / threaded and disable the button while generating

        if ui.button("Generate Maze").clicked() {
            info!("Generating {}x{} maze ...", self.width, self.height);

            let generator = self.generator_type.generator();
            let grid = {
                info!("Running maze generator {} ...", generator.name());

                let now = Instant::now();
                let grid = generator.generate(self.height, self.width);
                info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);

                grid
            };
            debug!("{:?}", grid);

            info!("Dead ends: {}", grid.get_dead_ends().len());

            let (root, goal) = {
                info!("Finding longest path ...");

                let now = Instant::now();
                let (root, goal) = grid.longest_path();
                info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);

                (root, goal)
            };

            let solver = self.solver_type.solver(grid, root.0, root.1);
            {
                info!(
                    "Running solver {} from {:?} to {:?} ...",
                    solver.name(),
                    root,
                    goal
                );

                let now = Instant::now();
                solver.solve(goal.0, goal.1);
                info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);
            }

            self.maze_ascii = solver.render_ascii();
            self.maze_renderable = Some(solver);
        }
    }

    fn add_save_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Save Maze").clicked() {
            /*let filename = ...;
            info!("Saving to {:?} ...", filename.as_ref());

            if let Err(err) = renderable.save_png(filename.as_ref(), 50) {
                error!("{}", err);
            }*/
        }
    }
}

impl epi::App for RunnerApp {
    fn name(&self) -> &str {
        "Maze Runner"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.add_generators_select(ui);
            self.add_solvers_select(ui);

            ui.add(egui::Slider::new(&mut self.width, 1..=100).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 1..=100).text("Height"));

            ui.horizontal(|ui| {
                self.add_generate_button(ui);
                self.add_save_button(ui);
            });

            ui.separator();

            // TODO: rendering as text is not what we ultimately want here
            // but for now, is there a way to disable vertical wrapping on it?
            egui::ScrollArea::both().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.maze_ascii.as_str())
                        .text_style(egui::TextStyle::Monospace),
                );
            });

            // TODO: fix the scroll area height and move the buttons down here
        });
    }
}
