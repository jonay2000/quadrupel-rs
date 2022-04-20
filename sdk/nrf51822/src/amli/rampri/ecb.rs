#[doc = "Register `ECB` reader"]
pub struct R(crate::R<ECB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECB` writer"]
pub struct W(crate::W<ECB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ECB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configuration field for RAM block 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM0_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM0_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM0` reader - Configuration field for RAM block 0."]
pub struct RAM0_R(crate::FieldReader<u8, RAM0_A>);
impl RAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM0_A> {
        match self.bits {
            0 => Some(RAM0_A::PRI0),
            2 => Some(RAM0_A::PRI2),
            4 => Some(RAM0_A::PRI4),
            6 => Some(RAM0_A::PRI6),
            8 => Some(RAM0_A::PRI8),
            10 => Some(RAM0_A::PRI10),
            12 => Some(RAM0_A::PRI12),
            14 => Some(RAM0_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM0_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM0_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM0_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM0_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM0_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM0_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM0_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM0_A::PRI14
    }
}
impl core::ops::Deref for RAM0_R {
    type Target = crate::FieldReader<u8, RAM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM0` writer - Configuration field for RAM block 0."]
pub struct RAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM0_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM0_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM0_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM0_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM0_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM0_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM0_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM0_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Configuration field for RAM block 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM1_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM1_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM1` reader - Configuration field for RAM block 1."]
pub struct RAM1_R(crate::FieldReader<u8, RAM1_A>);
impl RAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM1_A> {
        match self.bits {
            0 => Some(RAM1_A::PRI0),
            2 => Some(RAM1_A::PRI2),
            4 => Some(RAM1_A::PRI4),
            6 => Some(RAM1_A::PRI6),
            8 => Some(RAM1_A::PRI8),
            10 => Some(RAM1_A::PRI10),
            12 => Some(RAM1_A::PRI12),
            14 => Some(RAM1_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM1_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM1_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM1_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM1_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM1_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM1_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM1_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM1_A::PRI14
    }
}
impl core::ops::Deref for RAM1_R {
    type Target = crate::FieldReader<u8, RAM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM1` writer - Configuration field for RAM block 1."]
pub struct RAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM1_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM1_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM1_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM1_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM1_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM1_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM1_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM1_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Configuration field for RAM block 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM2_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM2_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM2` reader - Configuration field for RAM block 2."]
pub struct RAM2_R(crate::FieldReader<u8, RAM2_A>);
impl RAM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM2_A> {
        match self.bits {
            0 => Some(RAM2_A::PRI0),
            2 => Some(RAM2_A::PRI2),
            4 => Some(RAM2_A::PRI4),
            6 => Some(RAM2_A::PRI6),
            8 => Some(RAM2_A::PRI8),
            10 => Some(RAM2_A::PRI10),
            12 => Some(RAM2_A::PRI12),
            14 => Some(RAM2_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM2_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM2_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM2_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM2_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM2_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM2_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM2_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM2_A::PRI14
    }
}
impl core::ops::Deref for RAM2_R {
    type Target = crate::FieldReader<u8, RAM2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM2` writer - Configuration field for RAM block 2."]
pub struct RAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM2_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM2_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM2_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM2_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM2_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM2_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM2_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM2_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Configuration field for RAM block 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM3_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM3_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM3` reader - Configuration field for RAM block 3."]
pub struct RAM3_R(crate::FieldReader<u8, RAM3_A>);
impl RAM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM3_A> {
        match self.bits {
            0 => Some(RAM3_A::PRI0),
            2 => Some(RAM3_A::PRI2),
            4 => Some(RAM3_A::PRI4),
            6 => Some(RAM3_A::PRI6),
            8 => Some(RAM3_A::PRI8),
            10 => Some(RAM3_A::PRI10),
            12 => Some(RAM3_A::PRI12),
            14 => Some(RAM3_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM3_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM3_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM3_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM3_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM3_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM3_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM3_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM3_A::PRI14
    }
}
impl core::ops::Deref for RAM3_R {
    type Target = crate::FieldReader<u8, RAM3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM3` writer - Configuration field for RAM block 3."]
pub struct RAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM3_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM3_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM3_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM3_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM3_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM3_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM3_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM3_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Configuration field for RAM block 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM4_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM4_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM4` reader - Configuration field for RAM block 4."]
pub struct RAM4_R(crate::FieldReader<u8, RAM4_A>);
impl RAM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM4_A> {
        match self.bits {
            0 => Some(RAM4_A::PRI0),
            2 => Some(RAM4_A::PRI2),
            4 => Some(RAM4_A::PRI4),
            6 => Some(RAM4_A::PRI6),
            8 => Some(RAM4_A::PRI8),
            10 => Some(RAM4_A::PRI10),
            12 => Some(RAM4_A::PRI12),
            14 => Some(RAM4_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM4_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM4_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM4_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM4_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM4_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM4_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM4_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM4_A::PRI14
    }
}
impl core::ops::Deref for RAM4_R {
    type Target = crate::FieldReader<u8, RAM4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM4` writer - Configuration field for RAM block 4."]
pub struct RAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM4_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM4_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM4_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM4_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM4_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM4_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM4_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM4_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Configuration field for RAM block 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM5_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM5_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM5` reader - Configuration field for RAM block 5."]
pub struct RAM5_R(crate::FieldReader<u8, RAM5_A>);
impl RAM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM5_A> {
        match self.bits {
            0 => Some(RAM5_A::PRI0),
            2 => Some(RAM5_A::PRI2),
            4 => Some(RAM5_A::PRI4),
            6 => Some(RAM5_A::PRI6),
            8 => Some(RAM5_A::PRI8),
            10 => Some(RAM5_A::PRI10),
            12 => Some(RAM5_A::PRI12),
            14 => Some(RAM5_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM5_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM5_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM5_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM5_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM5_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM5_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM5_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM5_A::PRI14
    }
}
impl core::ops::Deref for RAM5_R {
    type Target = crate::FieldReader<u8, RAM5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM5` writer - Configuration field for RAM block 5."]
pub struct RAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM5_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM5_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM5_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM5_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM5_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM5_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM5_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM5_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Configuration field for RAM block 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM6_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM6_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM6` reader - Configuration field for RAM block 6."]
pub struct RAM6_R(crate::FieldReader<u8, RAM6_A>);
impl RAM6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM6_A> {
        match self.bits {
            0 => Some(RAM6_A::PRI0),
            2 => Some(RAM6_A::PRI2),
            4 => Some(RAM6_A::PRI4),
            6 => Some(RAM6_A::PRI6),
            8 => Some(RAM6_A::PRI8),
            10 => Some(RAM6_A::PRI10),
            12 => Some(RAM6_A::PRI12),
            14 => Some(RAM6_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM6_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM6_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM6_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM6_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM6_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM6_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM6_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM6_A::PRI14
    }
}
impl core::ops::Deref for RAM6_R {
    type Target = crate::FieldReader<u8, RAM6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM6` writer - Configuration field for RAM block 6."]
pub struct RAM6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM6_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM6_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM6_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM6_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM6_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM6_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM6_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM6_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Configuration field for RAM block 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM7_A {
    #[doc = "0: Priority 0."]
    PRI0 = 0,
    #[doc = "2: Priority 2."]
    PRI2 = 2,
    #[doc = "4: Priority 4."]
    PRI4 = 4,
    #[doc = "6: Priority 6."]
    PRI6 = 6,
    #[doc = "8: Priority 8."]
    PRI8 = 8,
    #[doc = "10: Priority 10."]
    PRI10 = 10,
    #[doc = "12: Priority 12."]
    PRI12 = 12,
    #[doc = "14: Priority 14."]
    PRI14 = 14,
}
impl From<RAM7_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM7` reader - Configuration field for RAM block 7."]
pub struct RAM7_R(crate::FieldReader<u8, RAM7_A>);
impl RAM7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM7_A> {
        match self.bits {
            0 => Some(RAM7_A::PRI0),
            2 => Some(RAM7_A::PRI2),
            4 => Some(RAM7_A::PRI4),
            6 => Some(RAM7_A::PRI6),
            8 => Some(RAM7_A::PRI8),
            10 => Some(RAM7_A::PRI10),
            12 => Some(RAM7_A::PRI12),
            14 => Some(RAM7_A::PRI14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        **self == RAM7_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        **self == RAM7_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        **self == RAM7_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        **self == RAM7_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        **self == RAM7_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        **self == RAM7_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        **self == RAM7_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        **self == RAM7_A::PRI14
    }
}
impl core::ops::Deref for RAM7_R {
    type Target = crate::FieldReader<u8, RAM7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM7` writer - Configuration field for RAM block 7."]
pub struct RAM7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM7_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM7_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM7_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM7_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM7_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM7_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM7_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM7_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Configuration field for RAM block 0."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configuration field for RAM block 1."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configuration field for RAM block 2."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configuration field for RAM block 3."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Configuration field for RAM block 4."]
    #[inline(always)]
    pub fn ram4(&self) -> RAM4_R {
        RAM4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Configuration field for RAM block 5."]
    #[inline(always)]
    pub fn ram5(&self) -> RAM5_R {
        RAM5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Configuration field for RAM block 6."]
    #[inline(always)]
    pub fn ram6(&self) -> RAM6_R {
        RAM6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Configuration field for RAM block 7."]
    #[inline(always)]
    pub fn ram7(&self) -> RAM7_R {
        RAM7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configuration field for RAM block 0."]
    #[inline(always)]
    pub fn ram0(&mut self) -> RAM0_W {
        RAM0_W { w: self }
    }
    #[doc = "Bits 4:7 - Configuration field for RAM block 1."]
    #[inline(always)]
    pub fn ram1(&mut self) -> RAM1_W {
        RAM1_W { w: self }
    }
    #[doc = "Bits 8:11 - Configuration field for RAM block 2."]
    #[inline(always)]
    pub fn ram2(&mut self) -> RAM2_W {
        RAM2_W { w: self }
    }
    #[doc = "Bits 12:15 - Configuration field for RAM block 3."]
    #[inline(always)]
    pub fn ram3(&mut self) -> RAM3_W {
        RAM3_W { w: self }
    }
    #[doc = "Bits 16:19 - Configuration field for RAM block 4."]
    #[inline(always)]
    pub fn ram4(&mut self) -> RAM4_W {
        RAM4_W { w: self }
    }
    #[doc = "Bits 20:23 - Configuration field for RAM block 5."]
    #[inline(always)]
    pub fn ram5(&mut self) -> RAM5_W {
        RAM5_W { w: self }
    }
    #[doc = "Bits 24:27 - Configuration field for RAM block 6."]
    #[inline(always)]
    pub fn ram6(&mut self) -> RAM6_W {
        RAM6_W { w: self }
    }
    #[doc = "Bits 28:31 - Configuration field for RAM block 7."]
    #[inline(always)]
    pub fn ram7(&mut self) -> RAM7_W {
        RAM7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurable priority configuration register for ECB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecb](index.html) module"]
pub struct ECB_SPEC;
impl crate::RegisterSpec for ECB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecb::R](R) reader structure"]
impl crate::Readable for ECB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecb::W](W) writer structure"]
impl crate::Writable for ECB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECB to value 0"]
impl crate::Resettable for ECB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
