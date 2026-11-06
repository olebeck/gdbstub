use super::prelude::*;
use crate::protocol::commands::ext::Threads;

impl<T: Target, C: Connection> GdbStubImpl<T, C> {
    pub(crate) fn handle_threads(
        &mut self,
        res: &mut ResponseWriter<'_, C>,
        target: &mut T,
        command: Threads<'_>,
    ) -> Result<HandlerStatus, Error<T::Error, C::Error>> {
        let ops = match target.support_threads() {
            Some(ops) => ops,
            None => return Ok(HandlerStatus::Handled),
        };

        crate::__dead_code_marker!("threads", "impl");

        let handler_status = match command {
            Threads::qXferThreadsRead(cmd) => {
                let ret = ops
                    .get_threads(cmd.offset, cmd.length, cmd.buf)
                    .handle_error()?;
                if ret == 0 {
                    res.write_str("l")?;
                } else {
                    res.write_str("m")?;
                    res.write_binary(cmd.buf.get(..ret).ok_or(Error::PacketBufferOverflow)?)?;
                }
                HandlerStatus::Handled
            }
        };

        Ok(handler_status)
    }
}
