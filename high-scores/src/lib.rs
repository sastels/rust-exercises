use std::cmp;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        let last = *(self.scores.last().unwrap());
        Some(last)
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        let max = self.scores.iter().max();
        Some(*max.unwrap())
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_vec: Vec<u32> = self.scores.to_vec();
        scores_vec.sort();
        scores_vec.reverse();
        scores_vec[0..cmp::min(3, scores_vec.len())].to_vec()
    }
}
