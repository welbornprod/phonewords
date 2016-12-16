SHELL=bash
CC=cargo
words_file=$(CURDIR)/words
dir_debug=$(CURDIR)/target/debug
words_debug=$(dir_debug)/words
dir_release=$(CURDIR)/target/release
words_release=$(dir_release)/words

.PHONY: all, clean, cleanmake, debug, help, makeclean, run, targets
.PHONY: makewords, makedebugwords, makereleasewords, words

all:
	-@make makereleasewords
	$(CC) build --release;

debug:
	-@make makedebugwords
	$(CC) build;

run:
	-@if [[ ! -e ./target/release/words ]]; then\
		ln -s "$(CURDIR)/words" "$(CURDIR)/target/release/words";\
	fi;
	$(CC) run --release -- --help;
	-@echo -e "\n\n...just use \`cargo run\`.";

clean:
	cargo clean;

cleanmake makeclean:
	@make --no-print-directory clean && make --no-print-directory;

help:
	-@echo -e "Use 'make targets' for a list of available targets.";

makedebugwords:
	-@if [[ -e $(words_debug) ]]; then\
		printf "Words file already exists: $(words_debug)\n";\
	else\
		if mkdir -p $(dir_debug); then\
			printf "Created dir: %s\n" "$(dir_debug)";\
		else\
			printf "\nFailed to create dir: %s\n" "$(dir_debug)" 1>&2;\
		fi;\
		if ln -s "$(words_file)" "$(words_debug)"; then\
			printf "Symlinked words file: %s\n" "$(words_debug)";\
		else\
			printf "\nFailed to create symbolic link:\n  from: $(words_file)\n    to: $(words_debug)" 1>&2;\
		fi;\
	fi;

makereleasewords:
	-@if [[ -e $(words_release) ]]; then\
		printf "Words file already exists: $(words_release)\n";\
	else\
		if mkdir -p $(dir_release); then\
			printf "Created dir: %s\n" "$(dir_release)";\
		else\
			printf "\nFailed to create dir: %s\n" "$(dir_release)" 1>&2;\
		fi;\
		if ln -s "$(words_file)" "$(words_release)"; then\
			printf "Symlinked words file: %s\n" "$(words_release)";\
		else\
			printf "\nFailed to create symbolic link:\n  from: $(words_file)\n    to: $(words_release)" 1>&2;\
		fi;\
	fi;

makewords:
	@make --no-print-directory makedebugwords
	@make --no-print-directory makereleasewords


targets:
	-@printf "Make targets available:\n\
	all        : Build a release executable.\n\
	clean      : Remove the ./target directory.\n\
	cleanmake  : Runs \`make clean && make\`.\n\
	debug      : Build a debug executable.\n\
	makeclean  : Alias for \`cleanmake\`  target.\n\
	makewords  : Ensure the debug & release words files exist.\n\
	run        : Run the release executable, build if needed.\n\
	targets    : Show this message.\n\
	words      : Check words file locations.\n";

words:
	@wfile_status="missing"; [[ -e "$(words_file)" ]] && wfile_status="$(words_file)";\
	printf "\n  Local words file: %s\n" "$$wfile_status";\
	wfile_status="missing"; [[ -e "$(words_release)" ]] && wfile_status="$(words_release)";\
	printf "Release words file: %s\n" "$$wfile_status";\
	wfile_status="missing"; [[ -e "$(words_debug)" ]] && wfile_status="$(words_debug)";\
	printf "  Debug words file: %s\n" "$$wfile_status";
