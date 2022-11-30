use std::collections::HashSet;

// Santa begins delivering presents to an infinite two-dimensional grid of houses.
// Moves are always exactly:
// (<) one house to the west
// (>) one house to the east
// (^) one house to the north
// (v) one house to the south

// How many houses receive at least one present?
pub fn process_moves_santa_only(input: String) -> i64 {
    let mut deliveries = vec![(0, 0)];
    let moves: Vec<(i32, i32)> = input
        .chars()
        .map(|x| {
            if x == '<' {
                (-1, 0)
            } else if x == '>' {
                (1, 0)
            } else if x == '^' {
                (0, 1)
            } else if x == 'v' {
                (0, -1)
            } else {
                (0, 0)
            }
        })
        .collect();

    for x in moves {
        let last_house = deliveries.last().unwrap();
        deliveries.push((last_house.0 + x.0, last_house.1 + x.1));
    }

    let unique_houses: HashSet<(i32, i32)> = HashSet::from_iter(deliveries.iter().cloned());

    unique_houses.len().try_into().unwrap()
}

// How many houses receive at least one present with Robo-Santa included?
pub fn process_moves_santa_robo_santa(input: String) -> i64 {
    let mut santa_deliveries = vec![(0, 0)];
    let mut robo_deliveries = vec![(0, 0)];

    input
        .chars()
        .map(|x| {
            if x == '<' {
                (-1, 0)
            } else if x == '>' {
                (1, 0)
            } else if x == '^' {
                (0, 1)
            } else if x == 'v' {
                (0, -1)
            } else {
                (0, 0)
            }
        })
        .enumerate()
        .for_each(|(index, x)| {
            if index % 2 == 0 {
                let last_delivery = santa_deliveries.last().unwrap();
                santa_deliveries.push((last_delivery.0 + x.0, last_delivery.1 + x.1));
            } else {
                let last_delivery = robo_deliveries.last().unwrap();
                robo_deliveries.push((last_delivery.0 + x.0, last_delivery.1 + x.1));
            }
        });

    santa_deliveries.append(&mut robo_deliveries);

    let unique_houses: HashSet<(i32, i32)> = HashSet::from_iter(santa_deliveries.iter().cloned());

    unique_houses.len().try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_moves_santa_only() {
        let expected1 = 2;
        let actual1 = process_moves_santa_only(String::from(">"));

        let expected2 = 4;
        let actual2 = process_moves_santa_only(String::from("^>v<"));

        let expected3 = 2;
        let actual3 = process_moves_santa_only(String::from("^v^v^v^v^v"));

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
        assert_eq!(expected3, actual3);
    }

    #[test]
    fn test_process_moves_santa_robo_santa() {
        let expected1 = 3;
        let actual1 = process_moves_santa_robo_santa(String::from("^v"));

        let expected2 = 3;
        let actual2 = process_moves_santa_robo_santa(String::from("^>v<"));

        let expected3 = 11;
        let actual3 = process_moves_santa_robo_santa(String::from("^v^v^v^v^v"));

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
        assert_eq!(expected3, actual3);
    }
}
