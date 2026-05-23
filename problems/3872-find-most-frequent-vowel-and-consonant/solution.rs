impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut freq = [0; 26];

        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let vowels = [0, 4, 8, 14, 20]; // a, e, i, o, u

        let mut max_vowel = 0;
        let mut max_consonant = 0;

        for i in 0..26 {
            if vowels.contains(&i) {
                max_vowel = max_vowel.max(freq[i]);
            } else {
                max_consonant = max_consonant.max(freq[i]);
            }
        }

        max_vowel + max_consonant
    }
}
