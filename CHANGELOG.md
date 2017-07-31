# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.5.0] - 2017-07-31

- Re-generate usign svd2rust v0.11.2

## [v0.4.1] - 2017-05-08

- Re-generate usign svd2rust v0.7.2

## [v0.4.0] - 2017-04-25

### Changed

- [breaking-change] Re-generated using svd2rust v0.7.0. NVIC and FPU API
  changed.

### Added

- API for the rest of the core peripherals. This API is just a re-export of the
  cortex-m one.

## [v0.3.2] - 2017-04-23

### Added

- enumeratedValues to some GPIO, TIM and RCC bitfields
- writeConstraint to some TIM bitfields

### Changed

- Re-generate using svd2rust v0.6.2

## [v0.3.1] - 2017-04-15

### Changed

- [breaking-change] Re-generate using svd2rust v0.6.1

## [v0.3.0] - 2017-04-11

### Changed

- [breaking-change] Re-generate using svd2rust v0.6.0

## [v0.2.0] - 2017-03-27

### Changed

- [breaking-change] Re-generate using svd2rust v0.5.0

## v0.1.0 - 2017-03-12

- Initial release

[Unreleased]: https://github.com/japaric/stm32f30x/compare/v0.4.0...HEAD
[v0.4.0]: https://github.com/japaric/stm32f30x/compare/v0.3.2...v0.4.0
[v0.3.2]: https://github.com/japaric/stm32f30x/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/japaric/stm32f30x/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/stm32f30x/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/stm32f30x/compare/v0.1.0...v0.2.0
