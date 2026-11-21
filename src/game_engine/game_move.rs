use std::{error::Error, fmt::Display};

use crate::game_engine::game_state::BoardPosition;

///Enumerates the different types of move a player could make on their turn
pub enum GameMove {
    ///Add a prong to one of your pods
    AddProng(AddProngData),
    ///Make a simple move from one position to an ajecent postion in the direction of the pods prong
    Move(MoveData),
    //Make a more complex jumping move that may have multiple jump segments and can optionally capture the jumped pieces
    Jump(JumpData),
}

pub struct AddProngData {
    pub pod_position: BoardPosition,
    pub prong_direction: Direction,
}

pub struct MoveData {
    /// starting position of the pod
    pub from: BoardPosition,
    /// direction the pod is moving in
    pub direction: Direction,
}

pub struct JumpData {
    pub from: BoardPosition,
    pub direction: Direction,
    pub capture: bool,
    pub next_segment: Option<Box<JumpData>>,
}

///Represents the 8 directions a pod can move, and the respective slots for prongs
/// Up, Down, Left and Right, and the intermediates Up+Right, Down+Left, etc.
#[derive(Clone, Copy)]
pub enum Direction {
    ///Up
    U,
    ///Up + Right
    Ur,
    ///Right
    R,
    ///Down + Right
    Dr,
    ///Down
    D,
    ///Down + Left
    Dl,
    /// Left
    L,
    ///Up + Left
    Ul,
}
