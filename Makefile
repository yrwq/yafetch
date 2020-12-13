PREFIX ?= /usr

default:
	@printf "targets:\n  make install\n  make uninstall\n"

install:
	@mkdir -p $(DESTDIR)$(PREFIX)/bin
	@install -Dm755 yafetch $(DESTDIR)$(PREFIX)/bin/yafetch

uninstall:
	@rm -f $(DESTDIR)$(PREFIX)/bin/yafetch
