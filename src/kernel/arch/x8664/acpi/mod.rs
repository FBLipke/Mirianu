use core::str;

#[derive(Debug)]
pub enum MadtError {
    UnexpectedEntry,
    InterruptOverrideEntryHasInvalidBus,
    InvalidLocalNmiLine,
    MpsIntiInvalidPolarity,
    MpsIntiInvalidTriggerMode,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RsdpError {
    NoValidRsdp,
    IncorrectSignature,
    InvalidOemId,
    InvalidChecksum,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Signature([u8; 4]);

impl Signature {
    pub const RSDT: Signature = Signature(*b"RSDT");
    pub const XSDT: Signature = Signature(*b"XSDT");
    pub const FADT: Signature = Signature(*b"FACP");
    pub const HPET: Signature = Signature(*b"HPET");
    pub const MADT: Signature = Signature(*b"APIC");
    pub const MCFG: Signature = Signature(*b"MCFG");
    pub const SSDT: Signature = Signature(*b"SSDT");
    pub const BERT: Signature = Signature(*b"BERT");
    pub const BGRT: Signature = Signature(*b"BGRT");
    pub const CPEP: Signature = Signature(*b"CPEP");
    pub const DSDT: Signature = Signature(*b"DSDT");
    pub const ECDT: Signature = Signature(*b"ECDT");
    pub const EINJ: Signature = Signature(*b"EINJ");
    pub const ERST: Signature = Signature(*b"ERST");
    pub const FACS: Signature = Signature(*b"FACS");
    pub const FPDT: Signature = Signature(*b"FPDT");
    pub const GTDT: Signature = Signature(*b"GTDT");
    pub const HEST: Signature = Signature(*b"HEST");
    pub const MSCT: Signature = Signature(*b"MSCT");
    pub const MPST: Signature = Signature(*b"MPST");
    pub const NFIT: Signature = Signature(*b"NFIT");
    pub const PCCT: Signature = Signature(*b"PCCT");
    pub const PHAT: Signature = Signature(*b"PHAT");
    pub const PMTT: Signature = Signature(*b"PMTT");
    pub const PSDT: Signature = Signature(*b"PSDT");
    pub const RASF: Signature = Signature(*b"RASF");
    pub const SBST: Signature = Signature(*b"SBST");
    pub const SDEV: Signature = Signature(*b"SDEV");
    pub const SLIT: Signature = Signature(*b"SLIT");
    pub const SRAT: Signature = Signature(*b"SRAT");
    pub const AEST: Signature = Signature(*b"AEST");
    pub const BDAT: Signature = Signature(*b"BDAT");
    pub const CDIT: Signature = Signature(*b"CDIT");
    pub const CEDT: Signature = Signature(*b"CEDT");
    pub const CRAT: Signature = Signature(*b"CRAT");
    pub const CSRT: Signature = Signature(*b"CSRT");
    pub const DBGP: Signature = Signature(*b"DBGP");
    pub const DBG2: Signature = Signature(*b"DBG2");
    pub const DMAR: Signature = Signature(*b"DMAR");
    pub const DRTM: Signature = Signature(*b"DRTM");
    pub const ETDT: Signature = Signature(*b"ETDT");
    pub const IBFT: Signature = Signature(*b"IBFT");
    pub const IORT: Signature = Signature(*b"IORT");
    pub const IVRS: Signature = Signature(*b"IVRS");
    pub const LPIT: Signature = Signature(*b"LPIT");
    pub const MCHI: Signature = Signature(*b"MCHI");
    pub const MPAM: Signature = Signature(*b"MPAM");
    pub const MSDM: Signature = Signature(*b"MSDM");
    pub const PRMT: Signature = Signature(*b"PRMT");
    pub const RGRT: Signature = Signature(*b"RGRT");
    pub const SDEI: Signature = Signature(*b"SDEI");
    pub const SLIC: Signature = Signature(*b"SLIC");
    pub const SPCR: Signature = Signature(*b"SPCR");
    pub const SPMI: Signature = Signature(*b"SPMI");
    pub const STAO: Signature = Signature(*b"STAO");
    pub const SVKL: Signature = Signature(*b"SVKL");
    pub const TCPA: Signature = Signature(*b"TCPA");
    pub const TPM2: Signature = Signature(*b"TPM2");
    pub const UEFI: Signature = Signature(*b"UEFI");
    pub const WAET: Signature = Signature(*b"WAET");
    pub const WDAT: Signature = Signature(*b"WDAT");
    pub const WDRT: Signature = Signature(*b"WDRT");
    pub const WPBT: Signature = Signature(*b"WPBT");
    pub const WSMT: Signature = Signature(*b"WSMT");
    pub const XENV: Signature = Signature(*b"XENV");

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.as_str())
    }
}

#[derive(Debug)]
pub enum AcpiError {
    Rsdp(RsdpError),

    SdtInvalidSignature(Signature),
    SdtInvalidOemId(Signature),
    SdtInvalidTableId(Signature),
    SdtInvalidChecksum(Signature),

    TableMissing(Signature),
    InvalidFacsAddress,
    InvalidDsdtAddress,
    InvalidMadt(MadtError),
    InvalidGenericAddress,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SdtHeader {
    pub signature: Signature,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl SdtHeader {
    /// Check that:
    ///     a) The signature matches the one given
    ///     b) The checksum of the SDT is valid
    ///
    /// This assumes that the whole SDT is mapped.
    pub fn validate(&self, signature: Signature) -> Result<(), AcpiError> {
        // Check the signature
        if self.signature != signature {
            return Err(AcpiError::SdtInvalidSignature(signature));
        }

        // Check the OEM id
        if str::from_utf8(&self.oem_id).is_err() {
            return Err(AcpiError::SdtInvalidOemId(signature));
        }

        // Check the OEM table id
        if str::from_utf8(&self.oem_table_id).is_err() {
            return Err(AcpiError::SdtInvalidTableId(signature));
        }

        // Validate the checksum
        let self_ptr = self as *const SdtHeader as *const u8;
        let mut sum: u8 = 0;
        for i in 0..self.length {
            sum = sum.wrapping_add(unsafe { *(self_ptr.offset(i as isize)) } as u8);
        }

        if sum > 0 {
            return Err(AcpiError::SdtInvalidChecksum(signature));
        }

        Ok(())
    }

    pub fn oem_id(&self) -> &str {
        // Safe to unwrap because checked in `validate`
        str::from_utf8(&self.oem_id).unwrap()
    }

    pub fn oem_table_id(&self) -> &str {
        // Safe to unwrap because checked in `validate`
        str::from_utf8(&self.oem_table_id).unwrap()
    }
}
