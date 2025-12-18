use std::cmp::min;

fn compute_last_occurrence(p: &[u8]) -> [isize; 256] {
    let mut last_occurence = [-1; 256];

    for (k, &byte) in p.iter().enumerate() {
        last_occurence[byte as usize] = k as isize;
    }

    last_occurence
}

pub fn boyer_moore_contains(t: &[u8], p: &[u8]) -> bool {
    let n = t.len();
    let m = p.len();

    if m == 0 {
        return true;
    }
    if n < m {
        return false;
    }

    let l_table = compute_last_occurrence(p);

    let mut i = m - 1;
    let mut j = m - 1;

    while i < n {
        if t[i] == p[j] {
            if j == 0 {
                return true;
            } else {
                i -= 1;
                j -= 1;
            }
        } else {
            let l = l_table[t[i] as usize];

            let shift = m as isize - min(j as isize, 1 + l);

            i += shift as usize;

            j = m - 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_occurrence_basic() {
        let pattern = b"ABC";
        // Wir rufen deine Funktion auf
        let table = compute_last_occurrence(pattern);

        // Prüfung 1: Buchstaben im Pattern
        assert_eq!(table[b'A' as usize], 0, "A sollte an Index 0 sein");
        assert_eq!(table[b'B' as usize], 1, "B sollte an Index 1 sein");
        assert_eq!(table[b'C' as usize], 2, "C sollte an Index 2 sein");

        // Prüfung 2: Buchstabe NICHT im Pattern
        assert_eq!(table[b'X' as usize], -1, "Nicht vorhandene sollten -1 sein");
    }

    #[test]
    fn test_last_occurrence_duplicates() {
        // Das wichtigste Szenario: Wiederholungen
        let pattern = b"ANANAS";
        // Indices: A=0, N=1, A=2, N=3, A=4, S=5

        let table = compute_last_occurrence(pattern);

        // Das letzte 'A' steht an Index 4 (nicht 0 oder 2!)
        assert_eq!(
            table[b'A' as usize], 4,
            "Sollte den letzen Index von A speichern"
        );

        // Das letzte 'N' steht an Index 3
        assert_eq!(
            table[b'N' as usize], 3,
            "Sollte den letzen Index von N speichern"
        );

        // 'S' steht an Index 5
        assert_eq!(table[b'S' as usize], 5);
    }
}
