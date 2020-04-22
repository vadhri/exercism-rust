#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    scores_sorted: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut scores_vector = scores.to_vec();
        let mut sorted_scores = scores_vector.clone();
        sorted_scores.sort();
        sorted_scores.reverse();

        HighScores {
            scores: scores_vector,
            scores_sorted: sorted_scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_sorted.first().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let max_scores = self.scores_sorted.len();
        match max_scores {
            x if x >= 3 => self.scores_sorted[0..=2].to_vec(),
            _ => self.scores_sorted[0..max_scores].to_vec()
        }
    }
}
