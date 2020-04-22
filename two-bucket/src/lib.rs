use core::cmp::min;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub fn next_move(from_bucket: &mut u8, to_bucket: &mut u8, from_cap: u8, to_cap: u8) {
    if *from_bucket == 0 {
        *from_bucket = from_cap;
    } else if *to_bucket < to_cap {
        let movement_into_to = min(to_cap - *to_bucket, *from_bucket);
        *from_bucket -= movement_into_to;
        *to_bucket += movement_into_to;
    } else {
        *to_bucket = 0;
    }
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    let mut no_of_moves: i32 = 0;
    let mut b1_current_marker = 0;
    let mut b2_current_marker = 0;

    let mut result: Option<BucketStats> = None;

    if *start_bucket == Bucket::One && capacity_2 == goal {
        result = Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: capacity_1
        });
    } else if *start_bucket == Bucket::Two && capacity_1 == goal {
        result = Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::One,
            other_bucket: capacity_2
        });
    } else {
        loop {
            if no_of_moves > std::u8::MAX as i32 {
                break;
            }

            if b1_current_marker == goal {
                result = Some(BucketStats {
                    moves: no_of_moves as u8,
                    goal_bucket: Bucket::One,
                    other_bucket: b2_current_marker
                });
                break;
            } else if b2_current_marker == goal {
                result = Some(BucketStats {
                    moves: no_of_moves as u8,
                    goal_bucket: Bucket::Two,
                    other_bucket: b1_current_marker
                });
                break;
            }

            match start_bucket {
                Bucket::One => next_move(&mut b1_current_marker, &mut b2_current_marker, capacity_1, capacity_2),
                Bucket::Two => next_move(&mut b2_current_marker, &mut b1_current_marker, capacity_2, capacity_1)
            }

            no_of_moves += 1;
        }
    }
    result
}
