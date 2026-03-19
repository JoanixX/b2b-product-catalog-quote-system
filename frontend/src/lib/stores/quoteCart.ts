// Store persistente del carrito de cotizacion
// los productos seleccionados se guardan en localStorage
// y se sincronizan entre paginas via nanostores

import { persistentAtom } from "@nanostores/persistent";

export interface QuoteCartItem {
  id: number;
  name: string;
  brand: string;
  slug: string;
  quantity: number;
}

// el store persistente sobrevive a recargas y navegacion
export const quoteCart = persistentAtom<QuoteCartItem[]>(
  "labmedical_quote_cart",
  [],
  {
    encode: JSON.stringify,
    decode: JSON.parse,
  },
);

// se agrega producto al carrito (evita duplicados, default quantity = 1)
export function addToCart(
  item: Omit<QuoteCartItem, "quantity">,
  quantity: number = 1,
): void {
  const current = quoteCart.get();
  const existing = current.find((p) => p.id === item.id);
  if (existing) {
    // si ya existe, sumar cantidad
    quoteCart.set(
      current.map((p) =>
        p.id === item.id ? { ...p, quantity: p.quantity + quantity } : p,
      ),
    );
  } else {
    quoteCart.set([...current, { ...item, quantity }]);
  }
}

// actualizar cantidad de un producto
export function updateQuantity(id: number, quantity: number): void {
  if (quantity < 1) return;
  const current = quoteCart.get();
  quoteCart.set(current.map((p) => (p.id === id ? { ...p, quantity } : p)));
}

// se quita producto del carrito
export function removeFromCart(id: number): void {
  const current = quoteCart.get();
  quoteCart.set(current.filter((p) => p.id !== id));
}

// se limpia el carrito completo
export function clearCart(): void {
  quoteCart.set([]);
}

// se obtiene la cantidad de productos en el carrito
export function getCartCount(): number {
  return quoteCart.get().length;
}

// se obtienen los IDs para enviar al backend
export function getCartProductIds(): number[] {
  return quoteCart.get().map((p) => p.id);
}