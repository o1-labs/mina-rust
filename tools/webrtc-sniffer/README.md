# WebRTC sniffer

Note: work in progress.

Uses `pcap` to intercept traffic. Intercepts on the specified interface and saves to the specified `pcap` file.
If the interface is not specified, it will read the traffic from the specified `pcap` file. 

To decrypt traffic, it must have the same `rng_seed` that the Openmina node uses.
Openmina has an optional parameter to specify the seed as a hex string.
The seed should be random and the same for the Openmina node and the sniffer.

Note: **Do not use `--rng-seed' in production**. It will allow you to break the p2p encryption.

## DTLS

The sniffer parses the DTLS protocol and reconstructs `pre_master_secret` for each connection. 

## tshark

It is planned to use the command line tool `tshark` to decrypt DTLS and parse SCTP.

Theoretically, `tshark` can give us the content of the data channels of a WebRTC session.