.PHONY: all build start stop clean

all: stop clean build start

build:
	docker compose build

start:
	docker compose up

stop:
	docker compose down -v

clean:
	docker system prune --volumes -f
