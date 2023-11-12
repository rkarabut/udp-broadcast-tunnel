# udp-broadcast-tunnel

This program retransmits UDP broadcast packets with a given destination port to a number of specified network addresses, which may be required for certain games (e.g. Torchlight 2) to establish LAN connections over a tun-based layer 3 VPN. Source port spoofing works since version 0.2.0. Multiple ports are not supported yet.

## Building (Windows 10)

Download Npcap SDK at https://nmap.org/npcap/#download, then copy the `Lib/` folder into the project root.
Run `build.bat`.

## Running (Windows 10)

You will need Npcap dlls also available at https://nmap.org/npcap/#download (take note of the licensing terms).

Run once on the clients without arguments to find out the interface names, then run with following arguments:

1. interface used by Windows for UDP broadcast (the one with the lowest metric if not specified in the game options itself), usually a real device, not virtual, for example `MediaTek Wi-Fi 6 MT7921 Wireless LAN Card`
2. the port used by the game lobby;
3. the coma-separated IP addresses of other players in VPN network;
4. (optional, if source port spoofing required) your own IP address in the same VPN network

```
udp-broadcast-tunnel.exe 3 4549 10.8.0.2,10.8.0.3,10.8.0.4 10.8.0.7
```

If you don't see retransmitted packets when creating/joining a game, disable your firewall!

## Verified games

Port and game name in alphabetical order:

- `42801` Titan Quest - Anniversary Edition
- `4549` Torchlight 2
- `6112` Warcraft 3 (original Frozen Throne)

Specifically for Torchlight 2 this must be done on both client and server side, as the server seems to rely on a broadcast while negotiating the connection as well.
