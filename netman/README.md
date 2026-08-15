# netman

A modular CLI network management tool for Windows.

`netman` provides a clean, unified command-line interface for managing Wi-Fi networks, Mobile Hotspots, LAN network adapters, IP discovery, and network diagnostics.

## Features

- **Unified Status Overview**: Quick overview of Wi-Fi, Ethernet, Mobile Hotspot, Internet connectivity, and local IP.
- **Wi-Fi Management**: Adapter power control (on/off), status check, network scanning, connecting, and disconnecting.
- **Mobile Hotspot**: Local persistence of hotspot profiles (SSID & password), easy startup (`netman hotspot up`), shutdown, and profile listing (never exposes passwords).
- **LAN Management**: Easily restart physical Ethernet/LAN network adapters.
- **Network Diagnostics**: Health check verifying Default Gateway reachability, DNS resolution, and Internet access.
- **IP Info**: Quick local IPv4 resolution.

---

## Command Reference

```text
netman
│
├── status                      # Show overall network status summary
│
├── wifi
│   ├── on                      # Turn Wi-Fi adapter on
│   ├── off                     # Turn Wi-Fi adapter off
│   ├── status                  # Show Wi-Fi state, connected SSID, signal %, local IP
│   ├── list                    # List available Wi-Fi networks & signal strengths
│   ├── connect <name> [pass]   # Connect to a Wi-Fi network
│   └── disconnect              # Disconnect from current Wi-Fi network
│
├── hotspot
│   ├── up [name]               # Start default/latest or specified hotspot profile
│   ├── down                    # Stop running mobile hotspot
│   ├── set <name> <password>   # Save/update a hotspot profile (password >= 8 chars)
│   └── list                    # List saved profiles (masks passwords)
│
├── lan
│   └── restart                 # Restart Ethernet/LAN adapter
│
├── ip                          # Display local IPv4 address
└── test                        # Run network connectivity diagnostics
```

---

## Installation

To install `netman` as a system CLI tool:

```bash
# Navigate to the netman project directory
cd netman

# Install netman globally or in your Python environment
pip install .
```

For editable development mode:
```bash
pip install -e .
```

After installation, you can directly run:
```bash
netman status
netman wifi status
netman hotspot up
netman ip
netman test
```

---

## Usage Examples

### Network Overview
```bash
netman status
```
Output:
```text
WiFi        Connected     NITJ
Ethernet    Disconnected
Hotspot     Off
Internet    Connected
IP          192.168.1.42
```

### Wi-Fi Operations
```bash
# Check Wi-Fi details
netman wifi status

# Scan available networks
netman wifi list

# Connect to a network
netman wifi connect HomeWi-Fi mysecretpass

# Disconnect
netman wifi disconnect
```

### Mobile Hotspot Operations
```bash
# Save a hotspot profile
netman hotspot set MyHotspot secretpass123

# List saved profiles (passwords are never displayed)
netman hotspot list

# Start default/latest hotspot profile
netman hotspot up

# Start specific profile by name
netman hotspot up OfficeHotspot

# Stop mobile hotspot
netman hotspot down
```

### LAN Operations
```bash
# Restart Ethernet network adapter
netman lan restart
```

### Diagnostic Test
```bash
netman test
```
Output:
```text
Running network connectivity test...

  [OK]   Local Gateway             : 192.168.1.1 (Reachable)
  [OK]   DNS Resolution            : Working
  [OK]   Internet Connectivity     : Connected

[+] Overall Status: Network connectivity is healthy.
```

---

## Uninstallation

To uninstall `netman` and remove the command from your PATH:

```bash
pip uninstall netman
```

To purge saved local hotspot configurations stored at `%LOCALAPPDATA%\netman`:
```cmd
rmdir /s /q "%LOCALAPPDATA%\netman"
```
