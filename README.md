# Hits-rs

A simple http server written in Rust, backed by SQLite, that serves a dynamic hits badge.

## Run using docker

Run:

```bash
docker run --rm -v hits_data:/app --env HITS_HOST=0.0.0.0:8088 -p 8088:8088 -it neslinesli93/hits-rs
```

You can replace the default port `8088` with whatever you like

## Run from source

Clone the repo:

```bash
$ git clone https://github.com/neslinesli93/hits-rs
```

Build the binary:

```bash
$ cargo build --release
```

Copy the `.env.example` file to `.env` and edit the environment variables if needed

Run the server:

```bash
$ ./target/release/hits-rs
```

## Usage

After launching the server, it will create a `hits.db` SQLite database and start listening on `127.0.0.1:8088`, serving the hits counter on `/hits.svg`.

Open [http://localhost:8088/hits.svg](http://localhost:8088/hits.svg) and check it out!
