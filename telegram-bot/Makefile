
# read configs from .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# connect to proxy server
proxy:
	export http_proxy=${MY_PROXY_URL}:${MY_PROXY_PORT}
