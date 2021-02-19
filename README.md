# cr-tools

[![dependency status](https://deps.rs/crate/cr-tools/1.1.1/status.svg)](https://deps.rs/crate/cr-tools/1.1.1)

A simple calculator for [Clash Royale](https://clashroyale.fandom.com/wiki/Cards)

This crate implements both a simple library and a [Yew](https://yew.rs/docs/en/) web application.

Check out a live deployment of the `main` branch at <https://cr-tools.vercel.app>.

## Build process

Since this app is deployed using Vercel, it needs to run a build script to set up the build dependencies. This can be accomplished by running the following command in the repo root: `INSTALL_RUST_WASM_DEPS=true ./build.sh`. The script is made for Amazon Linux, and is meant to be run inside of an ephemeral container-like environment.

If your package manager isn't yum, or you don't want to have random dependencies installed to your system, run the following command to build the application directly: `trunk build -d public`, or run `trunk serve -d public` to run a dev server.

This obviously requires [trunk](https://trunkrs.dev/) to [be installed](https://trunkrs.dev/#install). To do that run:

```zsh
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

After this is installed I'd recommend you use `trunk serve -d public` to run the app.

## Licence & Copyright

Copyright (c) 2021 Bernd-L. All rights reserved.

![AGPL v3: Free as in Freedom](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)

cr-tools is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

cr-tools is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with cr-tools. If not, see <https://www.gnu.org/licenses/>.

This project (including its source code and its documentation) is released under the terms of the [GNU Affero General Public License](/LICENSE.md).

"Clash Royale" may be a trademark of its owner, with which I'm not affiliated with at all.
