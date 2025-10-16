.PHONY: run run-release bundle-deb

run:
	dx serve

run-release:
	dx serve --release


bundle-deb:
	dx bundle --package-types deb --desktop --release --out-dir bundle_deb --fullstack false