INPUT := $(shell  find dataset -name '*.txt')
DUMMY := $(subst dataset/,dummy/,$(INPUT))

dummy/%.txt: dataset/%.txt 
	./target/release/rust-gkat $<

all: $(DUMMY)