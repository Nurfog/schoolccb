# 📚 API Documentation - SchoolCCB SaaS

## 📊 Resumen

- **Versión:** 1.0.0
- **Base URL:** `http://localhost:8080`
- **Autenticación:** JWT Bearer Token
- **Total Endpoints:** 89+
- **Estado:** 100% Completado

---

## 🔐 Autenticación

Todos los endpoints (excepto login/register) requieren header:
```
Authorization: Bearer {token}
```

---

## 📑 ÍNDICE

1. [Autenticación](#autenticación)
2. [Usuarios](#usuarios)
3. [Académico](#académico)
4. [SaaS](#saas)
5. [Comunicaciones](#comunicaciones)
6. [Seguridad](#seguridad)
7. [Finanzas](#finanzas)
8. [PDFs](#pdfs)
9. [IA/ML](#iaml)

---

## AUTENTICACIÓN

### POST /auth/login
Iniciar sesión
```json
{
  "email": "admin@schoolccb.com",
  "password": "SecurePass123!"
}
```
**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "uuid",
    "name": "Admin",
    "email": "admin@schoolccb.com",
    "role": "admin",
    "is_system_admin": true
  },
  "school": {...}
}
```

### POST /auth/register
Registrar usuario
```json
{
  "school_id": "uuid",
  "role_id": 1,
  "name": "Nuevo Usuario",
  "email": "nuevo@schoolccb.com",
  "password": "SecurePass123!"
}
```

### GET /auth/me
Obtener usuario actual

### POST /auth/2fa/setup
Iniciar configuración de 2FA

### POST /auth/2fa/verify
Verificar código 2FA
```json
{
  "code": "123456"
}
```

### POST /auth/2fa/disable
Deshabilitar 2FA
```json
{
  "code": "123456"
}
```

### GET /api/2fa/status
Verificar estado de 2FA

---

## USUARIOS

### GET /academic/teachers
Listar profesores del colegio

### POST /academic/teachers
Crear profesor
```json
{
  "name": "Profesor Nombre",
  "email": "profesor@schoolccb.com",
  "password": "SecurePass123!",
  "bio": "Biografía",
  "specialty": "Matemáticas"
}
```

### GET /academic/students
Listar estudiantes del colegio

### POST /academic/students
Crear estudiante
```json
{
  "name": "Estudiante Nombre",
  "email": "estudiante@schoolccb.com",
  "password": "SecurePass123!",
  "enrollment_number": "2026001",
  "parent_id": "uuid"
}
```

---

## ACADÉMICO

### Cursos

#### GET /academic/courses
Listar cursos

#### POST /academic/courses
Crear curso
```json
{
  "name": "Matemáticas 101",
  "description": "Curso de matemáticas",
  "teacher_id": "uuid",
  "grade_level": "10°"
}
```

### Matrículas

#### POST /academic/enrollments
Matricular estudiante en curso
```json
{
  "student_id": "uuid",
  "course_id": "uuid"
}
```

#### GET /academic/courses/{id}/students
Listar estudiantes de un curso

### Calificaciones

#### POST /academic/courses/{id}/grades
Agregar calificación
```json
{
  "student_id": "uuid",
  "name": "Prueba 1",
  "grade": 6.5
}
```

#### GET /academic/courses/{id}/grades
Listar calificaciones de un curso

### Asistencia

#### POST /academic/courses/{id}/attendance
Registrar asistencia
```json
{
  "student_id": "uuid",
  "date": "2026-03-20",
  "status": "present",
  "notes": "Llegó tarde"
}
```

### Boletín Estudiantil

#### GET /academic/my-report-card
Obtener boletín del estudiante autenticado

### Períodos Académicos

#### GET /academic/active-period
Obtener período académico activo

---

## SAAS

### Dashboard Root

#### GET /saas/dashboard
Dashboard de la consola Root (MRR, colegios, licencias)

### Colegios

#### GET /saas/schools
Listar todos los colegios

#### POST /saas/schools
Crear colegio
```json
{
  "name": "Colegio Nuevo",
  "subdomain": "nuevo",
  "country_id": 170
}
```

#### GET /saas/schools/{id}
Obtener colegio por ID

#### PUT /saas/schools/{id}
Actualizar colegio
```json
{
  "name": "Nuevo Nombre",
  "subdomain": "nuevo-subdomain",
  "country_id": 170
}
```

#### GET /saas/schools/stats
Listar colegios con estadísticas

### Licencias

#### GET /saas/licenses
Listar todas las licencias

#### GET /saas/licenses/expiring
Listar licencias por vencer

#### POST /saas/licenses
Crear/actualizar licencia
```json
{
  "school_id": "uuid",
  "plan_type": "premium",
  "status": "active",
  "expiry_date": "2026-12-31T23:59:59Z",
  "auto_renew": true
}
```

#### POST /saas/schools/{school_id}/license
Asignar licencia a colegio
```json
{
  "plan_type": "premium",
  "expiry_date": "2026-12-31T23:59:59Z",
  "auto_renew": true
}
```

### Países

#### GET /saas/countries
Listar países

#### POST /saas/countries
Crear país
```json
{
  "name": "Chile",
  "code": "CL"
}
```

### Estadísticas SaaS

#### GET /saas/stats
Obtener estadísticas SaaS

---

## COMUNICACIONES

### Notificaciones

#### GET /api/notifications
Listar notificaciones del usuario
```
Query: ?limit=20&offset=0
```

#### GET /api/notifications/unread-count
Contar notificaciones no leídas

#### PUT /api/notifications/{id}/read
Marcar notificación como leída

#### PUT /api/notifications/read-all
Marcar todas como leídas

#### DELETE /api/notifications/{id}
Eliminar notificación

### Preferencias

#### GET /api/notification-preferences
Obtener preferencias

#### PUT /api/notification-preferences
Actualizar preferencias
```json
{
  "email_enabled": true,
  "push_enabled": false,
  "sms_enabled": false,
  "in_app_enabled": true,
  "categories": {"academic": true, "financial": false},
  "quiet_hours_enabled": true,
  "quiet_hours_start": "22:00",
  "quiet_hours_end": "07:00"
}
```

### Plantillas

#### GET /api/templates
Listar plantillas
```
Query: ?category=academic
```

#### GET /api/templates/{code}
Obtener plantilla por código

### Comunicados

#### GET /api/announcements
Listar comunicados publicados
```
Query: ?category=urgent&limit=20&offset=0
```

#### GET /api/announcements/{id}
Ver comunicado

#### POST /api/announcements/{id}/read
Confirmar lectura de comunicado

#### GET /api/announcements/{id}/stats
Estadísticas de comunicado (admin)

#### POST /api/announcements
Crear comunicado (admin)
```json
{
  "title": "Comunicado Urgente",
  "content": "Contenido del comunicado",
  "summary": "Resumen corto",
  "category": "urgent",
  "target_audience": {"all": true},
  "priority": 3,
  "scheduled_at": "2026-03-20T10:00:00Z",
  "expires_at": "2026-03-27T23:59:59Z",
  "allow_comments": false,
  "requires_confirmation": true,
  "attachment_urls": ["https://..."]
}
```

#### POST /api/announcements/{id}/publish
Publicar comunicado (admin)

#### PUT /api/announcements/{id}
Actualizar comunicado (admin)

#### DELETE /api/announcements/{id}
Eliminar comunicado (admin)

### Justificaciones de Inasistencia

#### POST /api/parent/attendance-justification
Crear justificación (padre)
```json
{
  "student_id": "uuid",
  "absence_date": "2026-03-20",
  "absence_type": "full_day",
  "start_time": null,
  "end_time": null,
  "reason": "Motivo de la inasistencia"
}
```

#### GET /api/students/{id}/attendance-justifications
Listar justificaciones de estudiante

#### GET /api/school/attendance-justifications/pending
Listar justificaciones pendientes (admin/profesor)

#### POST /api/attendance-justifications/{id}/review
Revisar justificación (admin/profesor)
```json
{
  "status": "approved",
  "review_notes": "Aprobada"
}
```

#### GET /api/school/attendance-justifications/pending-count
Contar justificaciones pendientes

---

## SEGURIDAD

### Audit Logs

#### GET /api/audit/logs
Listar audit logs
```
Query: ?user_id=uuid&action=LOGIN&start_date=2026-01-01&end_date=2026-03-20
```

#### GET /api/audit/user/{user_id}/activity
Actividad de un usuario

#### GET /api/audit/suspicious-logins
Intentos sospechosos de login

### Sesiones

#### GET /api/sessions
Listar sesiones activas del usuario

#### POST /api/sessions/{session_id}/revoke
Revocar sesión específica
```json
{
  "reason": "Sesión comprometida"
}
```

#### POST /api/sessions/revoke-all
Revocar todas las sesiones excepto actual

---

## FINANZAS

### Pensiones

#### POST /api/finance/pensions
Crear pensión
```json
{
  "student_id": "uuid",
  "month": 3,
  "year": 2026,
  "amount": 100.00,
  "discount": 0,
  "surcharge": 0,
  "due_date": "2026-03-10"
}
```

#### GET /api/finance/students/{id}/pensions
Listar pensiones de estudiante
```
Query: ?year=2026
```

#### GET /api/finance/overdue-pensions
Listar pensiones vencidas
```
Query: ?limit=50
```

#### PUT /api/finance/pensions/{id}/status
Actualizar estado de pensión
```json
{
  "status": "paid"
}
```

### Pagos

#### POST /api/finance/payments
Registrar pago
```json
{
  "student_id": "uuid",
  "amount": 100.00,
  "payment_method": "transfer",
  "payment_reference": "TRF-2026-001",
  "notes": "Pago de pensión marzo",
  "pension_ids": ["uuid1", "uuid2"]
}
```

#### GET /api/finance/students/{id}/payments
Listar pagos de estudiante
```
Query: ?limit=20&offset=0
```

#### POST /api/finance/payments/{id}/apply
Aplicar pago a pensión
```json
{
  "pension_id": "uuid",
  "amount": 50.00
}
```

### Becas

#### POST /api/finance/scholarships
Crear beca
```json
{
  "student_id": "uuid",
  "name": "Beca por Rendimiento",
  "type_field": "percentage",
  "value": 50.00,
  "start_date": "2026-03-01",
  "end_date": "2026-12-31",
  "reason": "Promedio superior a 6.0"
}
```

#### GET /api/finance/students/{id}/scholarships
Listar becas de estudiante

### Facturas

#### POST /api/finance/invoices
Crear factura
```json
{
  "student_id": "uuid",
  "invoice_type": "receipt",
  "items": [
    {
      "description": "Pensión Marzo",
      "quantity": 1,
      "unit_price": 100.00,
      "payment_concept_id": "uuid"
    }
  ],
  "notes": "Factura de prueba"
}
```

#### GET /api/finance/students/{id}/invoices
Listar facturas de estudiante

### Dashboard Financiero

#### GET /api/finance/dashboard
Dashboard financiero del colegio

#### GET /api/finance/students/{id}/summary
Resumen financiero de estudiante

#### GET /api/finance/monthly-revenue
Ingresos mensuales
```
Query: ?months=12
```

---

## PDFS

### Boletines

#### GET /api/pdf/report-card/{student_id}
Generar boletín de calificaciones en PDF

### Certificados

#### GET /api/pdf/certificate/{student_id}
Generar certificado de estudio
```
Query: ?type=ESTUDIOS
```

### Constancias

#### GET /api/pdf/attendance-certificate/{student_id}
Generar constancia de asistencia

---

## IA/ML

### Chatbot

#### POST /api/ai/chatbot
Chatbot de soporte
```json
{
  "message": "¿Cuándo es la reunión de apoderados?",
  "history": [
    {"role": "user", "content": "Hola"},
    {"role": "assistant", "content": "Hola, ¿en qué puedo ayudarte?"}
  ]
}
```

### Análisis de Deserción

#### POST /api/ai/analyze-dropout-risk
Analizar riesgo de deserción
```json
{
  "attendance": 75.5,
  "average_grade": 5.2,
  "behavior_incidents": 3,
  "socioeconomic_factors": "Familia monoparental"
}
```

**Response:**
```json
{
  "analysis_type": "dropout_risk",
  "result": "ALTO",
  "confidence": 0.87,
  "recommendations": [
    "Coordinar reunión con apoderado",
    "Ofrecer apoyo psicológico",
    "Evaluar situación para beca"
  ],
  "metadata": {...}
}
```

### Feedback

#### POST /api/ai/generate-feedback
Generar feedback para estudiante
```json
{
  "student_name": "María González",
  "grades": [
    {"subject": "Matemáticas", "grade": 6.5},
    {"subject": "Lenguaje", "grade": 5.8}
  ],
  "attendance": 92.5,
  "teacher_comments": "Excelente participación"
}
```

### Clasificación

#### POST /api/ai/classify-query
Clasificar consulta
```json
{
  "query": "¿Cómo justifico una inasistencia?"
}
```

**Response:**
```json
{
  "classification": "ASISTENCIA",
  "type": "query_classification"
}
```

### Resumen

#### POST /api/ai/summarize
Resumir texto
```json
{
  "text": "Texto largo para resumir...",
  "max_words": 100
}
```

### Sentimiento

#### POST /api/ai/analyze-sentiment
Analizar sentimiento
```json
{
  "text": "Estoy muy satisfecho con el servicio"
}
```

**Response:**
```json
{
  "analysis_type": "sentiment_analysis",
  "result": "POSITIVO",
  "confidence": 0.92,
  "recommendations": [],
  "metadata": {...}
}
```

### Transcripción

#### POST /api/ai/transcribe
Transcribir audio (Whisper)
```json
{
  "audio_url": "https://storage.schoolccb.com/audio.mp3",
  "language": "es"
}
```

**Response:**
```json
{
  "transcription": "Texto transcrito...",
  "language": "es",
  "duration": 120.5,
  "type": "audio_transcription"
}
```

### Estado

#### GET /api/ai/status
Verificar estado de servicios de IA

**Response:**
```json
{
  "ollama": "connected",
  "model": "llama3.2",
  "ollama_url": "http://t-800.norteamericano.cl:11434",
  "whisper_url": "http://t-800.norteamericano.cl:9000"
}
```

---

## 📊 CÓDIGOS DE ERROR

| Código | Descripción |
|--------|-------------|
| 200 | Éxito |
| 201 | Creado |
| 400 | Bad Request - Validación fallida |
| 401 | No autorizado - Token inválido |
| 403 | Prohibido - Permisos insuficientes |
| 404 | No encontrado |
| 429 | Rate limit excedido |
| 500 | Error interno del servidor |

---

## 🔒 PERMISOS POR ROL

| Endpoint | Roles Permitidos |
|----------|------------------|
| `/auth/*` | Todos |
| `/academic/*` | admin, profesor, alumno |
| `/saas/*` | admin, root |
| `/api/notifications/*` | Todos |
| `/api/announcements/*` | admin (crear), todos (leer) |
| `/api/audit/*` | admin, root |
| `/api/sessions/*` | Todos (propias) |
| `/api/finance/*` | admin, padre |
| `/api/pdf/*` | admin, dueño |
| `/api/ai/*` | Ver documentación de cada endpoint |

---

## 📞 SOPORTE

**Documentación:** `/home/juan/dev/schoolccb/`  
**Issues:** GitHub Issues  
**Email:** soporte@schoolccb.com  

---

*Última actualización: Marzo 2026*  
*Versión: 1.0.0*
