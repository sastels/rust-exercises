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

fn dump_1_into_2(vol_1: &mut u8, _capacity_1: u8, vol_2: &mut u8, capacity_2: u8) {
    let transferred = (*vol_1).min(capacity_2 - *vol_2);
    *vol_1 -= transferred;
    *vol_2 += transferred;
}

fn empty_2(_vol_1: &mut u8, _capacity_1: u8, vol_2: &mut u8, _capacity_2: u8) {
    *vol_2 = 0;
}

fn fill_1(vol_1: &mut u8, capacity_1: u8, _vol_2: &mut u8, _capacity_2: u8) {
    *vol_1 = capacity_1;
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    println!(
        "{} {} goal {} start: {:?}",
        capacity_1, capacity_2, goal, start_bucket
    );

    let mut goal_bucket = Bucket::One;
    let mut capacity_1 = capacity_1;
    let mut capacity_2 = capacity_2;
    if *start_bucket == Bucket::Two {
        println!("+++ Swap +++");
        goal_bucket = Bucket::Two;
        std::mem::swap(&mut capacity_1, &mut capacity_2)
    }

    let mut moves = 0;
    let mut vol_1 = 0;
    let mut vol_2 = 0;

    loop {
        if moves > 0 && vol_1 == 0 && vol_2 == 0 {
            break;
        }
        if vol_1 == 0 {
            fill_1(&mut vol_1, capacity_1, &mut vol_2, capacity_2);
            moves += 1;
        }

        if vol_1 == goal {
            break;
        }

        if capacity_2 == goal {
            vol_2 = capacity_2;
            moves += 1;
            break;
        }
        dump_1_into_2(&mut vol_1, capacity_1, &mut vol_2, capacity_2);
        moves += 1;

        if vol_1 == goal || vol_2 == goal {
            break;
        }

        if vol_2 == capacity_2 {
            empty_2(&mut vol_1, capacity_1, &mut vol_2, capacity_2);
            moves += 1;
            println!("{}: {} {}", moves, vol_1, vol_2);
        }
    }

    let mut other_bucket = vol_2;
    if vol_1 != goal && vol_2 != goal {
        return None;
    } else if vol_2 == goal {
        other_bucket = vol_1;
        if goal_bucket == Bucket::One {
            goal_bucket = Bucket::Two;
        } else {
            goal_bucket = Bucket::One;
        }
    }

    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
}
