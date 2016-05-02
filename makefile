SHELL=bash
CC=cargo

.PHONY: all, clean, debug, help, run, targets

all:
	$(CC) build --release;

debug:
	$(CC) build;

run:
	$(CC) run --release -- --help;
	-@echo -e "\n\n...just use \`cargo run\`.";

clean:
	cargo clean;

help:
	-@echo -e "Use 'make targets' for a list of available targets.";

targets:
	-@echo -e "Make targets available:\n\
	all     : Build a release executable.\n\
	clean   : Remote the ./target directory.\n\
	debug   : Build a debug executable.\n\
	run     : Run the release executable, build if needed.\n\
	targets : Show this message.";
