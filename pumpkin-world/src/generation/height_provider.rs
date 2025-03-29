use pumpkin_util::random::RandomGenerator;
use serde::Deserialize;

use super::y_offset::YOffset;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HeightProvider {
    Uniform(UniformHeightProvider),
    Trapezoid(TrapezoidHeightProvider),
}

impl HeightProvider {
    pub fn get(&self, random: &mut RandomGenerator, min_y: i8, height: u16) -> i32 {
        match self {
            HeightProvider::Uniform(uniform) => uniform.get(random, min_y, height),
            HeightProvider::Trapezoid(uniform) => uniform.get(random, min_y, height),
        }
    }
}

#[derive(Deserialize)]
pub struct UniformHeightProvider {
    min_inclusive: YOffset,
    max_inclusive: YOffset,
}

impl UniformHeightProvider {
    pub fn get(&self, random: &mut RandomGenerator, min_y: i8, height: u16) -> i32 {
        let min = self.min_inclusive.get_y(min_y, height) as i32;
        let max = self.max_inclusive.get_y(min_y, height) as i32;

        random.next_bounded_i32(max - min + 1) + min
    }
}

#[derive(Deserialize)]
pub struct TrapezoidHeightProvider {
    min_inclusive: YOffset,
    max_inclusive: YOffset,
    plateau: Option<i32>,
}

impl TrapezoidHeightProvider {
    pub fn get(&self, random: &mut RandomGenerator, min_y: i8, height: u16) -> i32 {
        let plateau = self.plateau.unwrap_or(0);
        let i = self.min_inclusive.get_y(min_y, height);
        let j = self.max_inclusive.get_y(min_y, height);

        if i > j {
            log::warn!("Empty height range");
            return i as i32;
        }

        let k = j - i;
        if plateau >= k as i32 {
            return random.next_inbetween_i32(i as i32, j as i32);
        }

        let l = (k as i32 - plateau) / 2;
        let m = k as i32 - l;
        return i as i32
            + random.next_inbetween_i32(0, m as i32)
            + random.next_inbetween_i32(0, l as i32);
    }
}
