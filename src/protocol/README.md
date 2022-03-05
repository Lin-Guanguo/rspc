# rspc network protocol

## Channel Connect
TODO: send a byte to identify channel type

then use Message Frame to communicate

## Message Frame

```
RequestFrame {
    request_id: u64,
    method_id: u32,
    body_len: u32,
    body: Bytes,
}
16 Bytes header with body

ReplyFrame {
    request_id: u64,
    status_code: u32,
    body_len: u32,
    body: Bytes,
}
16 Bytes header with body

```

