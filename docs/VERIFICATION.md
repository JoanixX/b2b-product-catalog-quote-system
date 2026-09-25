# Verificación de entrega — 25 de septiembre de 2026

## Demo pública

URL: https://trazo-cotizaciones.pages.dev/

- Instalación reproducible con `npm ci`; `npm test`: **3/3**; `astro check`: **0 errores, 0 avisos**; build: **5 páginas estáticas**.
- Astro actualizado a 7.3.5 con lockfile. `npm audit`: **0 vulnerabilidades conocidas** en el momento de la instalación verificada. Esto no certifica ausencia de cualquier vulnerabilidad.
- Se usó una copia limpia de verificación del frontend porque un proceso previo retenía `node_modules` en el checkout. Se verificaron el mismo `package-lock.json`, páginas, biblioteca de estado y pruebas del commit entregado.
- Navegador de escritorio y viewport móvil 390×844: landing, solicitud ficticia, panel, borrador, cambios, aprobación, propuesta y seguimiento comprobados.
- Sesión nueva en otro navegador: panel vacío y `/propuesta` sin datos muestra «Propuesta no encontrada», con retorno al panel.
- Cantidad inválida impide aprobar; precio con céntimos conserva el cálculo (10 × S/1.500,25 = S/15.002,50). Persistencia al recargar comprobada.
- Restablecer ejemplo comprobado: vuelve a un solo EJ-001 recibido. La confirmación nativa la cerró/aceptó personalmente el usuario tras una interrupción del control del navegador.
- Impresión invocada desde el botón. **No se inspeccionó un PDF efectivamente guardado**: el diálogo nativo interrumpió la automatización. La propuesta HTML sí fue inspeccionada visualmente; no se afirma verificación del archivo PDF final.
- Contacto `mailto:alvaradocjosorio@gmail.com` y enlaces a portafolio/GitHub presentes. No se envió ningún correo de prueba ni se registraron leads.
- Cloudflare mostró `feat/commercial-demo`, «Implementaciones automáticas habilitadas» y despliegue de `3fd1b29`; el sitio público se visitó después de actualizar las dependencias y el estado local.

## Backend funcional

PostgreSQL 15 en Docker y Rust 1.96, `cargo test --locked`: **4/4**, sin pruebas ignoradas. Incluye una prueba de integración con múltiples aserciones contra una base PostgreSQL aislada y tres pruebas heredadas de validación.

Riesgos comprobados:

- 17 combinaciones método/ruta administrativas y del piloto, cada una sin token y con token inválido: **34 rechazos 401**.
- Cuenta eliminada/inexistente con JWT firmado: 401; contraseña incorrecta: 401; login correcto devuelve token.
- El administrador histórico conocido no existe tras migrar.
- Entradas SQL hostiles en filtros públicos y administrativos no amplían resultados; filtros normales mantienen conteos correctos.
- Guardado compartido, repetición idempotente, rechazo de UUID reutilizado con datos distintos.
- Rechazo de transición prematura, importes inválidos, versión obsoleta y aprobación sin confirmación humana.
- Edición → aprobación → propuesta → seguimiento; importes e impuesto exactos; propuesta inmutable; historial de seis eventos; una única solicitud guardada.

`cargo build --release --locked` completado. SQLx 0.7.4 emite un aviso de incompatibilidad con una futura versión de Rust; el toolchain probado está fijado. No se hizo una auditoría exhaustiva de todas las dependencias Rust.

Comprobación HTTP local tras desactivar el catálogo público: `/health` 200, `/api/admin/products` 401, `/api/pilot/quotes` 401, `/api/products` 404. `/pilot` se abrió en navegador y muestra acceso exclusivo y aviso de datos sintéticos. La prueba autenticada completa se ejecutó por integración de API; el acceso interactivo requiere crear un operador privado mediante bootstrap.

## Staging gratuito

- Neon: proyecto `trazo-piloto`, PostgreSQL 15, plan Free y rama sin vencimiento verificados en panel. Solo PostgreSQL habilitado; sin Auth, Functions, almacenamiento de objetos ni pasarela de IA.
- Render: servicio `trazo-piloto`, plan Free, raíz `backend`, rama `feat/commercial-demo`, build `cargo build --release --locked`, runtime Rust 1.96.0. La cuenta no tenía tarjeta. [Panel](https://dashboard.render.com/web/srv-dardl77avr4c73e8hvhg/deploys).
- `DATABASE_URL` y `JWT_SECRET` están guardados en **Render → trazo-piloto → Environment**. No están en Git ni en la demo. El secreto JWT inicial fue reemplazado antes de activar el servicio.
- Staging desplegado y visitado en navegador: https://trazo-piloto.onrender.com/pilot muestra el acceso controlado. Comprobación HTTP remota: `/health` 200, `/pilot` 200, `/api/pilot/quotes` 401, `/api/admin/products` 401 y `/api/products` 404. Como el servidor ejecuta migraciones antes de escuchar, el arranque confirma que superó esa fase. Falta el bootstrap personal y el recorrido autenticado online; no se presenta como piloto operativo para clientes.
- Preparado el alta inicial por `BOOTSTRAP_ADMIN_EMAIL` y `BOOTSTRAP_ADMIN_PASSWORD`; la contraseña debe introducirse personalmente en el panel privado. Después del alta hay que retirar ambas variables y devolver Start Command al binario directo.

## Identidad, alcance y costes

Se comprobó permiso admin sobre el repositorio, correo primario verificado y configuración Git local. No se modificó configuración global ni autoría anterior. Los nuevos commits tienen autor y committer `JoanixX <alvaradocjosorio@gmail.com>`.

Al dejar de responder github.com por Git HTTPS, se usó la API Git de GitHub con la credencial existente del gestor normal. Se comparó el SHA creado con el SHA local, se avanzó la rama con `force: false` y se verificó la asociación de ambos campos a JoanixX. No se reescribió historia ni se tocó main.

No hay evidencia encontrada de una restricción de titularidad del código; el nombre del usuario local o un correo histórico no bastan para atribuir el repositorio a una empresa.

Gasto efectuado: **S/0**. Sin tarjeta añadida, dominio comprado o plan pagado. No hay clientes reales, correos enviados, integraciones comerciales activas ni datos de clientes en estos ejemplos.
