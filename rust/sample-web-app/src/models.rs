use serde::{ Serialize, Deserialize, de };

#[derive(Serialize)]
pub struct Message {
    pub message: String
}

#[derive( Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: Name
}

// #[derive(Deserialize)] もできるが、Name::newには制約をつけているため、どんな文字列でもNameに変換できてしまうのを防ぐ
#[derive(Clone, Debug, Serialize)]
pub struct Name(String);

impl<'de>de::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Name::new(&s).map_err(de::Error::custom)
        }
}

impl Name {
    pub fn new(name: &str) -> Result<Self, String> {
        let size = name.chars().count();
        if size < 1 || size > 10 {
            return Err("名前は10文字以内で".to_string());
        }

        if name.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err("Use only A-Z, a-z".to_string());
        }

        Ok(Name(name.to_string()))
    }
}

impl std::str::FromStr for Name {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::new(s)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_name() {
    let ok_value = "Nrskt";
    assert!(Name::new(ok_value).is_ok());

    let ok_value = "N";
    assert!(Name::new(ok_value).is_ok());

    let ng_value = "0";
    assert!(Name::new(ng_value).is_err());
}