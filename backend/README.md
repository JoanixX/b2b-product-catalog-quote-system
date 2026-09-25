# Backend: piloto controlado

Rust 1.96, Axum y PostgreSQL 15. La demo de Astro no llama a esta API.

Desde la raíz, con Docker Desktop Linux:

```powershell
./scripts/Start-Pilot.ps1 -Test
./scripts/Start-Pilot.ps1 -Bootstrap
./scripts/Start-Pilot.ps1
```

Interfaz: http://localhost:3005/pilot. El bootstrap pide credenciales privadas y rechaza crear otra cuenta cuando ya existe un operador. No hay usuario/contraseña por defecto. No publiques el archivo `.pilot-private/local.env`.

## Ejecución sin Docker

Configurar privadamente `DATABASE_URL`, `JWT_SECRET` (aleatorio, mínimo 32 caracteres), `PORT` y `CORS_ORIGIN`. Los valores de `.env.example` son ficticios y no deben usarse como secretos.

```sh
cargo test --locked
cargo run --locked -- bootstrap-admin
cargo run --locked
cargo build --release --locked
```

Para bootstrap, proporcionar temporalmente `BOOTSTRAP_ADMIN_EMAIL` y `BOOTSTRAP_ADMIN_PASSWORD` (mínimo 16 caracteres). Retirar ambas después. El usuario de PostgreSQL para tests necesita permiso para crear bases aisladas; nunca ejecutar los tests contra una base de cliente. Las migraciones se ejecutan antes de abrir el servidor y están incluidas en el binario.

## Desplegar

Imagen reproducible:

```sh
docker build -t trazo-pilot ./backend
```

El Dockerfile compila release con lockfile y ejecuta como usuario sin privilegios. Inyectar `DATABASE_URL` y `JWT_SECRET` desde el gestor privado del proveedor; no incluirlos en la imagen. Puerto por `PORT`; ruta de salud `/health`; interfaz `/pilot`. La comprobación `/health` indica proceso activo, no sustituye probar escritura/lectura real en PostgreSQL.

Render nativo: rama `feat/commercial-demo`, raíz `backend`, `RUST_VERSION=1.96.0`, build `cargo build --release --locked`, inicio `./target/release/b2b_product_catalog_quote_system_api`. Seleccionar expresamente Free: el formulario inicialmente selecciona un plan de pago. No añadir tarjeta. No mantener contraseñas de bootstrap una vez creado el operador. Si no hay shell, hacer el bootstrap desde un equipo autorizado o mediante un comando de inicio temporal y retirarlo después.

Base remota: PostgreSQL con TLS, URL privada del proveedor y un proyecto exclusivo para datos sintéticos. El staging no debe depender de una base que caduque a corto plazo ni presentarse como producción.

## Alcance y seguridad

`/api/pilot/*` y administración requieren JWT con operador existente. El login es `/api/admin/login`, limitado a 20 intentos por minuto por proceso (límite global de la instancia, no protección distribuida). Los tokens duran dos horas; al cerrar sesión se borra el token de memoria del navegador. No se guarda en localStorage.

La API pública heredada solo se activa con `ENABLE_LEGACY_PUBLIC_API=true`. Los uploads requieren compilar con `legacy-uploads` y no están habilitados en este piloto. Las reglas médicas del catálogo original no se aplican al flujo genérico. No se configuró correo ni mensajería; la respuesta de guardado declara que la notificación está desactivada.

La migración adicional elimina únicamente el administrador histórico con su hash original conocido, antes de servir. Una cuenta modificada deliberadamente no se borra. En una instalación existente, auditar además las cuentas y rotar credenciales conocidas; no reescribir migraciones aplicadas.

Antes de un cliente real: cuentas propias, backups y ensayo de restauración, roles acordados, retención, protección perimetral/rate limiting distribuido, revisión de dependencias y soporte. SQLx 0.7.4 compila y pasa pruebas con Rust 1.96, pero emite aviso de incompatibilidad futura; planificar su actualización antes de cambiar toolchain.
