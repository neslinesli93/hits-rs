# Hits-rs

A simple http server written in Rust, backed by SQLite, that serves a dynamic hits badge.

## Run using docker

Run:

```bash
docker run --rm -v hits_data:/app/data -p 8088:8088 -it neslinesli93/hits-rs
```

## Run from source

Clone the repo:

```bash
$ git clone https://github.com/neslinesli93/hits-rs
```

Build the binary:

```bash
$ cargo build --release
```

(Optional) Copy the `.env.example` file to `.env` and edit the environment variables if needed

Run the server:

```bash
$ ./target/release/hits-rs
```

## Usage

After launching the server, it will create a `hits.db` SQLite database inside `data/` and start listening on `0.0.0.0:8088`, serving the hits counter on `/hits.svg`.

Open [http://localhost:8088/hits.svg](http://localhost:8088/hits.svg) and check it out!

## Environment variables

#### HITS_HOST

The **address** on which the http server listens to incoming requests. Defaults to `0.0.0.0`

#### HITS_HOST

The **port** on which the http server listens to incoming requests. Defaults to `8088`

#### HITS_ENDPOINT

The **route** registered on the http server to serve the image of the badge. Defaults to `/hits.svg`
