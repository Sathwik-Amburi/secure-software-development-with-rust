CARGO ?= cargo
RUSTFLAGS ?= -g

vulnerable:
	chmod +x custom_commands/myutility.sh
	$(CARGO) run --bin vulnerable

secure:
	chmod +x custom_commands/myutility.sh
	$(CARGO) run --bin secure

