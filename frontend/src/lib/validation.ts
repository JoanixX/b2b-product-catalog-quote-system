// validaciones compartidas entre frontend y logica de formulario

// Valida ruc peruano con algoritmo Modulo 11
// prefijos validos: 10 (persona natural), 15 (entidad publica),
// 17 (entidad con fin social), 20 (persona juridica)
export function validateRuc(ruc: string): { valid: boolean; error: string } {
  if (!/^\d{11}$/.test(ruc)) {
    return { valid: false, error: 'El RUC debe tener exactamente 11 digitos numericos' };
  }

  const digits = ruc.split('').map(Number);
  const prefix = digits[0] * 10 + digits[1];

  if (![10, 15, 17, 20].includes(prefix)) {
    return { valid: false, error: 'Prefijo de RUC invalido. Debe iniciar con 10, 15, 17 o 20' };
  }

  // algoritmo modulo 11
  const factors = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
  let sum = 0;
  for (let i = 0; i < 10; i++) {
    sum += digits[i] * factors[i];
  }

  const remainder = 11 - (sum % 11);
  let checkDigit: number;
  if (remainder === 10) checkDigit = 0;
  else if (remainder === 11) checkDigit = 1;
  else checkDigit = remainder;

  if (checkDigit !== digits[10]) {
    return { valid: false, error: 'El digito verificador del RUC no es valido' };
  }

  return { valid: true, error: '' };
}

// se valida el telefono (solo deben ser numeros y de entre 7-15 digitos)
export function validatePhone(phone: string): { valid: boolean; error: string } {
  const cleaned = phone.replace(/[\s\-\+\(\)]/g, '');
  if (!/^\d{7,15}$/.test(cleaned)) {
    return { valid: false, error: 'El telefono debe contener entre 7 y 15 digitos' };
  }
  return { valid: true, error: '' };
}

// se valida el email
export function validateEmail(email: string): { valid: boolean; error: string } {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(email)) {
    return { valid: false, error: 'Ingrese un email valido' };
  }
  return { valid: true, error: '' };
}

// Se valida un campo no vacio con longitud minima
export function validateRequired(value: string, fieldName: string, minLength = 2): { valid: boolean; error: string } {
  const trimmed = value.trim();
  if (!trimmed) {
    return { valid: false, error: `${fieldName} es obligatorio` };
  }
  if (trimmed.length < minLength) {
    return { valid: false, error: `${fieldName} debe tener al menos ${minLength} caracteres` };
  }
  return { valid: true, error: '' };
}