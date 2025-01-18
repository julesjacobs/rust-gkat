# benchmarking

_E250B5P10RD := $(shell find benchmark/e250b5p10rd -name '*.txt')
_E250B5P10EQ := $(shell find benchmark/e250b5p10eq -name '*.txt')
_E500B5P50RD := $(shell find benchmark/e500b5p50rd -name '*.txt')
_E500B5P50EQ := $(shell find benchmark/e500b5p50eq -name '*.txt')
_E1000B10P100RD := $(shell find benchmark/e1000b10p100rd -name '*.txt')
_E1000B10P100EQ := $(shell find benchmark/e1000b10p100eq -name '*.txt')
_E2000B20P200RD := $(shell find benchmark/e2000b20p200rd -name '*.txt')
_E2000B20P200EQ := $(shell find benchmark/e2000b20p200eq -name '*.txt')

E250B5P10RD := $(subst benchmark/e250b5p10rd/,e250b5p10rd/,$(_E250B5P10RD))
E250B5P10EQ := $(subst benchmark/e250b5p10eq/,e250b5p10eq/,$(_E250B5P10EQ))
E500B5P50RD := $(subst benchmark/e500b5p50rd/,e500b5p50rd/,$(_E500B5P50RD))
E500B5P50EQ := $(subst benchmark/e500b5p50eq/,e500b5p50eq/,$(_E500B5P50EQ))
E1000B10P100RD := $(subst benchmark/e1000b10p100rd/,e1000b10p100rd/,$(_E1000B10P100RD))
E1000B10P100EQ := $(subst benchmark/e1000b10p100eq/,e1000b10p100eq/,$(_E1000B10P100EQ))
E2000B20P200RD := $(subst benchmark/e2000b20p200rd/,e2000b20p200rd/,$(_E2000B20P200RD))
E2000B20P200EQ := $(subst benchmark/e2000b20p200eq/,e2000b20p200eq/,$(_E2000B20P200EQ))

e250b5p10rd/%.txt: benchmark/e250b5p10rd/%.txt 
	./target/release/rust-gkat -k ${kernel} $<
e250b5p10eq/%.txt: benchmark/e250b5p10eq/%.txt 
	./target/release/rust-gkat -k ${kernel} $<
e500b5p50rd/%.txt: benchmark/e500b5p50rd/%.txt 
	./target/release/rust-gkat -k ${kernel} $<
e500b5p50eq/%.txt: benchmark/e500b5p50eq/%.txt 
	./target/release/rust-gkat -k ${kernel} $<
e1000b10p100rd/%.txt: benchmark/e1000b10p100rd/%.txt 
	./target/release/rust-gkat -k ${kernel} $<
e1000b10p100eq/%.txt: benchmark/e1000b10p100eq/%.txt 
	./target/release/rust-gkat -k ${kernel} $<
e2000b20p200rd/%.txt: benchmark/e2000b20p200rd/%.txt 
	./target/release/rust-gkat -k ${kernel} $<
e2000b20p200eq/%.txt: benchmark/e2000b20p200eq/%.txt 
	./target/release/rust-gkat -k ${kernel} $<

e250b5p10rd: $(E250B5P10RD)
e250b5p10eq: $(E250B5P10EQ)
e500b5p50rd: $(E500B5P50RD)
e500b5p50eq: $(E500B5P50EQ)
e1000b10p100rd: $(E1000B10P100RD)
e1000b10p100eq: $(E1000B10P100EQ)
e2000b20p200rd: $(E2000B20P200RD)
e2000b20p200eq: $(E2000B20P200EQ)