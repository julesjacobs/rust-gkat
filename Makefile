INPUT := $(shell  find dataset -name '*.txt')
BDD := $(subst dataset/,bdd/,$(INPUT))
SDD := $(subst dataset/,sdd/,$(INPUT))

bdd/%.txt: dataset/%.txt 
	./target/release/rust-gkat -m bdd $<

sdd/%.txt: dataset/%.txt 
	./target/release/rust-gkat -m sdd $<

bdd: $(BDD)
sdd: $(SDD)