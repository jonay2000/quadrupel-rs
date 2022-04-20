#[doc = "Register `SCK` reader"]
pub struct R(crate::R<SCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCK` writer"]
pub struct W(crate::W<SCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCK_SPEC>;
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
impl From<crate::W<SCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCK_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for SCK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sck](index.html) module"]
pub struct SCK_SPEC;
impl crate::RegisterSpec for SCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sck::R](R) reader structure"]
impl crate::Readable for SCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sck::W](W) writer structure"]
impl crate::Writable for SCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCK to value 0xffff_ffff"]
impl crate::Resettable for SCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
