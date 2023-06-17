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
      --sloccount
          Use SLOCCount-style format
  -h, --help
          Print help
  -V, --version
          Print version
```

# Examples

```text
$ tokei
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Markdown                2           70            0           58           12
 TOML                    1           13           12            0            1
-------------------------------------------------------------------------------
 Rust                    1          113           94            4           15
 |- Markdown             1            7            0            7            0
 (Total)                            120           94           11           15
===============================================================================
 Total                   4          196          106           62           28
===============================================================================
```

```text
$ cocomo
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 106
Estimated Cost to Develop  | $2559.84
Estimated Schedule Effort  | 1.42 months
Estimated People Required  | 0.16
```

```text
$ cocomo --sloccount
Total Physical Source Lines of Code (SLOC)                    = 106
Development Effor Estimate, Person-Years (Person-Months)      = 0.02 (0.23)
  (Basic COCOMO model, Person-Months = 2.40*(KSLOC**1.05)*1.00)
Schedule Estimate, Years (Months)                             = 0.12 (1.42)
  (Basic COCOMO model, Months = 2.50*(person-months**0.38))
Estimated Average Number of Developers (Effort/Schedule)      = 0.16
Total Estimated Cost to Develop                               = $2560
  (average salary = $56286/year, overhead = 2.40)
```

