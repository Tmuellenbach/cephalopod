use std::{error::Error, fmt::Display};

use crate::game_engine::{
    game_logic::GameMoveError::{
        AnotherPodAtDestination, DestinationIsOutOfBounds, JumpedSamePodMultipleTimes,
        NoPodAtPosition, NoPodToJump, OutOfReserveProngs,
    },
    game_move::{GameMove, JumpData},
    game_state::{
        BoardPosition, GameState,
        PlayerColor::{self, Blue, Red},
    },
};

const RED_HOME_SPACES: [BoardPosition; 4] = [
    BoardPosition { x: 2, y: 2 },
    BoardPosition { x: 3, y: 2 },
    BoardPosition { x: 4, y: 2 },
    BoardPosition { x: 5, y: 2 },
];
const BLUE_HOME_SPACES: [BoardPosition; 4] = [
    BoardPosition { x: 2, y: 6 },
    BoardPosition { x: 3, y: 6 },
    BoardPosition { x: 4, y: 6 },
    BoardPosition { x: 5, y: 6 },
];

impl GameState {
    ///determines if a player has won in this gamestate
    /// this happens if a player has gotten one of their pods to one of the oposing players home spaces
    /// the function returns the winning players color or none if neither player has won yet
    pub fn is_won_gamestate(&self) -> Option<PlayerColor> {
        for home_space in RED_HOME_SPACES.into_iter() {
            if let Some(pod) = self.get_obj_at_position(home_space)
                && pod.color == Blue
            {
                return Some(Blue);
            }
        }
        for home_space in BLUE_HOME_SPACES.into_iter() {
            if let Some(pod) = self.get_obj_at_position(home_space)
                && pod.color == Red
            {
                return Some(Red);
            }
        }

        None
    }

    ///mutates this gamestate to the result of the move
    pub fn do_move(&mut self, game_move: GameMove) -> Result<(), GameMoveError> {
        match game_move {
            GameMove::AddProng(data) => {
                //first check if the active player has at least 1 reserve prong
                match self.turn {
                    Blue => {
                        if !self.blue_prong_reserve > 0 {
                            return Err(OutOfReserveProngs);
                        }
                    }
                    Red => {
                        if !self.red_prong_reserve > 0 {
                            return Err(OutOfReserveProngs);
                        }
                    }
                }

                //then get the pod at the specified position
                let pod = match self.get_obj_at_position_mut(data.pod_position) {
                    Some(p) => p,
                    None => {
                        return Err(NoPodAtPosition);
                    }
                };
                //and add a prong to it
                pod.add_prong(data.prong_direction)
            }
            GameMove::Move(data) => {
                let from_space = self.get_obj_at_position(data.from);
                if from_space.is_none() {
                    return Err(NoPodAtPosition);
                }
                let Ok(to_pos) = data.from.get_adjcent(data.direction) else {
                    return Err(DestinationIsOutOfBounds);
                };
                let to_space = self.get_obj_at_position(to_pos);
                if to_space.is_some() {
                    return Err(AnotherPodAtDestination);
                }

                //set the from space to none, set_obj_at_position returns the pod previously there
                let pod = self.set_obj_at_position(data.from, None);
                //set the destination space to the pod
                self.set_obj_at_position(to_pos, pod);

                Ok(())
            }
            GameMove::Jump(data) => {
                struct JumpedSpace {
                    pos: BoardPosition,
                    capture: bool,
                }

                //helper function to recursively check each jump leg and collect a vec of the jumped over pods
                fn check_jumps(
                    board: &GameState,
                    data: JumpData,
                    mut jumped_spaces: Vec<JumpedSpace>,
                ) -> Result<(Vec<JumpedSpace>, BoardPosition), GameMoveError> {
                    let from_space = board.get_obj_at_position(data.from);
                    if from_space.is_none() {
                        return Err(NoPodAtPosition);
                    }

                    let Ok(jump_pos) = data.from.get_adjcent(data.direction) else {
                        return Err(DestinationIsOutOfBounds);
                    };
                    let jump_space = board.get_obj_at_position(jump_pos);
                    if jump_space.is_none() {
                        return Err(NoPodToJump);
                    }

                    let Ok(to_pos) = jump_pos.get_adjcent(data.direction) else {
                        return Err(DestinationIsOutOfBounds);
                    };
                    let to_space = board.get_obj_at_position(to_pos);
                    if to_space.is_some() {
                        return Err(AnotherPodAtDestination);
                    }

                    for prev_jump in jumped_spaces.iter() {
                        if prev_jump.pos == jump_pos {
                            return Err(JumpedSamePodMultipleTimes);
                        }
                    }

                    jumped_spaces.push(JumpedSpace {
                        pos: jump_pos,
                        capture: data.capture,
                    });

                    match data.next_segment {
                        Some(next) => check_jumps(board, *next, jumped_spaces),
                        None => Ok((jumped_spaces, to_pos)),
                    }
                }

                let start_pos = data.from;

                //call check_jumps to get the list of jumped over spaces
                let (jumped_spaces, end_pos) = check_jumps(self, data, vec![])?;

                //remove all the pods that were marked for capture
                for space in jumped_spaces {
                    if space.capture {
                        self.set_obj_at_position(space.pos, None);
                    }
                }
                //set the from space to none, set_obj_at_position returns the pod previously there
                let pod = self.set_obj_at_position(start_pos, None);
                //set the destination space to the pod
                self.set_obj_at_position(end_pos, pod);

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum GameMoveError {
    OutOfReserveProngs,
    AddingProngToOccupiedSlot,
    NoPodAtPosition,
    DestinationIsOutOfBounds,
    AnotherPodAtDestination,
    NoPodToJump,
    JumpedSamePodMultipleTimes,
}

impl Display for GameMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for GameMoveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
