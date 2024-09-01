#[derive(Debug)]
pub struct Castle {
    pub name: String,
    pub creatures: Vec<String>,
}

pub fn parse_structure(content: &str) -> Vec<Castle> {
    let mut castles: Vec<Castle> = Default::default();
    let mut current = 0;
    for x in content.lines() {
        if x.is_empty() {
            current += 1;
            continue;
        }
        if castles.len() > current {
            castles.get_mut(current).as_mut().unwrap().creatures.push(x.to_string());
        } else {
            castles.push(Castle { name: x.to_string(), creatures: vec![] });
        }
    }
    castles
}

