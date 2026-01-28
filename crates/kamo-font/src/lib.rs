use std::collections::BTreeMap;
#[derive(Clone, Debug,PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            .enumerate()
            .map(|(i, &c)| (std::char::from_u32(i as u32).unwrap(), c))
            .collect::<BTreeMap<_, _>>();
        let rev = fwd
            .iter()
            .map(|(&k, &v)| (v, k))
            .collect::<BTreeMap<_, _>>();
        Self { fwd, rev }
    }
}
