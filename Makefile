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

# benchmarking

E250B5P10RD := $(shell find benchmark/e250b5p10rd -name '*.txt')
E250B5P10EQ := $(shell find benchmark/e250b5p10eq -name '*.txt')
E500B5P50RD := $(shell find benchmark/e500b5p50rd -name '*.txt')
E500B5P50EQ := $(shell find benchmark/e500b5p50eq -name '*.txt')
E1000B10P100RD := $(shell find benchmark/e1000b10p100rd -name '*.txt')
E1000B10P100EQ := $(shell find benchmark/e1000b10p100eq -name '*.txt')
E2000B20P200RD := $(shell find benchmark/e2000b20p200rd -name '*.txt')
E2000B20P200EQ := $(shell find benchmark/e2000b20p200eq -name '*.txt')

# kernel 1

K1E250B5P10RD := $(subst benchmark/e250b5p10rd/,k1e250b5p10rd/,$(E250B5P10RD))
K1E250B5P10EQ := $(subst benchmark/e250b5p10eq/,k1e250b5p10eq/,$(E250B5P10EQ))
K1E500B5P50RD := $(subst benchmark/e500b5p50rd/,k1e500b5p50rd/,$(E500B5P50RD))
K1E500B5P50EQ := $(subst benchmark/e500b5p50eq/,k1e500b5p50eq/,$(E500B5P50EQ))
K1E1000B10P100RD := $(subst benchmark/e1000b10p100rd/,k1e1000b10p100rd/,$(E1000B10P100RD))
K1E1000B10P100EQ := $(subst benchmark/e1000b10p100eq/,k1e1000b10p100eq/,$(E1000B10P100EQ))
K1E2000B20P200RD := $(subst benchmark/e2000b20p200rd/,k1e2000b20p200rd/,$(E2000B20P200RD))
K1E2000B20P200EQ := $(subst benchmark/e2000b20p200eq/,k1e2000b20p200eq/,$(E2000B20P200EQ))

k1e250b5p10rd/%.txt: benchmark/e250b5p10rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<
k1e250b5p10eq/%.txt: benchmark/e250b5p10eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<
k1e500b5p50rd/%.txt: benchmark/e500b5p50rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<
k1e500b5p50eq/%.txt: benchmark/e500b5p50eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<
k1e1000b10p100rd/%.txt: benchmark/e1000b10p100rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<
k1e1000b10p100eq/%.txt: benchmark/e1000b10p100eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<
k1e2000b20p200rd/%.txt: benchmark/e2000b20p200rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<
k1e2000b20p200eq/%.txt: benchmark/e2000b20p200eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k1 $<

k1e250b5p10rd: $(K1E250B5P10RD)
k1e250b5p10eq: $(K1E250B5P10EQ)
k1e500b5p50rd: $(K1E500B5P50RD)
k1e500b5p50eq: $(K1E500B5P50EQ)
k1e1000b10p100rd: $(K1E1000B10P100RD)
k1e1000b10p100eq: $(K1E1000B10P100EQ)
k1e2000b20p200rd: $(K1E2000B20P200RD)
k1e2000b20p200eq: $(K1E2000B20P200EQ)

# kernel 2

K2E250B5P10RD := $(subst benchmark/e250b5p10rd/,k2e250b5p10rd/,$(E250B5P10RD))
K2E250B5P10EQ := $(subst benchmark/e250b5p10eq/,k2e250b5p10eq/,$(E250B5P10EQ))
K2E500B5P50RD := $(subst benchmark/e500b5p50rd/,k2e500b5p50rd/,$(E500B5P50RD))
K2E500B5P50EQ := $(subst benchmark/e500b5p50eq/,k2e500b5p50eq/,$(E500B5P50EQ))
K2E1000B10P100RD := $(subst benchmark/e1000b10p100rd/,k2e1000b10p100rd/,$(E1000B10P100RD))
K2E1000B10P100EQ := $(subst benchmark/e1000b10p100eq/,k2e1000b10p100eq/,$(E1000B10P100EQ))
K2E2000B20P200RD := $(subst benchmark/e2000b20p200rd/,k2e2000b20p200rd/,$(E2000B20P200RD))
K2E2000B20P200EQ := $(subst benchmark/e2000b20p200eq/,k2e2000b20p200eq/,$(E2000B20P200EQ))

k2e250b5p10rd/%.txt: benchmark/e250b5p10rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<
k2e250b5p10eq/%.txt: benchmark/e250b5p10eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<
k2e500b5p50rd/%.txt: benchmark/e500b5p50rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<
k2e500b5p50eq/%.txt: benchmark/e500b5p50eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<
k2e1000b10p100rd/%.txt: benchmark/e1000b10p100rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<
k2e1000b10p100eq/%.txt: benchmark/e1000b10p100eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<
k2e2000b20p200rd/%.txt: benchmark/e2000b20p200rd/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<
k2e2000b20p200eq/%.txt: benchmark/e2000b20p200eq/%.txt 
	timeout 5m ./target/release/rust-gkat -k k2 $<

k2e250b5p10rd: $(K2E250B5P10RD)
k2e250b5p10eq: $(K2E250B5P10EQ)
k2e500b5p50rd: $(K2E500B5P50RD)
k2e500b5p50eq: $(K2E500B5P50EQ)
k2e1000b10p100rd: $(K2E1000B10P100RD)
k2e1000b10p100eq: $(K2E1000B10P100EQ)
k2e2000b20p200rd: $(K2E2000B20P200RD)
k2e2000b20p200eq: $(K2E2000B20P200EQ)