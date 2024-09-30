use xbinser_macros::EnumEncoded;

#[derive(Debug, Clone, Copy, PartialEq, EnumEncoded)]
pub enum Message {
    Hi,
    Bye
}