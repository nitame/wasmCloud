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
│ 49  │                          │ NC2GLHPS32LS6YFOHXBIVJ722QJPHGXJVUXGC7DE3QKXAUQCRSHLKFL2 │         │ 127.0.0.1:52591 │         │ 15s    │ 2       │ 2        │ 887 B    │ 392 B     │ 1    │
│ 52  │                          │ NC2GLHPS32LS6YFOHXBIVJ722QJPHGXJVUXGC7DE3QKXAUQCRSHLKFL2 │         │ 127.0.0.1:52592 │         │ 15s    │ 6       │ 19       │ 1.2 KiB  │ 12 KiB    │ 1    │
│ 58  │ NATS CLI Version 0.1.5   │ NC2GLHPS32LS6YFOHXBIVJ722QJPHGXJVUXGC7DE3QKXAUQCRSHLKFL2 │         │ 127.0.0.1:52607 │         │ 0s     │ 1       │ 0        │ 254 B    │ 0 B       │ 1    │
│ 35  │                          │ NC2GLHPS32LS6YFOHXBIVJ722QJPHGXJVUXGC7DE3QKXAUQCRSHLKFL2 │         │ 127.0.0.1:52590 │         │ 15s    │ 4       │ 1        │ 610 B    │ 16 B      │ 2    │
│ 57  │ http-server-provider     │ NC2GLHPS32LS6YFOHXBIVJ722QJPHGXJVUXGC7DE3QKXAUQCRSHLKFL2 │         │ 127.0.0.1:52603 │         │ 11s    │ 1       │ 1        │ 16 B     │ 0 B       │ 5    │
│ 6   │                          │ NC2GLHPS32LS6YFOHXBIVJ722QJPHGXJVUXGC7DE3QKXAUQCRSHLKFL2 │         │ 127.0.0.1:52586 │         │ 18s    │ 312     │ 336      │ 37 KiB   │ 128 KiB   │ 6    │
│ 36  │                          │ NC2GLHPS32LS6YFOHXBIVJ722QJPHGXJVUXGC7DE3QKXAUQCRSHLKFL2 │         │ 127.0.0.1:52589 │         │ 15s    │ 74      │ 71       │ 22 KiB   │ 12 KiB    │ 13   │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│     │ TOTALS FOR 7 CONNECTIONS │                                                          │         │                 │         │        │ 400     │ 430      │ 62 KIB   │ 152 KIB   │ 29   │
╰─────┴──────────────────────────┴──────────────────────────────────────────────────────────┴─────────┴─────────────────┴─────────┴────────┴─────────┴──────────┴──────────┴───────────┴──────╯
```

## Attemps to display names

Add name to nats client connection options in several crates: wash-lib, wash-cli, host(wasmbus),
provider-lattice-controller, secrets-nats-kv. But unnamed connections remained.

```text
╭─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│                                                                                Top 7 Connections out of 7 by subs                                                                               │
├─────┬──────────────────────────────┬──────────────────────────────────────────────────────────┬─────────┬─────────────────┬─────────┬────────┬─────────┬──────────┬──────────┬───────────┬──────┤
│ CID │ Name                         │ Server                                                   │ Cluster │ IP              │ Account │ Uptime │ In Msgs │ Out Msgs │ In Bytes │ Out Bytes │ Subs │
├─────┼──────────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│ 55  │ wasmcloud-washlibconfig-0176 │ NBDV5TBBVWBQL77XQFKQWDF7IUUMRX5HA5ZSKVJ6S6KKDK34JPMFSVPO │         │ 127.0.0.1:53069 │         │ 19s    │ 2       │ 2        │ 887 B    │ 392 B     │ 1    │
│ 56  │ wasmcloud-washlibconfig-5421 │ NBDV5TBBVWBQL77XQFKQWDF7IUUMRX5HA5ZSKVJ6S6KKDK34JPMFSVPO │         │ 127.0.0.1:53070 │         │ 18s    │ 3       │ 5        │ 240 B    │ 3.1 KiB   │ 1    │
│ 60  │ NATS CLI Version 0.1.5       │ NBDV5TBBVWBQL77XQFKQWDF7IUUMRX5HA5ZSKVJ6S6KKDK34JPMFSVPO │         │ 127.0.0.1:53081 │         │ 0s     │ 1       │ 0        │ 254 B    │ 0 B       │ 1    │
│ 49  │                              │ NBDV5TBBVWBQL77XQFKQWDF7IUUMRX5HA5ZSKVJ6S6KKDK34JPMFSVPO │         │ 127.0.0.1:53068 │         │ 19s    │ 4       │ 1        │ 610 B    │ 16 B      │ 3    │
│ 46  │                              │ NBDV5TBBVWBQL77XQFKQWDF7IUUMRX5HA5ZSKVJ6S6KKDK34JPMFSVPO │         │ 127.0.0.1:53066 │         │ 19s    │ 174     │ 187      │ 19 KiB   │ 72 KiB    │ 6    │
│ 59  │ http-server-provider         │ NBDV5TBBVWBQL77XQFKQWDF7IUUMRX5HA5ZSKVJ6S6KKDK34JPMFSVPO │         │ 127.0.0.1:53077 │         │ 14s    │ 1       │ 1        │ 16 B     │ 0 B       │ 6    │
│ 50  │                              │ NBDV5TBBVWBQL77XQFKQWDF7IUUMRX5HA5ZSKVJ6S6KKDK34JPMFSVPO │         │ 127.0.0.1:53067 │         │ 19s    │ 49      │ 56       │ 13 KiB   │ 10 KiB    │ 13   │
├─────┼──────────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│     │ TOTALS FOR 7 CONNECTIONS     │                                                          │         │                 │         │        │ 234     │ 252      │ 34 KIB   │ 86 KIB    │ 31   │
╰─────┴──────────────────────────────┴──────────────────────────────────────────────────────────┴─────────┴─────────────────┴─────────┴────────┴─────────┴──────────┴──────────┴───────────┴──────╯
```

Test is made with a simple http project. Add some components to wasm project to test with more connections. Connections
with messaging-image-processor-worker example:

```text
╭─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│                                                                               Top 10 Connections out of 10 by subs                                                                              │
├─────┬──────────────────────────────┬──────────────────────────────────────────────────────────┬─────────┬─────────────────┬─────────┬────────┬─────────┬──────────┬──────────┬───────────┬──────┤
│ CID │ Name                         │ Server                                                   │ Cluster │ IP              │ Account │ Uptime │ In Msgs │ Out Msgs │ In Bytes │ Out Bytes │ Subs │
├─────┼──────────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│ 43  │ wasmcloud-washlibconfig-3589 │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53760 │         │ 33s    │ 3       │ 5        │ 344 B    │ 3.5 KiB   │ 1    │
│ 58  │ wasmcloud-washlibconfig-1819 │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53765 │         │ 32s    │ 2       │ 2        │ 1.6 KiB  │ 456 B     │ 1    │
│ 64  │ NATS Messaging Provider      │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53787 │         │ 6s     │ 0       │ 0        │ 0 B      │ 0 B       │ 1    │
│ 67  │ NATS CLI Version 0.1.5       │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53792 │         │ 0s     │ 1       │ 0        │ 254 B    │ 0 B       │ 1    │
│ 53  │                              │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53764 │         │ 32s    │ 14      │ 3        │ 2.5 KiB  │ 48 B      │ 3    │
│ 49  │                              │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53762 │         │ 32s    │ 309     │ 338      │ 58 KiB   │ 195 KiB   │ 6    │
│ 65  │ http-client-provider         │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53789 │         │ 5s     │ 1       │ 1        │ 16 B     │ 0 B       │ 6    │
│ 63  │ messaging-nats-provider      │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53786 │         │ 6s     │ 1       │ 1        │ 16 B     │ 0 B       │ 7    │
│ 52  │                              │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53763 │         │ 32s    │ 96      │ 104      │ 38 KiB   │ 20 KiB    │ 14   │
│ 66  │ blobstore-fs-provider        │ NBZLGPTGNV66T7NELUKTKOCTPI6PG76ZE3J76NGBFAKFQKKXAGWGH3JF │         │ 127.0.0.1:53790 │         │ 5s     │ 1       │ 1        │ 16 B     │ 0 B       │ 19   │
├─────┼──────────────────────────────┼──────────────────────────────────────────────────────────┼─────────┼─────────────────┼─────────┼────────┼─────────┼──────────┼──────────┼───────────┼──────┤
│     │ TOTALS FOR 10 CONNECTIONS    │                                                          │         │                 │         │        │ 428     │ 455      │ 100 KIB  │ 218 KIB   │ 59   │
╰─────┴──────────────────────────────┴──────────────────────────────────────────────────────────┴─────────┴─────────────────┴─────────┴────────┴─────────┴──────────┴──────────┴───────────┴──────╯
```
