# udp-broadcast-tunnel

This program retransmits UDP broadcast packets with a given destination port to a number of specified network addresses, which may be required for certain games (e.g. Torchlight 2) to establish LAN connections over a tun-based layer 3 VPN. Games requiring a specific source port to work (e.g. Warcraft 3, Grim Dawn), are currently supported with a hack which might result in dropping some packets (should not be an issue for most games only using broadcast sparingly). Retransmitting over multiple ports is not supported yet.

## Building (Windows 10)

Download Npcap SDK at https://nmap.org/npcap/#download, then copy the `Lib/` folder into the project root.
Run `build.bat`.

## Running (Windows 10)

You will need Npcap dlls also available at https://nmap.org/npcap/#download (take note of the licensing terms).

Run once on the clients without arguments to find out the interface names, then run with the interface used by Windows for UDP broadcast (the one with the lowest metric if not specified in the game options itself), also passing the port used by the game lobby and the IP addresses of other players:

```
udp-broadcast-tunnel.exe 3 4549 10.8.0.2,10.8.0.3,10.8.0.4
```

Specifically for Torchlight 2 this must be done on both client and server side, as the server seems to rely on a broadcast while negotiating the connection as well.

In case source port spoofing is required, append the current client's IP address in the network:

```
udp-broadcast-tunnel.exe 3 4549 10.8.0.2,10.8.0.3,10.8.0.4 10.8.0.7
```

Use `set RUST_LOG=debug` to inspect the retransmitted packets. Check your firewall rules if you don't see any.

## Verified games

Ports used by game lobbies, in alphabetical order:

- `42801` Titan Quest - Anniversary Edition
- `4549` Torchlight 2
- `6112` Warcraft 3 (Frozen Throne)
