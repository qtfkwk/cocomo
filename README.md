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
COCOMO (Constructive Cost Model) CLI utility and library

<https://crates.io/crates/cocomo> / <https://github.com/qtfkwk/cocomo>

See also: <https://en.wikipedia.org/wiki/COCOMO>

---

Usage: cocomo [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Files / Directories [default: .]

Options:
      --sloc <N>
          Source lines of code [default: *calculate from Files / Directories
          argument(s)*]
      --average-wage <f64>
          Average Wage [default: 56286.0]
      --overhead <f64>
          Overhead [default: 2.4]
      --eaf <f64>
          Effort Adjustment Factor (EAF); typically 0.9 - 1.4 [default: 1.0]
      --project-type <TYPE>
          Project type [default: organic] [possible values: embedded, organic,
          semi-detached]
      --custom <f64,f64,f64>
          Custom parameters (a, b, c)
      --development-time <f64>
          Development time (d) [default: 2.5]
      --currency-symbol <STRING>
          Currency symbol [default: $]
  -o, --output-format <FORMAT>
          Output format [default: markdown-table] [possible values:
          markdown-table, sloccount]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

```text
$ cocomo -V
cocomo 0.7.4
```

# Examples

Use [tokei] CLI to count lines of code in a given directory

```text
$ tokei ~/github.com/XAMPPRocky/tokei
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 BASH                    4           48           30           10            8
 JSON                    1         1923         1923            0            0
 Shell                   1           49           38            1           10
 TOML                    3          125          104            5           16
-------------------------------------------------------------------------------
 HTML                    1           12            9            1            2
 |- JavaScript           1           15           11            4            0
 (Total)                             27           20            5            2
-------------------------------------------------------------------------------
 Markdown                5         1662            0         1339          323
 |- BASH                 1            3            3            0            0
 |- JSON                 1           46           46            0            0
 |- Rust                 1            7            4            3            0
 |- Shell                1           16           14            0            2
 (Total)                           1734           67         1342          325
-------------------------------------------------------------------------------
 Rust                   24         4557         3829          140          588
 |- Markdown            13          386            5          327           54
 (Total)                           4943         3834          467          642
===============================================================================
 Total                  39         8376         5933         1496          947
===============================================================================
```

Use [cocomo](https://crates.io/crates/cocomo) CLI to calculate COCOMO estimates

```text
$ cocomo ~/github.com/XAMPPRocky/tokei
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 5,933
Estimated Cost to Develop  | $175,218.39
Estimated Schedule Effort  | 7.10 months
Estimated People Required  | 2.19

```

Add `-o sloccount` to use the SLOCCount-style output format

```text
$ cocomo ~/github.com/XAMPPRocky/tokei -o sloccount
Total Physical Source Lines of Code (SLOC)                    = 5,933
Development Effort Estimate, Person-Years (Person-Months)     = 1.30 (15.57)
  (Basic COCOMO model, Person-Months = 2.40*(KSLOC**1.05)*1.00)
Schedule Estimate, Years (Months)                             = 0.59 (7.10)
  (Basic COCOMO model, Months = 2.50*(person-months**0.38))
Estimated Average Number of Developers (Effort/Schedule)      = 2.19
Total Estimated Cost to Develop                               = $175,218
  (average salary = $56,286/year, overhead = 2.40)

```

Pass `--sloc N` to calculate COCOMO estimates for a given number of lines of
code (without counting SLOC in any files or directories)

```text
$ cocomo --sloc 5794
Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 5,794
Estimated Cost to Develop  | $170,910.62
Estimated Schedule Effort  | 7.03 months
Estimated People Required  | 2.16

```

