mod runtime;

#[test]
fn sched_yield() {
    runtime::run_test("tests/misc-tests/bin/sched_yield.wasm")
}

#[test]
fn truncation_rights() {
    runtime::run_test("tests/misc-tests/bin/truncation_rights.wasm")
}

#[test]
fn unlink_directory() {
    runtime::run_test("tests/misc-tests/bin/unlink_directory.wasm")
}

#[test]
fn remove_nonempty_directory() {
    runtime::run_test("tests/misc-tests/bin/remove_nonempty_directory.wasm")
}

#[test]
fn interesting_paths() {
    runtime::run_test("tests/misc-tests/bin/interesting_paths.wasm")
}

#[test]
fn nofollow_errors() {
    runtime::run_test("tests/misc-tests/bin/nofollow_errors.wasm")
}

#[test]
fn symlink_loop() {
    runtime::run_test("tests/misc-tests/bin/symlink_loop.wasm")
}

#[test]
fn close_preopen() {
    runtime::run_test("tests/misc-tests/bin/close_preopen.wasm")
}

#[test]
fn clock_time_get() {
    runtime::run_test("tests/misc-tests/bin/clock_time_get.wasm")
}

#[test]
fn readlink_no_buffer() {
    runtime::run_test("tests/misc-tests/bin/readlink_no_buffer.wasm")
}

#[test]
fn isatty() {
    runtime::run_test("tests/misc-tests/bin/isatty.wasm")
}

#[test]
fn directory_seek() {
    runtime::run_test("tests/misc-tests/bin/directory_seek.wasm")
}

#[test]
fn big_random_buf() {
    runtime::run_test("tests/misc-tests/bin/big_random_buf.wasm")
}