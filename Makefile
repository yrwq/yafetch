PREFIX?=/usr
BIN?=$(PREFIX)/bin

default:
	@printf "targets:\n  make install\n  make uninstall\n"

install:
	install -Dm755 yafetch $(BIN)/yafetch

uninstall:
	rm -f $(BIN)/yafetch
