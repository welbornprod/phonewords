SHELL=bash
CC=cargo

.PHONY: all, clean, debug, help, run, targets

all:
	-@if [[ ! -e ./target/release/words ]]; then\
		ln -s "$(CURDIR)/words" "$(CURDIR)/target/release/words";\
	fi;
	$(CC) build --release;

debug:
	-@if [[ ! -e ./target/debug/words ]]; then\
		ln -s "$(CURDIR)/words" "$(CURDIR)/target/debug/words";\
	fi;
	$(CC) build;

run:
	-@if [[ ! -e ./target/release/words ]]; then\
		ln -s "$(CURDIR)/words" "$(CURDIR)/target/release/words";\
	fi;
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
