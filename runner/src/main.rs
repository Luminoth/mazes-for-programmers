mod options;

use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use core::algorithms::Algorithm;
use core::Grid;

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

    info!("Generating {}x{} grid ...", options.width, options.height);
    let mut grid = Grid::new(options.width, options.height);
    debug!("{:?}\n", grid);

    info!("Running algorithm: {:?} ...", options.algorithm);
    let algorithm = options.algorithm.algorithm();
    algorithm.run(&mut grid);
    debug!("{:?}\n", grid);

    println!();
    grid.render_ascii();
    println!();

    Ok(())
}
