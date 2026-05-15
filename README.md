# fanbox-dl

A high-performance CLI tool written in Rust to download content from Pixiv Fanbox.

## 🚀 Features

* **Sequential Downloading**: Uses the post's "blocks" structure to ensure images and files are downloaded in the exact order the creator intended.
* **Ordered Filenames**: Automatically prefixes files (e.g., `001_`, `002_`) so they sort correctly in your file explorer.
* **Chronological Organization**: Post directories are named using the `[YYYY-MM-DD] Title` format for easy browsing.
* **Retry Logic**: Robust error handling that waits and retries on network hiccups.
* **NordVPN Integration**: Automatically rotates your VPN server using the NordVPN CLI if you hit Cloudflare rate limits (`--auto-vpn`).

---

## 📦 Installation

Download the latest executable for your operating system from the [Releases](https://www.google.com/search?q=https://github.com/gamerkitten74/fanbox-dl/releases) page.

---

## 🛠 Usage

Open your terminal and run the binary with your Fanbox session ID and the creator's ID.

```bash
./fanbox-dl --creator <CREATOR_ID> --session-id <YOUR_SESSION_ID>

```

### Basic Example

```bash
./fanbox-dl --creator realcreator --session-id "12345678_abcdefghijklmnopqrstuvwxyz" --out-dir "./downloads"

```

### Advanced Usage (with VPN rotation)

```bash
./fanbox-dl --creator realcreator --session-id "12345678_abcdefghijklmnopqrstuvwxyz" --auto-vpn --dir-by-post=false --all

```

---

## 🔑 How to get your Session ID

1. Log into [fanbox.cc](https://fanbox.cc) in your browser.
2. Open **Developer Tools** (F12 or Right-click > Inspect).
3. Go to the **Application** tab (Chrome/Edge) or **Storage** tab (Firefox).
4. In the left sidebar, expand **Cookies** and select `https://www.fanbox.cc`.
5. Find the cookie named `FANBOXSESSID`.
6. Copy the **Value**—this is your `--session-id`.

---

## ⌨️ Command Line Arguments

| Argument | Description |
| --- | --- |
| `--creator` | The ID of the creator (found in the URL: `creatorid.fanbox.cc`) |
| `--session-id` | Your `FANBOXSESSID` cookie value |
| `--out-dir` | Directory where files will be saved (Default: `./downloads`) |
| `--dir-by-post` | Create a separate folder for each post (Default: `true`) |
| `--all` | Overwrite existing files instead of skipping them |
| `--sorting` | Sort order: `newest` or `oldest` (Default: `newest`) |
| `--skip-images` | Do not download images |
| `--skip-files` | Do not download attached files (.zip, .pdf, etc.) |
| `--auto-vpn` | Automatically rotate NordVPN servers when rate-limited |

<sub>The NordVPN implementation was made specifically for me. If you don't have NordVPN, whoops.</sub>

---



## ⚖️ License

This project is for personal use only. Please respect the creators and their terms of service. Do not redistribute paid content without permission.
