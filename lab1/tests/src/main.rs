mod meta;

use rand::Rng;
use std::fs::{File};
use std::io::{BufWriter, Write};

#[derive(Clone)]
struct Rule {
    lhs: &'static str,
    rhs: &'static str,
}

static RULES: &[Rule] = &[
    Rule { lhs: "cc", rhs: "aa" },
    Rule { lhs: "aaa", rhs: "aa" },
    Rule { lhs: "aba", rhs: "a" },
    Rule { lhs: "abc", rhs: "aa" },
    Rule { lhs: "acb", rhs: "abb" },
    Rule { lhs: "baa", rhs: "aa" },
    Rule { lhs: "caa", rhs: "aa" },
    Rule { lhs: "cbc", rhs: "c" },
    Rule { lhs: "aabc", rhs: "aa" },
    Rule { lhs: "aacb", rhs: "aa" },
    Rule { lhs: "abba", rhs: "aca" },
    Rule { lhs: "caca", rhs: "cba" },
    Rule { lhs: "aaaba", rhs: "bb" },
    Rule { lhs: "ababaabcba", rhs: "aaba" },
    Rule { lhs: "cac", rhs: "" },
];

const CASES: usize = 5000;
const MIN_LEN: usize = 5;
const MAX_LEN: usize = 100;
const MAX_REWRITES: usize = 50;
const OUT_FILE: &str = "fuzz_log.tsv";

fn main() -> std::io::Result<()> {
    let file = File::create(OUT_FILE)?;
    let mut w = BufWriter::new(file);
    writeln!(
        w,
        "orig\ttransformed\tlcs\tsteps_total\tsteps_orig\tsteps_transformed\trewrites_applied"
    )?;

    let mut rng = rand::thread_rng();

    for _ in 0..CASES {
        let orig = random_word(&mut rng, MIN_LEN, MAX_LEN);
        let (transformed, rewrites_applied) =
            apply_random_rewrites(&orig, &mut rng, MAX_REWRITES);

        let lcs = lcs_string(&orig, &transformed);
        let steps_orig = orig.len() - lcs.len();
        let steps_transformed = transformed.len() - lcs.len();
        let steps_total = steps_orig + steps_transformed;

        writeln!(
            w,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            orig, transformed, lcs, steps_total, steps_orig, steps_transformed, rewrites_applied
        )?;
    }

    w.flush()?;
    eprintln!("Готово. Логи: {}", OUT_FILE);

    meta::run_invariant_tests();

    Ok(())
}

fn random_word(rng: &mut impl Rng, min_len: usize, max_len: usize) -> String {
    let len = rng.gen_range(min_len..=max_len);
    let alphabet = [b'a', b'b', b'c'];
    let mut s = Vec::with_capacity(len);
    for _ in 0..len {
        let ch = alphabet[rng.gen_range(0..alphabet.len())];
        s.push(ch);
    }
    String::from_utf8(s).unwrap()
}

fn apply_at(s: &str, pos: usize, lhs_len: usize, rhs: &str) -> String {
    let mut out = String::with_capacity(s.len() + rhs.len().saturating_sub(lhs_len));
    out.push_str(&s[..pos]);
    out.push_str(rhs);
    out.push_str(&s[pos + lhs_len..]);
    out
}

fn all_matches(s: &str, rules: &[Rule]) -> Vec<(usize, usize)> {
    let bytes = s.as_bytes();
    let mut res = Vec::new();
    for (ri, rule) in rules.iter().enumerate() {
        let pat = rule.lhs.as_bytes();
        if pat.is_empty() || bytes.len() < pat.len() {
            continue;
        }
        let last = bytes.len() - pat.len();
        let mut i = 0usize;
        while i <= last {
            if &bytes[i..i + pat.len()] == pat {
                res.push((ri, i));
                i += 1;
            } else {
                i += 1;
            }
        }
    }
    res
}

fn apply_random_rewrites(s0: &str, rng: &mut impl Rng, max_rewrites: usize) -> (String, usize) {
    let mut s = s0.to_string();
    let target_steps = rng.gen_range(0..=max_rewrites);
    let mut applied = 0usize;

    for _step in 0..target_steps {
        let matches = all_matches(&s, RULES);
        if matches.is_empty() {
            break;
        }
        let (ri, pos) = matches[rng.gen_range(0..matches.len())];
        let rule = &RULES[ri];
        s = apply_at(&s, pos, rule.lhs.len(), rule.rhs);
        applied += 1;
    }

    (s, applied)
}

fn lcs_string(a: &str, b: &str) -> String {
    let aa = a.as_bytes();
    let bb = b.as_bytes();
    let n = aa.len();
    let m = bb.len();

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if aa[i] == bb[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    let mut i = 0usize;
    let mut j = 0usize;
    let mut out = Vec::with_capacity(dp[0][0]);
    while i < n && j < m {
        if aa[i] == bb[j] {
            out.push(aa[i]);
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }

    String::from_utf8(out).unwrap()
}
