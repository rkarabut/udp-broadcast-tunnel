# udp-broadcast-tunnel

This program retransmits UDP broadcast packets with a given destination port to a number of specified network addresses, which may be required for certain games (e.g. Torchlight 2) to establish LAN connections over a tun-based layer 3 VPN. While actual source spoofing is not implemented, games requiring a matching source port to work (e.g. Warcraft 3) are currently supported with a hack. Retransmitting over multiple ports is not supported yet.

## Building (Windows 10)

Download Npcap SDK at https://nmap.org/npcap/#download, then copy the `Lib/` folder into the project root.
Run `build.bat`.

## Running (Windows 10)

You will need Npcap dlls also available at https://nmap.org/npcap/#download (take note of the licensing terms).

Run once on the clients without arguments to find out the interface names, then run with the interface used by Windows for UDP broadcast (the one with the lowest metric if not specified in the game options itself, please check [here](http://ratmirkarabut.com/articles/setting-up-openvpn-server-for-torchlight-2/) for details), also passing the port used by the game lobby and the IP addresses of other players:

```
udp-broadcast-tunnel.exe 3 4549 10.8.0.2,10.8.0.3,10.8.0.4
```

Specifically for Torchlight 2 this must be done on both client and server side, as the server seems to rely on a broadcast while negotiating the connection as well.

Use `set RUST_LOG=debug` to inspect the retransmitted packets.

### Warcraft 3 hack 

In case the source port must be set and is also matching the destination (e.g. Warcraft 3), append the current client's IP address:

```
udp-broadcast-tunnel.exe 3 4549 10.8.0.2,10.8.0.3,10.8.0.4 10.8.0.7
```

This will make the tool attempt to momentarily bind to the given address to send out the captured packets, and should work for games listening for the lobby connections on `0.0.0.0`, temporarily preventing them from receiving packets while retransmitting as a side effect. It might get unstable with increasing number of clients.

## Games confirmed working

Ports used by the games, listed in alphabetical order:

- Titan Quest - Anniversary Edition `42801`
- Torchlight 2 `4549`
- Warcraft 3 (also Frozen Throne) `6112`

## Credits

[Vladyslav Tsilytskyi](https://github.com/tvladyslav) for figuring out a way to support Warcraft 3