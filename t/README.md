# About

CLI and library implementation of [COCOMO] (Constructive Cost Model) estimates
using [tokei] as a library to calculate total SLOC and [scc] as reference

See also [tokei#359].

[cocomo]: https://crates.io/crates/cocomo
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

Use [cocomo] CLI to calculate COCOMO estimates

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

# Notes

1. Prior to version 0.8.0 (2024-08-24), [cocomo] did not account for inflation,
   assuming the user could simply modify the default average wage as desired for
   a given country or other locale and/or time.
   [cocomo] now incorporates the USA inflation rates for 1995 (the year that the
   $56,286 average wage figure comes from) to 2024.
   The defaults remain the same as before, which means that [cocomo] gives
   cost figures in 1995 USD (as it always has, unless its use was customized).
   Going forward, if you want [cocomo] to report costs in the current year,
   please provide it via the `--inflation-year` option.
   Note that any given year outside the range above causes [cocomo] to revert to
   the inflation multiplier given via the `--inflation-multiplier` option which
   is `1.0` by default and effectively represents the year `1995`.

