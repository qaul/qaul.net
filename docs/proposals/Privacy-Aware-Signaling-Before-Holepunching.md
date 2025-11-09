# Privacy-Aware Singaling before Hole Punching on Internet Overlay

## Situation

qaul has a built-in Internet overlay interconnection possibilities.

Most users are behind a firewall. Directly interconnecting with another user requires firewall hole punching, which requires a directly connectable node on the Internet to initiate.

Direct connections also reveal IP addresses to the other party. Depending on the user's situation and attack vector, the user can decide to which other users he wants to connect to. The following options will be available:

- A manually curated white-list of users to connect to.

- Allow connections to trusted users

- Allow connections to all users

All of these options are opt-in and configurable by the user. By default, no P2P connection is accepted.

## Protocol

```mermaid
sequenceDiagram
    participant A as Alice
    participant R as Relay
    participant B as Bob

    Note over A,B: Alice want's to start a P2P connection with Bob
    Note over A,B: They communicate via the Relay on the signaling channel
    A-->>R: Connection Request
    R-->>B:
    activate B
    Note right of B: Check Rules
    B-->>R: Request Decision
    R-->>A:
    deactivate B
    A-->>R: Send Candidates
    R-->>B:
    activate B
    B-->>R: Send Candidates
    R-->>A:
    deactivate B
    Note over A,B: Start Hole Punching
    A -->> B:
    B -->> A: 
    Note over A,B: After Relay received a request from Alice and Bob,<br/> it starts the Hole Punching Mechanism
    A --> B: Connection established
```
