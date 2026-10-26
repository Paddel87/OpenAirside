//! Forced failure for the proof of step Q.5 (ADR-043): a red state on which a documentation
//! commit must not turn the checks green. Reverted right after the proof.

#[test]
fn forced_failure_for_the_ci_proof() {
    panic!("forced failure (Q.5, ADR-043), reverted after the proof");
}
