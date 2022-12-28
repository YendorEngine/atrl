use yendor::prelude::Line;

use crate::{prelude::*, utilities::testing::types::*};

pub fn generate_noise(config: &NoiseConfig) -> Vec<IVec2> {
    let mut rng = Pcg64::from_entropy();
    // let perlin = noise::Perlin::new(rng.next_u32());
    let fbm: Fbm<Perlin> = noise::Fbm::new(rng.next_u32())
        .set_octaves(config.octaves)
        .set_frequency(config.frequency)
        // .set_lacunarity(config.lacunarity)
        .set_persistence(config.persistence);

    let mut stars = Vec::new();

    for y in config.bottom..config.top {
        for x in config.left..config.right {
            let fx = x as f64;
            let fy = y as f64;
            let value = fbm.get([fx, fy]);
            if value > config.cutoff {
                stars.push(IVec2 { x, y });
            }
        }
    }

    stars
}

pub fn generate_spiral(config: &SpiralConfig) -> Vec<IVec2> {
    let mut stars = Vec::new();

    for arm in 0..config.arm_count {
        // Re-order arms drawing:
        let arm = match arm {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 1,
            4 => 3,
            5 => 5,
            _ => unreachable!(),
        };

        let mut last = IVec2::ZERO;
        for i in (0..config.max_stars).step_by(6) {
            let i = i as f32 + arm as f32;
            let (x, y) = polar_to_cartesian(i, i);
            let next = IVec2::new(x.floor() as i32, y.floor() as i32);
            if config.draw_lines {
                for point in Line::new(last, next) {
                    stars.push(point);
                }
            } else {
                stars.push(next);
            }
            last = next;
        }
    }
    stars
}

pub fn generate_seed() -> u64 {
    // Generate a random seed
    let mut rng = Pcg64::from_entropy();
    rng.next_u64()
}
