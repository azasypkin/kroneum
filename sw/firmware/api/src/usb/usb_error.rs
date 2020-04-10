/// Describes USB specific errors.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum USBError {
    /// Happens when we cannot parse received bytes as a valid command.
    InvalidCommand,
    /// Happens when USB host tried to talk to unsupported endpoint.
    InvalidEndpoint,
}
