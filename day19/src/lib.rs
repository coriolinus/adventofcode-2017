#[macro_use]
extern crate util;

use std::ops::Add;

type Maze = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    row: usize,
    column: usize,
}

impl Add<Direction> for Position {
    type Output = Option<Position>;
    fn add(self, rhs: Direction) -> Option<Position> {
        use Direction::*;
        match rhs {
            Up => {
                if self.row > 0 {
                    Some(Position {
                        row: self.row - 1,
                        ..self
                    })
                } else {
                    None
                }
            }
            Down => Some(Position {
                row: self.row + 1,
                ..self
            }),
            Left => {
                if self.column > 0 {
                    Some(Position {
                        column: self.column - 1,
                        ..self
                    })
                } else {
                    None
                }
            }
            Right => Some(Position {
                column: self.column + 1,
                ..self
            }),
        }
    }
}


struct LineFollower<'a> {
    maze: &'a Maze,
    position: Position,
    direction: Option<Direction>,
    prev_position: Option<Position>,
}

impl<'a> Iterator for LineFollower<'a> {
    type Item = Position;
    fn next(&mut self) -> Option<Position> {
        debug_println!(
            "At {:?} ('{}') heading {:?}",
            self.position,
            self.char_at(self.position),
            self.direction,
        );

        // if self.direction is None, we've run off the end and have nowhere left to go
        let direction = self.direction?;
        if let Some(next_position) = self.position + direction {
            if self.char_at(next_position) == ' ' {
                use Direction::*;
                self.direction = match self.direction {
                    Some(Up) | Some(Down) => {
                        if ((self.position + Right)?).column < self.maze[self.position.row].len() &&
                            self.char_at((self.position + Right)?) != ' '
                        {
                            Some(Right)
                        } else if self.char_at((self.position + Left)?) != ' ' {
                            Some(Left)
                        } else {
                            None
                        }
                    }
                    Some(Left) | Some(Right) => {
                        if ((self.position + Down)?).row < self.maze.len() &&
                            self.char_at((self.position + Down)?) != ' '
                        {
                            Some(Down)
                        } else if self.char_at((self.position + Up)?) != ' ' {
                            Some(Up)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
            }
        }

        if let Some(direction) = self.direction {
            if let Some(next_position) = self.position + direction {
                if next_position.row < self.maze.len() &&
                    next_position.column < self.maze[self.position.row].len()
                {
                    self.position = next_position;
                    self.prev_position = Some(self.position);
                    return self.prev_position;
                }
            }
        }
        None
    }
}

impl<'a> LineFollower<'a> {
    fn char_at(&self, position: Position) -> char {
        if self.maze.len() > position.row && self.maze[position.row].len() > position.column {
            self.maze[position.row][position.column]
        } else {
            // we can assume we're in an infinite grid of spaces
            ' '
        }
    }

    fn new(maze: &'a Maze) -> Option<LineFollower<'a>> {
        if maze.is_empty() || maze[0].iter().filter(|c| **c != ' ').count() != 1 {
            return None;
        }

        let position = Position {
            row: 0,
            column: maze[0].iter().position(|c| *c == '|')?,
        };

        Some(LineFollower {
            maze: maze,
            position: position,
            direction: Some(Direction::Down),
            prev_position: None,
        })
    }
}

pub fn collect_letters(maze: &Maze) -> Option<(Vec<char>, usize)> {
    let mut results = Vec::new();
    let mut steps = 1;
    for position in LineFollower::new(maze)? {
        steps += 1;
        if maze[position.row][position.column].is_alphabetic() {
            results.push(maze[position.row][position.column]);
        }
    }
    Some((results, steps))
}
