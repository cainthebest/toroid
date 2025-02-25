#![no_std]

use core::f32::consts::TAU;

/// A configurable "Donut" that can render ASCII frames without `std`.
///
/// The donut is rendered by sampling points on a torus surface using two angles,
/// and projecting those points into 2D screen space. The brightness of each
/// point is determined by the surface's orientation relative to a light source.
pub struct Donut<
    const WIDTH: u8 = 80,
    const HEIGHT: u8 = 22,
    //
    const VIEWER_DISTANCE: u8 = 5,
    const BRIGHTNESS_FACTOR: u8 = 8,
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
    // Rotation angle A (vertical axis)
    pub a_cos: f32,
    pub a_sin: f32,

    // Rotation angle B (horizontal axis)
    pub b_cos: f32,
    pub b_sin: f32,
}

impl<
    const WIDTH: u8,
    const HEIGHT: u8,
    //
    const VIEWER_DISTANCE: u8,
    const BRIGHTNESS_FACTOR: u8,
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
        VIEWER_DISTANCE,
        BRIGHTNESS_FACTOR,
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
    const J_STEP: f32 = match J_STEP_DENOM {
        0 | 1 => J_STEP_VALUE as f32,
        _ => (J_STEP_VALUE as f32) / (J_STEP_DENOM as f32),
    };

    const I_STEP: f32 = match I_STEP_DENOM {
        0 | 1 => I_STEP_VALUE as f32,
        _ => (I_STEP_VALUE as f32) / (I_STEP_DENOM as f32),
    };

    const NUM_J: usize = {
        let x = TAU / Self::J_STEP;
        let n = x as usize;
        if x > n as f32 { n + 1 } else { n }
    };

    const NUM_I: usize = {
        let x = TAU / Self::I_STEP;
        let n = x as usize;
        if x > n as f32 { n + 1 } else { n }
    };

    const X_CENTER: f32 = WIDTH as f32 / 2.0;
    const Y_CENTER: f32 = HEIGHT as f32 / 2.0;

    const X_SCALE: f32 = 30.0 * (WIDTH as f32 / 80.0);
    const Y_SCALE: f32 = 15.0 * (HEIGHT as f32 / 22.0);

    const BRIGHTNESS_RAMP: [char; 13] = [C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12];

    /// Create a new donut with initial rotation values set to represent 0 (cosine=1, sine=0).
    pub const fn new() -> Self {
        Self {
            a_cos: 1.0,
            a_sin: 0.0,
            b_cos: 1.0,
            b_sin: 0.0,
        }
    }

    /// Increment the rotation angles by `da` and `db`.
    ///
    /// Rotating the donut creates the animation effect.
    pub fn rotate(&mut self, da: f32, db: f32) {
        {
            let temp = self.a_cos;

            self.a_cos -= da * self.a_sin;
            self.a_sin += da * temp;

            let norm = (3.0 - (self.a_cos * self.a_cos + self.a_sin * self.a_sin)) / 2.0;

            self.a_cos *= norm;
            self.a_sin *= norm;
        }
        {
            let temp = self.b_cos;

            self.b_cos -= db * self.b_sin;
            self.b_sin += db * temp;

            let norm = (3.0 - (self.b_cos * self.b_cos + self.b_sin * self.b_sin)) / 2.0;

            self.b_cos *= norm;
            self.b_sin *= norm;
        }
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

        let (sa, ca) = (self.a_sin, self.a_cos);
        let (sb, cb) = (self.b_sin, self.b_cos);

        let mut j_cos = 1.0;
        let mut j_sin = 0.0;

        for _ in 0..Self::NUM_J {
            let mut i_cos = 1.0;
            let mut i_sin = 0.0;

            for _ in 0..Self::NUM_I {
                let h = j_cos + 2.0;
                let t = i_sin * h * ca - j_sin * sa;
                let d = 1.0 / (i_sin * h * sa + j_sin * ca + VIEWER_DISTANCE as f32);

                let x = (Self::X_CENTER + Self::X_SCALE * d * (i_cos * h * cb - t * sb)) as isize;
                let y = (Self::Y_CENTER + Self::Y_SCALE * d * (i_cos * h * sb + t * cb)) as isize;

                if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
                    let idx = (y * (WIDTH as isize) + x) as usize;

                    if d > zbuf[idx] {
                        zbuf[idx] = d;

                        let n = (BRIGHTNESS_FACTOR as f32
                            * ((j_sin * sa - i_sin * j_cos * ca) * cb
                                - i_sin * j_cos * sa
                                - j_sin * ca
                                - i_cos * j_cos * sb)) as isize;

                        output[idx] = Self::BRIGHTNESS_RAMP[n.clamp(0, 12) as usize];
                    }
                }
                {
                    let temp = i_cos;

                    i_cos -= Self::I_STEP * i_sin;
                    i_sin += Self::I_STEP * temp;

                    let norm = (3.0 - (i_cos * i_cos + i_sin * i_sin)) / 2.0;

                    i_cos *= norm;
                    i_sin *= norm;
                }
            }
            {
                let temp = j_cos;

                j_cos -= Self::J_STEP * j_sin;
                j_sin += Self::J_STEP * temp;

                let norm = (3.0 - (j_cos * j_cos + j_sin * j_sin)) / 2.0;

                j_cos *= norm;
                j_sin *= norm;
            }
        }
    }
}

impl Default for Donut {
    fn default() -> Self {
        Self::new()
    }
}
