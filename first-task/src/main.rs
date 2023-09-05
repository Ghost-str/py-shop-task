mod read_args;

use rand::Rng;
use read_args::get_args;

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home
                + if score_changed && home_score_change {
                    1
                } else {
                    0
                },
            away: previous_value.score.away
                + if score_changed && !home_score_change {
                    1
                } else {
                    0
                },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

#[inline(always)]
pub fn sum_tupls<F, S>(one: &(F, S), two: &(F, S)) -> (F, S)
where
    F: std::ops::Add<Output = F> + Copy,
    S: std::ops::Add<Output = S> + Copy,
{
    (one.0 + two.0, one.1 + two.1)
}

fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    let mut result_score = (0, 0);

    for index in 0..game_stamps.len() {
        let stamp = &game_stamps[index];
        let score = &stamp.score;

        if stamp.offset < offset {
            let result_from_score = (score.home, score.away);

            result_score = sum_tupls(&result_score, &result_from_score);
        } else {
            return result_score;
        }
    }

    result_score
}

fn main() {
    let args = get_args();

    let game = generate_game();

    let offset = get_score(&game, args.offset);

    println!("Result score for game {:?}", offset);
}

#[cfg(test)]
const TEST_DATA: [Stamp; 3] = [
    Stamp {
        offset: 10,
        score: Score { home: 1, away: 0 },
    },
    Stamp {
        offset: 20,
        score: Score { home: 0, away: 1 },
    },
    Stamp {
        offset: 30,
        score: Score { home: 0, away: 1 },
    },
];

#[test]
fn offset_smoller_than_first_stamps_offset() {
    let score = get_score(&TEST_DATA, 5);

    assert_eq!(score, (0, 0));
}

#[test]
fn offset_more_than_last_stamps_offset() {
    let score = get_score(&TEST_DATA, 40);

    assert_eq!(score, (1, 2));
}

#[test]
fn offset_in_middle_stamps_offset() {
    let score = get_score(&TEST_DATA, 25);

    assert_eq!(score, (1, 1));
}
