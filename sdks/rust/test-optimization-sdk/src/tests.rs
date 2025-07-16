// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

use crate::test_optimization::*;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn complete() {
    // Initialize library
    TestOptimization::init_mock();

    // session
    let session = TestSession::create(Some("cargo test"), None::<&str>);
    println!("Hello, world!");

    println!("{:?}", TestOptimization::get_settings());
    println!("{:?}", TestOptimization::get_flaky_test_retries_settings());
    println!("{:?}", TestOptimization::get_known_tests());
    println!("{:?}", TestOptimization::get_skippable_tests());
    println!("{:?}", TestOptimization::get_test_management_tests());

    println!("session id: {:?}", session.session_id);

    session.set_string_tag("Session-KeyFromRust", "Hello world");
    session.set_number_tag("Session-NumberFromRust", 42f64);

    // Session span
    let session_span = Span::create(session.session_id, "my-operation-name", "my-service", "session-resource-name", "span-type");
    println!("span_id (from session): {:?}", session_span.span_id);
    session_span.set_string_tag("Session-KeyFromRust", "Hello world");
    session_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("session_span close: {}", session_span.close());

    // module
    let module_name = String::from("my-test-module");
    let module = session.create_module(module_name, "Framework Name", "Framework Version");
    println!("module id: {:?}", module.module_id);

    module.set_string_tag("Module-KeyFromRust", "Hello world");
    module.set_number_tag("Module-NumberFromRust", 42f64);

    // Module span
    let module_span = Span::create(module.module_id, "my-operation-name", "my-service", "module-resource-name", "span-type");
    println!("span_id (from module): {:?}", module_span.span_id);
    module_span.set_string_tag("Session-KeyFromRust", "Hello world");
    module_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("module_span close: {}", module_span.close());

    // suite
    let suite = module.create_test_suite("My Suite");
    println!("suite id: {:?}", suite.suite_id);

    suite.set_string_tag("Suite-KeyFromRust", "Hello world");
    suite.set_number_tag("Suite-NumberFromRust", 42f64);

    // Suite span
    let suite_span = Span::create(suite.suite_id, "my-operation-name", "my-service", "suite-resource-name", "span-type");
    println!("span_id (from suite): {:?}", suite_span.span_id);
    suite_span.set_string_tag("Session-KeyFromRust", "Hello world");
    suite_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("suite_span close: {}", suite_span.close());

    // pass test
    let pass_test = suite.create_test("My PassTest");
    pass_test.set_string_tag("Pass-KeyFromRust", "Hello world");
    pass_test.set_number_tag("Pass-NumberFromRust", 42f64);
    pass_test.set_test_source("test.rs", &6, &58);
    pass_test.set_coverage_data(&["file.rs"]);
    pass_test.log("Hello world", Some("tag1=value1,tag2=value2"));
    pass_test.log("Hello world", None::<&str>);

    let mut measurement_data: HashMap<&str, f64> = HashMap::new();
    measurement_data.insert("data1", 42f64);
    measurement_data.insert("data2", 64f64);
    pass_test.set_benchmark_number_data("my_custom_measurement", &measurement_data);

    let mut measurement_strdata: HashMap<&str, String> = HashMap::new();
    measurement_strdata.insert("datastr1", "MyData".to_string());
    measurement_strdata.insert("datastr2", "MyData2".to_string());
    pass_test.set_benchmark_string_data("my_custom_measurement", &measurement_strdata);
    sleep(Duration::from_millis(1000));

    // Test span
    let test_span = Span::create(pass_test.test_id, "my-operation-name", "my-service", "test-resource-name", "span-type");
    println!("span_id (from test): {:?}", test_span.span_id);
    test_span.set_string_tag("Session-KeyFromRust", "Hello world");
    test_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("test_span close: {}", test_span.close());

    println!("pass test close: {}", pass_test.close(TestStatus::Pass));

    // fail test
    let fail_test = suite.create_test("My FailTest");
    fail_test.set_string_tag("Fail-KeyFromRust", "Hello world");
    fail_test.set_number_tag("Fail-NumberFromRust", 42f64);
    fail_test.set_error_info("custom_error_type", "error from rust lib", "...");
    sleep(Duration::from_millis(1000));
    println!("fail test close: {}", fail_test.close(TestStatus::Fail));

    // skip test
    let skip_test = suite.create_test("My SkipTest");
    skip_test.set_string_tag("Skip-KeyFromRust", "Hello world");
    skip_test.set_number_tag("Skip-KeyFromRust", 42f64);
    sleep(Duration::from_millis(1000));
    let skip_reason = String::from("skip because yes");
    println!("skip test close: {}", skip_test.close_with_skip_reason(skip_reason));

    // close everything
    println!("suite closed: {}", suite.close());
    println!("module closed: {}", module.close());
    session.close(0);
    
    // shutdown the library
    TestOptimization::shutdown();

    let spans = MockTracer::get_finished_spans();
    for span in spans {
        println!("span: {:?}", span);
    }
}
