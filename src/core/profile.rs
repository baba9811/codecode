pub const DIFFICULTIES: &[&str] = &["auto", "easy", "medium", "hard"];

pub fn default_difficulty() -> String {
    "auto".to_string()
}

pub fn normalize_difficulty(difficulty: &str) -> String {
    let difficulty = difficulty.trim().to_lowercase();
    if DIFFICULTIES.contains(&difficulty.as_str()) {
        difficulty
    } else {
        default_difficulty()
    }
}

pub fn parse_topic_list(value: &str) -> Vec<String> {
    let mut topics = Vec::new();
    for topic in value.split(',') {
        let topic = topic.trim().trim_start_matches('#').trim().to_lowercase();
        if !topic.is_empty() && !topics.contains(&topic) {
            topics.push(topic);
        }
    }
    topics
}

pub fn normalize_topic_list(topics: &[String]) -> Vec<String> {
    parse_topic_list(&topics.join(","))
}
