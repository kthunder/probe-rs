/// Vendor-specific `DAP_SetVref` command.
///
/// Not part of the ARM CMSIS-DAP specification. Matches the OpenOCD
/// implementation: command ID 0x9D, payload is VTref in units of 100 mV.
use super::super::{CommandId, Request, SendError, Status};

/// Set the target reference voltage of the probe.
///
/// `millivolts` is the requested VTref in millivolts. The probe firmware
/// accepts 1500..=3300 mV, sent as a single byte in units of 100 mV.
#[derive(Clone, Copy, Debug)]
pub struct SetVrefRequest {
    pub millivolts: u16,
}

impl SetVrefRequest {
    /// Creates a request, clamping the voltage to the firmware-supported
    /// range of 1500..=3300 mV.
    pub fn new(millivolts: u16) -> Self {
        Self {
            millivolts: millivolts.clamp(1500, 3300),
        }
    }
}

impl Request for SetVrefRequest {
    const COMMAND_ID: CommandId = CommandId::SetVref;

    type Response = SetVrefResponse;

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, SendError> {
        // Vendor payload: one byte, voltage in units of 100 mV
        // (OpenOCD: command[1] = ref_mv / 100, clamped 1500..3300 mV).
        buffer[0] = (self.millivolts / 100) as u8;
        Ok(1)
    }

    fn parse_response(&self, buffer: &[u8]) -> Result<Self::Response, SendError> {
        Ok(SetVrefResponse(Status::from_byte(buffer[0])?))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SetVrefResponse(pub(crate) Status);
