/// Vendor-specific `DAP_EnableCJtag` command.
///
/// Not part of the ARM CMSIS-DAP specification. Matches the OpenOCD
/// implementation: command ID 0x9F, payload is a fixed `1`. The firmware
/// only supports switching into cJTAG mode; there is no disable path.
use super::super::{CommandId, Request, SendError, Status};

/// Switch the probe into cJTAG (Compact JTAG, 2-wire oJTAG) mode.
///
/// Takes effect before connecting; a probe in cJTAG mode no longer speaks
/// classic 4-wire JTAG on the same session.
#[derive(Clone, Copy, Debug)]
pub struct EnableCJtagRequest;

impl Request for EnableCJtagRequest {
    const COMMAND_ID: CommandId = CommandId::EnableCJtag;

    type Response = EnableCJtagResponse;

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, SendError> {
        // Vendor payload: fixed 1 (OpenOCD: command[1] = 1).
        buffer[0] = 1;
        Ok(1)
    }

    fn parse_response(&self, buffer: &[u8]) -> Result<Self::Response, SendError> {
        Ok(EnableCJtagResponse(Status::from_byte(buffer[0])?))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EnableCJtagResponse(#[expect(dead_code)] pub(crate) Status);
