#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    pins_left: u16,
    frame: usize,
    frame_ball: usize,
    balls_per_frame: usize,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: vec![],
            pins_left: 10,
            frame: 1,
            frame_ball: 1,
            balls_per_frame: 2,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frame > 10 {
            Err(Error::GameComplete)
        } else if pins > self.pins_left {
            Err(Error::NotEnoughPinsLeft)
        } else {
            self.rolls.push(pins);

            if self.frame == 10 {
                if pins == self.pins_left {
                    if self.frame_ball < 3 {
                        self.balls_per_frame = 3;
                    }
                    self.pins_left = 10;
                } else {
                    self.pins_left -= pins;
                }
                if self.frame_ball >= self.balls_per_frame {
                    self.frame = 11;
                }
                self.frame_ball += 1;
            } else {
                // not frame 10
                if pins == self.pins_left || self.frame_ball == 2 {
                    self.frame += 1;
                    self.pins_left = 10;
                    self.frame_ball = 1;
                } else {
                    self.frame_ball += 1;
                    self.pins_left -= pins;
                }
            }

            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame <= 10 {
            None
        } else {
            let mut score = 0;
            let mut ball = 0;
            for _ in 0..10 {
                let first_ball = self.rolls[ball];
                let second_ball = self.rolls[ball + 1];
                let mut third_ball = 0;
                if first_ball == 10 || first_ball + second_ball == 10 {
                    third_ball = self.rolls[ball + 2];
                }
                score += first_ball + second_ball + third_ball;
                if first_ball == 10 {
                    ball += 1
                } else {
                    ball += 2
                }
            }
            Some(score)
        }
    }
}
