use fancy_regex::Regex;
use rand::Rng;
use std::process;

const MAX_ENUM_LEN: usize = 10;


const MAX_FUZZ_LEN: usize = 60;


const NUM_RANDOM_TESTS: usize = 10_000_000;


const N_NFA_STATES: usize = 26;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum NfaState {
    ALStar = 0,
    Mid,
    ALA,
    ALInAaa2,
    ALInBbb1,
    ALInBbb2,
    ALInCc1,
    ALAbLoop,
    ALAbCc1,
    ALAbCc2,
    BInA,
    BInAab2,
    BInB,
    BInB2,
    BInBbbb3,
    BAabLoop,
    BAabCc1,
    RStar,
    ARA,
    ARInAaa2,
    ARInBbb1,
    ARInBbb2,
    ARInCc1,
    ARAbLoop,
    ARAbCc1,
    ARAbCc2,
}


const NFA_STATES: [NfaState; N_NFA_STATES] = [
    NfaState::ALStar,
    NfaState::Mid,
    NfaState::ALA,
    NfaState::ALInAaa2,
    NfaState::ALInBbb1,
    NfaState::ALInBbb2,
    NfaState::ALInCc1,
    NfaState::ALAbLoop,
    NfaState::ALAbCc1,
    NfaState::ALAbCc2,
    NfaState::BInA,
    NfaState::BInAab2,
    NfaState::BInB,
    NfaState::BInB2,
    NfaState::BInBbbb3,
    NfaState::BAabLoop,
    NfaState::BAabCc1,
    NfaState::RStar,
    NfaState::ARA,
    NfaState::ARInAaa2,
    NfaState::ARInBbb1,
    NfaState::ARInBbb2,
    NfaState::ARInCc1,
    NfaState::ARAbLoop,
    NfaState::ARAbCc1,
    NfaState::ARAbCc2,
];

#[inline(always)]
const fn nfa_bit(s: NfaState) -> u64 {
    1u64 << (s as u8)
}


const NFA_EPS: [u64; N_NFA_STATES] = {
    let mut eps = [0u64; N_NFA_STATES];

    eps[NfaState::ALStar as usize] |= nfa_bit(NfaState::Mid);
    eps[NfaState::Mid as usize] |= nfa_bit(NfaState::RStar);
    eps[NfaState::ALAbLoop as usize] |= nfa_bit(NfaState::ALStar);
    eps[NfaState::BAabLoop as usize] |= nfa_bit(NfaState::Mid);
    eps[NfaState::ARAbLoop as usize] |= nfa_bit(NfaState::RStar);

    eps
};

const NFA_TRANS_A: [u64; N_NFA_STATES] = {
    let mut t = [0u64; N_NFA_STATES];

    t[NfaState::ALStar as usize] |= nfa_bit(NfaState::ALA);
    t[NfaState::Mid as usize] |= nfa_bit(NfaState::BInA);
    t[NfaState::ALA as usize] |= nfa_bit(NfaState::ALInAaa2);
    t[NfaState::ALA as usize] |= nfa_bit(NfaState::ALStar);
    t[NfaState::ALInAaa2 as usize] |= nfa_bit(NfaState::ALStar);
    t[NfaState::BInA as usize] |= nfa_bit(NfaState::Mid);
    t[NfaState::BInA as usize] |= nfa_bit(NfaState::BInAab2);
    t[NfaState::RStar as usize] |= nfa_bit(NfaState::ARA);
    t[NfaState::ARA as usize] |= nfa_bit(NfaState::ARInAaa2);
    t[NfaState::ARA as usize] |= nfa_bit(NfaState::RStar);
    t[NfaState::ARInAaa2 as usize] |= nfa_bit(NfaState::RStar);

    t
};

const NFA_TRANS_B: [u64; N_NFA_STATES] = {
    let mut t = [0u64; N_NFA_STATES];

    t[NfaState::ALStar as usize] |= nfa_bit(NfaState::ALInBbb1);
    t[NfaState::Mid as usize] |= nfa_bit(NfaState::BInB);
    t[NfaState::ALA as usize] |= nfa_bit(NfaState::ALAbLoop);
    t[NfaState::ALInBbb1 as usize] |= nfa_bit(NfaState::ALInBbb2);
    t[NfaState::ALInBbb2 as usize] |= nfa_bit(NfaState::ALStar);
    t[NfaState::BInAab2 as usize] |= nfa_bit(NfaState::BAabLoop);
    t[NfaState::BInB as usize] |= nfa_bit(NfaState::BInB2);
    t[NfaState::BInB2 as usize] |= nfa_bit(NfaState::Mid);
    t[NfaState::BInB2 as usize] |= nfa_bit(NfaState::BInBbbb3);
    t[NfaState::BInBbbb3 as usize] |= nfa_bit(NfaState::Mid);
    t[NfaState::RStar as usize] |= nfa_bit(NfaState::ARInBbb1);
    t[NfaState::ARA as usize] |= nfa_bit(NfaState::ARAbLoop);
    t[NfaState::ARInBbb1 as usize] |= nfa_bit(NfaState::ARInBbb2);
    t[NfaState::ARInBbb2 as usize] |= nfa_bit(NfaState::RStar);

    t
};

const NFA_TRANS_C: [u64; N_NFA_STATES] = {
    let mut t = [0u64; N_NFA_STATES];

    t[NfaState::ALStar as usize] |= nfa_bit(NfaState::ALInCc1);
    t[NfaState::ALInCc1 as usize] |= nfa_bit(NfaState::ALStar);
    t[NfaState::ALAbLoop as usize] |= nfa_bit(NfaState::ALAbCc1);
    t[NfaState::ALAbCc1 as usize] |= nfa_bit(NfaState::ALAbCc2);
    t[NfaState::ALAbCc2 as usize] |= nfa_bit(NfaState::ALAbLoop);
    t[NfaState::BAabLoop as usize] |= nfa_bit(NfaState::BAabCc1);
    t[NfaState::BAabCc1 as usize] |= nfa_bit(NfaState::BAabLoop);
    t[NfaState::RStar as usize] |= nfa_bit(NfaState::ARInCc1);
    t[NfaState::ARInCc1 as usize] |= nfa_bit(NfaState::RStar);
    t[NfaState::ARAbLoop as usize] |= nfa_bit(NfaState::ARAbCc1);
    t[NfaState::ARAbCc1 as usize] |= nfa_bit(NfaState::ARAbCc2);
    t[NfaState::ARAbCc2 as usize] |= nfa_bit(NfaState::ARAbLoop);

    t
};

fn epsilon_closure(mut states: u64, eps: &[u64; N_NFA_STATES]) -> u64 {
    loop {
        let mut added = 0u64;

        for &st in &NFA_STATES {
            let idx = st as usize;
            if (states >> idx) & 1 == 1 {
                added |= eps[idx];
            }
        }

        let new_bits = added & !states;
        if new_bits == 0 {
            return states;
        }
        states |= new_bits;
    }
}

fn nfa_accepts(word: &str) -> bool {
    let mut current = epsilon_closure(nfa_bit(NfaState::ALStar), &NFA_EPS);

    for b in word.bytes() {
        let mut next = 0u64;

        match b {
            b'a' => {
                for &st in &NFA_STATES {
                    let idx = st as usize;
                    if (current >> idx) & 1 == 1 {
                        next |= NFA_TRANS_A[idx];
                    }
                }
            }
            b'b' => {
                for &st in &NFA_STATES {
                    let idx = st as usize;
                    if (current >> idx) & 1 == 1 {
                        next |= NFA_TRANS_B[idx];
                    }
                }
            }
            b'c' => {
                for &st in &NFA_STATES {
                    let idx = st as usize;
                    if (current >> idx) & 1 == 1 {
                        next |= NFA_TRANS_C[idx];
                    }
                }
            }
            _ => return false,
        }

        if next == 0 {
            return false;
        }

        current = epsilon_closure(next, &NFA_EPS);
    }

    (current & nfa_bit(NfaState::RStar)) != 0
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum DfaState {
    Q0 = 0,
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
    Q6,
    Q7,
    Q8,
    Q9,
    Q10,
    Q11,
    Q12,
    Q13,
    Q14,
    Q15,
    Q16,
    Q17,
    Q18,
    Q19,
    Q20,
    Q21,
    Q22,
    Q23,
    Q24,
    Q25,
    Q26,
    Q27,
    Q28,
    Q29,
    Q30,
    Q31,
    Q32,
    Q33,
    Q34,
    Q35,
    Q36,
    Q37,
    Q38,
    Q39,
    Q40,
    Q41,
    Q42,
    Q43,
    Q44,
    Q45,
    Q46,
    Q47,
    Q48,
    Q49,
    Q50,
    Q51,
    Q52,
    Q53,
}

impl DfaState {
    #[inline(always)]
    fn idx(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn from_u8(v: u8) -> DfaState {
        match v {
            0 => DfaState::Q0,
            1 => DfaState::Q1,
            2 => DfaState::Q2,
            3 => DfaState::Q3,
            4 => DfaState::Q4,
            5 => DfaState::Q5,
            6 => DfaState::Q6,
            7 => DfaState::Q7,
            8 => DfaState::Q8,
            9 => DfaState::Q9,
            10 => DfaState::Q10,
            11 => DfaState::Q11,
            12 => DfaState::Q12,
            13 => DfaState::Q13,
            14 => DfaState::Q14,
            15 => DfaState::Q15,
            16 => DfaState::Q16,
            17 => DfaState::Q17,
            18 => DfaState::Q18,
            19 => DfaState::Q19,
            20 => DfaState::Q20,
            21 => DfaState::Q21,
            22 => DfaState::Q22,
            23 => DfaState::Q23,
            24 => DfaState::Q24,
            25 => DfaState::Q25,
            26 => DfaState::Q26,
            27 => DfaState::Q27,
            28 => DfaState::Q28,
            29 => DfaState::Q29,
            30 => DfaState::Q30,
            31 => DfaState::Q31,
            32 => DfaState::Q32,
            33 => DfaState::Q33,
            34 => DfaState::Q34,
            35 => DfaState::Q35,
            36 => DfaState::Q36,
            37 => DfaState::Q37,
            38 => DfaState::Q38,
            39 => DfaState::Q39,
            40 => DfaState::Q40,
            41 => DfaState::Q41,
            42 => DfaState::Q42,
            43 => DfaState::Q43,
            44 => DfaState::Q44,
            45 => DfaState::Q45,
            46 => DfaState::Q46,
            47 => DfaState::Q47,
            48 => DfaState::Q48,
            49 => DfaState::Q49,
            50 => DfaState::Q50,
            51 => DfaState::Q51,
            52 => DfaState::Q52,
            _ => DfaState::Q53,
        }
    }

    #[inline(always)]
    fn is_accepting(self) -> bool {
        DFA_ACCEPT[self.idx()]
    }

    #[inline(always)]
    fn step(self, b: u8) -> DfaState {
        let sym = match b {
            b'a' => 0,
            b'b' => 1,
            b'c' => 2,
            _ => return DfaState::Q53,
        };
        let next_idx = DFA_TRANS[self.idx()][sym] as u8;
        DfaState::from_u8(next_idx)
    }
}

const DFA_TRANS: [[usize; 3]; 54] = [
    // 0
    [19, 20, 21],
    // 1
    [28, 29, 27],
    // 2
    [28, 29, 2],
    // 3
    [3, 34, 27],
    // 4
    [19, 5, 21],
    // 5
    [25, 6, 27],
    // 6
    [25, 4, 27],
    // 7
    [25, 7, 27],
    // 8
    [11, 34, 27],
    // 9
    [25, 43, 10],
    // 10
    [28, 29, 9],
    // 11
    [8, 47, 27],
    // 12
    [19, 20, 12],
    // 13
    [13, 17, 21],
    // 14
    [19, 15, 21],
    // 15
    [19, 16, 21],
    // 16
    [25, 14, 27],
    // 17
    [19, 52, 18],
    // 18
    [53, 53, 12],
    // 19
    [49, 50, 53],
    // 20
    [53, 22, 53],
    // 21
    [53, 53, 0],
    // 22
    [53, 23, 53],
    // 23
    [19, 24, 21],
    // 24
    [25, 26, 27],
    // 25
    [38, 32, 53],
    // 26
    [53, 4, 53],
    // 27
    [53, 53, 1],
    // 28
    [31, 32, 53],
    // 29
    [53, 30, 53],
    // 30
    [53, 1, 53],
    // 31
    [3, 29, 27],
    // 32
    [28, 29, 33],
    // 33
    [53, 53, 2],
    // 34
    [28, 35, 33],
    // 35
    [53, 36, 53],
    // 36
    [28, 37, 27],
    // 37
    [28, 35, 27],
    // 38
    [8, 39, 27],
    // 39
    [25, 40, 41],
    // 40
    [53, 7, 53],
    // 41
    [53, 53, 42],
    // 42
    [25, 43, 41],
    // 43
    [53, 44, 53],
    // 44
    [53, 45, 53],
    // 45
    [25, 46, 27],
    // 46
    [25, 40, 27],
    // 47
    [25, 40, 48],
    // 48
    [53, 53, 9],
    // 49
    [13, 51, 21],
    // 50
    [19, 20, 18],
    // 51
    [25, 26, 41],
    // 52
    [53, 14, 53],
    // 53
    [53, 53, 53],
];

const DFA_ACCEPT: [bool; 54] = [
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true,
    false, false, false, false, false,
    true, true,
    false, false, false, false, false, false,
    true, true,
    false,
    true,
    false,
    true, true, true, true,
    false, false,
    true,
    false, false,
    true, true, true,
    false,
    true, true, true,
    false, false,
];

fn dfa_accepts(word: &str) -> bool {
    let mut state = DfaState::Q0;
    for b in word.bytes() {
        state = state.step(b);
    }
    state.is_accepting()
}









#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NoBcaState {
    Bc0,
    Bc1,
    Bc2,
    Dead,
}

impl NoBcaState {
    #[inline(always)]
    fn step(self, b: u8) -> Self {
        use NoBcaState::*;
        match (self, b) {
            (Bc0, b'b') => Bc1,
            (Bc0, b'a') | (Bc0, b'c') => Bc0,
            (Bc1, b'b') => Bc1,
            (Bc1, b'c') => Bc2,
            (Bc1, b'a') => Bc0,
            (Bc2, b'a') => Dead,
            (Bc2, b'b') => Bc1,
            (Bc2, b'c') => Bc0,
            (Dead, _) => Dead,
            _ => Dead,
        }
    }

    #[inline(always)]
    fn is_accepting(self) -> bool {
        !matches!(self, NoBcaState::Dead)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NoSingleCState {
    Cc0,
    Cc1,
    Cc2,
    Dead,
}

impl NoSingleCState {
    #[inline(always)]
    fn step(self, b: u8) -> Self {
        use NoSingleCState::*;
        match (self, b) {
            (Cc0, b'c') => Cc1,
            (Cc0, b'a') | (Cc0, b'b') => Cc0,

            (Cc1, b'c') => Cc2,
            (Cc1, b'a') | (Cc1, b'b') => Dead,

            (Cc2, b'c') => Cc2,
            (Cc2, b'a') | (Cc2, b'b') => Cc0,

            (Dead, _) => Dead,
            _ => Dead,
        }
    }

    #[inline(always)]
    fn is_accepting(self) -> bool {
        matches!(self, NoSingleCState::Cc0 | NoSingleCState::Cc2)
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum AbcInvState {
    Abc0,
    AbcAB,
    AbcNE,
    AbcC,
    Dead,
}

impl AbcInvState {
    #[inline(always)]
    fn step(self, b: u8) -> Self {
        use AbcInvState::*;
        match (self, b) {
            (Abc0, b'a') | (Abc0, b'b') => AbcAB,
            (Abc0, b'c') => AbcC,

            (AbcAB, b'a') | (AbcAB, b'b') => AbcAB,
            (AbcAB, b'c') => AbcNE,

            (AbcNE, b'c') => AbcC,
            (AbcNE, b'a') | (AbcNE, b'b') => Dead,

            (AbcC, b'c') => AbcC,
            (AbcC, b'a') | (AbcC, b'b') => AbcAB,

            (Dead, _) => Dead,
            _ => Dead,
        }
    }

    #[inline(always)]
    fn is_accepting(self) -> bool {
        matches!(self, AbcInvState::Abc0 | AbcInvState::AbcAB | AbcInvState::AbcC)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NoBBBlockLen2State {
    Bb0,
    Bb1,
    Bb2,
    Bb3,
    Dead,
}

impl NoBBBlockLen2State {
    #[inline(always)]
    fn step(self, b: u8) -> Self {
        use NoBBBlockLen2State::*;
        match (self, b) {
            (Bb0, b'b') => Bb1,
            (Bb0, b'a') | (Bb0, b'c') => Bb0,

            (Bb1, b'b') => Bb2,
            (Bb1, b'a') | (Bb1, b'c') => Bb0,

            (Bb2, b'b') => Bb3,
            (Bb2, b'a') | (Bb2, b'c') => Dead,

            (Bb3, b'b') => Bb3,
            (Bb3, b'a') | (Bb3, b'c') => Bb0,

            (Dead, _) => Dead,
            _ => Dead,
        }
    }

    #[inline(always)]
    fn is_accepting(self) -> bool {
        matches!(self, NoBBBlockLen2State::Bb0
            | NoBBBlockLen2State::Bb1
            | NoBBBlockLen2State::Bb3)
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NoCBlockAorBThenCState {
    Cb0,
    CbC,
    Cb1,
    Dead,
}

impl NoCBlockAorBThenCState {
    #[inline(always)]
    fn step(self, b: u8) -> Self {
        use NoCBlockAorBThenCState::*;
        match (self, b) {
            (Cb0, b'c') => CbC,
            (Cb0, b'a') | (Cb0, b'b') => Cb0,

            (CbC, b'c') => CbC,
            (CbC, b'a') | (CbC, b'b') => Cb1,

            (Cb1, b'c') => Dead,
            (Cb1, b'a') | (Cb1, b'b') => Cb0,

            (Dead, _) => Dead,
            _ => Dead,
        }
    }

    #[inline(always)]
    fn is_accepting(self) -> bool {
        !matches!(self, NoCBlockAorBThenCState::Dead)
    }
}

fn pka_accepts(word: &str) -> bool {
    let mut dfa = DfaState::Q0;
    let mut bc = NoBcaState::Bc0;
    let mut cc = NoSingleCState::Cc0;
    let mut abc = AbcInvState::Abc0;
    let mut bb = NoBBBlockLen2State::Bb0;
    let mut cb = NoCBlockAorBThenCState::Cb0;

    for b in word.bytes() {
        dfa = dfa.step(b);
        cc = cc.step(b);
        abc = abc.step(b);
        bb = bb.step(b);
        cb = cb.step(b);
        bc = bc.step(b);
    }

    dfa.is_accepting()
        && cc.is_accepting()
        && abc.is_accepting()
        && bb.is_accepting()
        && cb.is_accepting()
        && bc.is_accepting()
}


fn check_word(word: &str, re1: &Regex, re2: &Regex) {
    let c1 = re1.is_match(word).unwrap();
    let c2 = re2.is_match(word).unwrap();
    let c3 = nfa_accepts(word);
    let c4 = dfa_accepts(word);
    let c5 = pka_accepts(word);

    let all_true = c1 && c2 && c3 && c4 && c5;
    let all_false = !c1 && !c2 && !c3 && !c4 && !c5;

    if !(all_true || all_false) {
        eprintln!("================================");
        eprintln!("Найдено противоречие для слова: {:?}", word);
        eprintln!("  1) regex1  : {}", c1);
        eprintln!("  2) regex2  : {}", c2);
        eprintln!("  3) NFA     : {}", c3);
        eprintln!("  4) DFA     : {}", c4);
        eprintln!("  5) AFA     : {}", c5);
        eprintln!("================================");
        process::exit(1);
    }
}

fn check_all_words_up_to(max_len: usize, re1: &Regex, re2: &Regex) {
    let alphabet = [b'a', b'b', b'c'];

    check_word("", re1, re2);

    let mut buf = Vec::new();

    for len in 1..=max_len {
        let total = 3usize.pow(len as u32);
        buf.resize(len, b'a');

        for mut num in 0..total {
            for i in 0..len {
                let digit = (num % 3) as usize;
                buf[i] = alphabet[digit];
                num /= 3;
            }
            let word = std::str::from_utf8(&buf).unwrap();
            check_word(word, re1, re2);
        }
    }
}

fn fuzz_random(iterations: usize, max_len: usize, re1: &Regex, re2: &Regex) {
    let mut rng = rand::thread_rng();
    let alphabet = [b'a', b'b', b'c'];
    let mut buf = Vec::new();

    for i in 0..iterations {
        let len = rng.gen_range(0..=max_len);
        buf.clear();
        for _ in 0..len {
            let ch = alphabet[rng.gen_range(0..3)];
            buf.push(ch);
        }
        let word = std::str::from_utf8(&buf).unwrap();
        check_word(word, re1, re2);

        if (i + 1) % 100_000 == 0 {
            println!("  {} случайных слов проверено...", i + 1);
        }
    }
}


fn main() {
    let re1 = Regex::new(
        r"^(aaa|bbb|cc|ab(ccc)*|aa)*(aa|bbb|aab(cc)*|bbbb)*(aaa|bbb|cc|ab(ccc)*|aa)*$",
    )
        .expect("bad regex1");

    let re2 = Regex::new(
        r"^((bbb|cc|a((?=b)b(ccc)*|aa?))+)?((bbb((?=b)b)?|aa((?=b)b(cc)*)?)+)?((bbb|cc|a((?=b)b(ccc)*|aa?))+)?$",
    )
        .expect("bad regex2");

    println!("Полный перебор всех слов длины ≤ {}...", MAX_ENUM_LEN);
    check_all_words_up_to(MAX_ENUM_LEN, &re1, &re2);
    println!("OK для всех слов длины ≤ {}", MAX_ENUM_LEN);

    println!(
        "Запуск фазинга: {} случайных слов, макс. длина = {}...",
        NUM_RANDOM_TESTS, MAX_FUZZ_LEN
    );
    fuzz_random(NUM_RANDOM_TESTS, MAX_FUZZ_LEN, &re1, &re2);

    println!("Готово: контрпримеров не найдено.");
}
