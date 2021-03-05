PROJECT = yafetch
CFLAGS := $(shell pkg-config --cflags lua5.1)
LDFLAGS := $(shell pkg-config --libs lua5.1)
OBJECTS = src/script.o src/func.o src/main.o

PREFIX = /usr/local
BINDIR = $(PREFIX)/bin

all: $(PROJECT)

$(PROJECT): $(OBJECTS)
	gcc $^ $(LDFLAGS) -o $@

install: $(PROJECT)
	install -Dm755 yafetch $(BINDIR)

clean:
	rm -f $(PROJECT) $(OBJECTS)
