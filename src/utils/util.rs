
pub fn texto_es_valido(input: &str) -> bool {
    !input.trim().is_empty()
}
pub fn limpiar_nombre(input: &str) -> String {
    let limpio = input.trim();
    if limpio.is_empty() {
        return String::new();
    }
    
    let mut chars = limpio.chars();
    match chars.next() {
        None => String::new(),
        Some(primera) => primera.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
    }
}