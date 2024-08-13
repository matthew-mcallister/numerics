use std::time::Instant;
use std::{error::Error, fs::File};

use npyz::{WriteOptions, WriterBuilder};
use numerics::hyp2f1::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug)]
struct FRange(f64, f64, usize);

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.2 == 0 {
            return None;
        }
        let val = self.0;
        self.2 -= 1;
        self.0 += self.1;
        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.2, Some(self.2))
    }
}

impl ExactSizeIterator for FRange {}

fn linspace(start: f64, stop: f64, n: usize) -> FRange {
    if n < 2 {
        FRange(start, 0f64, n)
    } else {
        FRange(start, (stop - start) / ((n - 1) as f64), n)
    }
}

fn measure_series(a: f64, b: f64, c: f64, x: f64) -> f64 {
    let start = Instant::now();
    let n = 30;
    for _ in 0..n {
        std::hint::black_box(series(a, b, c, x));
    }
    start.elapsed().as_secs_f64() / n as f64
}

fn measure_abr_ste(a: f64, b: f64, c: f64, x: f64) -> f64 {
    let start = Instant::now();
    let n = 30;
    for _ in 0..n {
        std::hint::black_box(abr_ste_15_3_6(a, b, c, x));
    }
    start.elapsed().as_secs_f64() / n as f64
}

fn measure_hyp2f1(a: f64, b: f64, c: f64, x: f64) -> f64 {
    let start = Instant::now();
    let n = 30;
    for _ in 0..n {
        std::hint::black_box(hyp2f1(a, b, c, x));
    }
    start.elapsed().as_secs_f64() / n as f64
}

fn measure(target: &impl Fn(f64, f64, f64, f64) -> f64, path: &str) -> Result<()> {
    let n_cs = 21;
    let cs = linspace(-0.25, 1.95, n_cs);
    let n_xs = 40;
    let xs = linspace(0.0, 0.9999, n_xs);
    let mut data = Vec::with_capacity(n_cs * n_cs * n_cs * n_xs);

    for q_a in cs {
        for q_b in cs {
            for q_c in cs {
                for x in xs {
                    let a = 10f64.powf(q_a);
                    let b = 10f64.powf(q_b);
                    let c = 10f64.powf(q_c);
                    data.push(target(a, b, c, x));
                }
            }
        }
    }

    let file = File::create(path)?;
    let mut writer = WriteOptions::<f64>::new()
        .default_dtype()
        .writer(file)
        .shape(&[n_cs as _, n_cs as _, n_cs as _, n_xs as _])
        .begin_nd()?;

    writer.extend(data.iter().cloned())?;
    writer.finish()?;

    Ok(())
}

fn main() {
    let t1 = std::thread::spawn(|| {
        measure(&measure_series, "/home/mattmcal/pastes/t0.npy").unwrap();
    });
    let t2 = std::thread::spawn(|| {
        measure(&measure_abr_ste, "/home/mattmcal/pastes/t1.npy").unwrap();
    });
    let t3 = std::thread::spawn(|| {
        measure(&measure_hyp2f1, "/home/mattmcal/pastes/t2.npy").unwrap();
    });
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
