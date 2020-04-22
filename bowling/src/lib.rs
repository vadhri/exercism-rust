use core::fmt::Display;
use std::fmt;

#[allow(unused_must_use)]

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Frame {
    roll_0: u16,
    roll_1: u16,
    strike: bool,
    in_progress: bool,
    is_frame_complete: bool,
    score: u16
}

#[derive(Debug, PartialEq)]
pub struct BowlingGame {
    running_frame: i32,
    frames: Vec<Frame>
}

impl fmt::Display for BowlingGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res:fmt::Result = Ok(());
        for (index, item) in self.frames.iter().enumerate() {
            println!("Index : {} Roll = [{}, {}]; (Strike)->{} (In progress)->{} (Frame Comp.)->{} (Score)->{}",
                index + 1, item.roll_0, item.roll_1, item.strike, item.in_progress, item.is_frame_complete, item.score);
        }

        res
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        let default_frame = Frame {
            roll_0: 0 as u16,
            roll_1: 0 as u16,
            in_progress: false,
            strike: false,
            is_frame_complete: false,
            score: 0 as u16
        };

        BowlingGame {
            running_frame: 0,
            frames: vec![default_frame; 10]
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let running_frame = self.running_frame as usize;
        let mut result: Result<(), Error> = Ok(());

        if pins > 10 {
            result = Err(Error::NotEnoughPinsLeft);
        } else if running_frame >= 10 {
            if running_frame == 10
                && self.frames[running_frame-1].strike
                && !self.frames[running_frame-1].is_frame_complete {
                    if self.frames[running_frame-1].in_progress {
                        self.frames[running_frame-1].roll_1 = pins;

                        if self.frames[running_frame-1].roll_1 + self.frames[running_frame-1].roll_0 > 10
                            && self.frames[running_frame-1].roll_0 != 10 {
                            result = Err(Error::NotEnoughPinsLeft);
                        }
                        else {
                            self.frames[running_frame-1].is_frame_complete = true;
                            self.frames[running_frame-1].in_progress = false;
                        }
                    } else {
                        self.frames[running_frame-1].roll_0 = pins;
                        self.frames[running_frame-1].in_progress = true;
                    }
                } else if running_frame == 10
                    && !self.frames[running_frame-1].strike
                    && !self.frames[running_frame-1].is_frame_complete
                    && self.frames[running_frame-1].roll_0 + self.frames[running_frame-1].roll_1 == 10 {
                        self.frames[running_frame-1].strike = true;
                        self.frames[running_frame-1].in_progress = false;
                        self.frames[running_frame-1].roll_0 = pins;
                        self.frames[running_frame-1].is_frame_complete = true;
                        self.frames[running_frame-1].roll_1 = 0;

                }
            else {
                // Check if the last frame has a spare which needs to be counted for one.
                result = Err(Error::GameComplete);
            }
        } else if self.frames[running_frame].in_progress {
            if pins + self.frames[running_frame].roll_0 > 10 {
                result = Err(Error::NotEnoughPinsLeft);
            } else if pins == 10 && self.frames[running_frame].roll_0 == 0 {
                self.frames[running_frame].roll_1 = 10;
                self.frames[running_frame].strike = false;
                self.frames[running_frame].is_frame_complete = true;
                self.frames[running_frame].in_progress = false;
            } else {
                self.frames[running_frame].roll_1 = pins;
                if running_frame == 9
                    && self.frames[running_frame].roll_0 + self.frames[running_frame].roll_1 == 10 {
                    self.frames[running_frame].in_progress = true;
                    self.frames[running_frame].is_frame_complete = false;
                } else {
                    self.frames[running_frame].in_progress = false;
                    self.frames[running_frame].is_frame_complete = true;
                }
                self.running_frame += 1;
                result = Ok(());
            }
        } else if pins == 10 {
            // Strike at 10th element.
            if self.running_frame == 9 {
                self.frames[running_frame].is_frame_complete = false;
                self.frames[running_frame].in_progress = false;
            // Strike at elements 0 - 9
            } else {
                self.frames[running_frame].roll_1 = 10;
                self.frames[running_frame].is_frame_complete = true;
                self.frames[running_frame].in_progress = false;
            }

            self.frames[running_frame].strike = true;
            self.running_frame += 1;
            result = Ok(());
        } else if !self.frames[running_frame].in_progress {
            self.frames[running_frame].roll_0 = pins;
            self.frames[running_frame].strike = false;
            self.frames[running_frame].is_frame_complete = false;
            self.frames[running_frame].in_progress = true;
        }
        result
    }

    pub fn score(&self) -> Option<u16> {
        if self.running_frame < 10 || (self.running_frame == 10 && self.frames[9].is_frame_complete == false) {
            None
        }
        else {
            let mut temp = self.frames.clone();

            for (index, item) in temp.iter_mut().enumerate() {
                if item.strike == true {
                    // not 10th frame.
                    if index <= 8 {
                        if !self.frames[index+1].strike {
                            item.score = 10 + self.frames[index+1].roll_0 + self.frames[index+1].roll_1;
                        } else { // another strike. need roll_0 of the next frame.
                            if index + 2 < 10 {
                                if self.frames[index+2].strike {
                                    item.score = 10 + 10 + 10;
                                } else {
                                    item.score = 10 + 10 + self.frames[index+2].roll_0;
                                }
                            } else {
                                if self.frames[index+1].strike {
                                    item.score = 10 + 10 + self.frames[index+1].roll_0;
                                } else {
                                    item.score = 10 + self.frames[index+1].roll_0 + self.frames[index+1].roll_1;
                                }
                            }
                        }
                    } else {
                        if self.frames[index].strike {
                            // 9th index is the last strike.
                            item.score = 10 + item.roll_0 + item.roll_1
                        }
                    }
                } else if item.roll_0 + item.roll_1 == 10 {
                    item.score = 10 + self.frames[index+1].roll_0;
                } else {
                    item.score = item.roll_0 + item.roll_1;
                }
            }

            let x = temp.iter().fold(0, |sum, x| sum + x.score);
            Some(x)
        }
    }
}
