mod options;

use std::path::Path;
use std::time::Instant;

use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use mazecore::solvers::Solver;

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

    info!("Processing {}x{} maze ...", options.width, options.height);

    let generator = options.generator.generator();
    let grid = {
        info!("Running maze generator {} ...", options.generator);

        let now = Instant::now();
        let grid = generator.generate(options.height, options.width);
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

    let solver = options.generator.solver_type().solver(grid, root.0, root.1);
    {
        info!(
            "Running solver {} from {:?} to {:?} ...",
            options.generator.solver_type(),
            root,
            goal
        );

        let now = Instant::now();
        solver.solve(goal.0, goal.1);
        info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);
    }

    render(&*solver, &options.filename)?;

    Ok(())
}
