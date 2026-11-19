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
    pub prong_postion: ProngSlot,
}

///Represents the 8 slots that a prong can be put into a pod, and the respective directions that the pod can move
/// Up, Down, Left and Right, and the intermediates Up+Right, Down+Left, etc.
pub enum ProngSlot {
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

pub struct MoveData {
    pub from: BoardPosition,
    pub to: BoardPosition,
}

pub struct JumpData {
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub capture: bool,
    pub next_segment: Option<Box<JumpData>>,
}
