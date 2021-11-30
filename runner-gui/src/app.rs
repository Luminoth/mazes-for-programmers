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

use crate::image::Image;
use crate::texture::Texture;

// TODO: all of this would be cleaner with macros

// TODO: masking

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Derivative, Display)]
#[derivative(Default)]
pub enum GeneratorType {
    #[display(fmt = "None")]
    None,

    #[derivative(Default)]
    #[display(fmt = "Binary Tree")]
    BinaryTree,

    #[display(fmt = "Binary Tree (Parallel)")]
    BinaryTreeParallel,

    #[display(fmt = "Sidewinder")]
    Sidewinder,

    #[display(fmt = "Sidewinder (Parallel)")]
    SidewinderParallel,

    #[display(fmt = "Aldous-Broder")]
    AldousBroder,

    #[display(fmt = "Wilson's Algorithm")]
    Wilsons,

    #[display(fmt = "Hunt-and-Kill")]
    HuntAndKill,

    #[display(fmt = "Recursive Backtracker")]
    RecursiveBacktracker,
}

impl GeneratorType {
    fn generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorType::None => Box::new(NoneGenerator::default()),
            GeneratorType::BinaryTree => Box::new(BinaryTree::default()),
            GeneratorType::BinaryTreeParallel => Box::new(BinaryTreeParallel::default()),
            GeneratorType::Sidewinder => Box::new(Sidewinder::default()),
            GeneratorType::SidewinderParallel => Box::new(SidewinderParallel::default()),
            GeneratorType::AldousBroder => Box::new(AldousBroder::default()),
            GeneratorType::Wilsons => Box::new(Wilsons::default()),
            GeneratorType::HuntAndKill => Box::new(HuntAndKill::default()),
            GeneratorType::RecursiveBacktracker => Box::new(RecursiveBacktracker::default()),
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

    #[derivative(Default(value = "false"))]
    polar: bool,

    generator_type: GeneratorType,
    solver_type: SolverType,

    maze_renderable: Option<Box<dyn Solver>>,
    dead_ends: usize,
    generate_time: f64,
    longest_path_time: f64,
    solve_time: f64,
    maze_texture: Texture,
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
            .width(150.0)
            .show_ui(ui, |ui| {
                for generator_type in GeneratorType::iter() {
                    RunnerApp::add_selection(generator_type, ui, &mut self.generator_type);
                }
            });
    }

    fn add_solvers_select(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Maze Solver")
            .selected_text(format!("{}", self.solver_type))
            .width(150.0)
            .show_ui(ui, |ui| {
                for solver_type in SolverType::iter() {
                    RunnerApp::add_selection(solver_type, ui, &mut self.solver_type);
                }
            });
    }

    fn add_generate_button(&mut self, ui: &mut egui::Ui, frame: &mut epi::Frame<'_>) {
        // TODO: make this async / threaded and disable the button while generating

        if ui.button("Generate Maze").clicked() {
            info!("Generating {}x{} maze ...", self.width, self.height);

            let generator = self.generator_type.generator();
            let grid = {
                info!("Running maze generator {} ...", generator.name());

                let now = Instant::now();
                let grid = generator.generate(self.height, self.width, self.polar);
                self.generate_time = now.elapsed().as_secs_f64() * 1000.0;

                grid
            };
            debug!("{:?}", grid);

            let (root, goal) = {
                info!("Finding longest path ...");

                let now = Instant::now();
                let (root, goal) = grid.longest_path();
                self.longest_path_time = now.elapsed().as_secs_f64() * 1000.0;

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
                self.solve_time = now.elapsed().as_secs_f64() * 1000.0;
            }

            //println!("\n{}\n", solver.render_ascii());

            // render the maze texture
            let (size, pixels) = solver.render(25, true);
            let image = Image::from_pixels(size, pixels);
            self.maze_texture.load(frame, &image);

            self.dead_ends = solver.grid().get_dead_ends().len();
            self.maze_renderable = Some(solver);
        }
    }

    fn add_save_button(&self, ui: &mut egui::Ui) {
        if ui.button("Save Maze").clicked() {
            /*let filename = ...;
            info!("Saving to {:?} ...", filename.as_ref());

            if let Err(err) = renderable.save_png(filename.as_ref(), 50) {
                error!("{}", err);
            }*/
        }
    }

    fn add_stats(&self, ui: &mut egui::Ui) {
        ui.label(format!("Dead ends: {}", self.dead_ends));
        ui.label(format!("Generate time: {:.2}ms", self.generate_time));
        ui.label(format!(
            "Longest path time: {:.2}ms",
            self.longest_path_time
        ));
        ui.label(format!("Solve time: {:.2}ms", self.solve_time));
    }

    fn add_maze(&self, ui: &mut egui::Ui, texture_id: egui::TextureId) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.image(texture_id, self.maze_texture.size());
        });
    }
}

impl epi::App for RunnerApp {
    fn name(&self) -> &str {
        "Maze Runner"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.add_generators_select(ui);
            self.add_solvers_select(ui);

            ui.add(egui::Slider::new(&mut self.width, 1..=500).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 1..=500).text("Height"));
            ui.checkbox(&mut self.polar, "Polar");

            ui.horizontal(|ui| {
                self.add_generate_button(ui, frame);

                ui.set_enabled(self.maze_texture.id().is_some());
                self.add_save_button(ui);
                ui.set_enabled(true);
            });

            ui.separator();

            if let Some(texture_id) = self.maze_texture.id() {
                self.add_stats(ui);
                self.add_maze(ui, texture_id);
            } else {
                ui.label("Generate a maze!");
            }

            // TODO: fix the maze scroll area height and move the buttons down here
        });
    }
}
