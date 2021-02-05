use crate::generators::normal::NormalGenerator;

pub struct UniformTranslator {
    last_sample: f64,                   // The last sampled value of the walk.

    generator: NormalGenerator,
}

impl UniformTranslator {
    pub fn new(
        generator: NormalGenerator,
    ) -> UniformTranslator {
        UniformTranslator {
            last_sample: 0.0,
            generator,
        }
    }

    pub fn next(
        &mut self,
        current_time: f64,
    ) -> Option<f64> {
        if current_time < 0.0 {
            return None;
        }

        // Get a sample form a normally distributed random walk.
        let sample =
            self.generator
                .next(
                    current_time,
                )?
                .abs();

        // Bound it so that it lies between zero and one.
        let sample =
            if (sample % 2.0) < 1.0 {
                sample - sample.floor()
            } else {
                1.0 - sample + sample.floor()
            };

        self.last_sample = sample;

        Some(sample)
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

        if target_value < 0.0 && target_value > 1.0 {
            return None;
        }

        let last_normal_sample =
            self.generator
                .last();

        let sample =
            if last_normal_sample != f64::MAX {
                let target_normal_value =
                    if (last_normal_sample % 2.0) < 1.0 {
                        last_normal_sample + (target_value - self.last_sample)
                    } else {
                        last_normal_sample - (target_value - self.last_sample)
                    };

                self.generator
                    .next_interpolant(
                        current_time,
                        target_time,
                        target_normal_value,
                    )?
                    .abs()
            } else {
                self.generator
                    .next(current_time)?
                    .abs()
            };

        let sample =
            if (sample % 2.0) < 1.0 {
                sample - sample.floor()
            } else {
                1.0 - sample + sample.floor()
            };

        self.last_sample = sample;

        return Some(sample);
    }
}
