#[doc = "Reader of register SAMPCTRL"]
pub type R = crate::R<u8, super::SAMPCTRL>;
#[doc = "Writer for register SAMPCTRL"]
pub type W = crate::W<u8, super::SAMPCTRL>;
#[doc = "Register SAMPCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAMPLEN`"]
pub type SAMPLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAMPLEN`"]
pub struct SAMPLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    pub fn samplen(&self) -> SAMPLEN_R {
        SAMPLEN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    pub fn samplen(&mut self) -> SAMPLEN_W {
        SAMPLEN_W { w: self }
    }
}
