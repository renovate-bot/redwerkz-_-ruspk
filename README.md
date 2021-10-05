# Synorepo-rs

[![ruspk's current version badge](https://img.shields.io/crates/v/ruspk.svg)](https://crates.io/crates/ruspk)

synorepo-rs is a super fast synology repository server. It uses the existing database structure from [spkrepo](https://github.com/SynoCommunity/spkrepo)

Only postgres is supported at the moment.

## Install

```sh
cargo install diesel_cli
cargo install ruspk --features postgres
cargo install ruspk --no-default-features --features mysql
cargo install ruspk --no-default-features --features sqlite
echo 'DATABASE_URL=postgresql://user:pass@localhost/dbname' > .env
diesel migration --migration-dir migrations/postgres/ run
ruspk
```

Available Features: `mysql`, `postgres` and `sqlite`

### Test the API

```sh
# NAS package list request
curl -sv 'http://127.0.0.1:8080/?package_update_channel=beta&unique=synology_apollolake_418play&build=24922&language=enu&major=6&micro=2&arch=apollolake&minor=2&timezone=Melbourne&nano=4' | jq

# upload new package (wip)
http --verify=no --ignore-stdin --auth $PUBLISH_API_KEY: POST $PUBLISH_URL/packages @$SPK_FILE_NAME
```

## Configuration (`.env` file)

```env
## Log levels for each component
# RUST_LOG="ruspk=info,actix_web=info,actix_server=info"
## Or generic
RUST_LOG="info"
## For web server logs set one of
# RUST_LOG="info"
# RUST_LOG="actix_web=info"
## For verbose logs
# RUST_LOG="trace"

## Database connection
# DATABASE_URL=file:db/database.sqlite
# DATABASE_URL=mysql://user:pass@localhost/dbname
# DATABASE_URL=postgresql://user:pass@localhost/dbname

## IP address to Bind to and listen for connections
LISTEN=0.0.0.0

## Port to Bind to and listen for connections
PORT=80

## URL to prepend for spk archive, icon and screenshot files
URL=https://packages.synocommunity.com

## Public key to advertise for signed packages
PUBLIC_KEY_FILE=pubkey.pem

## Time in seconds to allow stale responses to be served from memory cache
CACHE_TTL=600
```

# Dev Guides

<https://diesel.rs/>

<https://actix.rs/>

<https://yew.rs/>

<https://github.com/SynoCommunity/spksrc/wiki/Package-Center-specifications>

<http://spkrepo.readthedocs.org/>

## Backup and restore database

```sh
cd server/db
pg_dump -U ruspk ruspk > ruspk.sql
psql -U ruspk -d ruspk -f ruspk.sql
```

## development

```sh
systemctl start postgresql
mkdir -p frontend/dist
cargo run
yarn --cwd frontend dev
# rustup override add nightly
# rustup override unset
```

### Debugging

`RUST_BACKTRACE=1 CACHE_TTL=0 RUST_LOG="warn,ruspk=trace,actix_web=info,actix_server=info" cargo run`

## lint

```sh
# formatting
# $ rustup component add rustfmt
cargo fmt
# fix common mmistakes
# $ rustup component add clippy
cargo clippy
# check security advisories
# $ cargo install cargo-audit
cargo audit
# Get latest versions defined in Cargo.toml
# $ cargo install cargo-update
cargo update
# check of errors
cargo check
# show errors in your favourite editor
# https://rust-analyzer.github.io/manual.html#rust-analyzer-language-server-binary
# $ rustup +nightly component add rust-analyzer-preview
```

## release

```sh
cargo build --release
RUST_LOG="warn" target/release/ruspk
yarn --cwd frontend export -o dist
cargo publish
```

Optimised build (**not** used for benchmarks and no measurable improvement):

`RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release`

<https://github.com/image-rs/image>
