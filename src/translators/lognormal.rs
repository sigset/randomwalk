use crate::generators::normal::NormalGenerator;

pub struct LogNormalTranslator {
    generator: NormalGenerator,
}

impl LogNormalTranslator {
    pub fn new(
        generator: NormalGenerator,
    ) -> LogNormalTranslator {
        LogNormalTranslator {
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

        self.generator
            .next(current_time)
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

        if target_value <= 0.0 {
            return None;
        }

        Some(
            self.generator
                .next_interpolant(
                    current_time,
                    target_time,
                    target_value.ln(),
                )?
                .exp()
        )
    }
}
