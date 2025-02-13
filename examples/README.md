# Examples

A set of examples showcasing how to use rust-libp2p.

## Getting started

- [Ping](ping.rs)

  Small `ping` clone, sending a ping to a peer, expecting a pong as a response. See
  [tutorial](../src/tutorials/ping.rs) for a step-by-step guide building the example.

## Individual libp2p features

- [Chat](./chat.rs)

   A basic chat application demonstrating libp2p and the mDNS and floodsub protocols.

    - [Gossipsub chat](./gossipsub-chat.rs)

      Same as the chat example but using mDNS and the Gossipsub protocol.

    - [Tokio based chat](./chat-tokio.rs)

      Same as the chat example but using tokio for all asynchronous tasks and I/O.

- [Distributed key-value store](./distributed-key-value-store.rs)

  A basic key value store demonstrating libp2p and the mDNS and Kademlia protocol.

- [Identify](../protocols/identify/examples/identify.rs)

  Demonstrates how to use identify protocol to query peer information.

- [IPFS Kademlia](ipfs-kad.rs)

  Demonstrates how to perform Kademlia queries on the IPFS network.

- [IPFS Private](ipfs-private.rs)

  Implementation using the gossipsub, ping and identify protocols to implement the ipfs private
  swarms feature.

- [Passive Discovery via MDNS](mdns-passive-discovery.rs)

  Discover peers on the same network via the MDNS protocol.
  
- [Hole punching tutorial](https://docs.rs/libp2p/latest/libp2p/tutorials/hole_punching/index.html)

  Tutorial on how to overcome firewalls and NATs with libp2p’s hole punching mechanism.

## Integration into a larger application

- [File sharing application](./file-sharing.rs)

  Basic file sharing application with peers either providing or locating and getting files by name.

  While obviously showcasing how to build a basic file sharing application with the Kademlia and
  Request-Response protocol, the actual goal of this example is **to show how to integrate
  rust-libp2p into a larger application**.

- [Chat with Identify and Kademlia DHT](kad-identify-chat/main.rs)

  The kad-identify-chat example implements simple chat functionality using the `Identify` protocol
  and the `Kademlia` DHT for peer discovery and routing. Broadcast messages are propagated using the
  `Gossipsub` behaviour, direct messages are sent using the `RequestResponse` behaviour.

  The primary purpose of this example is to demonstrate how these behaviours interact.
  
  A secondary purpose of this example is to show what integration of libp2p in a complete
  application might look like. This is similar to the [file sharing example](file-sharing.rs),
  but where that example is purposely more barebones, this example is a bit more expansive and 
  uses common crates like `thiserror` and `anyhow`. It also uses the tokio runtime instead of async_std.
