const EPS: f64 = 1e-13;

pub fn series(a: f64, b: f64, c: f64, x: f64) -> f64 {
    let mut i: usize = 0;
    let mut umax: f64 = 0.0;
    let mut s: f64 = 1.0;
    let mut u: f64 = 1.0;
    let mut k: f64 = 0.0;
    let mut m: f64;
    loop {
        if c.abs() < EPS {
            return f64::INFINITY;
        }
        m = k + 1.0;
        u = u * ((a + k) * (b + k) * x / ((c + k) * m));
        let old_s = s;
        s += u;
        if s == old_s {
            return s;
        }
        k = u.abs();
        if k > umax {
            umax = k;
        }
        k = m;
        i += 1;
        if i > 10_000 {
            return f64::NAN;
        }
    }
}

pub fn abr_ste_15_3_6(a: f64, b: f64, c: f64, x: f64) -> f64 {
    let s = 1.0 - x;
    let d = c - a - b;
    let gam_c = c.gamma();
    let c1 = d.gamma() / ((c - a).gamma() * (c - b).gamma()) * series(a, b, 1.0 - d, s);
    let c2 = (-d).gamma() / (a.gamma() * b.gamma()) * series(c - a, c - b, 1.0 + d, s);
    return gam_c * (c1 + s.powf(d) * c2);
}

#[rustfmt::skip]
fn thresh_factor(mut x: f64) -> f64 {
    let n = 150.0;
    let KNOTS = [-0.5, -0.25, -0.1, -0.05, -0.01, -0.001, 0.001, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75];
    let COEFFS = [
        [-9.47988380e-03, -2.60776301e-03,  0.00000000e+00, 3.62437577e-02],
        [-4.85256204e-02, -1.68275887e-02, -9.71767586e-03, 3.44068315e-02],
        [ 2.08380754e-02, -5.32218040e-02, -2.72300240e-02, 3.01674754e-02],
        [-2.94770312e+00, -4.38446701e-02, -4.17899952e-02, 2.49558097e-02],
        [ 1.36699272e+01, -4.86000138e-01, -6.82822355e-02, 2.23882354e-02],
        [-4.76488782e+02,  1.15439112e+00, -4.15465961e-02, 1.97542211e-02],
        [ 2.47957217e+03, -1.17108060e+01, -1.36554330e-01, 1.91264471e-02],
        [-5.61001600e+01,  3.16662699e+00, -1.53642688e-01, 1.88263318e-02],
        [-1.39066435e+01,  1.65192268e+00, -1.10275741e-01, 1.76591474e-02],
        [ 8.64417308e-01, -1.68745408e-02, -4.48738156e-02, 1.50011688e-02],
        [-2.09720279e-01,  1.12788055e-01, -4.00781398e-02, 1.28233439e-02],
        [-9.05631357e-03,  1.84139297e-02, -2.03978421e-02, 8.64154818e-03],
        [-8.99931062e-03,  1.16216946e-02, -1.28889360e-02, 4.55145337e-03],
        [-1.79900266e-03,  4.87221158e-03, -8.76545946e-03, 1.91496106e-03],
    ];

    if x < -n / 2.0 {
        x = -n - x;
    }
    let mut z = x / (x + n);

    let mut coeffs = COEFFS[COEFFS.len() - 1];
    for i in 0..KNOTS.len() {
        if z > KNOTS[i] {
            z = z - KNOTS[i];
            coeffs = COEFFS[i + 1];
            break;
        }
    }

    let z2 = z * z;
    let z3 = z * z2;
    coeffs[0] * z3 + coeffs[1] * z2 + coeffs[2] * z + coeffs[3]
}

fn threshold(a: f64, b: f64, c: f64) -> f64 {
    let coeff = 46.9583277545962;
    let fa = thresh_factor(a);
    let fb = thresh_factor(b);
    let fc = thresh_factor(c);
    coeff * fa * fb / fc - 0.001
}

pub fn hyp2f1(a: f64, b: f64, c: f64, x: f64) -> f64 {
    if x > threshold(a, b, c) {
        abr_ste_15_3_6(a, b, c, x)
    } else {
        series(a, b, c, x)
    }
}
