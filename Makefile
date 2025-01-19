# benchmarking
kernel = k1
solver = bdd

_DATASET0 := $(shell find dataset0 -name '*.txt')
_DATASET1 := $(shell find dataset1 -name '*.txt')
_E250B5P10RD := $(shell find benchmark/e250b5p10rd -name '*.txt')
_E250B5P10EQ := $(shell find benchmark/e250b5p10eq -name '*.txt')
_E500B5P50RD := $(shell find benchmark/e500b5p50rd -name '*.txt')
_E500B5P50EQ := $(shell find benchmark/e500b5p50eq -name '*.txt')
_E1000B10P100RD := $(shell find benchmark/e1000b10p100rd -name '*.txt')
_E1000B10P100EQ := $(shell find benchmark/e1000b10p100eq -name '*.txt')
_E2000B20P200RD := $(shell find benchmark/e2000b20p200rd -name '*.txt')
_E2000B20P200EQ := $(shell find benchmark/e2000b20p200eq -name '*.txt')
_E3000B30P200RD := $(shell find benchmark/e3000b30p200rd -name '*.txt')
_E3000B30P200EQ := $(shell find benchmark/e3000b30p200eq -name '*.txt')

DATASET0 := $(subst dataset0/,test0/,$(_DATASET0))
DATASET1 := $(subst dataset1/,test1/,$(_DATASET1))
E250B5P10RD := $(subst benchmark/e250b5p10rd/,e250b5p10rd/,$(_E250B5P10RD))
E250B5P10EQ := $(subst benchmark/e250b5p10eq/,e250b5p10eq/,$(_E250B5P10EQ))
E500B5P50RD := $(subst benchmark/e500b5p50rd/,e500b5p50rd/,$(_E500B5P50RD))
E500B5P50EQ := $(subst benchmark/e500b5p50eq/,e500b5p50eq/,$(_E500B5P50EQ))
E1000B10P100RD := $(subst benchmark/e1000b10p100rd/,e1000b10p100rd/,$(_E1000B10P100RD))
E1000B10P100EQ := $(subst benchmark/e1000b10p100eq/,e1000b10p100eq/,$(_E1000B10P100EQ))
E2000B20P200RD := $(subst benchmark/e2000b20p200rd/,e2000b20p200rd/,$(_E2000B20P200RD))
E2000B20P200EQ := $(subst benchmark/e2000b20p200eq/,e2000b20p200eq/,$(_E2000B20P200EQ))
E3000B30P200RD := $(subst benchmark/e3000b30p200rd/,e3000b30p200rd/,$(_E3000B30P200RD))
E3000B30P200EQ := $(subst benchmark/e3000b30p200eq/,e3000b30p200eq/,$(_E3000B30P200EQ))

test0/%.txt: dataset0/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
test1/%.txt: dataset1/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e250b5p10rd/%.txt: benchmark/e250b5p10rd/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e250b5p10eq/%.txt: benchmark/e250b5p10eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e500b5p50rd/%.txt: benchmark/e500b5p50rd/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e500b5p50eq/%.txt: benchmark/e500b5p50eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e1000b10p100rd/%.txt: benchmark/e1000b10p100rd/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e1000b10p100eq/%.txt: benchmark/e1000b10p100eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e2000b20p200rd/%.txt: benchmark/e2000b20p200rd/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e2000b20p200eq/%.txt: benchmark/e2000b20p200eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e3000b30p200rd/%.txt: benchmark/e3000b30p200rd/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e3000b30p200eq/%.txt: benchmark/e3000b30p200eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<

test0: $(DATASET0)
test1: $(DATASET1)
e250b5p10rd: $(E250B5P10RD)
e250b5p10eq: $(E250B5P10EQ)
e500b5p50rd: $(E500B5P50RD)
e500b5p50eq: $(E500B5P50EQ)
e1000b10p100rd: $(E1000B10P100RD)
e1000b10p100eq: $(E1000B10P100EQ)
e2000b20p200rd: $(E2000B20P200RD)
e2000b20p200eq: $(E2000B20P200EQ)
e3000b30p200rd: $(E3000B30P200RD)
e3000b30p200eq: $(E3000B30P200EQ)