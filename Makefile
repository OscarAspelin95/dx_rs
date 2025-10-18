.PHONY: all build start stop clean

all: stop build clean start

build:
	docker compose build

start:
	docker compose up

stop:
	docker compose down

clean:
	docker system prune --volumes -f
