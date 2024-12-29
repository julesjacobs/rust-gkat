INPUT := $(shell find dataset -name '*.txt')
INPUT_BIG := $(shell find dataset_big -name '*.txt')
K1 := $(subst dataset/,k1/,$(INPUT))
K2 := $(subst dataset/,k2/,$(INPUT))
BIG := $(subst dataset_big/,big/,$(INPUT_BIG))


k1/%.txt: dataset/%.txt 
	time ./target/release/rust-gkat -k k1 $<

k2/%.txt: dataset/%.txt 
	time ./target/release/rust-gkat -k k2 $<

big/%.txt: dataset_big/%.txt
	time ./target/release/rust-gkat -k k2 $<

k1: $(K1)
k2: $(K2)
big: $(BIG)