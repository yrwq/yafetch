# Variables

You can configure the compilation process using
variables pre-prended to the `make` command.

These variables are:

- **CFLAGS** -- what flags to pass to your C compiler, by default it's lua's flags, all other flags you pass are prepended to the lua flags
- **LDFLAGS** -- what linker flags to pass to your linker, by default it's lua's flags, extra flags are prepended
- **CONF** -- a directory where to put default configuration, by default it's `/usr/share/yafetch`
- **PREFIX** -- a directory where to put the compiled binary when installing, by default it's `/usr/local`
- **BINDIR** -- an exact full path to the binary folder when installing, by default it's `PREFX` + `/bin`
- **CC** -- your C compiler
- **DESTDIR** -- the destination directory
- **STRIPFLAGS** -- flags to pass to the binary stripper


# Example

```bash
CFLAGS='-O3 -pipe' LDFLAGS='-s' CC='clang' make -j8
```

