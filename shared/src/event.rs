use strum_macros::AsRefStr;

#[derive(Clone, AsRefStr, PartialEq, Eq)]
pub enum TauriEvent {
    #[strum(serialize = "request_image")]
    RequestImage,
    #[strum(serialize = "receive_image")]
    ReceiveImage,
}
