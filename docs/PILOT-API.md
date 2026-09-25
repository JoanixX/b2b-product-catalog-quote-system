# API del piloto genérico

Base local: `http://localhost:3005`. Todas las rutas siguientes, excepto login, requieren `Authorization: Bearer <token privado>`. No pegar tokens en ejemplos, capturas ni repositorio.

| Método y ruta | Resultado |
| --- | --- |
| POST `/api/admin/login` | Recibe correo y contraseña privados; devuelve token de dos horas. |
| POST `/api/pilot/quotes` | Guarda solicitud sintética. Campos: `id` UUID generado por cliente, `company`, `contact`, `product`, `quantity`, `notes`. |
| GET `/api/pilot/quotes` | Lista las 100 solicitudes más recientes. |
| GET `/api/pilot/quotes/:id` | Devuelve expediente e historial de eventos. |
| POST `/api/pilot/quotes/:id/action` | Requiere `version` actual y una acción válida. |

Acciones: `draft` (recibida → borrador), `edit` (solo borrador), `approve` (requiere `human_confirmed: true`), `proposal` (solo aprobada), `followup` (solo propuesta).

`edit` recibe `quantity`, `unit_price_cents` (entero), `tax_bps` (entero; 1800 = 18%), `currency` (PEN o USD) y `terms`. El servidor calcula importes enteros y redondea el impuesto al céntimo más cercano, mitad hacia arriba. La persona debe validar precio, impuesto y condiciones antes de aprobar.

Guardar devuelve `saved: true`, `notification_status: disabled` y `quote`. Repetir exactamente la misma solicitud y UUID devuelve `replayed: true`; cambia el UUID solo para una solicitud nueva. Si cambias contenido manteniendo UUID, recibes 409. Después de una interrupción de red, consultar o repetir el mismo UUID evita duplicación.

Una transición inválida o versión obsoleta devuelve 409; valores fuera de límites, 400; falta de token, token inválido o cuenta eliminada, 401. Login limita intentos y devuelve 429 cuando se excede el límite de la instancia. Al recibir 409, volver a consultar el expediente antes de editar.

La propuesta guarda una instantánea de cantidad, precio, impuesto, condiciones, aprobador y fecha; no admite edición posterior a aprobación. El historial se escribe dentro de la misma transacción. No hay acción de envío ni integración de correo activa.
