# cmus-discord-rpc

[![travis-ci Build Status][travis-ci-badge]][travis-ci-page] [![Discord][discord-badge]][discord-invite]

Discord Rich Presence integration for the C* Music Player (`cmus`).

## Installing

- If it isn't already on your system, install Rust, `cargo`, and `git`. On \*nix based systems this should be as simple as installing it from your package manager. Otherwise follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust, and [here](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) to install `git`.

- Next, clone the git repository with `git clone https://github.com/Bond-009/cmus-discord-rpc`.

- Change your directory into the cloned repository.

- Run `cargo install --path .`. This will install it to a directory in your home/user folder.

- `cargo` may warn that you need to add the installation directory to `PATH`. If you're not sure how to on your platform, check out [this handy guide](https://github.com/sindresorhus/guides/blob/master/set-environment-variables.md). `cargo` should tell you what directory to add to`PATH`.

- Lastly, type `source ~/.bashrc` on a \*nix based system using bash, or if you're running Windows simply restart CMD.

- If you set up your path correctly, simply type `cmus-discord-rpc` and it should run!

## Building

- Follow the above guide until the third step.

- Run `cargo build`

- You should see a directory called `target` be created. You can find your built binaries there.

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
along with this program.  If not, see <https://www.gnu.org/licenses/>.

[travis-ci-badge]: https://travis-ci.org/Bond-009/cmus-discord-rpc.svg?branch=master
[travis-ci-page]: https://travis-ci.org/Bond-009/cmus-discord-rpc
[discord-badge]: https://discordapp.com/api/guilds/261241776105455618/widget.png
[discord-invite]: https://discordapp.com/invite/thKXwJb
