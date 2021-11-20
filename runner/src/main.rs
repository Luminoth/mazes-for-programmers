mod analysis;
mod options;

use std::path::Path;
use std::time::Instant;

use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use mazecore::solvers::Solver;
use mazecore::{Grid, Mask};

use options::Options;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn render(solver: &dyn Solver, filename: &Option<impl AsRef<Path>>) -> anyhow::Result<()> {
    println!("\n{}\n", solver.render_ascii());

    if let Some(filename) = filename {
        info!("Saving to {:?} ...", filename.as_ref());

        solver.save_png(filename.as_ref(), 50)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logging()?;

    let options: Options = argh::from_env();

    if options.generator.is_analysis() {
        analysis::run(options.width, options.height, 100);
        return Ok(());
    }

    let generator = options.generator.generator();
    let grid = {
        info!(
            "Generating {}x{} maze (masked={}) ...",
            options.height,
            options.width,
            options.generator.use_mask()
        );

        let mut grid = if options.generator.use_mask() {
            let mut mask = Mask::new(options.height, options.width);
            mask.set(0, 0, false);
            mask.set(2, 2, false);
            mask.set(4, 4, false);
            Grid::from_mask(mask)
        } else {
            Grid::new(options.height, options.width)
        };

        info!("Running maze generator {} ...", generator.name());

        let now = Instant::now();
        generator.run(&mut grid);
        info!("{:.2}ms", now.elapsed().as_secs_f64() * 1000.0);

        grid
    };
    debug!("{:?}", grid);

    info!("Dead ends: {}", grid.get_dead_ends().len());

    let (root, goal) = {
        info!("Finding longest path ...");

        let now = Instant::now();
        let (root, goal) = grid.longest_path();
        info!("{:.2}ms", now.elapsed().as_secs_f64() * 1000.0);

        (root, goal)
    };

    let solver = options.generator.solver_type().solver(grid, root.0, root.1);
    {
        info!(
            "Running solver {} from {:?} to {:?} ...",
            solver.name(),
            root,
            goal
        );

        let now = Instant::now();
        solver.solve(goal.0, goal.1);
        info!("{:.2}ms", now.elapsed().as_secs_f64() * 1000.0);
    }

    if !options.norender {
        render(&*solver, &options.filename)?;
    }

    Ok(())
}
