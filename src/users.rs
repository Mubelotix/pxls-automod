use std::collections::HashMap;
use std::sync::LazyLock;

pub static USERS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("simon_girard", "ITI"),
        ("iris_dussuyer", "ITI"),
        ("dimitri_timoz", "ITI"),
        ("alix_anneraud", "ITI"),
        ("quentin_beral", "ITI"),
        ("lisa_levasseur", "ITI"),
        ("baptiste_vadebout", "EP"),
        ("leni_rallet", "EP"),
        ("lola_poupinel", "MRIE"),
        ("melanie_rochon", "MRIE"),
    ])
});
