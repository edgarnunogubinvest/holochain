use crate::prelude::*;

// #[cfg(feature = "mock")]
// use mockall::*;

pub const IDK_NOT_REGISTERED: &str = "IDK not registered";

/// This is a cell so it can be set many times.
/// Every test needs its own mock so each test needs to set it.
use core::cell::RefCell;

#[cfg(any(feature = "mock", not(target_arch = "wasm32")))]
thread_local!(pub static IDK: RefCell<Box<dyn IdkT>> = RefCell::new(Box::new(ErrIdk)));

#[cfg(all(not(feature = "mock"), target_arch = "wasm32"))]
thread_local!(pub static IDK: RefCell<Box<dyn IdkT>> = RefCell::new(Box::new(HostIdk)));

/// When mocking is enabled the mockall crate automatically builds a MockIdkT for us.
/// ```ignore
/// let mut mock = MockIdkT::new();
/// mock_idk.expect_foo().times(1).etc().etc();
/// set_idk(mock_idk);
/// ```
// #[cfg_attr(feature = "mock", automock)]
pub trait IdkT: Send + Sync {
    // Ed25519
    fn verify_signature(&self, verify_signature: VerifySignature) -> ExternResult<bool>;
    fn hash(&self, hash_input: HashInput) -> ExternResult<HashOutput>;
    fn must_get_entry(&self, must_get_entry_input: MustGetEntryInput) -> ExternResult<EntryHashed>;
    fn must_get_header(
        &self,
        must_get_header_input: MustGetHeaderInput,
    ) -> ExternResult<SignedHeaderHashed>;
    fn must_get_valid_element(
        &self,
        must_get_valid_element_input: MustGetValidElementInput,
    ) -> ExternResult<Element>;
    // Info
    fn dna_info(&self, dna_info_input: ()) -> ExternResult<DnaInfo>;
    fn zome_info(&self, zome_info_input: ()) -> ExternResult<ZomeInfo>;
    // Trace
    #[cfg(feature = "trace")]
    fn trace(&self, trace_msg: TraceMsg) -> ExternResult<()>;
    // XSalsa20Poly1305
    fn x_salsa20_poly1305_decrypt(
        &self,
        x_salsa20_poly1305_decrypt: XSalsa20Poly1305Decrypt,
    ) -> ExternResult<Option<XSalsa20Poly1305Data>>;
    fn x_25519_x_salsa20_poly1305_decrypt(
        &self,
        x_25519_x_salsa20_poly1305_decrypt: X25519XSalsa20Poly1305Decrypt,
    ) -> ExternResult<Option<XSalsa20Poly1305Data>>;
}

/// Used as a placeholder before any other Idk is registered.
/// Generally only useful for testing but technically can be set any time.
pub struct ErrIdk;

impl ErrIdk {
    fn err<T>() -> ExternResult<T> {
        Err(WasmError::Guest(IDK_NOT_REGISTERED.to_string()))
    }
}

/// Every call is an error for the ErrIdk.
impl IdkT for ErrIdk {
    fn verify_signature(&self, _: VerifySignature) -> ExternResult<bool> {
        Self::err()
    }
    fn hash(&self, _: HashInput) -> ExternResult<HashOutput> {
        Self::err()
    }
    fn must_get_entry(&self, _: MustGetEntryInput) -> ExternResult<EntryHashed> {
        Self::err()
    }
    fn must_get_header(&self, _: MustGetHeaderInput) -> ExternResult<SignedHeaderHashed> {
        Self::err()
    }
    fn must_get_valid_element(&self, _: MustGetValidElementInput) -> ExternResult<Element> {
        Self::err()
    }
    fn dna_info(&self, _: ()) -> ExternResult<DnaInfo> {
        Self::err()
    }
    fn zome_info(&self, _: ()) -> ExternResult<ZomeInfo> {
        Self::err()
    }
    // Trace
    #[cfg(feature = "trace")]
    fn trace(&self, _: TraceMsg) -> ExternResult<()> {
        Self::err()
    }
    fn x_salsa20_poly1305_decrypt(
        &self,
        _: XSalsa20Poly1305Decrypt,
    ) -> ExternResult<Option<XSalsa20Poly1305Data>> {
        Self::err()
    }
    fn x_25519_x_salsa20_poly1305_decrypt(
        &self,
        _: X25519XSalsa20Poly1305Decrypt,
    ) -> ExternResult<Option<XSalsa20Poly1305Data>> {
        Self::err()
    }
}

/// The IDK implemented as externs provided by the host.
pub struct HostIdk;

impl HostIdk {
    pub const fn new() -> Self {
        Self {}
    }
}

/// The real idk implements `host_call` for every idk function.
/// This is deferring to the standard `holochain_wasmer_guest` crate functionality.
/// Every function works exactly the same way with the same basic signatures and patterns.
/// Elsewhere in the idk are more high level wrappers around this basic trait.
#[cfg(all(not(feature = "mock"), target_arch = "wasm32"))]
impl IdkT for HostIdk {
    fn verify_signature(&self, verify_signature: VerifySignature) -> ExternResult<bool> {
        host_call::<VerifySignature, bool>(__verify_signature, verify_signature)
    }
    fn hash(&self, hash_input: HashInput) -> ExternResult<HashOutput> {
        host_call::<HashInput, HashOutput>(__hash, hash_input)
    }
    fn must_get_entry(&self, must_get_entry_input: MustGetEntryInput) -> ExternResult<EntryHashed> {
        host_call::<MustGetEntryInput, EntryHashed>(__must_get_entry, must_get_entry_input)
    }
    fn must_get_header(
        &self,
        must_get_header_input: MustGetHeaderInput,
    ) -> ExternResult<SignedHeaderHashed> {
        host_call::<MustGetHeaderInput, SignedHeaderHashed>(
            __must_get_header,
            must_get_header_input,
        )
    }
    fn must_get_valid_element(
        &self,
        must_get_valid_element_input: MustGetValidElementInput,
    ) -> ExternResult<Element> {
        host_call::<MustGetValidElementInput, Element>(
            __must_get_valid_element,
            must_get_valid_element_input,
        )
    }
    fn dna_info(&self, _: ()) -> ExternResult<DnaInfo> {
        host_call::<(), DnaInfo>(__dna_info, ())
    }
    fn zome_info(&self, _: ()) -> ExternResult<ZomeInfo> {
        host_call::<(), ZomeInfo>(__zome_info, ())
    }
    #[cfg(feature = "trace")]
    fn trace(&self, trace_msg: TraceMsg) -> ExternResult<()> {
        host_call::<TraceMsg, ()>(__trace, trace_msg)
    }
    fn x_salsa20_poly1305_decrypt(
        &self,
        x_salsa20_poly1305_decrypt: XSalsa20Poly1305Decrypt,
    ) -> ExternResult<Option<XSalsa20Poly1305Data>> {
        host_call::<XSalsa20Poly1305Decrypt, Option<XSalsa20Poly1305Data>>(
            __x_salsa20_poly1305_decrypt,
            x_salsa20_poly1305_decrypt,
        )
    }
    fn x_25519_x_salsa20_poly1305_decrypt(
        &self,
        x_25519_x_salsa20_poly1305_decrypt: X25519XSalsa20Poly1305Decrypt,
    ) -> ExternResult<Option<XSalsa20Poly1305Data>> {
        host_call::<X25519XSalsa20Poly1305Decrypt, Option<XSalsa20Poly1305Data>>(
            __x_25519_x_salsa20_poly1305_decrypt,
            x_25519_x_salsa20_poly1305_decrypt,
        )
    }
}

/// At any time the global IDK can be set to a different idk.
/// Generally this is only useful during rust unit testing.
/// When executing wasm without the `mock` feature, the host will be assumed.
pub fn set_idk<H: 'static>(idk: H) -> Box<dyn IdkT>
where
    H: IdkT,
{
    IDK.with(|h| std::mem::replace(&mut *h.borrow_mut(), Box::new(idk)))
}