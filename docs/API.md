# Documentacion de API - LabMedical

## URL Base

- **Desarrollo**: `http://localhost:3000`
- **Produccion**: `https://labmedical-api.onrender.com`

## Autenticacion

Los endpoints administrativos requieren autenticacion JWT (expiracion: 2 horas). Incluir el token en el header:

```
Authorization: Bearer <tu_token_jwt>
```

## Formato de Errores

Todos los errores siguen un formato estandarizado con codigos opacos:

```json
{
  "code": "ERR_VALIDATION",
  "message": "Error de validacion: ..."
}
```

| Codigo                | HTTP | Descripcion                                     |
| --------------------- | ---- | ----------------------------------------------- |
| `ERR_INTERNAL_SERVER` | 500  | Error interno (detalles logueados internamente) |
| `ERR_UNAUTHORIZED`    | 401  | Credenciales invalidas o token expirado         |
| `ERR_VALIDATION`      | 400  | Error de validacion en los datos                |
| `ERR_NOT_FOUND`       | 404  | Recurso no encontrado                           |
| `ERR_BAD_REQUEST`     | 400  | Solicitud malformada                            |
| `ERR_RATE_LIMIT`      | 429  | Demasiadas solicitudes                          |
| `ERR_INVALID_RUC`     | 400  | RUC peruano invalido (algoritmo Modulo 11)      |

---

## Endpoints Publicos

### Verificacion de Salud

```http
GET /health
```

**Respuesta:**

```
OK
```

---

### Listar Productos

```http
GET /api/products
```

**Parametros de Consulta:**

- `category` (opcional): Filtrar por slug de categoria
- `search` (opcional): Buscar en nombre y descripcion
- `page` (opcional, por defecto: 1): Numero de pagina
- `limit` (opcional, por defecto: 20, max: 100): Elementos por pagina

**Respuesta:**

```json
{
  "products": [
    {
      "id": 1,
      "name": "Monitor de Signos Vitales",
      "slug": "monitor-signos-vitales",
      "description": "Monitor multiparametro profesional",
      "category_id": 1,
      "brand": "Mindray",
      "model_number": "BeneVision N1",
      "origin_country": "China",
      "warranty_period": 24,
      "technical_sheet_url": "https://...",
      "registro_sanitario": "DM-12345",
      "specifications": {},
      "image_url": "https://...",
      "additional_images": [],
      "regulatory_info": {},
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "total": 100,
  "page": 1,
  "limit": 20
}
```

---

### Obtener Producto por Slug

```http
GET /api/products/:slug
```

**Respuesta:** Mismo formato que un producto individual del listado.

---

### Listar Categorias

```http
GET /api/categories
```

**Respuesta:**

```json
[
  {
    "id": 1,
    "name": "Equipos Medicos",
    "slug": "equipos-medicos",
    "description": "Equipos y dispositivos medicos profesionales",
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

---

### Enviar Solicitud de Cotizacion

```http
POST /api/quotes
```

**Cuerpo de la Solicitud:**

```json
{
  "company_name": "Clinica San Pablo",
  "company_tax_id": "20100047218",
  "contact_name": "Juan Perez",
  "email": "juan@clinicasanpablo.com",
  "phone": "+51987654321",
  "product_ids": [1, 2, 3],
  "estimated_quantity": "100 unidades",
  "message": "Necesitamos cotizacion urgente"
}
```

**Reglas de Validacion:**

- `company_name`: 2-255 caracteres (obligatorio)
- `company_tax_id`: RUC peruano de 11 digitos (obligatorio, validado con algoritmo Modulo 11)
  - Prefijos validos: 10 (persona natural), 15 (entidad publica), 17 (entidad con fin social), 20 (persona juridica)
- `contact_name`: 2-255 caracteres (obligatorio)
- `email`: Formato RFC 5322 (obligatorio)
- `phone`: Max 50 caracteres (opcional)
- `product_ids`: Al menos 1 producto (obligatorio)
- `estimated_quantity`: Max 1000 caracteres (opcional)
- `message`: Max 2000 caracteres (opcional)

> Todos los campos de texto se sanitizan automaticamente para prevenir XSS.

**Respuesta exitosa:**

```json
{
  "code": "OK",
  "message": "Solicitud de cotizacion enviada exitosamente"
}
```

**Error de RUC invalido:**

```json
{
  "code": "ERR_INVALID_RUC",
  "message": "El RUC proporcionado no es valido"
}
```

---

## Endpoints Administrativos

### Login

```http
POST /api/admin/login
```

**Cuerpo de la Solicitud:**

```json
{
  "email": "admin@labmedical.com",
  "password": "admin123"
}
```

**Respuesta:**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "admin": {
    "id": 1,
    "email": "admin@labmedical.com",
    "name": "Administrador"
  }
}
```

> El token JWT expira en 2 horas. Contraseñas hasheadas con Argon2id.

---

### Listar Productos (Admin)

```http
GET /api/admin/products
```

**Headers:**

```
Authorization: Bearer <token>
```

**Parametros de Consulta:**

- `page` (opcional, por defecto: 1)
- `limit` (opcional, por defecto: 50, max: 100)
- `active` (opcional): Filtrar por estado activo

---

### Crear Producto

```http
POST /api/admin/products
```

**Headers:**

```
Authorization: Bearer <token>
Content-Type: application/json
```

**Cuerpo de la Solicitud:**

```json
{
  "name": "Monitor de Signos Vitales",
  "slug": "monitor-signos-vitales",
  "description": "Monitor multiparametro profesional",
  "category_id": 1,
  "brand": "Mindray",
  "model_number": "BeneVision N1",
  "origin_country": "China",
  "warranty_period": 24,
  "technical_sheet_url": "https://ejemplo.com/ficha.pdf",
  "registro_sanitario": "DM-12345",
  "specifications": {
    "peso": "1.5kg",
    "pantalla": "10 pulgadas"
  },
  "regulatory_info": {
    "certificacion": "CE",
    "clase": "IIa"
  }
}
```

**Campos obligatorios:** `name`, `slug`, `brand`, `origin_country`, `registro_sanitario`

---

### Actualizar Producto

```http
PUT /api/admin/products/:id
```

**Cuerpo de la Solicitud:** (todos los campos opcionales)

```json
{
  "name": "Nombre Actualizado",
  "brand": "Nueva Marca",
  "warranty_period": 36,
  "is_active": false
}
```

---

### Eliminar Producto

```http
DELETE /api/admin/products/:id
```

---

### Alternar Estado Activo del Producto

```http
PATCH /api/admin/products/:id/toggle
```

---

### Subir Archivo

```http
POST /api/admin/upload
```

**Headers:**

```
Authorization: Bearer <token>
Content-Type: multipart/form-data
```

**Cuerpo de la Solicitud:**

- `file`: Archivo permitido (max 10MB)

**Tipos permitidos:**

| Tipo              | Extension | Uso                  |
| ----------------- | --------- | -------------------- |
| `image/jpeg`      | .jpg      | Imagenes de producto |
| `image/webp`      | .webp     | Imagenes de producto |
| `application/pdf` | .pdf      | Fichas tecnicas      |

> Los nombres de archivo se generan como UUID en el servidor para evitar colisiones y ataques de enumeracion.

**Respuesta:**

```json
{
  "code": "OK",
  "url": "https://bucket.s3.amazonaws.com/products/images/uuid.jpg"
}
```

---

### Listar Cotizaciones

```http
GET /api/admin/quotes
```

**Parametros de Consulta:**

- `status` (opcional): Filtrar por estado (pending, contacted, closed)
- `page` (opcional, por defecto: 1)
- `limit` (opcional, por defecto: 50, max: 100)

---

### Obtener Detalles de Cotizacion

```http
GET /api/admin/quotes/:id
```

---

### Actualizar Estado de Cotizacion

```http
PATCH /api/admin/quotes/:id/status
```

**Cuerpo de la Solicitud:**

```json
{
  "status": "contacted",
  "notes": "Cliente llamado, esperando respuesta"
}
```

---

## Notificaciones por Email

Cuando se crea una cotizacion, se envia un email HTML profesional al equipo de ventas que incluye:

- Razon social y RUC de la empresa
- Datos de contacto (nombre, email, telefono)
- Lista de productos solicitados
- Mensaje adicional del cliente

Los correos se envian a traves de la API de Resend.
