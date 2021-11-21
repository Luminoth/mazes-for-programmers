use bit_vec::BitVec;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Mask {
    pub(crate) rows: usize,
    pub(crate) cols: usize,

    bits: BitVec,
}

impl Mask {
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0 && cols > 0);

        let size = rows * cols;
        Self {
            rows,
            cols,

            bits: BitVec::from_elem(size, true),
        }
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        let index = row * self.cols + col;
        self.bits.get(index).unwrap()
    }

    pub fn set(&mut self, row: usize, col: usize, v: bool) {
        let index = row * self.cols + col;
        self.bits.set(index, v);
    }

    pub fn count(&self) -> usize {
        self.bits.iter().filter(|x| *x).count()
    }

    /// Returns a random enabled cell
    pub fn random(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();

        // TODO: this could be smarter and avoid looping
        loop {
            let row = rng.gen_range(0..self.rows);
            let col = rng.gen_range(0..self.cols);

            if self.get(row, col) {
                return (row, col);
            }
        }
    }
}
