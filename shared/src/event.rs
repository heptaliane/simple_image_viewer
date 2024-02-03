use strum_macros::AsRefStr;

#[derive(Clone, AsRefStr, PartialEq, Eq)]
pub enum TauriEvent {
    #[strum(serialize = "request_image")]
    RequestImage,
    #[strum(serialize = "receive_image")]
    ReceiveImage,
    #[strum(serialize = "move_next")]
    MoveNext,
    #[strum(serialize = "move_prev")]
    MovePrev,
}

#[derive(Clone, AsRefStr, PartialEq)]
pub enum KeyboardEvent {
    #[strum(serialize = "next_image")]
    NextImage,
    #[strum(serialize = "prev_image")]
    PrevImage,
}
