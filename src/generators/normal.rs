use rand::{Rng, SeedableRng};

pub struct NormalGenerator {
    // The last sampled value of the walk.
    last_sample: f64,
    // The last sampled time.
    last_sample_time: f64,

    // The mean and variance of the soft bound.
    baseline_mean: f64,
    baseline_variance: f64,

    // The effective covariance of the walk.
    sigma_xx: f64,

    generator: rand_hc::Hc128Rng,
    norm_dist: rand_distr::Normal<f64>,
}

impl NormalGenerator {
    pub fn new(
        mean: f64,
        variance: f64,
        sigma: f64,
        seed: u64,
    ) -> NormalGenerator {
        let generator =
            rand_hc
            ::Hc128Rng
            ::seed_from_u64(
                seed,
            );

        let norm_dist =
            rand_distr::Normal::new(
                0.0,
                1.0,
            )
                .unwrap();

        NormalGenerator {
            last_sample: 0.0,
            last_sample_time: -1.0,

            baseline_mean: mean,
            baseline_variance: variance,

            sigma_xx: sigma,

            generator,
            norm_dist,
        }
    }

    pub fn set(
        &mut self,
        current_time: f64,
        current_value: f64,
    ) {
        self.last_sample_time = current_time;
        self.last_sample = current_value;
    }

    pub fn next(
        &mut self,
        current_time: f64,
    ) -> Option<f64> {
        if current_time < 0.0 {
            return None;
        }

        let sample =
            if self.last_sample_time != -1.0 {
                let time_step = (current_time - self.last_sample_time).abs() * self.sigma_xx;

                if time_step > 0.0 {
                    let mean =
                        (
                            self.last_sample / time_step
                                + self.baseline_mean / self.baseline_variance
                        ) / (
                            1.0 / time_step
                                + 1.0 / self.baseline_variance
                        );

                    let variance = 1.0 / (1.0 / time_step + 1.0 / self.baseline_variance);

                    self.generator
                        .sample(
                            self.norm_dist,
                        )
                        * variance.sqrt()
                        + mean
                } else {
                    self.last_sample
                }
            } else {
                self.generator.sample(
                    self.norm_dist,
                )
                    * self.baseline_variance.sqrt()
                    + self.baseline_mean
            };

        self.last_sample = sample;
        self.last_sample_time = current_time;

        return Some(sample);
    }

    pub fn next_interpolant(
        &mut self,
        current_time: f64,
        target_time: f64,
        target_value: f64,
    ) -> Option<f64> {
        if current_time < 0.0 {
            return None;
        }

        if current_time > target_time {
            return None;
        }

        let sample =
            if self.last_sample_time != -1.0 {
                let time_step = (current_time - self.last_sample_time).abs() * self.sigma_xx;

                if time_step > 0.0 {
                    let time_to_target = (target_time - current_time) * self.sigma_xx;

                    if time_to_target > 0.0 {
                        let mean =
                            (
                                self.last_sample / time_step
                                    + self.baseline_mean / self.baseline_variance
                                    + target_value / time_to_target
                            ) / (
                                1.0 / time_step
                                    + 1.0 / self.baseline_variance
                                    + 1.0 / time_to_target
                            );

                        let variance = 1.0 / (1.0 / time_step + 1.0 / self.baseline_variance + 1.0 / time_to_target);

                        self.generator
                            .sample(
                                self.norm_dist,
                            )
                            * variance.sqrt()
                            + mean
                    } else {
                        target_value
                    }
                } else {
                    self.last_sample
                }
            } else {
                let time_to_target = (target_time - current_time).abs() * self.sigma_xx;

                if time_to_target > 0.0 {
                    let mean =
                        (
                            self.baseline_mean / self.baseline_variance
                                + target_value / time_to_target
                        ) / (
                            1.0 / (
                                self.baseline_variance
                                    + 1.0 / time_to_target
                            )
                        );

                    let variance = 1.0 / (1.0 / self.baseline_variance + 1.0 / time_to_target);

                    self.generator
                        .sample(
                            self.norm_dist,
                        )
                        * variance.sqrt()
                        + mean
                } else {
                    target_value
                }
            };

        self.last_sample = sample;
        self.last_sample_time = current_time;

        return Some(sample);
    }

    pub fn last(
        &self,
    ) -> f64 {
        if self.last_sample_time != -1.0 {
            self.last_sample
        } else {
            f64::MAX
        }
    }
}
