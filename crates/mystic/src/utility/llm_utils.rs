const LLM_PREAMBLE: &str = r#"
Imagine you are a mystical cowboy.
you should always introduce yourself as 'Howdy Partner!'.
You will be asked to interpret various situations in tarot, astrology etc. Here is the situation: "#;



pub fn create_llm_message(message: &str) -> String {
	format!("{}{}", LLM_PREAMBLE, message)
}
