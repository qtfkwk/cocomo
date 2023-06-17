# About

Implementation of COCOMO (Constructive Cost Model) estimates using [tokei] as a
library to calculate total SLOC and [scc] as reference

See also [tokei#359]

[tokei]: https://crates.io/crates/tokei
[tokei#359]: https://github.com/XAMPPRocky/tokei/issues/359
[scc]: https://github.com/boyter/scc

# Usage

```test
$ cocomo -h
COCOMO (Constructive Cost Model) CLI utility

Usage: cocomo [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Files / Directories [default: .]

Options:
      --average-wage <f64>
          Average Wage [default: 56286.0]
      --overhead <f64>
          Overhead [default: 2.4]
      --eaf <f64>
          Effort Adjustment Factor [default: 1.0]
      --project-type <TYPE>
          Project type (organic, semi-detached, embedded) [default: organic]
      --currency-symbol <STRING>
          Currency symbol [default: $]
  -h, --help
          Print help
  -V, --version
          Print version
```

# Example

```text
$ tokei
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Markdown                1           67            0           55           12
 TOML                    1           12           11            0            1
-------------------------------------------------------------------------------
 Rust                    1           82           64            4           14
 |- Markdown             1            6            0            6            0
 (Total)                             88           64           10           14
===============================================================================
 Total                   3          161           75           59           27
===============================================================================
```

```text
$ cocomo
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 75
Estimated Cost to Develop  | $1780.15
Estimated Schedule Effort  | 1.24 months
Estimated People Required  | 0.13
```

