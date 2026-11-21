use std::fmt::Error;

use crate::game_engine::{
    game_logic::GameMoveError,
    game_move::Direction,
    game_state::PlayerColor::{Blue, Red},
};

/// Represents a state of a game

pub struct GameState {
    pub turn: PlayerColor,

    pub blue_prong_reserve: u8,
    pub red_prong_reserve: u8,

    board: [[Option<Pod>; 6]; 7],
}

impl GameState {
    ///Creates a gamestate representing the beginning of a game with empty pods placed on the home rows
    pub fn new() -> GameState {
        GameState {
            turn: Blue,
            blue_prong_reserve: 12,
            red_prong_reserve: 12,
            board: [
                [None, None, None, None, None, None],
                [
                    None,
                    Some(Pod::new(Red)),
                    Some(Pod::new(Red)),
                    Some(Pod::new(Red)),
                    Some(Pod::new(Red)),
                    None,
                ],
                [None, None, None, None, None, None],
                [None, None, None, None, None, None],
                [None, None, None, None, None, None],
                [
                    None,
                    Some(Pod::new(Blue)),
                    Some(Pod::new(Blue)),
                    Some(Pod::new(Blue)),
                    Some(Pod::new(Blue)),
                    None,
                ],
                [None, None, None, None, None, None],
            ],
        }
    }

    ///gets the pod at a board position or none if there is not a pod there
    pub fn get_obj_at_position(&self, pos: BoardPosition) -> &Option<Pod> {
        &self.board[pos.y - 1][pos.x - 1]
    }

    ///gets the pod at a board position or none if there is not a pod there, allows mutation
    pub fn get_obj_at_position_mut(&mut self, pos: BoardPosition) -> &mut Option<Pod> {
        &mut self.board[pos.y - 1][pos.x - 1]
    }

    //sets the space at the board position to the specified value
    pub fn set_obj_at_position(&mut self, pos: BoardPosition, obj: Option<Pod>) -> Option<Pod> {
        let prev_obj = self.board[pos.y - 1][pos.x - 1];
        self.board[pos.y - 1][pos.x - 1] = obj;
        return prev_obj;
    }
}

#[derive(Clone, Copy)]
pub struct Pod {
    ///which player controls this pod
    pub color: PlayerColor,
    /// true if a prong is added, false otherwise, 0 represents up prong, 1 up +right, etc continuing clockwise
    prongs: [bool; 8],
}

impl Pod {
    ///Creates a pod with no prongs
    fn new(color: PlayerColor) -> Pod {
        Pod {
            color,
            prongs: [false, false, false, false, false, false, false, false],
        }
    }

    //Add a prong to this pod, returns Err if a prong is already in that position
    pub fn add_prong(&mut self, pos: Direction) -> Result<(), GameMoveError> {
        let mut try_add = |idx: usize| {
            if !self.prongs[idx] {
                self.prongs[idx] = true;
                Ok(())
            } else {
                Err(GameMoveError::AddingProngToOccupiedSlot)
            }
        };

        match pos {
            Direction::U => try_add(0),
            Direction::Ur => try_add(1),
            Direction::R => try_add(2),
            Direction::Dr => try_add(3),
            Direction::D => try_add(4),
            Direction::Dl => try_add(5),
            Direction::L => try_add(6),
            Direction::Ul => try_add(7),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerColor {
    Blue,
    Red,
}

///Represents the location of a space on the board
/// where 1, 1 is the top left corner and 6, 7 is the bottom right
/// x increases from left to right, in the range 1-6 inclusive
/// y increases from top to bottom in the range 1-7 inclusive
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

impl BoardPosition {
    ///create a new boardposition from the given coordinates
    /// returns err if coordinates are out of bounds
    pub fn new(x: usize, y: usize) -> Result<BoardPosition, Error> {
        if x < 1 || x > 6 || y < 1 || y > 7 {
            return Err(Error);
        } else {
            Ok(BoardPosition { x: x, y: y })
        }
    }

    /// creates a boardposition for the space adjcent to this one in the given direction
    /// returns err if the position is out of bounds
    pub fn get_adjcent(&self, dir: Direction) -> Result<BoardPosition, Error> {
        match dir {
            Direction::U => todo!(),
            Direction::Ur => todo!(),
            Direction::R => todo!(),
            Direction::Dr => todo!(),
            Direction::D => todo!(),
            Direction::Dl => todo!(),
            Direction::L => todo!(),
            Direction::Ul => todo!(),
        }
    }
}
