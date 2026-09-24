# Trazo — demo comercial de cotizaciones B2B

Esta rama (`feat/commercial-demo`) es una demostración local y ficticia de un proceso de cotización: **solicitud → datos organizados → borrador → revisión y aprobación humana → propuesta → seguimiento**. La rama `main` conserva el proyecto original. El backend Rust original permanece en `backend/` sin modificaciones.

Todas las empresas, personas, productos e importes mostrados en la demo son **ejemplos ficticios**. La interfaz comercial no envía correos, mensajes ni datos a servicios externos. Su estado se guarda únicamente en el `localStorage` del navegador.

## Ejecutar la demo

Requisitos: Node.js 18 o superior y npm. Desde la raíz de este repositorio:

```powershell
cd frontend
npm ci
npm run dev
```

Abrir `http://localhost:4321`. No hace falta configurar `.env` ni iniciar PostgreSQL o Rust para la demo. Para verificar el paquete estático:

```powershell
npm run build
npm run preview
```

La demo local se inicia vacía. En la landing, **Ver el recorrido** abre el formulario con datos ficticios precargados. Desde el panel también puedes pulsar **Cargar ejemplo**. **Restablecer ejemplo** sustituye los datos guardados en ese navegador por una solicitud inicial.

## Personalizar marca y contenido

Edita `frontend/src/lib/demo.ts`:

- `brand.name`, `descriptor`, `primary`, `primaryDark`, `accent` y `logoUrl` controlan la marca. Coloca el logo en `frontend/public/` y usa una ruta como `/logo-agencia.svg`.
- `brand.salesContactUrl` habilita el enlace real de la llamada a la acción. Déjalo vacío hasta configurar un canal autorizado; la página avisará que el contacto está pendiente.
- `demoContent` contiene empresa, contacto, producto, importe, condiciones y observaciones de ejemplo. Mantenlos ficticios y señalados como ejemplo.

La simulación mantiene un único tipo de producto por solicitud y calcula el importe como cantidad × precio unitario, sin impuestos ni flete. Los datos creados durante el recorrido se guardan en el navegador donde se hizo la demostración. No se incluyen credenciales de demo.

## Qué funciona y qué es simulado

| Parte | Estado |
| --- | --- |
| Landing, formulario, panel, edición del borrador, aprobación, cambios de estado y vista de propuesta | Funcionan en el navegador. Se pueden repetir con el ejemplo y persisten por `localStorage`. |
| Imprimir o guardar PDF | Usa el diálogo de impresión del navegador. No hay generación de PDF en servidor. |
| Solicitud, aprobación, propuesta y seguimiento de la demo | Simulados localmente; no son registros de producción ni se comunican con el backend. |
| Backend original | Implementa API de catálogo, recepción de solicitudes y cambio básico de estado con PostgreSQL; requiere configuración propia. Esta rama no añade esos flujos al panel comercial. |
| Correo, mensajería, pagos, integraciones activas | No conectados en la demo. |

Para un piloto de producción faltan al menos: autenticación y permisos, persistencia compartida, historial de cambios y aprobaciones, reglas comerciales y de impuestos, plantilla documental revisada, envío por un canal autorizado, gestión de errores, actualización y auditoría de dependencias, y seguridad operacional. No se presentan resultados medidos ni promesas de ahorro porcentual.

Las páginas comerciales antiguas están guardadas en `frontend/legacy-pages/`. No forman parte de las rutas ni del build de esta demo porque contenían afirmaciones y datos de un proveedor no definido. El código cliente API original sigue en `frontend/src/lib/api.ts`; su configuración opcional histórica está en `frontend/.env.example`. Para usar el sistema original con PostgreSQL y Rust, cambia a `main` y consulta su README.

## Guion de grabación (60–90 segundos, sin mostrar la cara)

1. **0–10 s — Landing.** Muestra titular y flujo. Locución: «Una solicitud de cotización entra y todo el proceso queda visible en un solo lugar».
2. **10–22 s — Solicitud.** Pulsa **Ver el recorrido**, enseña los datos marcados como ejemplo y **Registrar solicitud**. «El pedido llega con empresa, producto, cantidad y observaciones ordenadas».
3. **22–35 s — Panel.** Señala el estado **Solicitud recibida** y pulsa **Preparar borrador**. «El equipo recibe un borrador con importe referencial».
4. **35–52 s — Revisión.** Cambia el precio o las condiciones y pulsa **Aprobar borrador**. «Una persona revisa y aprueba antes de preparar la propuesta».
5. **52–70 s — Propuesta.** Pulsa **Generar propuesta** y **Ver propuesta**. Muestra el documento y la opción **Imprimir o guardar PDF**. «La propuesta queda lista para revisar y compartir por el canal que la empresa decida».
6. **70–85 s — Seguimiento.** Vuelve al panel y pulsa **Marcar en seguimiento**. «El panel muestra el estado actual y conserva el contexto del pedido».

Mantén visible la etiqueta **Demo local** y evita describir el documento como enviado al cliente.
