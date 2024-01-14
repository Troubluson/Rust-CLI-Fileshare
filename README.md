# Rust-CLI-Fileshare
A simple tool for easy filesharing through commandline.

## Motivation
Sending files over 15MB in size is often not allowed through many online communication services, such as Discord, and I don't want to go through the trouble of always figuring out how I am going to share files, so I built a small comandline tool to help me with that in an afternoon to get a bit familiar with Rust and see what the hype is all about.

I found that Rust is not right technology to send multi-part forms due to type safety, as subcommands that the API allows requires boolean or integer values, and the Reqwest multipart module only allows bytes or text to be sent. This is why I am going to do a Python implementation as well to test out. 

## Requirements (to build)

* Rust and Cargo

No api-keys or other forms of authentication are required

## Usage

Build using `Cargo build` and the built `.exe` can be used using syntax

`cli_fileshare.exe [-o <output_file_name>] <Path_to_file>`

If uploading succeed, the path to the download is copied to clipboard as well as displayed in the output, which can be shared to the recipient.
> **_NOTE:_**  Allows up to 2GB files to be sent

## To Be Done

* Pattern matching for file if correct name isn't given or doesn't exist in current directory as well as picking and confirmation of possible matching files.
* Build same tool in Python, which should allow setting optional subcommands correctly and requires less code.
* Add more information to `--help`

