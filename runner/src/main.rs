mod options;

use std::time::Instant;

use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

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
    let mut grid = Grid::new(options.height, options.width);
    debug!("{:?}", grid);

    info!("Running maze generator: {:?} ...", options.generator);
    let generator = options.generator.generator();
    {
        let now = Instant::now();
        generator.generate(&mut grid);
        info!("{}ms", now.elapsed().as_secs_f64() * 1000.0);
    }
    debug!("{:?}", grid);

    println!();
    grid.render_ascii();
    println!();

    if let Some(filename) = options.filename {
        info!("Saving to {:?} ...", filename);

        grid.save_png(&filename, 50)?;
    }

    Ok(())
}
