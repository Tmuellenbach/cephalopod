use std::{error::Error, fmt::Display};

use crate::game_engine::{
    game_logic::GameMoveError::{
        AnotherPodAtDestination, NoPodAtSpecifiedPosition, OutOfReserveProngs,
    },
    game_move::GameMove,
    game_state::{
        BoardPosition, GameState,
        PlayerColor::{self, Blue, Red},
        Pod,
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
    fn is_won_gamestate(&self) -> Option<PlayerColor> {
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
    fn do_move(&mut self, game_move: GameMove) -> Result<(), GameMoveError> {
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
                        return Err(NoPodAtSpecifiedPosition(data.pod_position));
                    }
                };
                //and add a prong to it
                pod.add_prong(data.prong_postion)
            }
            GameMove::Move(data) => {
                let from_space = self.get_obj_at_position(data.from);
                if from_space.is_none() {
                    return Err(NoPodAtSpecifiedPosition(data.from));
                }
                let to_space = self.get_obj_at_position(data.to);
                if to_space.is_some() {
                    return Err(AnotherPodAtDestination(data.to));
                }

                //set the from space to none, set_obj_at_position returns the pod previously there
                let pod = self.set_obj_at_position(data.from, None);
                //set the destination space to the pod
                self.set_obj_at_position(data.to, pod);

                Ok(())
            }
            GameMove::Jump(data) => {
                todo!()
            }
        }
    }
}

#[derive(Debug)]
pub enum GameMoveError {
    OutOfReserveProngs,
    AddingProngToOccupiedSlot,
    NoPodAtSpecifiedPosition(BoardPosition),
    AnotherPodAtDestination(BoardPosition),
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
