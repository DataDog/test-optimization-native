# Datadog Test Optimization SDK for Rust

This SDK provides integration with Datadog's Test Optimization features for Rust applications.

## Features

- Test Session Management
- Test Module Management
- Test Suite Management
- Test Management
- Early Flake Detection
- Auto test retries
- Test Skipping
- Known Tests Tracking
- Flaky Tests Management
- Performance Monitoring with Spans
- Debugging with Mock Tracer

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
test-optimization-sdk = "0.0.1"
```

## Usage

### Initialization

First, initialize the SDK:

```rust
use test_optimization_sdk::TestOptimization;

// Basic initialization
TestOptimization::init();

// Or with a working directory
TestOptimization::init_with_working_dir("/path/to/working/dir");

// Or with mock tracer for testing
TestOptimization::init_mock();
```

### Test Session Management

Create and manage test sessions:

```rust
use test_optimization_sdk::TestSession;

// Create a new test session
let session = TestSession::create(
    Some("my_framework"),
    Some("1.0.0")
);

// Set tags
session.set_string_tag("environment", "staging");
session.set_number_tag("timeout", 30.0);

// Set error information if needed
session.set_error_info(
    "TestFailure",
    "Test failed due to assertion error",
    "stack trace here"
);

// Create a test module
let module = session.create_module(
    "my_module",
    "my_framework",
    "1.0.0"
);

// Close the session when done
session.close(0); // 0 for success, non-zero for failure
```

### Test Module Management

Manage test modules within a session:

```rust
use test_optimization_sdk::TestModule;

// Create a test module (from a session)
let module = session.create_module(
    "my_module",
    "my_framework",
    "1.0.0"
);

// Set module tags
module.set_string_tag("module_type", "integration");
module.set_number_tag("timeout", 60.0);

// Create a test suite
let suite = module.create_test_suite("my_suite");

// Close the module when done
module.close();
```

### Test Suite Management

Manage test suites within a module:

```rust
use test_optimization_sdk::TestSuite;

// Create a test suite (from a module)
let suite = module.create_test_suite("my_suite");

// Set suite tags
suite.set_string_tag("suite_type", "regression");
suite.set_number_tag("priority", 1.0);

// Set source code information
suite.set_test_source("src/my_test.rs", 10, 20);

// Create a test
let test = suite.create_test("my_test");

// Close the suite when done
suite.close();
```

### Test Management

Manage individual tests within a suite:

```rust
use test_optimization_sdk::Test;

// Create a test (from a suite)
let test = suite.create_test("my_test");

// Set test tags
test.set_string_tag("test_type", "unit");
test.set_number_tag("timeout", 5.0);

// Set source code information
test.set_test_source("src/my_test.rs", 15, 25);

// Close the test with status
test.close(TestStatus::Pass);

// Or close with skip reason
test.close_with_skip_reason("Test skipped due to missing dependencies");
```

### Performance Monitoring with Spans

Monitor performance using spans:

```rust
use test_optimization_sdk::Span;

// Create a new span
let span = Span::create(
    0, // parent_id (0 for root span)
    "test_execution",
    "my_service",
    "my_resource",
    "test"
);

// Set span tags
span.set_string_tag("environment", "staging");
span.set_number_tag("duration", 1.5);

// Set error information if needed
span.set_error_info(
    "PerformanceIssue",
    "Test execution exceeded timeout",
    "stack trace here"
);

// Close the span when done
span.close();
```

### Debugging with Mock Tracer

Use the mock tracer for debugging and testing:

```rust
use test_optimization_sdk::MockTracer;

// Reset the mock tracer
MockTracer::reset();

// Get all finished spans
let finished_spans = MockTracer::get_finished_spans();

// Get all currently open spans
let open_spans = MockTracer::get_open_spans();
```

### Settings and Configuration

Access and configure various settings:

```rust
use test_optimization_sdk::TestOptimization;

// Get current settings
let settings = TestOptimization::get_settings();

// Get flaky test retry settings
let retry_settings = TestOptimization::get_flaky_test_retries_settings();

// Get known tests
let known_tests = TestOptimization::get_known_tests();

// Get skippable tests
let skippable_tests = TestOptimization::get_skippable_tests();

// Get test management tests
let managed_tests = TestOptimization::get_test_management_tests();
```

## Settings Structure

The SDK provides various settings structures for configuration:

### Settings
- `code_coverage`: Enable/disable code coverage
- `early_flake_detection`: Early flake detection settings
- `flaky_test_retries_enabled`: Enable/disable flaky test retries
- `itr_enabled`: Enable/disable intelligent test runner
- `require_git`: Enable/disable git integration
- `tests_skipping`: Enable/disable test skipping
- `known_tests_enabled`: Enable/disable known tests tracking
- `test_management`: Test management settings

### Early Flake Detection Settings
- `enabled`: Enable/disable early flake detection
- `slow_test_retries`: Settings for slow test retries
- `faulty_session_threshold`: Threshold for faulty session detection

### Test Management Settings
- `enabled`: Enable/disable test management
- `attempt_to_fix_retries`: Number of retries for attempt-to-fix operations

## Shutdown

When you're done with the SDK, call the shutdown function:

```rust
use test_optimization_sdk::TestOptimization;

TestOptimization::shutdown();
```

## License

This project is licensed under the Apache License Version 2.0 - see the LICENSE file for details.

## Contributing

Please read CONTRIBUTING.md for details on our code of conduct and the process for submitting pull requests. 