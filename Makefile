
PARAMS := 
CARGO := $(shell which cargo)

run: check
	$(CARGO) run -- $(PARAMS)

build:
	$(CARGO) build 

clean:
	$(CARGO) clean

check:
	$(CARGO) check

test:
	$(CARGO) test