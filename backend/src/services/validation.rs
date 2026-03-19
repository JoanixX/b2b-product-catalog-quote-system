use regex::Regex;
use std::sync::LazyLock;

// Regex para validar formato de ruc peruano (11 digitos)
static RUC_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\d{11}$").unwrap()
});

// valida un ruc peruano usando el algoritmo modulo 11
// prefijos validos: 10 (persona natural), 15 (entidad publica),
// 17 (entidad con fin social), 20 (persona juridica)
pub fn validate_ruc(ruc: &str) -> bool {
    if !RUC_REGEX.is_match(ruc) {
        return false;
    }

    let digits: Vec<u32> = ruc.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 11 {
        return false;
    }

    // verificar prefijo valido
    let prefix = digits[0] * 10 + digits[1];
    if !matches!(prefix, 10 | 15 | 17 | 20) {
        return false;
    }

    // algoritmo modulo 11 para verificar digito de control
    let factors = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits.iter()
        .zip(factors.iter())
        .map(|(d, f)| d * f)
        .sum();

    let remainder = 11 - (sum % 11);
    let check_digit = match remainder {
        10 => 0,
        11 => 1,
        d => d,
    };

    check_digit == digits[10]
}

// Sanitiza texto para prevenir xss usando ammonia y elimina tags
// html peligrosos manteniendo el texto limpio
pub fn sanitize_text(input: &str) -> String {
    ammonia::clean(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruc_formato_invalido() {
        assert!(!validate_ruc("123"));
        assert!(!validate_ruc(""));
        assert!(!validate_ruc("abcdefghijk"));
        assert!(!validate_ruc("1234567890"));
        assert!(!validate_ruc("123456789012"));
    }

    #[test]
    fn test_ruc_prefijo_invalido() {
        // prefijo 30 por ejemplo no es valido
        assert!(!validate_ruc("30000000001"));
    }

    #[test]
    fn test_sanitize_xss() {
        let input = "<script>alert('xss')</script>Texto seguro";
        let result = sanitize_text(input);
        assert!(!result.contains("<script>"));
        assert!(result.contains("Texto seguro"));
    }
}