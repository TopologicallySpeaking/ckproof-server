# Ckproof Server

This is the server which runs [ckproof.io](http://ckproof.io/).

## Building

This server uses Rust Nightly and Cargo as a build system. As of right now, the
easiest way to download the necessary files is through git.

```shell
mkdir ckproof && cd ckproof
git clone https://github.com/TopologicallySpeaking/ckproof.git
git clone https://github.com/TopologicallySpeaking/ckproof-treatise.git
git clone https://github.com/TopologicallySpeaking/ckproof-server.git
```

After that, you must also download the web fonts used. They are open source, but
are not distributed with this repository.

```shell
wget -O ckproof-server/static/fonts/iosevka-aile.zip https://github.com/be5invis/Iosevka/releases/download/v4.0.3/webfont-iosevka-aile-4.0.3.zip
unzip -d ckproof-server/static/fonts/ ckproof-server/static/fonts/iosevka-aile.zip
```

We can then compile the treatise.

```shell
cd ckproof/
cargo run ../ckproof-treatise/ ../ckproof-server/rendered.json
cd ..
```

Finally, run the server.

```shell
cd ckproof-server/
cargo run
```

After startup, the site may be accessed from `http://localhost:8000`.
Optionally, a different address or port may be used by passing the `BIND_ADDR`
environment variable.

```shell
BIND_ADDR=127.0.0.1:1337 cargo run
```

## Contributing

There are three repositories for this project hosted on GitHub. The
proof-checker is found [here](https://github.com/TopologicallySpeaking/ckproof),
the server is found
[here](https://github.com/TopologicallySpeaking/ckproof-server), and the
treatise is found
[here](https://github.com/TopologicallySpeaking/ckproof-treatise).

As of right now, I am not really looking for help on the proof checker or the
treatise. It took me about three years to figure out how the proof checker is
supposed to work, and it's going to take a while just to implement that plan. I
will accept bug fixes, for example, but at this stage things like feature
requests are going to cause more issues than they solve.

However, the server and frontend are going to need some TLC. In particular, the
[Table of Contents](http://ckproof.io/toc) is practically unusable. Furthermore,
there is no way to get to the homepage except by manually changing the url. If
someone can fix this and save me the trouble, that would be more than welcome.

## Licensing

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with this program.  If not, see <https://www.gnu.org/licenses/>.
