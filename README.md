# Hits-rs

<a href="https://github.com/neslinesli93/hits-rs/actions"><img src="https://img.shields.io/github/workflow/status/neslinesli93/hits-rs/CI/master" alt="Build status (master)" /></a></div>
<a href="https://hub.docker.com/r/neslinesli93/hits-rs/tags"><img src="https://img.shields.io/docker/v/neslinesli93/hits-rs" alt="Docker Image Version (latest by date)" /></a></div>
<a href="https://github.com/neslinesli93/hits-rs/releases"><img src="https://img.shields.io/github/v/release/neslinesli93/hits-rs" alt="GitHub release (latest by date)" /></a></div>

A simple http server written in Rust, backed by SQLite, that renders an SVG image with a dynamic hits counter.

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

## Params

- `lower`

Decides whether to show the `hits` text in lowercase. Defaults to false.

## Environment variables

#### HITS_HOST

The **address** on which the http server listens to incoming requests. Defaults to `0.0.0.0`

#### HITS_HOST

The **port** on which the http server listens to incoming requests. Defaults to `8088`

#### HITS_ENDPOINT

The **route** registered on the http server to serve the image of the badge. Defaults to `/hits.svg`
