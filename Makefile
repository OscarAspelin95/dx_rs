.PHONY: all build start stop clean clean-db

all: stop clean build start

build:
	docker compose build

start:
	docker compose up

stop:
	docker compose down -v

clean:
	docker system prune --volumes -f

clean-db:
	sudo rm -rf ./db/surrealdb/*
