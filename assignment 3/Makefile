# If the first argument is "run"...
ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif


build:
	cargo build --release
run:
	target/release/assignment3 $(RUN_ARGS)
run-manual:
	target/release/assignment3 -m
run-bench:
	target/release/assignment3 -b "$(EXPR)" "$(N)"
clean:
	cargo clean