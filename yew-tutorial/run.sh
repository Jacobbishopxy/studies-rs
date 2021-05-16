#!/usr/bin/env bash

# source .env
# export py=${PY}

build(){
    wasm-pack build --target web
}

pack(){
   rollup ./main.js --format iife --file ./pkg/bundle.js
}

run(){
    python3 -m "http.server" "8080" &
    # $py -m "http.server" "8080" &
    PID=$!
    echo $PID > .serverpid
}

build
pack
run
