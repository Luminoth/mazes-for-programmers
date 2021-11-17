use std::collections::HashMap;

use tracing::info;

use mazecore::generators::*;

pub fn run(rows: usize, cols: usize, tries: usize) {
    let mut generators: Vec<Box<dyn Generator>> = vec![
        Box::new(BinaryTree::default()),
        Box::new(Sidewinder::default()),
        Box::new(AldousBroder::default()),
        Box::new(Wilsons::default()),
        Box::new(HuntAndKill::default()),
        Box::new(RecursiveBacktracker::default()),
    ];

    let mut averages = HashMap::new();
    for generator in &generators {
        info!("Running generator {} ...", generator.name());

        let mut deadend_counts = Vec::new();
        for _ in 0..tries {
            let grid = generator.generate(rows, cols);
            deadend_counts.push(grid.get_dead_ends().len());
        }

        let total_deadends: usize = deadend_counts.iter().sum();
        averages.insert(
            generator.name().to_string(),
            total_deadends as f32 / deadend_counts.len() as f32,
        );
    }

    println!();

    let size = rows * cols;
    info!(
        "Average dead-ends per {}x{} maze ({} cells):",
        rows, cols, size
    );

    println!();

    generators.sort_by(|x, y| averages[y.name()].partial_cmp(&averages[x.name()]).unwrap());

    for generator in generators {
        let average = averages[generator.name()];
        let percentage = average * 100.0 / size as f32;
        info!(
            "{:22}: {:3} / {} ({}%)",
            generator.name(),
            average as usize,
            size,
            percentage as usize
        );
    }
}
