# About

CLI and library implementation of COCOMO (Constructive Cost Model) estimates
using [tokei] as a library to calculate total SLOC and [scc] as reference

See also [tokei#359].

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
$ tokei ~/github.com/XAMPPRocky/tokei
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 BASH                    4           48           30           10            8
 JSON                    1         1706         1706            0            0
 Shell                   1           49           38            1           10
 TOML                    3          125          104            5           16
-------------------------------------------------------------------------------
 HTML                    1           12            9            1            2
 |- JavaScript           1           15           11            4            0
 (Total)                             27           20            5            2
-------------------------------------------------------------------------------
 Markdown                5         1518            0         1211          307
 |- JSON                 1           47           47            0            0
 |- Rust                 1            7            4            3            0
 |- Shell                1           16           14            0            2
 (Total)                           1588           65         1214          309
-------------------------------------------------------------------------------
 Rust                   23         4385         3680          130          575
 |- Markdown            13          374            5          318           51
 (Total)                           4759         3685          448          626
===============================================================================
 Total                  38         7843         5567         1358          918
===============================================================================
```

```text
$ cocomo ~/github.com/XAMPPRocky/tokei
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 5567
Estimated Cost to Develop  | $163886.77
Estimated Schedule Effort  | 6.92 months
Estimated People Required  | 2.10
```

```text
$ cocomo --sloccount ~/github.com/XAMPPRocky/tokei
Total Physical Source Lines of Code (SLOC)                    = 5567
Development Effort Estimate, Person-Years (Person-Months)     = 1.21 (14.56)
  (Basic COCOMO model, Person-Months = 2.40*(KSLOC**1.05)*1.00)
Schedule Estimate, Years (Months)                             = 0.58 (6.92)
  (Basic COCOMO model, Months = 2.50*(person-months**0.38))
Estimated Average Number of Developers (Effort/Schedule)      = 2.10
Total Estimated Cost to Develop                               = $163887
  (average salary = $56286/year, overhead = 2.40)
```

