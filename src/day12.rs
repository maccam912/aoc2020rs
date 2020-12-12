#[derive(Debug)]
struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn rot_left(&mut self, amt: i64) {
        let steps = amt / 90;
        for _ in 0..steps {
            let tmp = self.y;
            self.y = self.x;
            self.x = -tmp;
        }
    }

    fn rot_right(&mut self, amt: i64) {
        let steps = amt / 90;
        for _ in 0..steps {
            let tmp = self.y;
            self.y = -self.x;
            self.x = tmp;
        }
    }
}
#[derive(Debug)]
struct Ship {
    heading: i64,
    x: i64,
    y: i64,
    waypoint: Waypoint,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            heading: 90,
            x: 0,
            y: 0,
            waypoint: Waypoint { x: 10, y: 1 },
        }
    }

    fn step_a(&mut self, rule: (char, i64)) {
        match rule.0 {
            'N' => self.y += rule.1,
            'S' => self.y -= rule.1,
            'E' => self.x += rule.1,
            'W' => self.x -= rule.1,
            'R' => self.heading += rule.1,
            'L' => self.heading -= rule.1,
            'F' => match self.heading {
                0 => self.y += rule.1,
                90 => self.x += rule.1,
                180 => self.y -= rule.1,
                270 => self.x -= rule.1,
                _ => panic!("Heading not one of 0,90,180,270"),
            },
            _ => panic!("Unrecognized rule char"),
        };
        self.heading = self.heading.rem_euclid(360);
        assert!(self.heading >= 0);
        assert!(self.heading < 360);
    }

    fn step_b(&mut self, rule: (char, i64)) {
        match rule.0 {
            'N' => self.waypoint.y += rule.1,
            'S' => self.waypoint.y -= rule.1,
            'E' => self.waypoint.x += rule.1,
            'W' => self.waypoint.x -= rule.1,
            'R' => self.waypoint.rot_right(rule.1),
            'L' => self.waypoint.rot_left(rule.1),
            'F' => {
                self.x += self.waypoint.x * rule.1;
                self.y += self.waypoint.y * rule.1;
            }
            _ => panic!("Unrecognized rule char"),
        };
        self.heading = self.heading.rem_euclid(360);
        assert!(self.heading >= 0);
        assert!(self.heading < 360);
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn parse_line(line: &str) -> (char, i64) {
    let letter = line.chars().next().unwrap();
    let num = line[1..].parse::<i64>().unwrap();
    (letter, num)
}

fn day12a(lines: &[String]) -> i64 {
    let mut ship = Ship::new();
    let rules: Vec<(char, i64)> = lines.iter().map(|x| parse_line(x.trim())).collect();
    for rule in rules {
        ship.step_a(rule);
    }
    ship.manhattan_distance()
}

fn day12b(lines: &[String]) -> i64 {
    let mut ship = Ship::new();
    let rules: Vec<(char, i64)> = lines.iter().map(|x| parse_line(x.trim())).collect();
    for rule in rules {
        ship.step_b(rule);
    }
    ship.manhattan_distance()
}

pub fn day12(lines: &[String], part: char) -> i64 {
    match part {
        'a' => day12a(lines),
        'b' => day12b(lines),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day12;

    #[test]
    fn test_case() {
        let input = "F10
        N3
        F7
        R90
        F11";
        let lines: Vec<String> = input.split('\n').map(|x| x.trim().to_string()).collect();
        let rules: Vec<(char, i64)> = lines.iter().map(|x| day12::parse_line(x)).collect();
        //println!("{:?}", rules);
        let mut ship = day12::Ship::new();
        for rule in &rules {
            ship.step_a(*rule);
            //println!("{:?}", ship);
        }
        assert_eq!(ship.manhattan_distance(), 25);

        let mut ship2 = day12::Ship::new();
        for rule in &rules {
            ship2.step_b(*rule);
        }
        assert_eq!(ship2.manhattan_distance(), 286);
    }

    #[test]
    fn test_waypoint_rotate() {
        let mut w = day12::Waypoint { x: 1, y: 1 };
        w.rot_left(90);
        assert_eq!(w.x, -1);
        assert_eq!(w.y, 1);
    }
}
