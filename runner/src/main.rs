mod options;

use std::time::Instant;

use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use options::Options;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logging()?;

    let options: Options = argh::from_env();

    info!("Processing {}x{} maze ...", options.width, options.height);

    let generator = options.generator.generator();
    let grid = {
        info!("Running maze generator: {:?} ...", options.generator);

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

    let solver = options.generator.solver().solver(grid, root.0, root.1);
    {
        info!("Running solver: {:?} ...", options.generator.solver());

        let now = Instant::now();
        solver.solve(goal.0, goal.1);
        info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);
    }

    println!();
    solver.render_ascii();
    println!();

    if let Some(filename) = options.filename {
        info!("Saving to {:?} ...", filename);

        solver.save_png(&filename, 50)?;
    }

    Ok(())
}
