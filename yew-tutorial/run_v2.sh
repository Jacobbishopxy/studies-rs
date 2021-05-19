#!/bin/bash

help() {
    echo "Run this script to get a dev-env"
}

build(){
    wasm-pack build --target web
}

pack(){
    cmd="rollup -c"
    $cmd &
    PID=$!
    echo $PID > .serverpid.rollup
}

run(){
    thttp -p 8080 &
    PID=$!
    echo $PID > .serverpid
}

watch() {
    cargo watch -w ./src --ignore "*.js" --postpone -s "./run_v2.sh --reload"
}

stop() {
    kill $(cat .serverpid)
    rm .serverpid
}

while [ "$1" != "" ]; do
    case $1 in
        -r | --reload )         stop
                                build
                                run
                                exit
                                ;;
        -h | --help )           help
                                exit
                                ;;
    esac
    shift
done


if [ -f .serverpid ]; then
    echo "Server already running, or in an inconsistent state"
    exit 1
fi

build && pack && run && watch
