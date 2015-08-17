/// Implementation of the Simplex Noise algorithm to generate a Noise pattern
pub struct SimplexNoise {
    /// The P Vector to use for this instance of the Simplex Noise Algorithm
    p: Vec<u8>,
}

impl SimplexNoise {
    pub fn new() -> SimplexNoise {
        let p = (0u16..256u16)
            .map(|x| x as u8)
            .collect::<Vec<u8>>();
        SimplexNoise { p: p }
    }

    /// Generate a Noise function for a given x- and y-ordinate
    pub fn noise(&self, x: f32, y: f32) -> f32 {
        0f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_simplex_noise() {
        let sn = SimplexNoise::new();
        let p = sn.p;
        assert_eq!(256, p.len());
    }
}
