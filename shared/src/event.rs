use strum_macros::AsRefStr;

#[derive(Clone, AsRefStr, PartialEq, Eq)]
pub enum TauriEvent {
    #[strum(serialize = "request_file")]
    RequestFile,
    #[strum(serialize = "receive_file")]
    ReceiveFile,
    #[strum(serialize = "next_file")]
    NextFile,
    #[strum(serialize = "prev_file")]
    PrevFile,
}

#[derive(Clone, AsRefStr, PartialEq)]
pub enum KeyboardEvent {
    #[strum(serialize = "next_image")]
    NextImage,
    #[strum(serialize = "prev_image")]
    PrevImage,
}
