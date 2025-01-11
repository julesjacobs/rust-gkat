INPUT0 := $(shell find dataset0 -name '*.txt')
INPUT1 := $(shell find dataset1 -name '*.txt')
INPUT2 := $(shell find dataset2 -name '*.txt')
K1 := $(subst dataset0/,k1/,$(INPUT0))
K2 := $(subst dataset0/,k2/,$(INPUT0))
D1 := $(subst dataset1/,d1/,$(INPUT1))
D2 := $(subst dataset2/,d2/,$(INPUT2))

k1/%.txt: dataset0/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<

k2/%.txt: dataset0/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<

d1/%.txt: dataset1/%.txt
	timeout 5m ./target/release/rust-gkat -k k2 $<

d2/%.txt: dataset2/%.txt
	timeout 5m ./target/release/rust-gkat -k k2 $<

k1: $(K1)
k2: $(K2)
d1: $(D1)
d2: $(D2)