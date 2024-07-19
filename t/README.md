# About

CLI and library implementation of [COCOMO] (Constructive Cost Model) estimates
using [tokei] as a library to calculate total SLOC and [scc] as reference

See also [tokei#359].

[COCOMO]: https://en.wikipedia.org/wiki/COCOMO
[tokei]: https://crates.io/crates/tokei
[tokei#359]: https://github.com/XAMPPRocky/tokei/issues/359
[scc]: https://github.com/boyter/scc

# Usage

```text
$ cocomo -h
!run:../target/release/cocomo -h
```

```text
$ cocomo -V
!run:../target/release/cocomo -V
```

# Examples

Use [tokei] CLI to count lines of code in a given directory

```text
$ tokei ~/github.com/XAMPPRocky/tokei
!run:tokei ~/github.com/XAMPPRocky/tokei
```

Use [cocomo](https://crates.io/crates/cocomo) CLI to calculate COCOMO estimates

```text
$ cocomo ~/github.com/XAMPPRocky/tokei
!run:../target/release/cocomo ~/github.com/XAMPPRocky/tokei
```

Add `-o sloccount` to use the SLOCCount-style output format

```text
$ cocomo ~/github.com/XAMPPRocky/tokei -o sloccount
!run:../target/release/cocomo ~/github.com/XAMPPRocky/tokei -o sloccount
```

Pass `--sloc N` to calculate COCOMO estimates for a given number of lines of
code (without counting SLOC in any files or directories)

```text
$ cocomo --sloc 5794
!run:../target/release/cocomo --sloc 5794
```

