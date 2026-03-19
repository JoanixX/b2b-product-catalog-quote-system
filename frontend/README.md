# Frontend LabMedical

Frontend para la plataforma de catalogo medico B2B LabMedical, construido con Astro y TypeScript.

## Stack Tecnologico

- **Framework**: Astro 4.x (Static Site Generation)
- **Lenguaje**: TypeScript
- **Estado**: Nano Stores + @nanostores/persistent (carrito de cotizacion)
- **Estilos**: CSS vanilla con variables CSS
- **Tipografia**: Inter (Google Fonts)
- **Hosting**: Vercel (free tier)

## Caracteristicas

- Catalogo de productos con busqueda y filtrado por categoria
- Carrito de cotizacion persistente con cantidad por producto (sobrevive recarga y navegacion)
- Detalle de producto con tablas de especificaciones y descarga de ficha tecnica
- Formulario de cotizacion con validacion de RUC peruano (algoritmo Modulo 11)
- Centro de documentacion para procesos de licitacion
- Paginas legales detalladas (terminos y condiciones B2B, politica de privacidad Ley 29733)
- Pagina 404 personalizada
- SEO dinamico con meta tags y Open Graph por producto
- Accesibilidad WCAG 2.1 (skip link, focus-visible, ARIA roles)
- Navegacion responsive con menu hamburguesa

## Arquitectura

```
frontend/src/
├── layouts/
│   └── Layout.astro         # Layout base con SEO, OG, Inter font
├── components/
│   ├── Navbar.astro          # Nav sticky con badge de carrito
│   └── Footer.astro          # Footer con links legales
├── lib/
│   ├── api.ts                # Cliente API tipado
│   ├── validation.ts         # Validacion RUC, telefono, email
│   └── stores/
│       └── quoteCart.ts      # Store persistente del carrito
├── pages/
│   ├── index.astro           # Landing B2B
│   ├── productos.astro       # Catalogo con filtros
│   ├── productos/[slug].astro # Detalle con tablas y CTAs
│   ├── cotizacion.astro      # Formulario corporativo
│   ├── documentacion.astro   # Centro documental
│   ├── terminos.astro        # Terminos y condiciones
│   ├── privacidad.astro      # Politica de privacidad
│   └── 404.astro             # Error 404
```

## Comenzando

### Prerequisitos

- Node.js 18+
- Backend corriendo en `http://localhost:3000` (necesario para build)

### Instalacion

```bash
npm install
cp .env.example .env
npm run dev
```

El frontend estara disponible en `http://localhost:4321`

### Build

```bash
npm run build
```

> El backend debe estar corriendo para que `getStaticPaths` genere las paginas de productos.

## Paleta de Colores

| Variable               | Valor   | Uso                          |
| ---------------------- | ------- | ---------------------------- |
| `--azul-institucional` | #1e40af | Acciones principales, navbar |
| `--azul-hover`         | #1e3a8a | Hover de botones             |
| `--gris-pizarra`       | #475569 | Textos secundarios           |
| `--gris-texto`         | #1f2937 | Textos principales           |
| `--gris-fondo`         | #f8fafc | Fondo de pagina              |

## Variables de Entorno

- `PUBLIC_API_URL`: URL del backend API (default: `http://localhost:3000`)
