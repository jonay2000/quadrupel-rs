#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0e00],
    #[doc = "0xe00..0xe18 - RAM configurable priority configuration structure."]
    pub rampri: RAMPRI,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RAMPRI {
    #[doc = "0x00 - Configurable priority configuration register for CPU0."]
    pub cpu0: crate::Reg<self::rampri::cpu0::CPU0_SPEC>,
    #[doc = "0x04 - Configurable priority configuration register for SPIS1."]
    pub spis1: crate::Reg<self::rampri::spis1::SPIS1_SPEC>,
    #[doc = "0x08 - Configurable priority configuration register for RADIO."]
    pub radio: crate::Reg<self::rampri::radio::RADIO_SPEC>,
    #[doc = "0x0c - Configurable priority configuration register for ECB."]
    pub ecb: crate::Reg<self::rampri::ecb::ECB_SPEC>,
    #[doc = "0x10 - Configurable priority configuration register for CCM."]
    pub ccm: crate::Reg<self::rampri::ccm::CCM_SPEC>,
    #[doc = "0x14 - Configurable priority configuration register for AAR."]
    pub aar: crate::Reg<self::rampri::aar::AAR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "RAM configurable priority configuration structure."]
pub mod rampri;
