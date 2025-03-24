"""
Complete test for the test optimization SDK.

This test covers all the functionality of the SDK, including:
- Session management
- Module management
- Suite management
- Test management
- Span management
- Coverage data
- Benchmark data
"""

import time
from typing import Dict

from test_optimization_sdk import (
    TestOptimization,
    TestSession,
    TestModule,
    TestSuite,
    Test,
    TestStatus,
    Span,
    MockTracer,
)


def test_complete():
    """Test all functionality of the test optimization SDK."""
    # Initialize library
    TestOptimization.init_mock()

    # Create session
    session = TestSession.create("pytest", None)
    print("Hello, world!")

    # Print settings and configuration
    print(TestOptimization.get_settings())
    print(TestOptimization.get_flaky_test_retries_settings())
    print(TestOptimization.get_known_tests())
    print(TestOptimization.get_skippable_tests())
    print(TestOptimization.get_test_management_tests())

    print(f"session id: {session.session_id}")

    # Set session tags
    session.set_string_tag("Session-KeyFromPython", "Hello world")
    session.set_number_tag("Session-NumberFromPython", 42.0)

    # Create and manage session span
    session_span = Span.create(
        session.session_id,
        "my-operation-name",
        "my-service",
        "session-resource-name",
        "span-type",
    )
    print(f"span_id (from session): {session_span.span_id}")
    session_span.set_string_tag("Session-KeyFromPython", "Hello world")
    session_span.set_number_tag("Session-NumberFromPython", 42.0)
    time.sleep(0.5)
    print(f"session_span close: {session_span.close()}")

    # Create and manage module
    module = session.create_module("my-test-module", "Framework Name", "Framework Version")
    print(f"module id: {module.module_id}")

    module.set_string_tag("Module-KeyFromPython", "Hello world")
    module.set_number_tag("Module-NumberFromPython", 42.0)

    # Create and manage module span
    module_span = Span.create(
        module.module_id,
        "my-operation-name",
        "my-service",
        "module-resource-name",
        "span-type",
    )
    print(f"span_id (from module): {module_span.span_id}")
    module_span.set_string_tag("Session-KeyFromPython", "Hello world")
    module_span.set_number_tag("Session-NumberFromPython", 42.0)
    time.sleep(0.5)
    print(f"module_span close: {module_span.close()}")

    # Create and manage suite
    suite = module.create_test_suite("My Suite")
    print(f"suite id: {suite.suite_id}")

    suite.set_string_tag("Suite-KeyFromPython", "Hello world")
    suite.set_number_tag("Suite-NumberFromPython", 42.0)

    # Create and manage suite span
    suite_span = Span.create(
        suite.suite_id,
        "my-operation-name",
        "my-service",
        "suite-resource-name",
        "span-type",
    )
    print(f"span_id (from suite): {suite_span.span_id}")
    suite_span.set_string_tag("Session-KeyFromPython", "Hello world")
    suite_span.set_number_tag("Session-NumberFromPython", 42.0)
    time.sleep(0.5)
    print(f"suite_span close: {suite_span.close()}")

    # Create and manage pass test
    pass_test = suite.create_test("My PassTest")
    pass_test.set_string_tag("Pass-KeyFromPython", "Hello world")
    pass_test.set_number_tag("Pass-NumberFromPython", 42.0)
    pass_test.set_test_source("test.py", 6, 58)
    pass_test.set_coverage_data(["file.py"])

    # Set benchmark data for pass test
    measurement_data: Dict[str, float] = {
        "data1": 42.0,
        "data2": 64.0,
    }
    pass_test.set_benchmark_number_data("my_custom_measurement", measurement_data)

    measurement_strdata: Dict[str, str] = {
        "datastr1": "MyData",
        "datastr2": "MyData2",
    }
    pass_test.set_benchmark_string_data("my_custom_measurement", measurement_strdata)
    time.sleep(1.0)

    # Create and manage test span
    test_span = Span.create(
        pass_test.test_id,
        "my-operation-name",
        "my-service",
        "test-resource-name",
        "span-type",
    )
    print(f"span_id (from test): {test_span.span_id}")
    test_span.set_string_tag("Session-KeyFromPython", "Hello world")
    test_span.set_number_tag("Session-NumberFromPython", 42.0)
    time.sleep(0.5)
    print(f"test_span close: {test_span.close()}")

    print(f"pass test close: {pass_test.close(TestStatus.Pass)}")

    # Create and manage fail test
    fail_test = suite.create_test("My FailTest")
    fail_test.set_string_tag("Fail-KeyFromPython", "Hello world")
    fail_test.set_number_tag("Fail-NumberFromPython", 42.0)
    fail_test.set_error_info("custom_error_type", "error from python lib", "...")
    time.sleep(1.0)
    print(f"fail test close: {fail_test.close(TestStatus.Fail)}")

    # Create and manage skip test
    skip_test = suite.create_test("My SkipTest")
    skip_test.set_string_tag("Skip-KeyFromPython", "Hello world")
    skip_test.set_number_tag("Skip-KeyFromPython", 42.0)
    time.sleep(1.0)
    skip_reason = "skip because yes"
    print(f"skip test close: {skip_test.close_with_skip_reason(skip_reason)}")

    # Close everything
    print(f"suite closed: {suite.close()}")
    print(f"module closed: {module.close()}")
    session.close(0)

    # Shutdown the library
    TestOptimization.shutdown()

    # Print finished spans
    spans = MockTracer.get_finished_spans()
    for span in spans:
        print(f"span: {span}")


if __name__ == "__main__":
    test_complete() 