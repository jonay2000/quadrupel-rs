#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Start SPI transaction."]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x14 - Stop SPI transaction."]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x1c - Suspend SPI transaction."]
    pub tasks_suspend: crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>,
    #[doc = "0x20 - Resume SPI transaction."]
    pub tasks_resume: crate::Reg<tasks_resume::TASKS_RESUME_SPEC>,
    _reserved4: [u8; 0xe0],
    #[doc = "0x104 - SPI transaction has stopped."]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x110 - End of RXD buffer reached."]
    pub events_endrx: crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x120 - End of TXD buffer reached."]
    pub events_endtx: crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>,
    _reserved7: [u8; 0x28],
    #[doc = "0x14c - Transaction started."]
    pub events_started: crate::Reg<events_started::EVENTS_STARTED_SPEC>,
    _reserved8: [u8; 0x01b4],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved10: [u8; 0x01f4],
    #[doc = "0x500 - Enable SPIM."]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x508..0x514 - Pin select configuration."]
    pub psel: PSEL,
    _reserved12: [u8; 0x10],
    #[doc = "0x524 - SPI frequency."]
    pub frequency: crate::Reg<frequency::FREQUENCY_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x534..0x540 - RXD EasyDMA configuration and status."]
    pub rxd: RXD,
    _reserved14: [u8; 0x04],
    #[doc = "0x544..0x550 - TXD EasyDMA configuration and status."]
    pub txd: TXD,
    _reserved15: [u8; 0x04],
    #[doc = "0x554 - Configuration register."]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved16: [u8; 0x68],
    #[doc = "0x5c0 - Over-read character."]
    pub orc: crate::Reg<orc::ORC_SPEC>,
    _reserved17: [u8; 0x0a38],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK."]
    pub sck: crate::Reg<self::psel::sck::SCK_SPEC>,
    #[doc = "0x04 - Pin select for MOSI."]
    pub mosi: crate::Reg<self::psel::mosi::MOSI_SPEC>,
    #[doc = "0x08 - Pin select for MISO."]
    pub miso: crate::Reg<self::psel::miso::MISO_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Pin select configuration."]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer."]
    pub ptr: crate::Reg<self::rxd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of buffer bytes to receive."]
    pub maxcnt: crate::Reg<self::rxd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes received in the last transaction."]
    pub amount: crate::Reg<self::rxd::amount::AMOUNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "RXD EasyDMA configuration and status."]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer."]
    pub ptr: crate::Reg<self::txd::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of buffer bytes to send."]
    pub maxcnt: crate::Reg<self::txd::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of bytes sent in the last transaction."]
    pub amount: crate::Reg<self::txd::amount::AMOUNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA configuration and status."]
pub mod txd;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start SPI transaction."]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop SPI transaction."]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend SPI transaction."]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME register accessor: an alias for `Reg<TASKS_RESUME_SPEC>`"]
pub type TASKS_RESUME = crate::Reg<tasks_resume::TASKS_RESUME_SPEC>;
#[doc = "Resume SPI transaction."]
pub mod tasks_resume;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "SPI transaction has stopped."]
pub mod events_stopped;
#[doc = "EVENTS_ENDRX register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "End of RXD buffer reached."]
pub mod events_endrx;
#[doc = "EVENTS_ENDTX register accessor: an alias for `Reg<EVENTS_ENDTX_SPEC>`"]
pub type EVENTS_ENDTX = crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>;
#[doc = "End of TXD buffer reached."]
pub mod events_endtx;
#[doc = "EVENTS_STARTED register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "Transaction started."]
pub mod events_started;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPIM."]
pub mod enable;
#[doc = "FREQUENCY register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "SPI frequency."]
pub mod frequency;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register."]
pub mod config;
#[doc = "ORC register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Over-read character."]
pub mod orc;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
