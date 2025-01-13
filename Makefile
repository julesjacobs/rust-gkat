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

E250B5P10RD  := $(shell find benchmark/e250b5p10rand  -name '*.txt')
E250B5P10EQ := $(shell find benchmark/e250b5p10equal -name '*.txt')
E500B5P50RD  := $(shell find benchmark/e500b5p50rand  -name '*.txt')
E500B5P50EQ := $(shell find benchmark/e500b5p50equal -name '*.txt')
E1000B10P100RD  := $(shell find benchmark/e1000b10p100rand  -name '*.txt')
E1000B10P100EQ := $(shell find benchmark/e1000b10p100equal -name '*.txt')

K1E250B5P10RAND  := $(subst benchmark/e250b5p10rand/,k1e250b5p10rand/,$(E250B5P10RAND))
K1E250B5P10EQUAL := $(subst benchmark/e250b5p10equal/,k1e250b5p10equal/,$(E250B5P10EQUAL))
K1E500B5P50RAND  := $(subst benchmark/e500b5p50rand/,k1e500b5p50rand/,$(E500B5P50RAND))
K1E500B5P50EQUAL := $(subst benchmark/e500b5p50equal/,k1e500b5p50equal/,$(E500B5P50EQUAL))
K1E1000B10P100RAND  := $(subst benchmark/e1000b10p100rand/,k1e1000b10p100rand/,$(E1000B10P100RAND))
K1E1000B10P100EQUAL := $(subst benchmark/e1000b10p100equal/,k1e1000b10p100equal/,$(E1000B10P100EQUAL))

k1e250b5p10rand/%.txt: benchmark/e250b5p10rand/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<

k1e250b5p10equal/%.txt: benchmark/e250b5p10equal/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<

k1e500b5p50rand/%.txt: benchmark/e500b5p50rand/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<

k1e500b5p50equal/%.txt: benchmark/e500b5p50equal/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<

k1e250b5p10rand : $(K1E250B5P10RAND)
k1e250b5p10equal: $(K1E250B5P10EQUAL)
k1e500b5p50rand : $(K1E500B5P50RAND)
k1e500b5p50equal: $(K1E500B5P50EQUAL)