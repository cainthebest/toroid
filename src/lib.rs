#![no_std]

use {
    core::f32::consts::TAU,
    libm::{cosf, sinf},
};

/// A configurable "Donut" that can render ASCII frames without `std`.
///
/// The donut is rendered by sampling points on a torus surface using two angles,
/// and projecting those points into 2D screen space. The brightness of each
/// point is determined by the surface's orientation relative to a light source.
pub struct Donut<
    const WIDTH: u8 = 80,
    const HEIGHT: u8 = 22,
    //
    const J_STEP_VALUE: u8 = 7,
    const J_STEP_DENOM: u8 = 100,
    const I_STEP_VALUE: u8 = 2,
    const I_STEP_DENOM: u8 = 100,
    //
    const CHAR_BRIGHTNESS_0: char = ' ',
    const CHAR_BRIGHTNESS_1: char = '.',
    const CHAR_BRIGHTNESS_2: char = ',',
    const CHAR_BRIGHTNESS_3: char = '-',
    const CHAR_BRIGHTNESS_4: char = '~',
    const CHAR_BRIGHTNESS_5: char = ':',
    const CHAR_BRIGHTNESS_6: char = ';',
    const CHAR_BRIGHTNESS_7: char = '=',
    const CHAR_BRIGHTNESS_8: char = '!',
    const CHAR_BRIGHTNESS_9: char = '*',
    const CHAR_BRIGHTNESS_10: char = '#',
    const CHAR_BRIGHTNESS_11: char = '$',
    const CHAR_BRIGHTNESS_12: char = '@',
> {
    /// Rotation angle A (around the vertical axis)
    pub a: f32,
    /// Rotation angle B (around the horizontal axis)
    pub b: f32,
}

impl<
    const WIDTH: u8,
    const HEIGHT: u8,
    //
    const J_STEP_VALUE: u8,
    const J_STEP_DENOM: u8,
    const I_STEP_VALUE: u8,
    const I_STEP_DENOM: u8,
    //
    const C0: char,
    const C1: char,
    const C2: char,
    const C3: char,
    const C4: char,
    const C5: char,
    const C6: char,
    const C7: char,
    const C8: char,
    const C9: char,
    const C10: char,
    const C11: char,
    const C12: char,
>
    Donut<
        WIDTH,
        HEIGHT,
        J_STEP_VALUE,
        J_STEP_DENOM,
        I_STEP_VALUE,
        I_STEP_DENOM,
        C0,
        C1,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        C10,
        C11,
        C12,
    >
{
    /// Step size for the outer loop (angle j) over the donut's circular cross section.
    const J_STEP: f32 = match J_STEP_DENOM {
        0 | 1 => J_STEP_VALUE as f32,
        _ => (J_STEP_VALUE as f32) / (J_STEP_DENOM as f32),
    };

    /// Step size for the inner loop (angle i) over the donut's circular tube.
    const I_STEP: f32 = match I_STEP_DENOM {
        0 | 1 => I_STEP_VALUE as f32,
        _ => (I_STEP_VALUE as f32) / (I_STEP_DENOM as f32),
    };

    /// Brightness ramp used to select an ASCII character based on lighting.
    const BRIGHTNESS_RAMP: [char; 13] = [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12];

    /// Create a new donut with initial rotation angles set to 0.0.
    pub const fn new() -> Self {
        Self { a: 0.0, b: 0.0 }
    }

    /// Increment the rotation angles by `da` and `db`.
    ///
    /// Rotating the donut creates the animation effect.
    pub fn rotate(&mut self, da: f32, db: f32) {
        self.a += da;
        self.b += db;
    }

    /// **Render** one ASCII frame **in-place**:
    /// - output should be (WIDTH*HEIGHT) in length, for storing characters.
    /// - zbuf   should also be (WIDTH*HEIGHT) in length, for storing depth.
    ///
    /// All these slots will be overwritten with the new frame's data.  
    ///
    /// You can then print the output slice as lines of WIDTH characters.
    pub fn render_frame_in_place(&self, output: &mut [char], zbuf: &mut [f32]) {
        output.fill(C0);
        zbuf.fill(0.0);

        let (sa, ca) = (sinf(self.a), cosf(self.a));
        let (sb, cb) = (sinf(self.b), cosf(self.b));

        let mut j = 0.0;
        while j < TAU {
            let (u, v) = (sinf(j), cosf(j));

            let mut i = 0.0;
            while i < TAU {
                let (w, c) = (sinf(i), cosf(i));

                let h = v + 2.0;

                let d = 1.0 / (w * h * sa + u * ca + 5.0);
                let t = w * h * ca - u * sa;

                let x = (40.0 + 30.0 * d * (c * h * cb - t * sb)) as isize;
                let y = (12.0 + 15.0 * d * (c * h * sb + t * cb)) as isize;

                if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
                    let idx = (y * (WIDTH as isize) + x) as usize;

                    if d > zbuf[idx] {
                        zbuf[idx] = d;

                        let n = (8.0
                            * ((u * sa - w * v * ca) * cb - w * v * sa - u * ca - c * v * sb))
                            as isize;

                        output[idx] = Self::BRIGHTNESS_RAMP[n.clamp(0, 12) as usize];
                    }
                }

                i += Self::I_STEP;
            }

            j += Self::J_STEP;
        }
    }
}

impl Default for Donut {
    fn default() -> Self {
        Self::new()
    }
}
