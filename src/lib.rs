//! # Basic Stochastics
//!
//! `basic_stochastics` is a small collection of utilities to make performing basic stochastic
//! calculations more convenient.

// sigma to percent
pub const ONE_SIGMA: f64 = 1.0; // ~68,27%
pub const TWO_SIGMA: f64 = 2.0; // ~95.45%
pub const THREE_SIGMA: f64 = 3.0; // ~99.73%

// percent to sigma
pub const FIFTY_TO_SIGMA: f64 = 0.675;
pub const SIXTY_TO_SIGMA: f64 = 0.842;
pub const SEVENTY_TO_SIGMA: f64 = 1.036;
pub const EIGHTY_TO_SIGMA: f64 = 1.282;
pub const NINETY_TO_SIGMA: f64 = 1.645;
pub const NINETY_FIVE_TO_SIGMA: f64 = 1.960;
pub const NINETY_NINE_TO_SIGMA: f64 = 2.576;


/// You can either provide a specific sigma value or use one from the given constants.
/// Please see https://en.wikipedia.org/wiki/Normal_distribution and sigma rooms for further information.
///
/// # Examples
///
/// ```
/// let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];
///
/// assert_eq!(::matches_sigma_environment(&data, basic_stochastics::ONE_SIGMA, 3.4), true);
/// assert_eq!(::matches_sigma_environment(&data, basic_stochastics::ONE_SIGMA, 1.4), true);
/// assert_eq!(::matches_sigma_environment(&data, basic_stochastics::ONE_SIGMA, 5.0), false);
///
/// assert_eq!(::matches_sigma_environment(&data, basic_stochastics::TWO_SIGMA, 3.4), true);
/// assert_eq!(::matches_sigma_environment(&data, basic_stochastics::TWO_SIGMA, 5.0), false);
///
/// assert_eq!(::matches_sigma_environment(&data, 2.576, 3.4), true);
/// ```
pub fn matches_sigma_environment(data: &Vec<f64>, sigma: f64, to_check: f64) -> bool {
    let average = average(data);
    let sigma = empiric_deviation(data);

    ((average - sigma) < to_check) && (to_check < (average + sigma))
}

/// Calculates the average of the given vector.
///
/// # Examples
///
/// ```
/// let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];
///
/// assert_eq!(basic_stochastics::average(&data), 2.4);
/// ```
pub fn average(data: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in data {
        sum += *i;
    }

    return sum / data.len() as f64;
}


/// Calculates the variance of the given vector.
/// Please see https://en.wikipedia.org/wiki/Variance for further explanation.
///
/// # Examples
///
/// ```
/// let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];
///
/// assert_eq!(basic_stochastics::variance(&data), 1.04);
/// ```
pub fn variance(data: &Vec<f64>) -> f64 {
    let average = average(data);

    let mut variance = 0.0;
    for i in data {
        let difference: f64 = *i - average;
        variance += difference * difference;
    }

    return variance / data.len() as f64;
}

/// Calculates the square root of the variance, representing the scattering of the given data
///
/// # Examples
///
/// ```
///  let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];
///
///  assert_eq!(basic_stochastics::empiric_deviation(&data), 1.019803902718557)
/// ```
pub fn empiric_deviation(data: &Vec<f64>) -> f64 {
    return variance(data).sqrt();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_average() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];
        assert_eq!(::average(&data), 2.4);
    }

    #[test]
    fn test_variance() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];
        assert_eq!(::variance(&data), 1.04);
    }

    #[test]
    fn test_empiric_deviation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];
        assert_eq!(::empiric_deviation(&data), 1.019803902718557);
    }

    #[test]
    fn test_sigma_environment() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 2.0];

        assert_eq!(::matches_sigma_environment(&data, ::ONE_SIGMA, 3.4), true);
        assert_eq!(::matches_sigma_environment(&data, ::ONE_SIGMA, 1.4), true);
        assert_eq!(::matches_sigma_environment(&data, ::ONE_SIGMA, 5.0), false);

        assert_eq!(::matches_sigma_environment(&data, ::TWO_SIGMA, 3.4), true);
        assert_eq!(::matches_sigma_environment(&data, ::TWO_SIGMA, 5.0), false);
    }
}