# issue 3519

## Steps to reproduce

Create a wasmcloud component
`wash new component hello --template-name hello-worl-rust`

Start wash in dev mode
`wash dev`

List NATS connections
`nats server report connections`

The output display several connections which contains unnamed

```text
╭─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│                                                                              Top 7 Connections out of 7 by subs                                                                             │
├─────┬──────────────────────────┬──────────────────────────────────────────────────────────┬─────────┬─────────────────┬─────────┬────────┬─────────┬──────────┬──────────┬───────────┬──────┤
│ CID │ Name                     │ Server                                                   │ Cluster │ IP              │ Account │ Uptime │ In Msgs │ Out Msgs │ In Bytes │ Out Bytes │ Subs │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│ 43  │                          │ NCJE6UCDNO3A5XGH2DCEEOW4J4M2IFNN4UUBVQP7LPCQYNIJV3BZHH2Q │         │ 127.0.0.1:53662 │         │ 41m37s │ 2       │ 2        │ 887 B    │ 392 B     │ 1    │
│ 46  │                          │ NCJE6UCDNO3A5XGH2DCEEOW4J4M2IFNN4UUBVQP7LPCQYNIJV3BZHH2Q │         │ 127.0.0.1:53663 │         │ 41m37s │ 6       │ 19       │ 1.2 KiB  │ 12 KiB    │ 1    │
│ 79  │ NATS CLI Version 0.1.5   │ NCJE6UCDNO3A5XGH2DCEEOW4J4M2IFNN4UUBVQP7LPCQYNIJV3BZHH2Q │         │ 127.0.0.1:53942 │         │ 0s     │ 1       │ 0        │ 254 B    │ 0 B       │ 1    │
│ 32  │                          │ NCJE6UCDNO3A5XGH2DCEEOW4J4M2IFNN4UUBVQP7LPCQYNIJV3BZHH2Q │         │ 127.0.0.1:53661 │         │ 41m37s │ 113     │ 95       │ 688 B    │ 2.6 KiB   │ 2    │
│ 57  │ http-server-provider     │ NCJE6UCDNO3A5XGH2DCEEOW4J4M2IFNN4UUBVQP7LPCQYNIJV3BZHH2Q │         │ 127.0.0.1:53673 │         │ 41m20s │ 95      │ 107      │ 2.6 KiB  │ 78 B      │ 5    │
│ 6   │                          │ NCJE6UCDNO3A5XGH2DCEEOW4J4M2IFNN4UUBVQP7LPCQYNIJV3BZHH2Q │         │ 127.0.0.1:53659 │         │ 41m37s │ 3,406   │ 3,677    │ 303 KiB  │ 1.9 MiB   │ 6    │
│ 31  │                          │ NCJE6UCDNO3A5XGH2DCEEOW4J4M2IFNN4UUBVQP7LPCQYNIJV3BZHH2Q │         │ 127.0.0.1:53660 │         │ 41m37s │ 512     │ 1,334    │ 288 KiB  │ 86 KiB    │ 13   │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│     │ TOTALS FOR 7 CONNECTIONS │                                                          │         │                 │         │        │ 4,135   │ 5,234    │ 597 KIB  │ 2.0 MIB   │ 29   │
╰─────┴──────────────────────────┴──────────────────────────────────────────────────────────┴─────────┴─────────────────┴─────────┴────────┴─────────┴──────────┴──────────┴───────────┴──────╯
```

## Attemps to display names

Adding name to nats connections (async_nats) does not change anything. Maybe it's due to nats cli tooling, I'm not sure
how to configure system account with nats.

After strugling a bit to find where to add name to connection options, it works:

```text
╭─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│                                                                              Top 7 Connections out of 7 by subs                                                                             │
├─────┬──────────────────────────┬──────────────────────────────────────────────────────────┬─────────┬─────────────────┬─────────┬────────┬─────────┬──────────┬──────────┬───────────┬──────┤
│ CID │ Name                     │ Server                                                   │ Cluster │ IP              │ Account │ Uptime │ In Msgs │ Out Msgs │ In Bytes │ Out Bytes │ Subs │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│ 57  │ belos-washlib-config     │ NCXSY5CLW4VPGOKSJML4C3YFEBZMNVD7H4LK3XBANLY42T7MLMZSTKVL │         │ 127.0.0.1:51382 │         │ 7s     │ 2       │ 2        │ 887 B    │ 392 B     │ 1    │
│ 58  │ belos-washlib-config     │ NCXSY5CLW4VPGOKSJML4C3YFEBZMNVD7H4LK3XBANLY42T7MLMZSTKVL │         │ 127.0.0.1:51383 │         │ 7s     │ 2       │ 5        │ 240 B    │ 3.1 KiB   │ 1    │
│ 62  │ NATS CLI Version 0.1.5   │ NCXSY5CLW4VPGOKSJML4C3YFEBZMNVD7H4LK3XBANLY42T7MLMZSTKVL │         │ 127.0.0.1:51390 │         │ 0s     │ 1       │ 0        │ 254 B    │ 0 B       │ 1    │
│ 51  │                          │ NCXSY5CLW4VPGOKSJML4C3YFEBZMNVD7H4LK3XBANLY42T7MLMZSTKVL │         │ 127.0.0.1:51381 │         │ 7s     │ 4       │ 1        │ 610 B    │ 16 B      │ 2    │
│ 61  │ http-server-provider     │ NCXSY5CLW4VPGOKSJML4C3YFEBZMNVD7H4LK3XBANLY42T7MLMZSTKVL │         │ 127.0.0.1:51388 │         │ 5s     │ 1       │ 1        │ 16 B     │ 0 B       │ 5    │
│ 48  │                          │ NCXSY5CLW4VPGOKSJML4C3YFEBZMNVD7H4LK3XBANLY42T7MLMZSTKVL │         │ 127.0.0.1:51379 │         │ 7s     │ 174     │ 186      │ 18 KiB   │ 93 KiB    │ 6    │
│ 52  │                          │ NCXSY5CLW4VPGOKSJML4C3YFEBZMNVD7H4LK3XBANLY42T7MLMZSTKVL │         │ 127.0.0.1:51380 │         │ 7s     │ 64      │ 83       │ 32 KiB   │ 16 KiB    │ 13   │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│     │ TOTALS FOR 7 CONNECTIONS │                                                          │         │                 │         │        │ 248     │ 278      │ 52 KIB   │ 113 KIB   │ 29   │
╰─────┴──────────────────────────┴──────────────────────────────────────────────────────────┴─────────┴─────────────────┴─────────┴────────┴─────────┴──────────┴──────────┴───────────┴──────╯
```

### Test without wasmcloud

Set up a nats cluster with Docker as it's describe in
nats [doc](https://docs.nats.io/running-a-nats-service/nats_docker/nats-docker-tutorial).
`docker run -p 4222:4222 -p 8222:8222 -p 6222:6222 --name nats-server -ti nats:latest`

With this nats config, I can see connections info from the browser at http://localhost:8222

Set up a nats connection with [async_nats](https://docs.rs/async-nats/latest/async_nats/). Setup new project with cargo
and copy/paste example async_nats code into main.rs.

Here is the code sample I use:

```rust
use bytes::Bytes;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // Connect to the NATS server
    async_nats::ConnectOptions::new()
        .name("rust-service")
        .connect("0.0.0.0:4222") // nats server expose to port 4222 with docker
        .await?;

    // Subscribe to the "messages" subject
    let mut subscriber = client.subscribe("messages").await?;

    // Publish messages to the "messages" subject
    for _ in 0..10 {
        client.publish("messages", "data".into()).await?;
    }

    // Receive and process messages
    while let Some(message) = subscriber.next().await {
        println!("Received message {:?}", message);
    }

    Ok(())
}
```

When I start the nats connection with async_nats: `cargo run` the connection and publish works fine and I can see the
connection's name on the browser http://localhost:8222/connz

Moreover I can see connection name with nats cli:
```text
╭───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│                                                                                 Top 2 Connections out of 2 by subs                                                                                │
├─────┬──────────────────────────┬──────────────────────────────────────────────────────────┬────────────┬──────────────────┬─────────┬──────────┬─────────┬──────────┬──────────┬───────────┬──────┤
│ CID │ Name                     │ Server                                                   │ Cluster    │ IP               │ Account │ Uptime   │ In Msgs │ Out Msgs │ In Bytes │ Out Bytes │ Subs │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────┼────────────┼──────────────────┼─────────┼──────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│ 5   │ rust-service             │ NCPL2SUNLVUONWTLOU4VQ6JENUJMGKHBREYUKAWM5QJRYQKGVSRJCAGU │ my_cluster │ 172.17.0.1:61368 │         │ 1h43m23s │ 10      │ 10       │ 40 B     │ 40 B      │ 1    │
│ 50  │ NATS CLI Version 0.1.5   │ NCPL2SUNLVUONWTLOU4VQ6JENUJMGKHBREYUKAWM5QJRYQKGVSRJCAGU │ my_cluster │ 172.17.0.1:63918 │         │ 0s       │ 1       │ 0        │ 254 B    │ 0 B       │ 1    │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────┼────────────┼──────────────────┼─────────┼──────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│     │ TOTALS FOR 2 CONNECTIONS │                                                          │            │                  │         │          │ 11      │ 10       │ 294 B    │ 40 B      │ 2    │
╰─────┴──────────────────────────┴──────────────────────────────────────────────────────────┴────────────┴──────────────────┴─────────┴──────────┴─────────┴──────────┴──────────┴───────────┴──────╯
```

### Test with docker nats cluster and wasmcloud

I have to find a way to have monitoring view of nats cluster launched by wasmcloud. Probably there are some settings to
modify to plug wasmcloud nats cluster to observabilty tooling of the docker nats cluster.

I didn't succeed to connect `wash dev` with docker nats cluster.


