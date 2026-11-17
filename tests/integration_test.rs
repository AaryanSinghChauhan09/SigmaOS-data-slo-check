// Integration test for SigmaOS-data-slo-check
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_data_slo_check as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
