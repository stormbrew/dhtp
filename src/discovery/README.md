# Distributed Peer Discovery and NAT Traversal Protocol

This module defines a mechanism for finding another host by a public key
through the Mainline DHT, and also using the DHT to make an attempt at
NAT punching if necessary without requiring a dedicated STUN server.

It should also create some degree of privacy with regards to who is
connecting to who, though that is not an absolute requirement.

## Operation

### Infohashes

DHT infohashes are used to establish knowledge about peers across networks
without actually storing any meaningful information in the DHT itself. A
sequence of advertisements of specially crafted INFOHASHes will allow the peers
to find each other and verify their identities.

These infohashes are derived from the following pieces of information:

- Public key of one side of the connection.
- The expected 'name' of the host being connected to/from.
- The direction of the advertisement (`want-connections`, `want-connect`, `want-p2p`).
- Depending on the type of advertisement, a timestamp to allow for rotation and
make it more difficult to DoS a particular target by spamming the DHT.

Constraints:
 - The infohash should not be guessable by someone who does not have both 
 the details the hash is based on (public key, name, direction)
 - If a host or service has more than one name it wants to be discoverable by, it
 should advertise multiple infohashes. Since the same host will be
 advertising all of these, it should not be possible to correlate these back
 to either the host or the public key that originated them if you have all
 or some of them.

### Client/Server Operation

In this mode there's a host with an identity that multiple clients might want to
connect to. In this case there is asymmetric behaviour between the two sides.

#### Server

- Creates an INFOHASH based on its public key (probably a
host key, but could really be any kind of key), hostname, and a direction
of `want-connections`. It advertises this infohash on the DHT. 
- Listens on a known port for UDP packets matching the connection-initiation
protocol described below.
- Periodically searches the DHT for an identical infohash on the DHT, but with `want-connect`
as the direction and a timestamp of the current time rounded to the last 5 minute
boundary.
 - Whenever a `want-connect` infohash is discovered, sends connection-initiation
 packets periodically to hosts advertising the infohash, in order to facilitate
 STUN-style NAT-traversal.
- Once a connection handshake is achieved, switch over to the real protocol
(probably QUIC) and connect to the service being advertised.


#### Client

- Potentially, the client will make the following simple and naive attempts to connect 
to the peer before attempting DHT traversal:
 - if the hostname is resolvable, just attempt to connect to it (especially
 if it's on a local network).
 - if the hostname is not resolvable, potentially use any cached information
 about previously known IPs for that host (again, especially if it's on a local
 network).
- If that fails, search the DHT for a `want-connections` infohash matching the public key(s)
and name being searched, with a timestamp of the current time rounded down to the last 5
minute boundary. If it's found, attempt to start a handshake, as described
below, with the host advertising the infohash.
- Simultaneously to that, or perhaps after attempting for a brief period, advertise
a `want-connect` message on the DHT indicating the host's intention to connect and need
to find a NAT punch.
- Continue sending handshake initiations periodically to facilitate hole punching, until
a handshake is achieved and connection can proceed to the underlying protocol.

### Peer-to-Peer Operation

In peer-to-peer mode, the two sides must know each others' public keys and names, and
these things must be separately distinct (ie. bob can't connect to bob and bob's only key
can't connect to bob's only key).

After potentially performing attempts to discover each other by more basic needs, as with the
Client above, both sides will perform the following steps to find each other across the
DHT:

- Each peer will advertise a `want-p2p` infohash for the others' key and name, along with a
timestamp (similer to the client key above).
- They will also be searching the DHT for `want-p2p` infohashes for their own key and name, along
with the same timestamp value.
- When a peer is discovered, they will connect to each other through the standard handshake process,
while continuing to advertise their presence on the DHT.

## Handshake Process

(needs describing in more detail -- based on prolog of wireguard protocol? At very least, in prolog
should not ever return an error -- which probably rules out just using quic as-is)

## Notes

- Clients and servers should make a best-effort to act as responsible nodes in the DHT, 
including advertising and discovering peers so long as they're connected, setting 
flags appropriate to short-lived clients if necessary, and otherwise not making life
miserable for other users just because this is not a traditional use of the DHT.
- The use of Mainline DHT is mostly to solve a bootstrapping problem, as it is a widespread
and perfectly capable DHT on its own. It should be entirely possible to adapt
this protocol to other DHT implementations, or to deploy a use-case-specific seeded
network for this protocol.