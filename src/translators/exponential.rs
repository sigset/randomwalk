use crate::translators::uniform::UniformTranslator;

pub struct ExponentialTranslator {
    mean: f64,

    translator: UniformTranslator,
}

impl ExponentialTranslator {
    pub fn new(
        mean: f64,
        translator: UniformTranslator,
    ) -> Option<ExponentialTranslator> {
        if mean <= 0.0 {
            return None;
        }

        Some(
            ExponentialTranslator {
                mean,

                translator,
            },
        )
    }

    pub fn next(
        &mut self,
        time: f64,
    ) -> Option<f64> {
        Some(
            -self.mean
                * self.translator
                .next(
                    time,
                )?
                .ln()
        )
    }

    pub fn next_interpolant(
        &mut self,
        time: f64,
        target_time: f64,
        target_value: f64,
    ) -> Option<f64> {
        if time < 0.0 {
            return None;
        }

        if time > target_time {
            return None;
        }

        Some(
            -self.mean
                * self.translator
                .next_interpolant(
                    time,
                    target_time,
                    (-target_value / self.mean).exp(),
                )?
                .ln()
        )
    }
}
