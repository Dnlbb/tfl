use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mat2(pub [[u8; 2]; 2]);

impl Mat2 {
    pub const fn zero() -> Self { Mat2([[0, 0], [0, 0]]) }
    pub const fn id() -> Self { Mat2([[1, 0], [0, 1]]) }

    pub fn mul(self, other: Self) -> Self {
        let a = self.0;
        let b = other.0;
        let mut c = [[0u8; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                c[i][j] = (a[i][0] & b[0][j]) ^ (a[i][1] & b[1][j]);
            }
        }
        Mat2(c)
    }
}

pub fn phi_char(ch: char) -> Mat2 {
    match ch {
        'a' => Mat2::zero(),
        'b' => Mat2([[0, 1], [0, 0]]),
        'c' => Mat2([[0, 0], [1, 0]]),
        _ => panic!("unexpected symbol {ch}"),
    }
}

pub fn phi_word(w: &str) -> Mat2 {
    w.chars().fold(Mat2::id(), |acc, ch| acc.mul(phi_char(ch)))
}

fn fmt(m: Mat2) -> String {
    let a = m.0;
    format!("[[{},{}],[{},{}]]", a[0][0], a[0][1], a[1][0], a[1][1])
}

fn random_rewrite_step(rng: &mut StdRng, w: &str, rules: &[(&str, &str)]) -> Option<String> {
    let mut candidates = Vec::new();
    for (ri, (lhs, _)) in rules.iter().enumerate() {
        let mut start = 0;
        while let Some(pos) = w[start..].find(lhs) {
            candidates.push((ri, start + pos));
            start += pos + 1;
        }
    }
    if candidates.is_empty() { return None; }
    let (ri, pos) = candidates[rng.gen_range(0..candidates.len())];
    let (lhs, rhs) = rules[ri];
    let mut out = String::new();
    out.push_str(&w[..pos]);
    out.push_str(rhs);
    out.push_str(&w[pos + lhs.len()..]);
    Some(out)
}

fn random_trace_and_check(
    rules: &[(&str, &str)],
    seed: u64,
    samples: usize,
    max_steps: usize,
) -> bool {
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..samples {
        let len = rng.gen_range(1..=30);
        let mut w: String = (0..len)
            .map(|_| ['a','b','c'][rng.gen_range(0..3)])
            .collect();
        let phi0 = phi_word(&w);
        let steps = rng.gen_range(0..=max_steps);
        for _ in 0..steps {
            if let Some(next) = random_rewrite_step(&mut rng, &w, rules) {
                let p = phi_word(&next);
                if p != phi0 {
                    eprintln!("invariant fail: {w} → {next}, Φ0={} Φ1={}", fmt(phi0), fmt(p));
                    return false;
                }
                w = next;
            } else {
                break;
            }
        }
    }
    true
}

pub fn run_invariant_tests() {
    let base_rules = vec![
        ("cc", "aa"),
        ("aaa", "aa"),
        ("aba", "a"),
        ("abc", "aa"),
        ("acb", "abb"),
        ("baa", "aa"),
        ("caa", "aa"),
        ("cbc", "c"),
        ("aabc", "aa"),
        ("aacb", "aa"),
        ("abba", "aca"),
        ("caca", "cba"),
        ("aaaba", "bb"),
        ("ababaabcba", "aaba"),
    ];
    let extended_rules = {
        let mut r = base_rules.clone();
        r.extend([
            ("bba", "aa"),
            ("abb", "aa"),
            ("aaca", "aa"),
            ("bb", "aa"),
            ("aab", "aa"),
            ("aac", "aa"),
            ("aca", "aa"),
            ("cba", "aa"),
            ("ac", "aa"),
        ]);
        r
    };

    println!("Base system...");
    assert!(random_trace_and_check(&base_rules, 0xAAA0, 2000, 100));
    println!("Base system OK");

    println!("Extended system...");
    assert!(random_trace_and_check(&extended_rules, 0xBBB0, 2000, 100));
    println!("Extended system OK");
}
