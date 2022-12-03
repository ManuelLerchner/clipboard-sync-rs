# Clipboard-Sync-rs

A simple clipboard sync tool written in Rust.

It works by listening to the `Senders` clipboard and sending the contents to a supplied URL using a POST request.
On the receiving end, the server can then read the sent data and can update the `Receivers` clipboard accordingly.

## Usage

Download the latest executable from the [releases](https://github.com/ManuelLerchner/clipboard-sync-rs/releases)-page. And start the programm with the correct arguments.

### Receiver Mode

To start the application, in `listening` mode, run:
This starts a local server and listens for incoming clipboard updates.

```bash
  .\clipboard-sync-rs.exe --sync-mode receiver
```

### Sender Mode

To start the application, in `sending` mode, run:
This starts a daemon that sends clipboard updates to each specified receiver.

```bash
  .\clipboard-sync-rs.exe --sync-mode sender --reporting-url <url1>
```

### Bidirectional Mode

To start the application, in `bidirectional` mode, run:

```bash
  .\clipboard-sync-rs.exe --sync-mode bidirectional --reporting-url <url1>
```
