use crate::protocol::common::qxfer::ParseAnnex;
use crate::protocol::common::qxfer::QXferReadBase;

pub type qXferThreadsRead<'a> = QXferReadBase<'a, ThreadsAnnex>;

#[derive(Debug)]
pub struct ThreadsAnnex;

impl ParseAnnex<'_> for ThreadsAnnex {
    #[inline(always)]
    fn from_buf(buf: &[u8]) -> Option<Self> {
        if buf != b"" {
            return None;
        }

        Some(ThreadsAnnex)
    }
}
