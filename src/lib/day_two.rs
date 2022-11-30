// Elves need to submit an order for wrapping paper
// They have a list of the dimensions (length l, width w and height h)
// Every present is a perfect box
// The elves also need a little extra paper for each present: the area of the smallest side

// How much wrapping paper is required for all the presents?
pub fn process_presents_wrapper(input: String) -> i64 {
    input
        .split("\n")
        .map(|x| {
            let raw_box = x
                .split("x")
                .map(|y| y.parse::<i64>().unwrap_or_default())
                .collect::<Vec<i64>>();

            let present = Present {
                length: raw_box[0],
                width: raw_box[1],
                height: raw_box[2],
            };

            present.smallest_face_area() + present.surface_area()
        })
        .reduce(|accum: i64, item: i64| accum + item)
        .unwrap_or_default()
}

// How much ribbon is required to decorate all the presents?
pub fn process_presents_ribbon(input: String) -> i64 {
    input
        .split("\n")
        .map(|x| {
            let raw_box = x
                .split("x")
                .map(|y| y.parse::<i64>().unwrap_or_default())
                .collect::<Vec<i64>>();

            let present = Present {
                length: raw_box[0],
                width: raw_box[1],
                height: raw_box[2],
            };

            present.smallest_face_permiter() + present.volume()
        })
        .reduce(|accum: i64, item: i64| accum + item)
        .unwrap_or_default()
}

#[derive(PartialEq, Debug)]
pub struct Present {
    length: i64,
    width: i64,
    height: i64,
}

trait Dimensional {
    fn smallest_face_permiter(&self) -> i64;
    fn smallest_face_area(&self) -> i64;
    fn surface_area(&self) -> i64;
    fn volume(&self) -> i64;
}

impl Dimensional for Present {
    fn smallest_face_area(&self) -> i64 {
        let mut xs = vec![self.length, self.width, self.height];
        xs.sort();

        xs[0] * xs[1]
    }

    fn surface_area(&self) -> i64 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn volume(&self) -> i64 {
        self.length * self.height * self.width
    }

    fn smallest_face_permiter(&self) -> i64 {
        let mut xs = vec![self.length, self.width, self.height];
        xs.sort();

        2 * (xs[0] + xs[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_wrapper() {
        let expected = 101;
        let actual = process_presents_wrapper(String::from("2x3x4\n1x1x10"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_ribbon() {
        let expected = 48;
        let actual = process_presents_ribbon(String::from("2x3x4\n1x1x10"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_smallest_face_area() {
        let expected1 = 6;
        let expected2 = 1;

        let actual1 = Present {
            length: 2,
            width: 3,
            height: 4,
        }
        .smallest_face_area();
        let actual2 = Present {
            length: 1,
            width: 10,
            height: 1,
        }
        .smallest_face_area();

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
    }

    #[test]
    fn test_surface_area() {
        let expected1 = 52;
        let expected2 = 42;

        let actual1 = Present {
            length: 2,
            width: 3,
            height: 4,
        }
        .surface_area();
        let actual2 = Present {
            length: 1,
            width: 10,
            height: 1,
        }
        .surface_area();

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
    }

    #[test]
    fn test_volume() {
        let expected1 = 24;
        let expected2 = 10;

        let actual1 = Present {
            length: 2,
            width: 3,
            height: 4,
        }
        .volume();
        let actual2 = Present {
            length: 1,
            width: 10,
            height: 1,
        }
        .volume();

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
    }

    #[test]
    fn test_smallest_perimiter() {
        let expected1 = 10;
        let expected2 = 4;

        let actual1 = Present {
            length: 2,
            width: 3,
            height: 4,
        }
        .smallest_face_permiter();
        let actual2 = Present {
            length: 1,
            width: 1,
            height: 10,
        }
        .smallest_face_permiter();

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
    }
}
