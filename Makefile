INPUT := $(shell  find input -name '*.txt')
BDD := $(subst input/,bdd/,$(INPUT))
SDD := $(subst input/,sdd/,$(INPUT))

bdd/%.txt: input/%.txt 
	time ./target/release/rust-gkat -m bdd $<

sdd/%.txt: input/%.txt 
	time ./target/release/rust-gkat -m sdd $<

bdd: $(BDD)
sdd: $(SDD)