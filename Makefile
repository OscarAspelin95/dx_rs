.PHONY: restart start stop clean


start:
	docker compose up


stop:
	docker compose down


clean:
	docker system prune --volumes -f


restart:
	stop clean start

# run:
# 	dx serve --release --desktop


# bundle-deb:
# 	dx bundle --package-types deb --desktop --release --out-dir bundle_deb --fullstack false
