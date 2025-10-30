.PHONY: all build start stop clean logs-% clean-db clean-nats clean-minio clean-all

all: stop clean build start

build:
	docker compose build

start:
	docker compose up

stop:
	docker compose down -v

clean:
	docker system prune --volumes

logs-%:
	docker compose logs -f $*

clean-db:
	sudo rm -rf ./db/surrealdb/*

clean-nats:
	sudo rm -rf ./nats/*

clean-minio:
	sudo rm -rf ./minio/*

clean-all:
	sudo rm -rf ./data/surrealdb/* && sudo rm -rf ./data/nats/* && sudo rm -rf ./data/minio/* && docker system prune --volumes -f
