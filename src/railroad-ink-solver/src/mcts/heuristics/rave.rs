#[derive(Debug, Clone)]
pub struct Value {
    pub visits: u128,
    pub total_score: i128,
    mean: f64,
}

impl Value {
    #[must_use]
    pub fn new(score: i32) -> Self {
        Self {
            visits: 1,
            total_score: i128::from(score),
            mean: f64::from(score),
        }
    }

    #[must_use]
    pub fn new_with_data(visits: u128, total_score: i128) -> Self {
        Self {
            visits,
            total_score,
            mean: total_score as f64 / visits as f64,
        }
    }

    pub fn update(&mut self, score: i32) {
        self.total_score += i128::from(score);
        self.visits += 1;
        self.mean = self.total_score as f64 / self.visits as f64;
    }

    #[must_use]
    pub fn get_mean(&self) -> f64 {
        self.mean
    }

    #[must_use]
    pub fn get_total(&self) -> i128 {
        self.total_score
    }

    #[must_use]
    pub fn get_visits(&self) -> u128 {
        self.visits
    }
    pub fn uniform(&mut self, weight: u128) {
        self.total_score = (self.mean * weight as f64) as i128;
        self.visits = weight;
    }
}

impl Default for Value {
    fn default() -> Self {
        Self {
            visits: 0,
            total_score: 0,
            mean: 0.,
        }
    }
}
