# Test Optimization SDK Documentation

## Overview
The Test Optimization SDK provides a comprehensive API for managing test sessions, modules, suites, and individual tests. It supports features like test tagging, error reporting, benchmarking, and tracing.

## Main Components

### TestSession
The root component for managing test execution sessions.

```rust
pub struct TestSession {
    pub session_id: u64,
}
```

#### Methods
- `init() -> Self`
  - Initializes a new test session with default settings
  - Returns a new TestSession instance

- `init_with_working_dir(working_dir: &str) -> Self`
  - Initializes a new test session with a specified working directory
  - Parameters:
    - `working_dir`: The working directory path
  - Returns a new TestSession instance

- `init_mock() -> Self`
  - Initializes a new test session with mock tracer enabled
  - Returns a new TestSession instance

- `init_mock_with_working_dir(working_dir: &str) -> Self`
  - Initializes a new test session with mock tracer and specified working directory
  - Parameters:
    - `working_dir`: The working directory path
  - Returns a new TestSession instance

- `set_string_tag(key: impl AsRef<str>, value: impl AsRef<str>) -> bool`
  - Sets a string tag for the session
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_number_tag(key: impl AsRef<str>, value: f64) -> bool`
  - Sets a numeric tag for the session
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_error_info(error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool`
  - Sets error information for the session
  - Parameters:
    - `error_type`: Type of error
    - `error_message`: Error message
    - `error_stacktrace`: Error stack trace
  - Returns success status

- `close(exit_code: i32)`
  - Closes the test session
  - Parameters:
    - `exit_code`: Exit code for the session

- `create_module(name: impl AsRef<str>, framework_name: impl AsRef<str>, framework_version: impl AsRef<str>) -> TestModule`
  - Creates a new test module
  - Parameters:
    - `name`: Module name
    - `framework_name`: Name of the test framework
    - `framework_version`: Version of the test framework
  - Returns a new TestModule instance

### TestModule
Represents a test module within a session.

```rust
pub struct TestModule {
    pub module_id: u64,
}
```

#### Methods
- `set_string_tag(key: impl AsRef<str>, value: impl AsRef<str>) -> bool`
  - Sets a string tag for the module
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_number_tag(key: impl AsRef<str>, value: f64) -> bool`
  - Sets a numeric tag for the module
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_error_info(error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool`
  - Sets error information for the module
  - Parameters:
    - `error_type`: Type of error
    - `error_message`: Error message
    - `error_stacktrace`: Error stack trace
  - Returns success status

- `close() -> bool`
  - Closes the test module
  - Returns success status

- `create_test_suite(name: impl AsRef<str>) -> TestSuite`
  - Creates a new test suite
  - Parameters:
    - `name`: Suite name
  - Returns a new TestSuite instance

### TestSuite
Represents a test suite within a module.

```rust
pub struct TestSuite {
    pub suite_id: u64,
}
```

#### Methods
- `set_string_tag(key: impl AsRef<str>, value: impl AsRef<str>) -> bool`
  - Sets a string tag for the suite
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_number_tag(key: impl AsRef<str>, value: f64) -> bool`
  - Sets a numeric tag for the suite
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_error_info(error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool`
  - Sets error information for the suite
  - Parameters:
    - `error_type`: Type of error
    - `error_message`: Error message
    - `error_stacktrace`: Error stack trace
  - Returns success status

- `set_test_source(file: impl AsRef<str>, start_line: *const i32, end_line: *const i32) -> bool`
  - Sets source code information for the suite
  - Parameters:
    - `file`: Source file path
    - `start_line`: Starting line number
    - `end_line`: Ending line number
  - Returns success status

- `close() -> bool`
  - Closes the test suite
  - Returns success status

### Test
Represents an individual test within a suite.

```rust
pub struct Test {
    pub test_id: u64,
}
```

#### Methods
- `set_string_tag(key: impl AsRef<str>, value: impl AsRef<str>) -> bool`
  - Sets a string tag for the test
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_number_tag(key: impl AsRef<str>, value: f64) -> bool`
  - Sets a numeric tag for the test
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_error_info(error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool`
  - Sets error information for the test
  - Parameters:
    - `error_type`: Type of error
    - `error_message`: Error message
    - `error_stacktrace`: Error stack trace
  - Returns success status

- `set_test_source(file: impl AsRef<str>, start_line: *const i32, end_line: *const i32) -> bool`
  - Sets source code information for the test
  - Parameters:
    - `file`: Source file path
    - `start_line`: Starting line number
    - `end_line`: Ending line number
  - Returns success status

- `close(status: TestStatus) -> bool`
  - Closes the test with a status
  - Parameters:
    - `status`: Test status (Pass, Fail, or Skip)
  - Returns success status

- `close_with_skip_reason(skip_reason: impl AsRef<str>) -> bool`
  - Closes the test with a skip status and reason
  - Parameters:
    - `skip_reason`: Reason for skipping the test
  - Returns success status

- `set_benchmark_number_data<K: AsRef<str>>(measure_type: impl AsRef<str>, data: &HashMap<K, f64>) -> bool`
  - Sets benchmark numeric data for the test
  - Parameters:
    - `measure_type`: Type of measurement
    - `data`: HashMap of benchmark data
  - Returns success status

### Span
Represents a tracing span for performance monitoring.

```rust
pub struct Span {
    pub span_id: u64,
    pub parent_id: u64,
}
```

#### Methods
- `create(parent_id: u64, operation_name: impl AsRef<str>, service_name: impl AsRef<str>, resource_name: impl AsRef<str>, span_type: impl AsRef<str>) -> Self`
  - Creates a new span
  - Parameters:
    - `parent_id`: ID of the parent span
    - `operation_name`: Name of the operation
    - `service_name`: Name of the service
    - `resource_name`: Name of the resource
    - `span_type`: Type of the span
  - Returns a new Span instance

- `set_string_tag(key: impl AsRef<str>, value: impl AsRef<str>) -> bool`
  - Sets a string tag for the span
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_number_tag(key: impl AsRef<str>, value: f64) -> bool`
  - Sets a numeric tag for the span
  - Parameters:
    - `key`: Tag key
    - `value`: Tag value
  - Returns success status

- `set_error_info(error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool`
  - Sets error information for the span
  - Parameters:
    - `error_type`: Type of error
    - `error_message`: Error message
    - `error_stacktrace`: Error stack trace
  - Returns success status

- `close() -> bool`
  - Closes the span
  - Returns success status

### MockTracer
Provides debugging capabilities for tracing.

```rust
pub struct MockTracer;
```

#### Methods
- `reset() -> bool`
  - Resets the mock tracer
  - Returns success status

- `get_finished_spans() -> Vec<MockSpan>`
  - Gets all finished spans
  - Returns a vector of MockSpan instances

- `get_open_spans() -> Vec<MockSpan>`
  - Gets all currently open spans
  - Returns a vector of MockSpan instances

## Enums

### TestStatus
```rust
pub enum TestStatus {
    Pass = 0,
    Fail = 1,
    Skip = 2,
}
```

## Settings Types

### Settings
```rust
pub struct Settings {
    pub code_coverage: bool,
    pub early_flake_detection: EfDSettings,
    pub flaky_test_retries_enabled: bool,
    pub itr_enabled: bool,
    pub require_git: bool,
    pub tests_skipping: bool,
    pub known_tests_enabled: bool,
    pub test_management: TestManagementSettings,
}
```

### EfDSettings
```rust
pub struct EfDSettings {
    pub enabled: bool,
    pub slow_test_retries: EfdSlowTestRetriesSettings,
    pub faulty_session_threshold: i32,
}
```

### TestManagementSettings
```rust
pub struct TestManagementSettings {
    pub enabled: bool,
    pub attempt_to_fix_retries: i32,
}
``` 