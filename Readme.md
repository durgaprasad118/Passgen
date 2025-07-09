# cli-pass

A fast, flexible command-line password generator.

## Install

`cargo install cli-pass`

## Usage

`cli-pass --length 12 --numbers --symbols --upper --down`

| Flag            | Description                  |
| --------------- | ---------------------------- |
| `-l, --length`  | Password length (default: 6) |
| `-n, --numbers` | Include numbers              |
| `-s, --symbols` | Include symbols              |
| `-u, --upper`   | Include uppercase letters    |
| `-d, --down`    | Include lowercase letters    |

**At least one of `--numbers` or `--symbols` must be included.**

## Example

`cli-pass -l 10 -n -s -u -d`

- Your password is : 9@qP2aF#bL
