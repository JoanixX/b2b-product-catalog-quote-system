# 🏢 B2B Product Catalog & Quotation Platform
Plataforma web full-stack de catálogo de productos con sistema de cotizaciones orientada a empresas B2B.

Este proyecto demuestra cómo construir una solución profesional para compañías que no venden directamente en línea, sino que generan leads y gestionan ventas mediante cotizaciones.

---

## 🚀 Características

### Públicas
* Catálogo de productos con búsqueda y filtros
* Navegación por categorías
* Página de detalle con especificaciones técnicas
* Visualización de fichas técnicas (PDF)
* Formulario de solicitud de cotización
* Validación de RUC peruano (algoritmo Módulo 11)

### Administrativas
* Autenticación segura (JWT + Argon2id)
* CRUD completo de productos y categorías
* Gestión de solicitudes de cotización
* Carga de archivos (imágenes y PDFs)
* Panel básico de administración

---

## 🧠 Stack Tecnológico

### Backend
* Rust
* Axum
* PostgreSQL (compatible con Neon)
* Autenticación JWT
* Hashing con Argon2id
* Almacenamiento compatible con S3

### Frontend
* Astro
* TypeScript
* Nano Stores
* CSS vanilla

---

## 🏗️ Estructura del Proyecto

b2b-product-catalog-quote-system/
├── backend/          # API REST en Rust/Axum
├── frontend/         # Aplicación web Astro/TypeScript
├── scripts/          # Utilidades (no incluidas en producción)
├── docs/             # Documentación
└── README.md

---

## ⚙️ Instalación y Ejecución Local

### Prerrequisitos
* Rust 1.70+
* Node.js 18+
* Base de datos PostgreSQL (local o en la nube)

---

### 🔧 Backend
cd backend
cp .env.example .env

Editar variables de entorno:
DATABASE_URL=postgresql://user:password@localhost:5432/db
JWT_SECRET=your-secret-key

Ejecutar:
cargo run

El backend estará disponible en:
http://localhost:3000

---

### 💻 Frontend
cd frontend
cp .env.example .env
npm install
npm run dev

El frontend estará disponible en:
http://localhost:4321

---

### ▶️ Ejecución completa
Terminal 1:
cd backend
cargo run

Terminal 2:
cd frontend
npm run dev

---

## 🔐 Seguridad
* Autenticación mediante JWT con expiración
* Contraseñas hasheadas con Argon2id
* Validación de RUC peruano
* Sanitización de entradas (prevención XSS)
* Restricción de tipos de archivo (JPEG, WebP, PDF)
* Manejo seguro de errores (sin exposición de datos internos)

---

## 🗄️ Esquema de Base de Datos
* products → catálogo de productos
* categories → clasificación
* quotes → solicitudes de cotización
* admins → usuarios administrativos

---

## 🔌 API Endpoints
### Públicos
* GET /api/products
* GET /api/products/:slug
* GET /api/categories
* POST /api/quotes

### Administrador
* POST /api/admin/login
* CRUD productos
* CRUD categorías
* Gestión de cotizaciones
* Upload de archivos

---

## 🎯 Caso de Uso

Este proyecto está diseñado para:
* Empresas distribuidoras
* Proveedores industriales o médicos
* Negocios B2B sin e-commerce directo

No incluye pagos ni carrito de compras.
El enfoque es la generación de leads mediante cotizaciones.

---

## ⚠️ Disclaimer
Este repositorio es una versión de portafolio basada en un proyecto real.

* Todos los datos sensibles han sido eliminados
* Las credenciales son de ejemplo
* No contiene información de ninguna empresa real

---

## 📄 Licencia

Uso educativo y de portafolio.
