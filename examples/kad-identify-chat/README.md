# kad-identify-chat
The kad-identify-chat example implements simple chat functionality.

It does peer discovery using the Kademlia DHT, combined with the identify behaviour.
Broadcast messages are sent using the `Gossipsub` behaviour, direct messages are sent using
the `RequestResponse` behaviour.

The primary purpose of this example is to demonstrate how these behaviours interact.
Please see the docs on `MyChatNetworkBehaviour` for more details.

A secondary purpose of this example is to show how integration of libp2p in a more complex application might look like.
It implements a `Network` component that wraps the `Swarm` and a network client that communicates with it
through mpsc channels.

Finally, it shows a way to organise your business logic using custom `EventHandler` and
`InstructionHandler` traits. This is by no means *the* way to do it, but it worked well
in the real-world application this example was derived from.

Note how all business logic is concentrated in only four functions:
- `MyChatNetworkClient::handle_user_input()`: takes input from the cli and turns it into an
  `Instruction` for the `Network`.
- `MyChatNetworkBehaviour::handle_instruction()`: acts on the `Instruction` using one of its
  composed `NetworkBehaviour`s. Most likely by sending a message through the `Swarm`.
- `MyChatNetworkBehaviour::handle_event()`: receives events from the `Swarm` and turns them
  into `Notification`s for `MyChatNetworkClient`.
- `MyChatNetworkClient::handle_notification()`: receives `Notification`s and acts on them, often
  by formatting them and displaying them to the user.

# The CLI
Once peers are running, the following 'commands' can be typed into their consoles:
- press `<enter>`: sends a broadcast to all known peers through GossipSub. (used below)
- `dm 2 my message`: sends 'my message' to peer number 2.
- `ls`: shows the list of `PeerId`s this peer is aware of.

> A note on peer numbers. For this example, peers are started with a peer number. This is then used to deterministically generate their peer id. This is a convenience for testing and prevents us from having to copy-pasting peer ids when sending a dm. So when a peer was started with `--peer-no 3`, you can send a message to it 
> using `dm 3 some message`.

# Running a network of peers
The goal of running this example is to show three peers interacting:
- Peer 1 is a so-called bootstrap node. Every peer that joins the network should initially connect to one bootstrap node to discover the rest of the network. After that initial discovery they no longer need the bootstrap node. Note that the only differences between a bootstrap node and a normal node are: 1) that it does not necessarily connect to a specified bootstrap node on startup and 2) that the bootstrap node is required to be running at all times so they can be an entrypoint to the network for new joining node. Note that it is fine to have multiple bootstrap nodes in a network for robustness.
- Peer 2 and 3 connect to the bootstrap node and register themselves in the Kademlia DHT. They also discover other nodes they can reach on the DHT.
- Peer 2 will send a BROADCAST pubsub message.
- The example shows that the pubsub message reaches peer 3, even though they did not have a direct connection between themselves at the time. The message is relayed through pubsub protocol by peer 1, who has a connection to both peer 2 and 3.
- Next peer 1 and 3 will automatically reply to the BROADCAST sent by peer 2 with a direct message to peer 2: 
  - make a direct connection (dialing) to peer 2
  - send a direct message addressed to peer 2
 
> `RUST_LOG=INFO` is a good starting log level that shows only relevant communication between peers. But if you want to see more details on how the nodes discover each other and communicate, `RUST_LOG=kad_identify_chat=DEBUG` is good. 

To run this example, open three terminals. 
In terminal 1, run peer 1:

```sh
$ RUST_LOG=INFO cargo run --features="full" --example kad-identify-chat -- --peer-no 1 
```

In terminal 2, run peer 2:

```sh
$ RUST_LOG=INFO cargo run --features="full" --example kad-identify-chat -- --peer-no 2 --bootstrap-peer-no 1 
```

In terminal 3, run peer 3:

```sh
$ RUST_LOG=INFO cargo run --features="full" --example kad-identify-chat -- --peer-no 3 --bootstrap-peer-no 1
```

Let all of them run for a few seconds to ensure that they have discovered each other, then press enter in the terminal of peer 2 to trigger the broadcast message.

Expected behaviour: 

Observe the broadcast message appearing in the log of peer 1 and 3. Note that peer 3 receives the message from peer 1, and not from peer 2 as expected.
Next, observe the direct messages being sent from both peer 1 and 3. Finally, observe those direct message being received by peer 2. 
This concludes the interaction. 

## Optional next step
An optional next step is to add a peer 4 and 5, such that: 
- peer 5 is connected only to peer 4 (peer 4 is its bootstrap node).
- peer 4 is only connected to peer 3 (peer 3 is its bootstrap node).
- we know already that peer 2 and 3 were initially only connected to their bootstrap peer, peer 1.

Peer 5 should also receive a broadcast message from peer 2. It should also be able to send back a dm to peer 2, even though it is not connected to it yet. 
This will be possible because the listen address for peer 2 will be in the DHT (because of the Identify behaviour), 
which will be synced to peer 5 as soon as it joins the network.

To run it, first kill all peers from above and restart them. This ensures none of them have a direct connection
between them, except to their bootstrap node.

Next, in terminal 4, run peer 4:

```sh
$ RUST_LOG=INFO cargo run --features="full" --example kad-identify-chat -- --peer-no 4 --bootstrap-peer-no 3
```

In terminal 5, run peer 5:

```sh
$ RUST_LOG=INFO cargo run --features="full" --example kad-identify-chat -- --peer-no 5  --bootstrap-peer-no 4
```

Then we execute the same test as before: press enter in the terminal of peer 2.

Expected behaviour: 
- Peer 3 receives the broadcast from peer 1
- Peer 4 receives it from peer 3
- Peer 5 receives it from peer 4
- They all send back a direct message to peer 2, so we should see a dm from peer 5 in the log of peer 2
