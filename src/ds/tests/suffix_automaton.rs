use std::{collections::HashSet, ops::Range};

use crate::{
    ds::suffix_automaton::LowerCaseSuffixAutomaton, types::str::Str, utils::collections::IntoVecExt,
};

#[test]
fn contains() {
    let alphabet = b'a'..b'd';
    for s in generate_all_strings(alphabet.clone(), 5) {
        check_contains(s, alphabet.clone());
    }
}

fn check_contains(s: Str, alphabet: Range<u8>) {
    let mut automaton = LowerCaseSuffixAutomaton::new();
    automaton.add_all(s.iter().cloned());
    let substrings = generate_all_substrings(&s);
    for cand in generate_all_strings(alphabet.clone(), s.len()) {
        let expected = substrings.contains(&cand);
        let actual = automaton.contains(cand.iter().cloned());
        assert_eq!(actual, expected, "'{cand}' in '{s}'")
    }
}

fn generate_all_substrings(s: &Str) -> HashSet<Str> {
    (0..s.len())
        .flat_map(|i| {
            (i..s.len())
                .map(|j| Str((i..=j).map(|k| s[k]).collect()))
                .into_vec()
        })
        .chain(std::iter::once(Str::empty()))
        .collect()
}

fn generate_all_strings(alphabet: Range<u8>, max_len: usize) -> Vec<Str> {
    let mut state = vec![Str::empty()];
    for i in 0.. {
        let cur = state[i].clone();
        if cur.len() == max_len {
            break;
        }
        for c in alphabet.clone() {
            let mut nxt = cur.clone();
            nxt.push(c);
            state.push(nxt);
        }
    }
    state
}
