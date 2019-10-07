# cmus-discord-rpc

[![travis-ci Build Status][travis-ci-badge]][travis-ci-page] [![Discord][discord-badge]][discord-invite]

Discord Rich Presence integration for the C* Music Player (`cmus`).

## Installing

- If it isn't already on your system, install `rust`, and `cargo`. On \*nix based systems this should be as simple as installing it from your package manager.

- Obtain the sources. You can either do this by cloning the repository using `git` or downloading an archive of the repository.

      git clone https://github.com/Bond-009/cmus-discord-rpc

  or alternatively:

       wget https://github.com/Bond-009/cmus-discord-rpc/archive/master.zip

       unzip master.zip

- Change your directory into where the sources were cloned/extracted to.

      cd cmus-discord-rpc-master

- Next, build and install it to your home directory.
    
      cargo install --path .

- Once `cargo`'s installation directory is in your `PATH` (`cargo` should tell you where the end of the previous step) simply run `cmus-discord-rpc` and it should start!

## Building

- Obtain the sources. You can either do this by cloning the repository or downloading an archive of the repository.

- Change your directory into where the sources were cloned/extracted to.

- Finally to build, use the following commands:
  
  For debugging:
        
      cargo build

  For production use:
      
      cargo build --release

- You should see a new directory called `target`. You can find subfolders for each of your build targets.

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.

[travis-ci-badge]: https://travis-ci.org/Bond-009/cmus-discord-rpc.svg?branch=master
[travis-ci-page]: https://travis-ci.org/Bond-009/cmus-discord-rpc
[discord-badge]: https://discordapp.com/api/guilds/261241776105455618/widget.png
[discord-invite]: https://discordapp.com/invite/thKXwJb
