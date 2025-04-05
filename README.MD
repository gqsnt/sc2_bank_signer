# SC2 Bank Signer

A command-line utility written in Rust to parse, validate, and optionally re-sign StarCraft II `.SC2Bank` files.

## Overview

StarCraft II uses `.SC2Bank` files to store persistent data for custom maps and mods. These files are XML-based and contain a digital signature to ensure data integrity. This tool allows developers, modders, and researchers to:
1.  **Parse** the structure of a `.SC2Bank` file.
2.  **Calculate** the expected signature based on the bank's content and associated metadata (author handle, player handle, bank name).
3.  **Compare** the calculated signature against the signature currently stored within the file.
4.  **Update** (re-sign) the bank file with the newly calculated signature, which is useful after modifying bank data outside the game client or for analysis purposes.

## Features

* **XML Parsing:** Reads and interprets the structure of `.SC2Bank` files, including Sections, Keys, and Value elements with various attribute types (`int`, `fixed`, `flag`, `string`, `text`).
* **Signature Validation:** Compares the computed signature against the existing `<Signature value="..."/>` tag in the bank file.
* **Signature Replacement:** Optionally overwrites the existing signature in the bank file with the re-calculated one using the `--write` flag.
* **Path Metadata Extraction:** Attempts to automatically determine the `Author Handle`, `Player Handle`, and `Bank Name` required for signing based on the standard StarCraft II bank file directory structure.
* **Metadata Override:** Allows explicit specification of `Author Handle`, `Player Handle`, and `Bank Name` via command-line arguments, useful if the file path doesn't match the standard structure or if simulating different contexts.
* **Cross-Platform:** Built with Rust, it should compile and run on Windows, macOS, and Linux.

## Installation

### Prerequisites

* **Rust:** Requires Rust and Cargo. You can install them using [rustup](https://rustup.rs/).

### Building from Source

1.  **Clone the repository:**
    ```bash
    git clone <your-repository-url>
    cd sc2_bank_signer
    ```
2.  **Build the release executable:**
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/sc2_bank_signer` (or `target\release\sc2_bank_signer.exe` on Windows). You can copy this executable to a location in your system's PATH for easier access.

## Usage

The tool is operated via the command line.

```bash
sc2_bank_signer [OPTIONS] <BANK_PATH>
```

### Arguments

- `<BANK_PATH>`: (Required)  Filepath to the bank file (.SC2Bank).

####  Options

- ` -n, --bank-name <BANK_NAME>`:<br>Overrides the bank name derived from the filename. Use the logical name the bank should have for signing.

- `-a, --author <AUTHOR_HANDLE>`:<br>Overrides the author handle (e.g., 1-S2-1-AUTHOR_ID) derived from the file path.

- `-p, --player <PLAYER_HANDLE>`:<br>Overrides the player handle (e.g., 2-S2-1-PLAYER_ID) derived from the file path.

- `-w, --write`:<br>If specified, the tool will replace the existing signature in the file with the newly computed one if they differ. If signatures match, no changes are made. Requires an existing `<Signature .../>` tag in the file.

- `-h, --help`:<br>Prints help information.

- `-V, --version`:<br>Prints version information.

## Examples

1. Check the signature of a bank file (automatic path detection):
    ```bash 
        sc2_bank_signer "C:\Users\user\Documents\StarCraft II\Accounts\ACCOUNT_ID\2-S2-1-PLAYER_ID\Banks\1-S2-1-AUTHOR_ID\MyBank.SC2Bank"
    ```
2. Check and update the signature if it's incorrect:
    ```bash
        sc2_bank_signer "C:\Users\user\Documents\StarCraft II\Accounts\ACCOUNT_ID\2-S2-1-PLAYER_ID\Banks\1-S2-1-AUTHOR_ID\MyBank.SC2Bank" --write
    ```

3. Check a bank where path detection might fail, providing overrides:
    ```bash
        sc2_bank_signer "/path/to/some/folder/MyBank.SC2Bank" -n "MyBank" --author "1-S2-1-AUTHOR_ID" --player "2-S2-1-PLAYER_ID"
    ```

4. Check and update a bank with overrides:
    ```bash
        sc2_bank_signer "/path/to/some/folder/MyBank.SC2Bank" -n "MyBank" -a "1-S2-1-AUTHOR_ID" -p "2-S2-1-PLAYER_ID" --write
    ```


### Expected File Structure for Auto-Detection

For the automatic detection of `Author Handle`, `Player Handle`, and `Bank Name` to work correctly, the tool expects the `.SC2Bank` file to be located in a path matching the standard StarCraft II structure:

`... / StarCraft II / Accounts / <account_id> / <PLAYER_HANDLE> / Banks / <AUTHOR_HANDLE> / <BankName>.SC2Bank`

If your file is not in such a path, you must use the `--bank-name`, `--author`, and `--player` options to provide the correct metadata for accurate signature calculation.

## Error Handling
The tool provides error messages if:
- The specified bank file cannot be found or read.
- The file path structure doesn't allow automatic handle detection (and overrides are not provided).
- The XML structure is invalid or missing required attributes (`name` for Section/Key).
- An IO error occurs during file writing (when using `--write`).
- The `--write` flag is used but no `<Signature .../>` tag exists in the file.