use std::fs::File;
use std::io::prelude::*;

pub fn create_ya_make(module_path: &String) {
    let mut ya_make = File::create(format!("{}/ya.make", module_path))
        .expect("Error encountered while creating ya.make!");
    let ya_make_text = "\
JAVA_LIBRARY()

OWNER(g:mediabilling)

INCLUDE(${ARCADIA_ROOT}/media-billing/common.inc)
INCLUDE(${ARCADIA_ROOT}/media-billing/lombok.inc)

PEERDIR(
#TODO dependencies
)

JAVA_SRCS(SRCDIR src/main/java **/*)

END()

RECURSE_FOR_TESTS(src/test)
";
    ya_make.write_all(ya_make_text.as_bytes())
        .expect("Error while writing to ya.make");
}

pub fn create_tests_ya_make(module_name: &String) {
    let mut ya_make = File::create(format!("{}/src/test/ya.make", module_name))
        .expect("Error encountered while creating ya.make!");
    let ya_make_text = b"\
JUNIT5()

ENV(DISABLE_JUNIT_COMPATIBILITY_TEST=1)

OWNER(g:mediabilling)

INCLUDE(${ARCADIA_ROOT}/media-billing/tests-common.inc)

PEERDIR(
    contrib/java/org/junit/jupiter/junit-jupiter
)

EXCLUDE(
    devtools/jtest-annotations/junit4
)

JAVA_SRCS(SRCDIR java **/*)

END()
";
    ya_make.write_all(ya_make_text)
        .expect("Error while writing to tests ya.make");
}
