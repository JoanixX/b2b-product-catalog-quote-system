# API LabMedical - Backend

API Backend para la plataforma de catalogo medico B2B LabMedical, construida con Rust y Axum.

## Stack Tecnologico

- **Framework**: Axum 0.7
- **Base de Datos**: PostgreSQL con SQLx (Neon.tech)
- **Autenticacion**: JWT con Argon2id
- **Almacenamiento de Archivos**: Cloudflare R2 / AWS S3
- **Email**: API Resend (plantillas HTML)
- **Validacion**: crate `validator` + RUC peruano (Modulo 11)
- **Sanitizacion**: crate `ammonia` (prevencion XSS)
- **Deployment**: Render.com

## Caracteristicas

- API RESTful con endpoints publicos y administrativos
- Autenticacion JWT con Argon2id (resistente a GPU y side-channel attacks)
- Catalogo de productos con campos regulatorios (registro sanitario, ficha tecnica, marca, garantia)
- Sistema de cotizaciones con validacion de RUC peruano (algoritmo Modulo 11)
- Sanitizacion XSS automatica en todos los inputs de texto
- Carga de archivos con validacion MIME estricta (solo JPEG, WebP, PDF)
- Nombres UUID generados en servidor para archivos subidos
- Sistema de errores opacos con codigos estandarizados
- Notificaciones por email con plantillas HTML profesionales
- CORS estricto sin AllowAll
- Logging estructurado con tracing

## Arquitectura

```
backend/src/
├── main.rs              # Punto de entrada, servidor y router
├── config.rs            # Carga de variables de entorno
├── db.rs                # Pool de conexiones a PostgreSQL
├── error.rs             # Errores opacos con codigos estandarizados
├── models/              # Structs de datos y DTOs
│   ├── product.rs       # Producto con campos regulatorios
│   ├── category.rs      # Categorias
│   ├── quote.rs         # Cotizaciones con RUC obligatorio
│   └── admin.rs         # Administradores
├── routes/              # Handlers de endpoints
│   ├── public.rs        # Endpoints publicos (catalogo, cotizaciones)
│   └── admin.rs         # Endpoints de administracion (CRUD)
├── services/            # Logica de negocio
│   ├── auth.rs          # Argon2id + JWT (expiracion 2h)
│   ├── email.rs         # Notificaciones HTML via Resend
│   ├── s3.rs            # Archivos a S3 con validacion MIME
│   └── validation.rs    # RUC peruano (Modulo 11) + sanitizacion XSS
└── middleware/           # Middleware de autenticacion
    └── auth.rs          # Verificacion de JWT
```

## Comenzando

### Prerequisitos

- Rust 1.70+
- Cuenta en Neon.tech (PostgreSQL gratuito en la nube)
- Cuenta en Cloudflare R2 o AWS S3 (almacenamiento)
- Cuenta en Resend (emails gratuitos, 3000/mes)

### Instalacion

1. Clonar el repositorio

2. Copiar `.env.example` a `.env` y configurar:

   ```bash
   cp .env.example .env
   ```

3. Obtener connection string de Neon.tech y pegarlo en `DATABASE_URL`

4. Iniciar el servidor (las migraciones se ejecutan automaticamente):

   ```bash
   cargo run
   ```

La API estara disponible en `http://localhost:3000`

> No es necesario instalar PostgreSQL localmente.

### Generar secretos para produccion

```bash
python scripts/generate_secrets.py
```

Esto genera claves seguras con CSPRNG para JWT, base de datos y admin.

## Endpoints de API

### Endpoints Publicos

- `GET /health` - Verificacion de salud
- `GET /api/products` - Listar productos (paginacion, busqueda, filtros)
- `GET /api/products/:slug` - Obtener producto por slug
- `GET /api/categories` - Listar todas las categorias
- `POST /api/quotes` - Enviar solicitud de cotizacion (requiere RUC peruano valido)

### Endpoints Administrativos (requieren JWT)

- `POST /api/admin/login` - Login de administrador
- `GET /api/admin/products` - Listar todos los productos (vista admin)
- `POST /api/admin/products` - Crear producto
- `PUT /api/admin/products/:id` - Actualizar producto
- `DELETE /api/admin/products/:id` - Eliminar producto
- `PATCH /api/admin/products/:id/toggle` - Alternar estado activo del producto
- `GET /api/admin/categories` - Listar categorias
- `POST /api/admin/categories` - Crear categoria
- `PUT /api/admin/categories/:id` - Actualizar categoria
- `DELETE /api/admin/categories/:id` - Eliminar categoria
- `GET /api/admin/quotes` - Listar cotizaciones
- `GET /api/admin/quotes/:id` - Obtener detalles de cotizacion
- `PATCH /api/admin/quotes/:id/status` - Actualizar estado de cotizacion
- `POST /api/admin/upload` - Subir archivo (JPEG, WebP o PDF, max 10MB)

## Codigos de Error

El API devuelve errores opacos con codigos estandarizados:

| Codigo                | Descripcion                                     |
| --------------------- | ----------------------------------------------- |
| `ERR_INTERNAL_SERVER` | Error interno (detalles logueados internamente) |
| `ERR_UNAUTHORIZED`    | Credenciales invalidas o token expirado         |
| `ERR_VALIDATION`      | Error de validacion en los datos enviados       |
| `ERR_NOT_FOUND`       | Recurso no encontrado                           |
| `ERR_BAD_REQUEST`     | Solicitud malformada                            |
| `ERR_RATE_LIMIT`      | Demasiadas solicitudes                          |
| `ERR_INVALID_RUC`     | RUC peruano invalido                            |

## Credenciales de Administrador por Defecto

- Email: `admin@labmedical.com`
- Contraseña: `admin123`

**⚠️ Cambiar estas credenciales inmediatamente en produccion**

## Variables de Entorno

Ver `.env.example` para todas las variables de entorno requeridas.

Archivos de entorno disponibles:

- `.env` - Desarrollo local (no se sube al repo)
- `.env.production` - Produccion (no se sube al repo)
- `.env.example` - Plantilla de referencia (se sube al repo)

## Deployment

Configurado para deployment en Render.com. La configuracion se realiza desde el dashboard de Render (variables de entorno manuales, sin render.yaml).

## Licencia

Propietario - LabMedical
