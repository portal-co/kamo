use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontObf {
    pub fwd: BTreeMap<char, char>,
    pub rev: BTreeMap<char, char>,
}
impl FontObf {
    pub fn new(mut x: impl FnMut(&mut [char])) -> Self {
        let mut chars = (0..=0x10FFFF)
            .filter_map(std::char::from_u32)
            .collect::<Vec<_>>();
        x(&mut chars);
        let fwd = chars
            .iter()
            .zip((0..=0x10FFFF).filter_map(std::char::from_u32))
            .map(|(&c, i)| (i, c))
            .collect::<BTreeMap<_, _>>();
        let rev = fwd
            .iter()
            .map(|(&k, &v)| (v, k))
            .collect::<BTreeMap<_, _>>();
        Self { fwd, rev }
    }
    pub fn js_obf(&self) -> JsFontObf<'_> {
        JsFontObf { font_obf: self }
    }
}
struct JsObfMap<'a> {
    map: &'a BTreeMap<char, char>,
}
impl Display for JsObfMap<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object.freeze({{__proto__:null,")?;
        for (k, v) in self.map.iter() {
            write!(f, "'{}':'{}'", k.escape_default(), v.escape_default())?;
        }
        write!(f, "}})")?;
        Ok(())
    }
}
pub struct JsFontObf<'a> {
    pub font_obf: &'a FontObf,
}
impl Display for JsFontObf<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Object.freeze({{__proto__:null,fwd:{},rev:{}}})",
            JsObfMap {
                map: &self.font_obf.fwd
            },
            JsObfMap {
                map: &self.font_obf.rev
            }
        )?;
        Ok(())
    }
}
