# Trazo — demo comercial y base de piloto B2B

**Demo pública:** https://trazo-cotizaciones.pages.dev/

Responsable: **Joaquín Alvarado** · [Portafolio](https://joanixx.github.io/) · [GitHub](https://github.com/JoanixX) · [Contacto comercial](mailto:alvaradocjosorio@gmail.com).

## Dos entornos separados

| Entorno | Arquitectura y alcance |
| --- | --- |
| Demo pública | Astro estático en Cloudflare Pages. Datos ficticios en `localStorage`; no usa API, base de datos ni correo. Funciona aunque el piloto esté detenido. |
| Piloto controlado | Rust/Axum + PostgreSQL; interfaz `/pilot`, acceso JWT, solicitudes compartidas, borrador editable, aprobación humana, propuesta congelada e historial. Solo datos sintéticos. |
| Cliente real | Todavía no existe. Requiere cuentas propias, requisitos acordados, tratamiento de datos, costes, respaldo y soporte. |

La simulación pública no almacena leads. El contacto real es el enlace de correo: abre el cliente de correo del visitante, no envía automáticamente. No hay clientes, resultados comerciales, WhatsApp, pagos ni correos enviados que se atribuyan a esta demo.

## Demo: ejecutar y publicar

Node.js **22.12 o superior** (versión fijada en `frontend/.node-version`) y npm:

```powershell
cd frontend
npm ci
npm test
npm run build
npm run preview
```

Para desarrollar: `npm run dev`. La salida estática es `frontend/dist`. No requiere `.env`.

Cloudflare Pages usa integración Git con `JoanixX/b2b-product-catalog-quote-system`, rama de producción `feat/commercial-demo`, raíz `frontend`, build `npm run build`, salida `dist`. Los pushes a esa rama disparan despliegues automáticamente. No se modificó `main` ni la autoría histórica.

[Panel de Cloudflare](https://dash.cloudflare.com/205644fa506ec0bae6a6dcaa58c405d2/pages/view/trazo-cotizaciones). Sin dominio comprado ni secretos del backend en el frontend.

## Preparar una reunión o grabación

1. Abre `/demo`, pulsa **Restablecer ejemplo** y acepta reemplazar únicamente los datos ficticios de ese navegador.
2. Comprueba que aparece `EJ-001` en **Solicitud recibida**.
3. Para grabar desde la landing, usa una sesión nueva y pulsa **Ver el recorrido**. Una sesión nueva empieza vacía.
4. Recorre solicitud → panel → preparar borrador → guardar cambios → aprobar → generar propuesta → ver propuesta → imprimir → volver al panel → seguimiento.

La propuesta no se comparte entre navegadores; compartir su URL no transmite sus datos. Para mostrar a otra persona, comparte la landing. `Imprimir o guardar PDF` abre el diálogo del navegador; la selección del destino y guardado se hacen allí.

[Guion exacto de 60–90 segundos](docs/RECORDING.md) · [Verificación y límites](docs/VERIFICATION.md).

## Piloto local reproducible

Requisitos: Docker Desktop con contenedores Linux y PowerShell. Desde la raíz:

```powershell
./scripts/Start-Pilot.ps1 -Test
./scripts/Start-Pilot.ps1 -Bootstrap
./scripts/Start-Pilot.ps1
```

El bootstrap solicita el correo y una contraseña única de al menos 16 caracteres, sin mostrarla. Solo permite crear el primer operador si no existe ninguno. Abre **http://localhost:3005/pilot** e inicia sesión. La primera compilación tarda varios minutos.

El script genera `POSTGRES_PASSWORD` y `JWT_SECRET` aleatorios en `.pilot-private/local.env` (ignorado por Git). PostgreSQL no publica un puerto al host; la API está enlazada solo a `127.0.0.1`. El volumen `pilot-data` conserva los datos. No uses `docker compose down -v` si deseas conservarlos.

Para detener sin borrar datos:

```powershell
docker compose --env-file .pilot-private/local.env -f compose.pilot.yml stop
```

La ruta funcional es: **guardar solicitud compartida → preparar borrador → configurar precio, impuesto y condiciones → guardar → confirmar revisión humana → aprobar → generar propuesta revisable → registrar seguimiento**. El piloto genérico no exige RUC ni registro sanitario. Admite una línea por solicitud, PEN/USD y un impuesto configurable; todos los valores deben validarse por una persona.

## Seguridad y comportamiento real

- Todas las rutas `/api/admin` salvo login, y todas las `/api/pilot`, requieren JWT y un operador existente. El login tiene límite de intentos por proceso.
- Filtros SQL parametrizados, paginación acotada, cuerpo máximo 64 KiB y límites de valores.
- Una nueva migración retira la cuenta histórica con hash conocido antes de servir peticiones; la migración original no se reescribe.
- Solicitudes del piloto idempotentes por UUID; repetir el mismo pedido no duplica, reutilizarlo con otro contenido devuelve 409.
- Edición y transiciones usan transacción, bloqueo y versión; una edición obsoleta o transición inválida devuelve 409. La propuesta conserva una instantánea de los valores aprobados.
- Guardar no depende del correo: se informa `saved: true` y `notification_status: disabled`. No se envían mensajes.
- La API pública del catálogo médico heredado está desactivada por defecto. Solo se habilita explícitamente con `ENABLE_LEGACY_PUBLIC_API=true`; no forma parte del piloto genérico.
- Los uploads heredados están desactivados sin la feature `legacy-uploads`. No se configuró AWS.

[Configuración del backend](backend/README.md) · [Contrato de API](docs/API.md).

## Servicios y costes

Cloudflare Pages Free: 500 builds/mes, una compilación simultánea y hasta 20.000 archivos por sitio, según [límites oficiales](https://developers.cloudflare.com/pages/platform/limits/) consultados el 25-09-2026. La demo entra en estos límites.

Se creó en la cuenta de Joaquín el proyecto Neon **trazo-piloto**, PostgreSQL 15, plan Free, 0,5 GB de almacenamiento, suspensión al quedar inactivo y rama sin fecha de caducidad en el panel. [Panel de Neon](https://console.neon.tech/app/projects/proud-resonance-02823206/branches/br-rough-recipe-b57qv94j). Esto no constituye almacenamiento de producción con SLA o estrategia de respaldo.

Render Free se evaluó para el staging: 512 MB RAM, 0,1 CPU, suspensión tras 15 minutos y 750 horas/mes compartidas por workspace; sin shell ni jobs puntuales. El panel de la cuenta no tenía tarjeta. PostgreSQL gratuito de Render se descartó porque expira a los 30 días. Consulta [límites oficiales](https://render.com/docs/free). El estado comprobado del staging está en `docs/VERIFICATION.md`; no lo confundas con la demo pública.

**Gasto efectuado en este trabajo: S/0.** No se añadió tarjeta, dominio ni plan pagado.

## Convertir un trato en piloto

1. Acordar un único proceso, productos, moneda, impuestos, aprobador y criterio de aceptación con el comprador.
2. Definir datos permitidos, consentimiento/base aplicable, retención, respaldo, recuperación, presupuesto y responsable de soporte.
3. Crear cuentas y despliegue del cliente separados de la demo. Guardar `DATABASE_URL` y `JWT_SECRET` solo en variables privadas; TLS obligatorio para la base remota.
4. Ejecutar migraciones, dar de alta el primer operador por bootstrap y retirar las variables temporales `BOOTSTRAP_ADMIN_EMAIL` y `BOOTSTRAP_ADMIN_PASSWORD`.
5. Probar con datos sintéticos el flujo, reintentos, aprobación, recuperación y acceso no autorizado. Validar el documento con el comprador.
6. Incorporar correo u otra integración solo si se acuerda, inicialmente en sandbox. Registrar quién paga cada cuenta. Activar datos reales únicamente tras aceptación del piloto.

Límites: un solo negocio por despliegue, sin multiempresa ni permisos granulares, sin recuperación de contraseña por correo, sin envío real y sin facturación electrónica. El backend es una base funcional para un piloto; no una certificación de producción.

Estado del staging y variables privadas: [verificación de entrega](docs/VERIFICATION.md). La cuenta propietaria de GitHub es JoanixX; Cloudflare pertenece a alvaradocjosorio@gmail.com; Render está en «Joaquin's workspace» y Neon en la organización de Joaquín. El entorno local pertenece al equipo del usuario y no es una URL pública.

### Estado comprobado del staging

El acceso de https://trazo-piloto.onrender.com/pilot está desplegado por HTTPS. La API rechaza peticiones sin token. Quedan pendientes la contraseña personal del primer operador y la comprobación autenticada online; el flujo completo ya pasó pruebas de integración locales con PostgreSQL. No se deben introducir datos reales.

| Servicio | Finalidad | Cuenta propietaria | Rama | Plan y estado |
| --- | --- | --- | --- | --- |
| Cloudflare Pages | Demo pública | alvaradocjosorio@gmail.com | feat/commercial-demo | Free; sitio y despliegue automático comprobados |
| GitHub | Código fuente | JoanixX | feat/commercial-demo | Repositorio público; commits asociados a JoanixX |
| Render | Backend de staging | Joaquin's workspace | feat/commercial-demo | Free; HTTPS y rechazo sin token comprobados; alta personal pendiente |
| Neon | Base del staging | Joaquín | production (rama de base, no Git) | Free; PostgreSQL 15 creado y backend iniciado con migraciones |
| Docker local | Desarrollo y pruebas | Equipo del usuario | Checkout comercial | Sin hosting contratado; pruebas del flujo completo aprobadas |

Secretos del staging: únicamente `DATABASE_URL` y `JWT_SECRET`, guardados en Render → trazo-piloto → Environment. Las variables temporales de bootstrap deben retirarse tras el alta del operador. Coste incurrido: S/0.
