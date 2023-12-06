// Common

struct Race {
    time: u64,
    distance: u64,
}

fn race_possibilities(race: Race) -> u64 {
    return (1..race.time)
        .filter(|t| t * (race.time - t) > race.distance)
        .count() as u64;
}

// Part 1

fn part1() -> u64 {
    let races = [
        Race {
            time: 35,
            distance: 212,
        },
        Race {
            time: 93,
            distance: 2060,
        },
        Race {
            time: 73,
            distance: 1201,
        },
        Race {
            time: 66,
            distance: 1044,
        },
    ];

    return races.map(race_possibilities).iter().product::<u64>();
}

// Part 2

fn part2() -> u64 {
    let race = Race {
        time: 35937366,
        distance: 212206012011044,
    };

    return race_possibilities(race);
}

// Main

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 114400);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 21039729);
    }
}
