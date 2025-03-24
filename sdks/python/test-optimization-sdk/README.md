# Datadog Test Optimization SDK for Python

This SDK provides integration with Datadog's Test Optimization features for Python applications.

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

Install the package using pip:

```bash
pip install test-optimization-sdk
```

## Usage

### Initialization

First, initialize the SDK:

```python
from test_optimization_sdk import TestOptimization

# Basic initialization
TestOptimization.init()

# Or with a working directory
TestOptimization.init_with_working_dir("/path/to/working/dir")

# Or with mock tracer for testing
TestOptimization.init_mock()
```

### Test Session Management

Create and manage test sessions:

```python
from test_optimization_sdk import TestSession

# Create a new test session
session = TestSession.create(
    framework="my_framework",
    framework_version="1.0.0"
)

# Set tags
session.set_string_tag("environment", "staging")
session.set_number_tag("timeout", 30.0)

# Set error information if needed
session.set_error_info(
    error_type="TestFailure",
    error_message="Test failed due to assertion error",
    error_stacktrace="stack trace here"
)

# Create a test module
module = session.create_module(
    name="my_module",
    framework_name="my_framework",
    framework_version="1.0.0"
)

# Close the session when done
session.close(0)  # 0 for success, non-zero for failure
```

### Test Module Management

Manage test modules within a session:

```python
from test_optimization_sdk import TestModule

# Create a test module (from a session)
module = session.create_module(
    name="my_module",
    framework_name="my_framework",
    framework_version="1.0.0"
)

# Set module tags
module.set_string_tag("module_type", "integration")
module.set_number_tag("timeout", 60.0)

# Create a test suite
suite = module.create_test_suite("my_suite")

# Close the module when done
module.close()
```

### Test Suite Management

Manage test suites within a module:

```python
from test_optimization_sdk import TestSuite

# Create a test suite (from a module)
suite = module.create_test_suite("my_suite")

# Set suite tags
suite.set_string_tag("suite_type", "regression")
suite.set_number_tag("priority", 1.0)

# Set source code information
suite.set_test_source("src/my_test.py", 10, 20)

# Create a test
test = suite.create_test("my_test")

# Close the suite when done
suite.close()
```

### Test Management

Manage individual tests within a suite:

```python
from test_optimization_sdk import Test, TestStatus

# Create a test (from a suite)
test = suite.create_test("my_test")

# Set test tags
test.set_string_tag("test_type", "unit")
test.set_number_tag("timeout", 5.0)

# Set source code information
test.set_test_source("src/my_test.py", 15, 25)

# Close the test with status
test.close(TestStatus.PASS)

# Or close with skip reason
test.close_with_skip_reason("Test skipped due to missing dependencies")
```

### Performance Monitoring with Spans

Monitor performance using spans:

```python
from test_optimization_sdk import Span

# Create a new span
span = Span.create(
    parent_id=0,  # 0 for root span
    operation_name="test_execution",
    service_name="my_service",
    resource_name="my_resource",
    span_type="test"
)

# Set span tags
span.set_string_tag("environment", "staging")
span.set_number_tag("duration", 1.5)

# Set error information if needed
span.set_error_info(
    error_type="PerformanceIssue",
    error_message="Test execution exceeded timeout",
    error_stacktrace="stack trace here"
)

# Close the span when done
span.close()
```

### Debugging with Mock Tracer

Use the mock tracer for debugging and testing:

```python
from test_optimization_sdk import MockTracer

# Reset the mock tracer
MockTracer.reset()

# Get all finished spans
finished_spans = MockTracer.get_finished_spans()

# Get all currently open spans
open_spans = MockTracer.get_open_spans()
```

### Settings and Configuration

Access and configure various settings:

```python
from test_optimization_sdk import TestOptimization

# Get current settings
settings = TestOptimization.get_settings()

# Get flaky test retry settings
retry_settings = TestOptimization.get_flaky_test_retries_settings()

# Get known tests
known_tests = TestOptimization.get_known_tests()

# Get skippable tests
skippable_tests = TestOptimization.get_skippable_tests()

# Get test management tests
managed_tests = TestOptimization.get_test_management_tests()
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

## Development

To set up the development environment:

```bash
# Clone the repository
git clone https://github.com/DataDog/test-optimization-native.git
cd test-optimization-native/sdks/python/test-optimization-sdk

# Create a virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Format code
black .
isort .

# Type checking
mypy .
``` 