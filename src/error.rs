/// Size of the discriminant that match each error enum 
/// 
/// Note
/// Max 65535 enum, make u32 if not enough 
pub type EthosErrorDiscriminantSize = u16;

#[derive(Debug)]
#[repr(u16)]
pub enum EthosError {
    /// Happens when server or client received an invalid message.
    InvalidNetMessage = 1,
}