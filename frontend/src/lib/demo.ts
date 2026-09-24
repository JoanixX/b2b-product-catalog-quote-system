// Único lugar para adaptar marca y contenido de esta demostración.
export const brand = {
  name: 'Trazo',
  descriptor: 'Cotizaciones con un proceso claro',
  logoUrl: '', // Ruta pública opcional, por ejemplo /logo-agencia.svg
  primary: '#284c85',
  primaryDark: '#193657',
  accent: '#087b72',
  salesContactUrl: 'mailto:alvaradocjosorio@gmail.com?subject=Demo%20de%20cotizaciones%20B2B',
};

export const demoContent = {
  company: 'Suministros Horizonte S.A.C. (ejemplo)',
  contact: 'Área de compras (ejemplo)',
  product: 'Estación de empaque modular (ejemplo)',
  productCode: 'EM-240 (ejemplo)',
  quantity: 12,
  unitPrice: 1450,
  currency: 'PEN',
  request: 'Se requiere entrega escalonada y una propuesta con instalación incluida. (Ejemplo)',
  terms: 'Entrega y condiciones por confirmar con el cliente. (Ejemplo)',
  taxNote: 'Importes referenciales de ejemplo; impuestos y flete no incluidos.',
};

export type Stage = 'recibida' | 'borrador' | 'aprobada' | 'propuesta' | 'seguimiento';
export type DemoQuote = {
  id: string;
  company: string;
  contact: string;
  product: string;
  quantity: number;
  unitPrice: number;
  request: string;
  terms: string;
  stage: Stage;
  createdAt: string;
  updatedAt: string;
};

export const stageLabels: Record<Stage, string> = {
  recibida: 'Solicitud recibida',
  borrador: 'Borrador en revisión',
  aprobada: 'Aprobada',
  propuesta: 'Propuesta generada',
  seguimiento: 'En seguimiento',
};

const STORAGE_KEY = 'b2b-commercial-demo-quotes-v1';

export function sampleQuote(): DemoQuote {
  const now = new Date().toISOString();
  return {
    id: 'EJ-001',
    company: demoContent.company,
    contact: demoContent.contact,
    product: demoContent.product,
    quantity: demoContent.quantity,
    unitPrice: demoContent.unitPrice,
    request: demoContent.request,
    terms: demoContent.terms,
    stage: 'recibida',
    createdAt: now,
    updatedAt: now,
  };
}

export function readQuotes(): DemoQuote[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    const parsed: unknown = JSON.parse(raw);
    if (!Array.isArray(parsed) || !parsed.every((item): item is DemoQuote =>
      !!item && typeof item === 'object' &&
      typeof item.id === 'string' && typeof item.company === 'string' &&
      typeof item.contact === 'string' && typeof item.product === 'string' &&
      typeof item.request === 'string' && typeof item.terms === 'string' &&
      typeof item.createdAt === 'string' && typeof item.updatedAt === 'string' &&
      typeof item.quantity === 'number' && Number.isSafeInteger(item.quantity) && item.quantity > 0 &&
      typeof item.unitPrice === 'number' && Number.isFinite(item.unitPrice) && item.unitPrice >= 0 &&
      typeof item.stage === 'string' && item.stage in stageLabels
    )) throw new Error('Formato inválido');
    return parsed;
  } catch {
    throw new Error('No pudimos leer los datos guardados de esta demo. Restablécela para continuar.');
  }
}

export function saveQuotes(quotes: DemoQuote[]): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(quotes));
  } catch {
    throw new Error('No se pudieron guardar los cambios en este navegador. Revisa el espacio o los permisos de almacenamiento.');
  }
}

export function resetQuotes(): DemoQuote[] {
  const quotes = [sampleQuote()];
  saveQuotes(quotes);
  return quotes;
}

export function formatMoney(amount: number): string {
  return new Intl.NumberFormat('es-PE', { style: 'currency', currency: demoContent.currency, maximumFractionDigits: 0 }).format(amount);
}

export function formatDate(value: string): string {
  return new Intl.DateTimeFormat('es-PE', { day: 'numeric', month: 'short', year: 'numeric' }).format(new Date(value));
}

export function escapeHtml(value: string): string {
  return value.replace(/[&<>"']/g, (char) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[char] || char);
}
