use std::ffi::OsStr;
use std::str::FromStr;

#[derive(Debug)]
pub enum CatChoice {
    Felix,
    Whiskers,
    Mittens,
}

impl From<&OsStr> for CatChoice {
    fn from(os_str: &OsStr) -> Self {
        match os_str.to_str() {
            Some("felix") => CatChoice::Felix,
            Some("whiskers") => CatChoice::Whiskers,
            Some("mittens") => CatChoice::Mittens,
            _ => panic!("Invalid cat choice"),
        }
    }
}

impl FromStr for CatChoice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "felix" => Ok(CatChoice::Felix),
            "whiskers" => Ok(CatChoice::Whiskers),
            "mittens" => Ok(CatChoice::Mittens),
            _ => Err(()),
        }
    }
}
