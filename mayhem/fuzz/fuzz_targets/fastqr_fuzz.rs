#![no_main]

// fastqr_fuzz — ported verbatim from mayhemheroes/fast_qr@2e6f778 (fuzz/fuzz_targets/fastqr_fuzz.rs),
// the harness the anchor run (fastqr-fuzz/5) actually built. It forces Version::V02 and unwraps
// build(), so any content that doesn't fit in that tiny fixed version panics via
// QRCodeError::SpecifiedVersion — this is the run's one relevant defect (CWE-20).
use fast_qr::qr::QRBuilder;
use fast_qr::Version;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (u8, &[u8])| {
    let (_version, bytes) = data;
    QRBuilder::new(bytes).version(Version::V02).build().unwrap();
});
