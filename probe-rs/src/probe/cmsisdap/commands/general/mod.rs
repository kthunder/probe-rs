pub mod connect;
pub mod disconnect;
pub mod enable_cjtag;
pub mod host_status;
pub mod info;
pub mod reset;
// Not wired into the connect flow yet; kept for bring-up.
#[allow(dead_code)]
pub mod set_vref;
