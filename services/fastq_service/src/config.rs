/// Input arguments to fastq_rs.
pub struct FilterConfig {
    pub min_len: usize,
    pub max_len: usize,
    pub min_error: f64,
    pub max_error: f64,
    pub min_softmasked: usize,
    pub max_softmasked: usize,
    pub min_ambiguous: usize,
    pub max_ambiguous: usize,
}

impl FilterConfig {
    pub fn default() -> Self {
        Self {
            min_len: 0,
            max_len: usize::MAX,
            min_error: 0.0,
            max_error: 0.05,
            min_softmasked: 0,
            max_softmasked: usize::MAX,
            min_ambiguous: 0,
            max_ambiguous: usize::MAX,
        }
    }
}
