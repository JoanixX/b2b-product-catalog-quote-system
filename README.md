# LabMedical - Plataforma de Catalogo Medico B2B

Plataforma profesional B2B de catalogo para LabMedical, proveedor de productos medicos. Esta plataforma sirve como catalogo digital con sistema de cotizaciones para clientes empresariales.

## Estructura del Proyecto

```
labmedical_web/
├── backend/          # API REST en Rust/Axum
│   ├── src/          # Codigo fuente del servidor
│   ├── migrations/   # Migraciones de base de datos
│   └── .env.example  # Variables de entorno
├── frontend/         # Aplicacion web Astro/TypeScript
│   ├── src/          # Codigo fuente del frontend
│   │   ├── components/  # Componentes reutilizables
│   │   ├── layouts/     # Layouts base
│   │   ├── pages/       # Paginas de la aplicacion
│   │   └── lib/         # Cliente API y utilidades
│   └── .env.example  # Variables de entorno
├── scripts/          # Utilidades (no se suben al repo)
├── docs/             # Documentacion del proyecto
└── README.md
```

## Stack Tecnologico

### Backend

- **Lenguaje**: Rust
- **Framework**: Axum 0.7
- **Base de Datos**: PostgreSQL (Neon.tech - free tier)
- **Autenticacion**: JWT + Argon2id
- **Almacenamiento**: Cloudflare R2 (compatible con S3)
- **Email**: API Resend
- **Hosting**: Render.com

### Frontend

- **Framework**: Astro 4.x (Static Site Generation)
- **Lenguaje**: TypeScript
- **Estado**: Nano Stores (carrito de cotizacion persistente)
- **Estilos**: CSS vanilla con variables CSS
- **Tipografia**: Inter (Google Fonts)
- **Hosting**: Vercel

## Funcionalidades

### Funcionalidades Publicas

- Pagina de inicio con informacion de la empresa
- Catalogo de productos con busqueda y filtros por categoria
- Paginas de detalle de productos con especificaciones tecnicas
- Ficha tecnica y registro sanitario por producto
- Navegacion por categorias
- Formulario de solicitud de cotizaciones con validacion de RUC peruano

### Funcionalidades Administrativas

- Autenticacion segura con Argon2id
- Gestion de productos (CRUD) con campos regulatorios
- Gestion de categorias
- Bandeja de entrada y gestion de cotizaciones
- Carga de archivos (imagenes JPEG/WebP y PDFs) a S3
- Panel de control con metricas

## Modelo de Negocio

Esta es una **plataforma B2B**, no un sitio de comercio electronico:

- Sin pagos directos ni carrito de compras
- Los clientes navegan el catalogo y solicitan cotizaciones
- El equipo de ventas maneja las negociaciones offline
- Enfoque en presentacion profesional y generacion de leads

## Comenzando

### Prerequisitos

- Rust 1.70+ (para backend)
- Node.js 18+ (para frontend)
- Cuenta en Neon.tech (PostgreSQL gratuito en la nube)
- Cuenta en Cloudflare R2 o AWS S3 (almacenamiento)
- Cuenta en Resend (emails)

### Configuracion del Backend

Ver [backend/README.md](./backend/README.md) para instrucciones detalladas.

Inicio rapido:

```bash
cd backend
cp .env.example .env
# Editar .env con tu connection string de Neon.tech
cargo run
# Las migraciones se ejecutan automaticamente
```

El backend estara disponible en `http://localhost:3000`

### Configuracion del Frontend

Ver [frontend/README.md](./frontend/README.md) para instrucciones detalladas.

Inicio rapido:

```bash
cd frontend
cp .env.example .env
npm install
npm run dev
```

El frontend estara disponible en `http://localhost:4321`

### Ejecucion Local Completa

Para ejecutar el proyecto completo localmente:

1. **Terminal 1 - Backend**:

```bash
cd backend
cargo run
```

2. **Terminal 2 - Frontend**:

```bash
cd frontend
npm run dev
```

3. Abrir navegador en `http://localhost:4321`

> No es necesario instalar PostgreSQL localmente, se usa Neon.tech en la nube.

## Documentacion

- [Documentacion de API](./docs/API.md)
- [README Backend](./backend/README.md)
- [README Frontend](./frontend/README.md)

## Acceso de Administrador por Defecto

- Email: `admin@labmedical.com`
- Contraseña: `admin123`

**⚠️ Cambiar inmediatamente en produccion usando `scripts/generate_secrets.py`**

## Deployment

### Backend (Render.com)

- Tier gratuito permanente
- Deployment automatico desde GitHub
- Configuracion desde el dashboard de Render (variables de entorno manuales)
- Root directory: `backend`

### Frontend (Vercel)

- Tier gratuito permanente con CDN global
- Deployment automatico desde GitHub
- Root directory: `frontend`

## Seguridad

- **Autenticacion**: JWT con expiracion de 2 horas
- **Hashing**: Argon2id (resistente a GPU y side-channel attacks)
- **Validacion**: RUC peruano (algoritmo Modulo 11), sanitizacion XSS con ammonia
- **CORS**: Metodos y headers explicitos, sin AllowAll
- **Archivos**: Solo JPEG, WebP y PDF permitidos, nombres UUID generados en servidor
- **Errores opacos**: Nunca se exponen detalles de base de datos al cliente
- **Codigos de error**: Estandarizados (ERR_INVALID_RUC, ERR_UNAUTHORIZED, etc.)

## Esquema de Base de Datos

- **products**: Catalogo con marca, modelo, registro sanitario, ficha tecnica, garantia
- **categories**: Categorizacion de productos
- **quotes**: Solicitudes de cotizacion con RUC obligatorio
- **admins**: Usuarios administradores con contraseñas Argon2id

## Endpoints de API

### Publicos

- `GET /api/products` - Listar productos
- `GET /api/products/:slug` - Detalles de producto
- `GET /api/categories` - Listar categorias
- `POST /api/quotes` - Enviar solicitud de cotizacion (requiere RUC valido)

### Administrador (requiere JWT)

- `POST /api/admin/login` - Autenticacion
- CRUD completo para productos, categorias y cotizaciones
- `POST /api/admin/upload` - Carga de archivos (JPEG, WebP, PDF)

## Licencia

Propietario - LabMedical

## Contacto

Para preguntas o soporte, contactar: alvaradocjosorio@gmail.com

---

Construido para LabMedical
