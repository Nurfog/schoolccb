# Roadmap del Proyecto

Este documento describe las fases de desarrollo planificadas para el Sistema de Administración de Colegios SaaS.

> **📊 Actualizado:** Marzo 2026 - Incluye nuevas tecnologías y módulos propuestos para escalamiento enterprise.

---

## ✅ Fase 1: Cimientos e Infraestructura (Completada - Marzo 2026)
- [x] Definición de arquitectura (Docker, Rust, Postgres).
- [x] Configuración de `docker-compose.yml` y `Dockerfile` multi-stage.
- [x] Configuración inicial de PostgreSQL (Esquemas, Usuarios).
- [x] Backend Rust básico con Actix-web (endpoints de salud).
- [x] Frontend React básico con Vite.
- [x] Proxy reverso Nginx configurado y funcional.
- [x] "Hola Mundo" conectado entre Frontend -> Backend -> DB.
- [x] Sistema de Logging y Monitoreo básico.
- [x] Docker Compose funcionando correctamente.
- [x] Estructura de directorios organizada y limpia.
- [x] **Optimizaciones de rendimiento** (imágenes distroless, tracing JSON, 25+ índices DB).

- [x] **Módulo Académico Básico (Core):**
    - [x] Definición de esquemas para Alumnos, Profesores y Cursos.
    - [x] Endpoints base para gestión académica.
    - [x] Creación de recursos (Cursos, Profesores, Alumnos) desde la UI.
    - [x] **Módulo de Matriculación:** Inscripción de alumnos en cursos (Backend & UI).

## ✅ Fase 2: Autenticación y RBAC (Completada - Marzo 2026)
- [x] Sistema de autenticación JWT con Argon2id.
- [x] Roles y permisos (admin, profesor, alumno, padre).
- [x] Claims-based RBAC con permisos granulares.
- [x] Multi-tenancy por colegio (school_id).

## ✅ Fase 3: Producto Operativo (Completada - Marzo 2026)
- [x] Integración completa Frontend -> Backend con Auth.
- [x] Gestión automática de usuarios y perfiles académicos.
- [x] Sistema de matriculación funcional.
- [x] Calificaciones y asistencia básicas.

## ✅ Fase 4: SaaS Enterprise & Root Console (Completada - Marzo 2026)
- [x] **Infraestructura SaaS:**
    - [x] Gestión de múltiples colegios y subdominios.
    - [x] Clasificación por países y regiones.
- [x] **Consola de Plataforma (Root Console):**
    - [x] Dashboard financiero integral (MRR, Forecast Anual).
    - [x] Gestión centralizada de licencias y planes.
    - [x] Vista de detalle y edición de instituciones cliente.
    - [x] Aislamiento total de datos académicos para el dueño de la plataforma.

## 🚧 Fase 5: Expansión y Polish (En Progreso - Q1 2026)
- [x] **Importación Masiva:** Carga de usuarios vía Excel/CSV (`POST /admin/bulk-import`).
- [x] **Visibilidad Académica:** Vista de boletines para alumnos y padres (`GET /academic/my-report-card`).
- [x] **Personalización:** Gestión de logos y marca blanca por colegio (`PUT /admin/branding`).
- [x] **Internacionalización:** ES/EN con `react-i18next`, persistencia en `localStorage`.
- [x] **Infraestructura Avanzada:**
    - [x] Pipeline de CI/CD con GitHub Actions (fmt, lint, audit, test, build).
    - [x] Automatización de SSL con Certbot/Nginx.
    - [x] Redis para caching (perfil opcional).
    - [ ] API preparada para futura App Móvil (endpoints optimizados, rate limiting).
    - [ ] Documentación OpenAPI/Swagger con `utoipa`.

---

## 🚀 Fase 6: Comunicación y Seguimiento (Q2 2026)

### Módulo de Notificaciones Unificado 🔔
- [ ] **Email:** Integración con SMTP/SendGrid/AWS SES
- [ ] **Push Notifications:** Firebase FCM / OneSignal
- [ ] **SMS:** Twilio / AWS SNS (opcional)
- [ ] **Notificaciones In-App:** WebSockets con Actix
- [ ] **Preferencias:** Configuración por usuario (qué, cuándo recibir)
- [ ] **Plantillas:** Sistema de templates para diferentes eventos

### Portal para Padres 👨‍👩‍👧
- [ ] Vista simplificada y mobile-first
- [ ] Seguimiento de asistencia en tiempo real
- [ ] Visualización de calificaciones por período
- [ ] Recepción de comunicados y notificaciones
- [ ] Justificación de inasistencias en línea
- [ ] Agenda de eventos y reuniones

### Comunicados Escolares 📢
- [ ] Publicación de noticias institucionales
- [ ] Circulares y comunicados oficiales
- [ ] Calendario de eventos (integración con FullCalendar)
- [ ] Confirmación de lectura (tracking)
- [ ] Categorización por tipo (urgente, informativo, académico)

### Tecnologías a Implementar
```yaml
Nuevas dependencias:
  - actix-web-actors (WebSockets)
  - lettre (email desde Rust) o integración con SendGrid
  - redis (pub/sub para notificaciones en tiempo real)
  - react-hot-toast (notificaciones en frontend)
```

---

## 🛡️ Fase 7: Seguridad y Auditoría (Q3 2026)

### Auditoría de Acciones (Audit Logs) 🔍
- [ ] Middleware de auditoría automática para todos los endpoints
- [ ] Registro de: usuario, acción, entidad, valores antes/después, IP, user-agent
- [ ] Búsqueda avanzada de logs (integración con Elasticsearch opcional)
- [ ] Retención configurable (ej. 7 años para compliance)
- [ ] Exportación de logs a formatos estándar (JSON, CSV)

### Autenticación Multi-Factor (2FA) 🔐
- [ ] TOTP (Google Authenticator, Authy)
- [ ] Backup codes para recuperación
- [ ] 2FA obligatorio para roles admin/root
- [ ] 2FA opcional para otros usuarios
- [ ] QR generation para setup

### Backups Automáticos 💾
- [ ] Backups diarios de PostgreSQL a S3/MinIO
- [ ] Backups incrementales (WAL archiving)
- [ ] Retención configurable (7, 30, 90 días)
- [ ] Pruebas de restauración automáticas
- [ ] Alertas de backup fallido

### Gestión de Sesiones 🔑
- [ ] Listado de sesiones activas por usuario
- [ ] Revocación de sesiones específicas
- [ ] Timeout de inactividad configurable
- [ ] Límite de sesiones concurrentes por usuario
- [ ] Logging de intentos de acceso fallidos

### Tecnologías a Implementar
```yaml
Nuevas dependencias:
  - totp-rs (2FA TOTP)
  - aws-sdk-s3 (backups a S3)
  - elasticsearch (búsqueda en audit logs, opcional)
  - middleware de auditoría personalizado
```

---

## 💰 Fase 8: Finanzas y Documentación (Q4 2026)

### Módulo Financiero Básico 💳
- [ ] Gestión de pensiones mensuales
- [ ] Control de pagos y recibos
- [ ] Morosidad y reportes de deuda
- [ ] Integración con Stripe para pagos en línea
- [ ] Recordatorios de pago automáticos (email/push)
- [ ] Dashboard financiero por colegio

### Generador de Documentos PDF 📄
- [ ] Boletines de calificaciones oficiales
- [ ] Certificados de estudio y matrícula
- [ ] Constancias de asistencia
- [ ] Reportes de notas por curso
- [ ] Plantillas personalizables por colegio
- [ ] Firma digital básica (nombre + cargo)

### Facturación Electrónica (según país) 🧾
- [ ] Integración con proveedores locales (ej. DIAN en Colombia)
- [ ] Generación de XML/JSON según estándar
- [ ] Timbrado y validación en línea
- [ ] Almacenamiento de facturas emitidas
- [ ] Reportes tributarios básicos

### Tecnologías a Implementar
```yaml
Nuevas dependencias:
  - printpdf o lopdf (generación PDF desde Rust)
  - stripe (pagos en línea)
  - react-pdf (generación PDF desde frontend)
  - handlebars (templates para documentos)
```

---

## 📈 Fase 9: Analítica y Business Intelligence (Q1 2027)

### Monitoreo y Observabilidad 🔭
- [ ] OpenTelemetry para tracing distribuido
- [ ] Jaeger/Tempo para visualización de traces
- [ ] Prometheus para métricas en tiempo real
- [ ] Grafana para dashboards de monitoreo
- [ ] Alertas configurables (Slack, email, PagerDuty)

### Dashboard de Analítica 📊
- [ ] KPIs por rol (rector, profesor, administrador)
- [ ] Tendencias de rendimiento académico
- [ ] Tasas de deserción y retención
- [ ] Análisis de asistencia histórica
- [ ] Comparativas entre períodos y cursos
- [ ] Exportación a Excel/CSV

### Reportes Programados 📬
- [ ] Envío automático de reportes por email
- [ ] Frecuencia configurable (diario, semanal, mensual)
- [ ] Personalización de destinatarios
- [ ] Adjuntar PDFs generados automáticamente
- [ ] Historial de reportes enviados

### Tecnologías a Implementar
```yaml
Nuevas dependencias:
  - opentelemetry-api + opentelemetry-otlp
  - prometheus (métricas)
  - grafana (dashboards, puede ser self-hosted)
  - apache-superset o metabase (BI embeddable, opcional)
```

---

## 📱 Fase 10: Mobile y Offline (Q2 2027)

### Aplicación Móvil (React Native / Flutter) 📲
- [ ] App nativa iOS y Android
- [ ] Login con biometría (FaceID, TouchID)
- [ ] Notificaciones push nativas
- [ ] Vista de calificaciones y asistencia
- [ ] Recepción de comunicados
- [ ] Justificación de inasistencias desde la app
- [ ] Modo offline básico (caché de datos)

### Progressive Web App (PWA) 🌐
- [ ] Service Workers para caché de assets
- [ ] Funcionalidad offline limitada
- [ ] Instalación desde navegador
- [ ] Sincronización diferida al recuperar conexión
- [ ] Notificaciones push web

### Tecnologías a Implementar
```yaml
Nuevas tecnologías:
  - React Native con Expo (recomendado)
  - o Flutter (si se prefiere Dart)
  - workbox (PWA service workers)
  - watermelondb (sync offline-first)
```

---

## 🧠 Fase 11: IA y Predictivo (Q3 2027)

### Detección Temprana de Deserción 🎯
- [ ] Modelo predictivo basado en:
  - Asistencia histórica
  - Rendimiento académico
  - Comportamiento disciplinario
  - Datos socioeconómicos (si disponibles)
- [ ] Alertas proactivas a orientadores
- [ ] Planes de intervención sugeridos
- [ ] Seguimiento de casos en riesgo

### Recomendación de Rutas de Mejora 💡
- [ ] Sugerencias personalizadas para estudiantes
- [ ] Recursos educativos recomendados
- [ ] Planes de refuerzo académico
- [ ] Comparativa con estudiantes similares

### Chatbot para Soporte 🤖
- [ ] Respuestas a preguntas frecuentes
- [ ] Guía en procesos (matrícula, pagos, etc.)
- [ ] Escalamiento a humano cuando sea necesario
- [ ] Aprendizaje continuo de interacciones

### Tecnologías a Implementar
```yaml
Nuevas tecnologías:
  - Python + scikit-learn (modelos ML)
  - o Rust + linfa (ML nativo en Rust)
  - LangChain (orquestación de IA)
  - Integración con LLMs (OpenAI, Anthropic, o modelos open-source)
```

---

## 🎯 Metas por Milestone

### Milestone 1 (Marzo 2026) ✅
- [x] Infraestructura completa funcionando
- [x] "Hola Mundo" end-to-end
- [x] Optimizaciones de rendimiento implementadas
- [x] Documentación inicial y OPTIMIZACIONES.md

### Milestone 2 (Marzo 2026) ✅
- [x] MVP funcional con autenticación y RBAC
- [x] Gestión completa de usuarios, cursos, calificaciones y asistencia
- [x] Consola Root con dashboard SaaS financiero
- [x] Importación masiva, boletín estudiantil y personalización
- [x] Pipeline de CI/CD y automatización SSL

### Milestone 3 (Junio 2026) - Comunicación
- [ ] Sistema de notificaciones (email, push, in-app)
- [ ] Portal para Padres funcional
- [ ] Comunicados escolares publicados
- [ ] WebSockets implementados para tiempo real
- [ ] 5+ colegios piloto integrados

### Milestone 4 (Septiembre 2026) - Seguridad
- [ ] Audit Logs completos
- [ ] 2FA implementado y operativo
- [ ] Backups automáticos configurados
- [ ] Gestión de sesiones avanzada
- [ ] 10+ colegios cliente

### Milestone 5 (Diciembre 2026) - Finanzas
- [ ] Módulo financiero básico en producción
- [ ] Integración con pasarela de pagos
- [ ] Generador de PDFs (boletines, certificados)
- [ ] Facturación electrónica (según país)
- [ ] 15+ colegios cliente, MRR positivo

### Milestone 6 (Marzo 2027) - Analítica
- [ ] OpenTelemetry + Grafana en producción
- [ ] Dashboards de analítica por rol
- [ ] Reportes programados por email
- [ ] Alertas proactivas configuradas
- [ ] 20+ colegios cliente

### Milestone 7 (Junio 2027) - Mobile
- [ ] App móvil en TestFlight / Play Store beta
- [ ] PWA con modo offline funcional
- [ ] Notificaciones push nativas
- [ ] Sincronización diferida implementada
- [ ] 25+ colegios cliente

### Milestone 8 (Septiembre 2027) - IA
- [ ] Modelo predictivo de deserción en producción
- [ ] Sistema de recomendación activo
- [ ] Chatbot de soporte básico
- [ ] 30+ colegios cliente
- [ ] Break-even financiero alcanzado

---

## 📊 KPIs de Éxito

| Categoría | Métrica | Objetivo | Actual |
|-----------|---------|----------|--------|
| **Técnicos** | Uptime | 99.9% | TBD |
| | Response time (p95) | <500ms | TBD |
| | Error rate | <0.1% | TBD |
| | Image size (backend) | <50MB | ~30MB ✅ |
| **Negocio** | Colegios cliente | 50 | 0 (inicial) |
| | MRR objetivo | $10,000 USD | $0 (inicial) |
| | Churn rate | <5% anual | TBD |
| **Usuario** | Satisfacción (CSAT) | >4.5/5 | TBD |
| | Retención | >90% anual | TBD |
| | NPS | >50 | TBD |

---

## 🔄 Proceso de Desarrollo

- **Metodología:** Scrum con sprints de 2 semanas
- **Herramientas:** 
  - GitHub Projects para gestión de tareas
  - Discord para comunicación del equipo
  - Notion para documentación interna
  - Sentry para monitoreo de errores
- **Testing:** 
  - TDD con Rust cuando sea aplicable
  - Testing Library con React
  - Tests de integración con PostgreSQL
  - E2E con Playwright (futuro)
- **Code Quality:** 
  - Clippy (Rust) con `-D warnings`
  - ESLint + Prettier (Frontend)
  - Revisión de código obligatoria (1+ approvals)
  - Cobertura mínima: 70% (futuro)

---

## 📚 Nuevas Tecnologías Propuestas

| Tecnología | Propósito | Fase | Prioridad |
|------------|-----------|------|-----------|
| **Redis** | Caché y sesiones | 5 | ✅ Implementado |
| **WebSockets** | Notificaciones tiempo real | 6 | Alta |
| **OpenTelemetry** | Distributed tracing | 9 | Alta |
| **Prometheus + Grafana** | Métricas y alertas | 9 | Alta |
| **MinIO / S3** | Almacenamiento de archivos | 7 | Alta |
| **Elasticsearch** | Búsqueda full-text | 7 | Media |
| **Stripe** | Pagos en línea | 8 | Alta |
| **React Native** | App móvil | 10 | Alta |
| **printpdf** | Generación de PDFs | 8 | Alta |
| **totp-rs** | 2FA | 7 | Alta |
| **LangChain** | IA y chatbots | 11 | Media |

---

## 🚧 Módulo de Inventario (Backlog)

Módulos adicionales considerados para fases futuras (2028+):

- [ ] **Biblioteca Digital:** Catálogo, préstamos, reservas
- [ ] **Transporte Escolar:** Rutas, GPS, seguimiento
- [ ] **Salud Escolar:** Historial médico, vacunación
- [ ] **Recursos Humanos:** Nómina, contratos, evaluaciones
- [ ] **Inventarios:** Activos, mantenimiento, compras
- [ ] **Admisiones:** CRM de prospectos, proceso de ingreso
- [ ] **Actividades Extracurriculares:** Clubes, deportes, eventos
- [ ] **Encuestas y Satisfacción:** NPS, feedback continuo

---

### Leyenda
- [x] Completado
- [ ] Pendiente
- [~] En progreso
- ✅ Implementado parcialmente

### Notas
*   Los módulos premium se desarrollarán como servicios modulares para facilitar su activación según el plan contratado.
*   La internacionalización usa `react-i18next` con archivos de traducción JSON para soportar ES y EN desde el inicio.
*   Las fechas son estimadas y pueden ajustarse según prioridades del negocio y feedback de clientes.
*   **Importante:** Cada fase debe mantener compatibilidad hacia atrás con las anteriores.
*   Ver [`OPTIMIZACIONES.md`](OPTIMIZACIONES.md) para detalle de mejoras técnicas implementadas.

---

*Última actualización: Marzo 2026*