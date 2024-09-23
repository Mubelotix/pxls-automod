use std::collections::HashMap;
use std::sync::LazyLock;

pub const FACTIONS: &[(&str, usize)] = &[
    ("MECA", 0xffea00),
    ("MRIE", 0x59ff21),
    ("CFI", 0xff2200),
    ("EP", 0xff8ac0),
    ("GM", 0x0600bd),
    ("ITI", 0x00fffb),
    ("LH", 0xff7700)
];

pub static USERS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("rick_astley", "ITI"),
    ])
});
