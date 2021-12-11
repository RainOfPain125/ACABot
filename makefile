# This file is part of aca_bot.

# aca_bot is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# aca_bot is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with aca_bot.  If not, see <https://www.gnu.org/licenses/>.

target = target/

build:
	cargo build --release &
	@disown -a

all:
	cargo build --release &
	@disown -a

dev:
	cargo run &
	@disown -a

test:
	cargo test &
	@disown -a

clean:
	rm -rf target

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

bench:
	cargo bench --all-features &
	@disown -a

doc:
	cargo doc --open --no-deps

help:
	@echo "make			- Build crate"
	@echo "make build		- Alias of make"
	@echo "make all		- Alias of make"
	@echo "make dev		- Build unoptimized dev build and run it"
	@echo "make doc		- Compile and view docs in web browser"
	@echo "make clean		- Delete target/ directory"
	@echo "make test		- Build and run tests"
	@echo "make bench		- Build and run benches"
	@echo "make style-check	- Applies idiomatic styling to code"
	@echo "make help		- Print this help message"