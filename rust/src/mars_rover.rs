type RoverError = String;
type Position = (u8, u8);

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Clone, PartialEq, Debug)]
struct Rover {
    position: Position,
    direction: Direction,
}

impl Rover {
    fn new(position: Position, direction: Direction) -> Rover {
        Rover {
            position,
            direction,
        }
    }

    fn update_position(&self, new_position: Position) -> Rover {
        Rover {
            position: new_position,
            direction: self.direction.clone()
        }
    }

    fn update_x_position(&self, new_x: u8) -> Rover {
        self.update_position((new_x, self.position.1))
    }

    fn update_y_position(&self, new_y: u8) -> Rover {
        self.update_position((self.position.0, new_y))
    }

    fn update_direction(&self, new_direction: Direction) -> Rover {
        Rover {
            position: self.position.clone(),
            direction: new_direction
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Command {
    Forward,
    Backward,
    Left,
    Right,
}

fn parse_commands(commands: &str) -> Result<Vec<Command>, RoverError> {
    use Command::*;
    let mut result: Vec<Command> = vec![];
    for command in commands.chars() {
        match command {
            'f' => result.push(Forward),
            'b' => result.push(Backward),
            'l' => result.push(Left),
            'r' => result.push(Right),
            _ => return Err(format!("{} cannot be parsed", command))
        }
    }
    Ok(result)
}

fn compute_command(rover: &Rover, command: Command, obstacles: &Vec<Position>) -> Result<Rover, (Rover, RoverError)> {
    use Command::*;
    use Direction::*;
    let result= match command {
        Forward => match rover.direction {
            East => rover.update_x_position(rover.position.0.wrapping_add(1)), // Update X
            West => rover.update_x_position(rover.position.0.wrapping_sub(1)),
            North => rover.update_y_position(rover.position.1.wrapping_add(1)), // Update Y
            South => rover.update_y_position(rover.position.1.wrapping_sub(1))
        },
        Backward => match rover.direction {
            East => rover.update_x_position(rover.position.0.wrapping_sub(1)), // Update X
            West => rover.update_x_position(rover.position.0.wrapping_add(1)),
            North => rover.update_y_position(rover.position.1.wrapping_sub(1)), // Update Y
            South => rover.update_y_position(rover.position.1.wrapping_add(1))
        },
        Left => match rover.direction {
            East => rover.update_direction(North),
            West => rover.update_direction( South),
            North => rover.update_direction( West),
            South => rover.update_direction( East),
        },
        Right => match rover.direction {
            East => rover.update_direction( South),
            West => rover.update_direction( North),
            North => rover.update_direction( East),
            South => rover.update_direction( West),
        },
    };

    if let Some((x, y)) = obstacles.iter().find(|obstacle| **obstacle == result.position) {
        Err((rover.clone(), format!("Collision with obstacle in {},{}", x, y)))
    } else { Ok(result) }
}

fn mars_rover(rover: &Rover, commands: &str, obstacles: &Vec<Position>) -> Result<Rover, (Rover, RoverError)> {
    let commands = parse_commands(commands).map_err(|err| (rover.clone(), err))?;
    commands.into_iter()
        .fold(Ok(rover.clone()),
              |acc, command|
                  acc.and_then(|r| compute_command(&r, command, &obstacles)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mars_rover_should_accept_commands() {
        let rover = Rover::new((0, 0), Direction::North);
        assert!(mars_rover(&rover, "fffllfrb", &vec![]).is_ok());
        assert!(mars_rover(&rover, "", &vec![]).is_ok());
    }

    #[test]
    fn mars_rover_should_accept_single_forward_command() {
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(mars_rover(&rover, "f", &vec![]), Ok(Rover::new((1, 0), Direction::East)))
    }

    #[test]
    fn mars_rover_should_accept_multiples_forward_command() {
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(mars_rover(&rover, "fffff", &vec![]), Ok(Rover::new((5, 0), Direction::East)))
    }

    #[test]
    fn mars_rover_should_accept_single_backward_command() {
        let rover = Rover::new((5, 0), Direction::East);
        assert_eq!(mars_rover(&rover, "b", &vec![]), Ok(Rover::new((4, 0), Direction::East)))
    }

    #[test]
    fn mars_rover_should_accept_multiples_backward_command() {
        let rover = Rover::new((5, 0), Direction::East);
        assert_eq!(mars_rover(&rover, "bbbbb", &vec![]), Ok(Rover::new((0, 0), Direction::East)))
    }

    #[test]
    fn parse_command_should_success_parsing_empty_commands() {
        assert_eq!(parse_commands(""), Ok(vec![]));
    }

    #[test]
    fn parse_command_should_parse_command() {
        use Command::*;
        assert_eq!(parse_commands("f"), Ok(vec![Forward]));
        assert_eq!(parse_commands("ffff"), Ok(vec![Forward, Forward, Forward, Forward]));
        assert_eq!(parse_commands("f "), Err("  cannot be parsed".to_string()));
        assert_eq!(parse_commands(" f"), Err("  cannot be parsed".to_string()));

        assert_eq!(parse_commands("b"), Ok(vec![Backward]));
        assert_eq!(parse_commands("bbbb"), Ok(vec![Backward, Backward, Backward, Backward]));
        assert_eq!(parse_commands("b "), Err("  cannot be parsed".to_string()));
        assert_eq!(parse_commands(" b"), Err("  cannot be parsed".to_string()));

        assert_eq!(parse_commands("l"), Ok(vec![Left]));
        assert_eq!(parse_commands("llll"), Ok(vec![Left, Left, Left, Left]));
        assert_eq!(parse_commands("l "), Err("  cannot be parsed".to_string()));
        assert_eq!(parse_commands(" l"), Err("  cannot be parsed".to_string()));

        assert_eq!(parse_commands("r"), Ok(vec![Right]));
        assert_eq!(parse_commands("rrrr"), Ok(vec![Right, Right, Right, Right]));
        assert_eq!(parse_commands("r "), Err("  cannot be parsed".to_string()));
        assert_eq!(parse_commands(" r"), Err("  cannot be parsed".to_string()));

        assert_eq!(parse_commands("lfrb"), Ok(vec![Left, Forward, Right, Backward]));

        assert_eq!(parse_commands("a"), Err("a cannot be parsed".to_string()));
        assert_eq!(parse_commands("alfrb"), Err("a cannot be parsed".to_string()));
        assert_eq!(parse_commands("lfarb"), Err("a cannot be parsed".to_string()));
        assert_eq!(parse_commands("lfrba"), Err("a cannot be parsed".to_string()));
    }

    #[test]
    fn compute_command_should_update_position() {
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((1, 0), Direction::East)));
        let rover = Rover::new((1, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((0, 0), Direction::East)));

        let rover = Rover::new((0, 0), Direction::North);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((0, 1), Direction::North)));
        let rover = Rover::new((0, 1), Direction::North);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((0, 0), Direction::North)));

        let rover = Rover::new((1, 0), Direction::West);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((0, 0), Direction::West)));
        let rover = Rover::new((1, 0), Direction::West);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((2, 0), Direction::West)));

        let rover = Rover::new((0, 1), Direction::South);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((0, 0), Direction::South)));
        let rover = Rover::new((0, 1), Direction::South);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((0, 2), Direction::South)));
    }

    #[test]
    fn compute_command_should_update_direction() {
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Left, &vec![]), Ok(Rover::new((0, 0), Direction::North)));
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Right, &vec![]), Ok(Rover::new((0, 0), Direction::South)));

        let rover = Rover::new((0, 0), Direction::North);
        assert_eq!(compute_command(&rover, Command::Left, &vec![]), Ok(Rover::new((0, 0), Direction::West)));
        let rover = Rover::new((0, 0), Direction::North);
        assert_eq!(compute_command(&rover, Command::Right, &vec![]), Ok(Rover::new((0, 0), Direction::East)));

        let rover = Rover::new((0, 0), Direction::West);
        assert_eq!(compute_command(&rover, Command::Left, &vec![]), Ok(Rover::new((0, 0), Direction::South)));
        let rover = Rover::new((0, 0), Direction::West);
        assert_eq!(compute_command(&rover, Command::Right, &vec![]), Ok(Rover::new((0, 0), Direction::North)));

        let rover = Rover::new((0, 0), Direction::South);
        assert_eq!(compute_command(&rover, Command::Left, &vec![]), Ok(Rover::new((0, 0), Direction::East)));
        let rover = Rover::new((0, 0), Direction::South);
        assert_eq!(compute_command(&rover, Command::Right, &vec![]), Ok(Rover::new((0, 0), Direction::West)));
    }

    #[test]
    fn mars_rover_should_accept_command_with_lefts_and_rights() {
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(mars_rover(&rover, "lfrfrbl", &vec![]), Ok(Rover::new((1, 2), Direction::East)))
    }

    #[test]
    fn compute_command_should_warp_at_the_endf_of_the_world() {
        let rover = Rover::new((255, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((0, 0), Direction::East)));
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((255, 0), Direction::East)));

        let rover = Rover::new((0, 255), Direction::North);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((0, 0), Direction::North)));
        let rover = Rover::new((0, 0), Direction::North);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((0, 255), Direction::North)));

        let rover = Rover::new((0, 0), Direction::West);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((255, 0), Direction::West)));
        let rover = Rover::new((255, 0), Direction::West);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((0, 0), Direction::West)));

        let rover = Rover::new((0, 0), Direction::South);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![]), Ok(Rover::new((0, 255), Direction::South)));
        let rover = Rover::new((0, 255), Direction::South);
        assert_eq!(compute_command(&rover, Command::Backward, &vec![]), Ok(Rover::new((0, 0), Direction::South)));
    }

    #[test]
    fn compute_command_should_failed_if_connecting_to_an_obstacle() {
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![(1, 0)]), Err((rover, "Collision with obstacle in 1,0".to_string())));
        let rover = Rover::new((255, 0), Direction::East);
        assert_eq!(compute_command(&rover, Command::Forward, &vec![(0, 0)]), Err((rover, "Collision with obstacle in 0,0".to_string())));
    }

    #[test]
    fn mars_rover_should_move_to_the_last_possible_step_before_collision() {
        let rover = Rover::new((0, 0), Direction::East);
        assert_eq!(mars_rover(&rover, "lfrfrbl", &vec![(1, 2)]),
                   Err((Rover::new((1, 1), Direction::South), "Collision with obstacle in 1,2".to_string())))
    }
}