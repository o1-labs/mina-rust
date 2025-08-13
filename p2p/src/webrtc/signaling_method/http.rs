//! HTTP signaling transport configuration.
//!
//! This module defines the HTTP-specific signaling transport configuration
//! for WebRTC connections in OpenMina's peer-to-peer network.
//!
//! ## HTTP Signaling
//!
//! HTTP signaling provides a simple, widely-supported transport method for
//! WebRTC offer/answer exchange. It uses standard HTTP requests to POST
//! WebRTC offers to signaling servers and receive answers in response.
//!
//! ## Transport Characteristics
//!
//! - **Request/Response Model**: Uses HTTP POST for offer delivery
//! - **Stateless**: Each signaling exchange is independent
//! - **Firewall Friendly**: Works through most corporate firewalls and proxies
//! - **Simple Implementation**: Requires only basic HTTP client functionality
//!
//! ## URL Structure
//!
//! HTTP signaling info encodes the host and port information needed to
//! construct signaling server URLs. The format is:
//!
//! - String representation: `/{host}/{port}`
//! - Full URL: `http(s)://{host}:{port}/mina/webrtc/signal`
//!
//! ## Security Considerations
//!
//! HTTP signaling can use either HTTP or HTTPS depending on the signaling
//! method variant. HTTPS is recommended for production environments to
//! protect signaling data and prevent tampering during transmission.

use std::{fmt, str::FromStr};

use binprot_derive::{BinProtRead, BinProtWrite};
use serde::{Deserialize, Serialize};

use crate::webrtc::Host;

use super::SignalingMethodParseError;

/// HTTP signaling server connection information.
///
/// `HttpSignalingInfo` encapsulates the network location information needed
/// to connect to an HTTP-based WebRTC signaling server. This includes the
/// host address and port number required for establishing HTTP connections.
///
/// # Usage
///
/// This struct is used by both HTTP and HTTPS signaling methods, as well as
/// HTTPS proxy configurations. It provides the fundamental addressing
/// information needed to construct signaling URLs and establish connections.
///
/// # Fields
///
/// - `host`: The server hostname, IP address, or multiaddr
/// - `port`: The TCP port number for the HTTP service
///
/// # Examples
///
/// ```
/// use openmina::webrtc::Host;
/// use openmina::signaling_method::HttpSignalingInfo;
///
/// // IPv4 signaling server
/// let info = HttpSignalingInfo {
///     host: Host::Ipv4("192.168.1.100".parse()?),
///     port: 8080,
/// };
///
/// // Domain-based signaling server
/// let info = HttpSignalingInfo {
///     host: Host::Domain("signal.example.com".into()),
///     port: 443,
/// };
/// ```
#[derive(BinProtWrite, BinProtRead, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct HttpSignalingInfo {
    /// The host address for the HTTP signaling server.
    ///
    /// This can be a domain name, IPv4 address, IPv6 address, or multiaddr
    /// depending on the network configuration and addressing requirements.
    pub host: Host,

    /// The TCP port number for the HTTP signaling server.
    ///
    /// Standard ports are 80 for HTTP and 443 for HTTPS, but custom
    /// ports can be used depending on the server configuration.
    pub port: u16,
}

impl fmt::Display for HttpSignalingInfo {
    /// Formats the HTTP signaling info as a path component string.
    ///
    /// This creates a string representation suitable for inclusion in
    /// signaling method URLs. The format is `/{host}/{port}` where the
    /// host and port are formatted according to their respective types.
    ///
    /// # Example Output
    ///
    /// - IPv4: `/192.168.1.100/8080`
    /// - Domain: `/signal.example.com/443`
    /// - IPv6: `/[::1]/8080`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}/{}", self.host, self.port)
    }
}

impl From<([u8; 4], u16)> for HttpSignalingInfo {
    /// Creates HTTP signaling info from an IPv4 address and port tuple.
    ///
    /// This convenience constructor allows easy creation of `HttpSignalingInfo`
    /// from raw IPv4 address bytes and a port number.
    ///
    /// # Parameters
    ///
    /// * `value` - A tuple containing (IPv4 address bytes, port number)
    ///
    /// # Example
    ///
    /// ```
    /// let info = HttpSignalingInfo::from(([192, 168, 1, 100], 8080));
    /// assert_eq!(info.port, 8080);
    /// ```
    fn from(value: ([u8; 4], u16)) -> Self {
        Self {
            host: Host::Ipv4(value.0.into()),
            port: value.1,
        }
    }
}

impl FromStr for HttpSignalingInfo {
    type Err = SignalingMethodParseError;

    /// Parses a string representation into HTTP signaling info.
    ///
    /// This method parses path-like strings that contain host and port
    /// information separated by forward slashes. The expected format is
    /// `{host}/{port}` or `/{host}/{port}`.
    ///
    /// # Format
    ///
    /// - Input: `{host}/{port}` (leading slash optional)
    /// - Host: Domain name, IPv4, IPv6, or multiaddr format
    /// - Port: 16-bit unsigned integer (0-65535)
    ///
    /// # Examples
    ///
    /// ```
    /// use openmina::signaling_method::HttpSignalingInfo;
    ///
    /// // Domain and port
    /// let info: HttpSignalingInfo = "signal.example.com/443".parse()?;
    ///
    /// // IPv4 and port
    /// let info: HttpSignalingInfo = "192.168.1.100/8080".parse()?;
    ///
    /// // With leading slash
    /// let info: HttpSignalingInfo = "/localhost/8080".parse()?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`SignalingMethodParseError`] for:
    /// - Missing host or port components
    /// - Invalid host format (not a valid hostname, IP, or multiaddr)
    /// - Invalid port number (not a valid 16-bit unsigned integer)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('/').filter(|v| !v.trim().is_empty());
        let host_str = iter
            .next()
            .ok_or(SignalingMethodParseError::NotEnoughArgs)?;
        let host = Host::from_str(host_str)
            .map_err(|err| SignalingMethodParseError::HostParseError(err.to_string()))?;

        let port = iter
            .next()
            .ok_or(SignalingMethodParseError::NotEnoughArgs)?
            .parse::<u16>()
            .map_err(|err| SignalingMethodParseError::PortParseError(err.to_string()))?;

        Ok(Self { host, port })
    }
}

impl Serialize for HttpSignalingInfo {
    /// Serializes the HTTP signaling info as a string.
    ///
    /// This uses the `Display` implementation to convert the signaling
    /// info to its string representation for serialization. The output
    /// format is `/{host}/{port}`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for HttpSignalingInfo {
    /// Deserializes HTTP signaling info from a string.
    ///
    /// This uses the [`FromStr`] implementation to parse the string
    /// representation back into an [`HttpSignalingInfo`] instance.
    /// The expected format is `{host}/{port}` or `/{host}/{port}`.
    ///
    /// # Errors
    ///
    /// Returns a deserialization error if the string cannot be parsed
    /// as valid HTTP signaling info (invalid host, port, or format).
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
