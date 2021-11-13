use std::fmt;
use std::time::Instant;

use derivative::Derivative;
use derive_more::Display;
use eframe::{egui, epi};
use strum::{EnumIter, IntoEnumIterator};
use tracing::{debug, error, info};

use mazecore::generators::*;
use mazecore::solvers::*;
use mazecore::{Grid, Renderable};

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
}

impl GeneratorType {
    fn generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorType::BinaryTree => Box::new(BinaryTree::default()),
            GeneratorType::Sidewinder => Box::new(Sidewinder::default()),
            GeneratorType::AldousBroder => Box::new(AldousBroder::default()),
            GeneratorType::Wilsons => Box::new(Wilsons::default()),
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
    fn solver(&self, grid: Grid, root_row: usize, root_col: usize) -> Option<Box<dyn Solver>> {
        match self {
            SolverType::None => None,
            SolverType::Djikstra => Some(Box::new(Djikstra::new(grid, root_row, root_col))),
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
}

impl RunnerApp {
    pub fn add_selection<T>(generator_type: T, ui: &mut egui::Ui, selection: &mut T)
    where
        T: Copy + PartialEq + fmt::Display,
    {
        ui.selectable_value(selection, generator_type, format!("{}", generator_type));
    }

    fn add_generators(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Maze Generator")
            .selected_text(format!("{}", self.generator_type))
            .width(130.0)
            .show_ui(ui, |ui| {
                for generator_type in GeneratorType::iter() {
                    RunnerApp::add_selection(generator_type, ui, &mut self.generator_type);
                }
            });
    }

    fn add_solvers(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Maze Solver")
            .selected_text(format!("{}", self.solver_type))
            .width(130.0)
            .show_ui(ui, |ui| {
                for solver_type in SolverType::iter() {
                    RunnerApp::add_selection(solver_type, ui, &mut self.solver_type);
                }
            });
    }
}

impl epi::App for RunnerApp {
    fn name(&self) -> &str {
        "Maze Runner"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.add_generators(ui);
            self.add_solvers(ui);

            ui.add(egui::Slider::new(&mut self.width, 1..=100).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 1..=100).text("Height"));

            // TODO: the maze render should go here

            // TODO: disable while generating
            // TODO: do the generation async and render it over time
            if ui.button("Generate Maze").clicked() {
                // TODO: this should generate to a grid that we render
                info!("Processing {}x{} maze ...", self.width, self.height);

                let generator = self.generator_type.generator();
                let grid = {
                    info!("Running maze generator {} ...", self.generator_type);

                    let now = Instant::now();
                    let grid = generator.generate(self.height, self.width);
                    info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);

                    grid
                };
                debug!("{:?}", grid);

                let (root, goal) = {
                    info!("Finding longest path ...");

                    let now = Instant::now();
                    let (root, goal) = grid.longest_path();
                    info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);

                    (root, goal)
                };

                if self.solver_type != SolverType::None {
                    let solver = self.solver_type.solver(grid, root.0, root.1).unwrap();

                    {
                        info!(
                            "Running solver {} from {:?} to {:?} ...",
                            self.solver_type, root, goal
                        );

                        let now = Instant::now();
                        solver.solve(goal.0, goal.1);
                        info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);
                    }

                    if let Err(err) = render(&*solver) {
                        //, &options.filename) {
                        error!("{}", err);
                    }
                } else {
                    if let Err(err) = render(&grid) {
                        //, &options.filename) {
                        error!("{}", err);
                    }
                }
            }
        });
    }
}

fn render(
    renderable: &(impl Renderable + ?Sized),
    //filename: &Option<impl AsRef<Path>>,
) -> anyhow::Result<()> {
    println!();
    renderable.render_ascii();
    println!();

    /*if let Some(filename) = filename {
        info!("Saving to {:?} ...", filename.as_ref());

        renderable.save_png(filename.as_ref(), 50)?;
    }*/

    Ok(())
}
