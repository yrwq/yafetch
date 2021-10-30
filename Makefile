PROJECT = yafetch
CFLAGS := $(shell pkg-config --cflags lua5.1)
LDFLAGS := $(shell pkg-config --libs lua5.1)
OBJECTS = src/script.o src/func.o src/main.o

CONF = /usr/share/yafetch
PREFIX = /usr/local
BINDIR = $(PREFIX)/bin

all: $(PROJECT)

$(PROJECT): $(OBJECTS)
	gcc $^ $(LDFLAGS) -o $@

config: $(PROJECT)
	mkdir -p $(CONF)
	cp init.lua $(CONF)/init.lua

install: $(PROJECT)
	mkdir -p $(BINDIR)
	install -Dm755 yafetch $(BINDIR)

clean:
	rm -f $(PROJECT) $(OBJECTS)
