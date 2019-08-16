#!/usr/bin/env bash

export PG_URL=postgres://shortener:tentenshortener@localhost:5432/shortener

cargo watch -x run
