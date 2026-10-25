//! Distinct source-global lifetimes held by the existing process session lock.
pub(crate) struct SourceState {
    pub random: crate::read_ephemeris::RandomState,
    pub max_observations: i32,
    pub dmmodel_warning: i32,
    pub tim_info: i32,
    pub ecliptic_obliquity: f64,
    pub very_fast: i32,
    pub force_global_fit: i32,
}
impl Default for SourceState {
    fn default() -> Self {
        Self {
            random: Default::default(),
            max_observations: 20000,
            dmmodel_warning: 0xffff,
            tim_info: -1,
            ecliptic_obliquity: 84381.4059,
            very_fast: 0,
            force_global_fit: 0,
        }
    }
}
