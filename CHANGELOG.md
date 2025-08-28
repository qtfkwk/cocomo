# Changelog

* 0.1.0 (2023-06-17): Initial
    * 0.1.1 (2023-06-17): Add changelog, fix readme
* 0.2.0 (2023-06-17): Add `--sloccount` option; update dependencies
    * 0.2.1 (2023-06-17): Fix typo
* 0.3.0 (2023-06-18): Expose as library; convert PARAMS static HashMap into ProjectType enum and remove `lazy_static` dependency; add `--exclude` option; update dependencies
    * 0.3.1 (2023-06-18): Fix typo
    * 0.3.2 (2023-06-18): Update readme
    * 0.3.3 (2023-06-18): Update readme
* 0.4.0 (2023-06-19): CLI: add `--custom` option; library: add `Cocomo` struct and `total_sloc`, `cocomo` functions; apply clippy fixes; run cargo fmt
    * 0.4.1 (2023-06-19): Fix changelog
* 0.5.0 (2023-06-21): Add `--development-time` option / split from parameters; unify the report methods and add `OutputFormat` enum; improve readme; update dependencies
* 0.6.0 (2023-06-24): Improve number formatting; update dependencies
* 0.7.0 (2023-06-24): Add `--sloc` option; improve readme; update dependencies
    * 0.7.1 (2023-06-24): Fix readme
    * 0.7.2 (2024-07-19): Fix changelog and readme; update dependencies
    * 0.7.3 (2024-08-06): Add `Makefile.md`; update dependencies
    * 0.7.4 (2024-08-23): Add `commit` target to makefile; update dependencies
* 0.8.0 (2024-08-24): Add USA inflation rates for 1995 (origin of $56,286 average wage) to 2024; add `--inflation-multiplier` and `--inflation-year` options; add more details for the `--project-type` and `--custom` options; fix changelog
    * 0.8.1 (2024-08-24): Fix readme
* 0.9.0 (2024-09-05): Apply inflation multiplier to average wage during creation and restore `cocomo` and `estimate_cost` function signatures to pre-0.8.0; add sloccount-inflation output format; update dependencies
* 0.10.0 (2024-10-26): Fix changelog; add clap color; update dependencies
    * 0.10.1 (2024-12-04): Update dependencies
    * 0.10.2 (2025-02-21): Update dependencies; fix changelog
    * 0.10.3 (2025-04-16): Update dependencies
* 0.11.0 (2025-08-28): Update dependencies; 2024 edition

