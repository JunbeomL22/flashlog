# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [0.3.4] - 2025-07-31
 - Deafult message flush buffer size and time interval are set to zero (immediate flush
 - Configuration stage is in logger launching not in the definition of logger thread

## [0.3.3] - 2025-07-22
 - Replace expect with unwrap for crossbeam try_send and write_all operations to avoid overhead
 - Update flush condition logic to enable immediate flushing with zero buffer size and interval
   - Changed from `(buffer_size + new_msg_length > msg_buffer_size) || (current_timestamp - last_flush_time > msg_flush_interval)`
   - To `(buffer_size + new_msg_length >= msg_buffer_size) || (current_timestamp >= msg_flush_interval + last_flush_time)`
 - Update immediate_log example to use only flash_xxx_ct! macros

## [0.3.2] - 2025-07-07
 - crossbeam version update
 - replace expect with unwrap

## [0.3.1] - 2025-03-14
 - performance table in doc comments
 - `flash_xxx_ct!` macros uses only compile-time filter

## [0.3.0] - 2024-03-14
 - in the log macro, the struct data is cloned. The cloning is done in the current thread and serialized in the logging thread.
 - include_unixnano option added
 - user can choose what core to choose for affinity (with_logger_core)
 - compile time filter added
 - timestamp is made in logger thread

## [0.2.4]
 - rolling file function is added

## [0.2.3]
 - once_cell dependency changed
 
## [0.2.2]
 - minor bug fix

## [0.2.0]
 - report level load optimization (todo)
 - lazy string interned (todo)
 - log_info, log_debug, etc, deprecated (todo)
 = `topic` and `message` are base keys in Json output

## [0.1.5] - 2024-09-12
 - dropping the guard also flushes
 - Some minor changes in docs and README
 
## [0.1.3] - 2024-09-12
 - hot fix: now console and file report are both an option
 
## [0.1.1] - 2024-08-29
### Fixed
- Corrected GitHub repository URL in project metadata

## [0.1.0] - 2024-08-29
### Added
- Initial release of the crate
