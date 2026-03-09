# Roadmap del Proyecto

Este documento describe las fases de desarrollo planificadas para el Sistema de Administración de Colegios SaaS.

## ✅ Fase 1: Cimientos e Infraestructura (Completada - Marzo 2026)
- [x] Definición de arquitectura (Docker, Rust, Postgres).
- [x] Configuración de `docker-compose.yml` y `Dockerfile` multi-stage.
- [x] Configuración inicial de PostgreSQL (Esquemas, Usuarios).
- [x] Backend Rust básico con Actix-web (endpoints de salud).
- [x] Frontend React básico con Create React App.
- [x] Proxy reverso Nginx configurado y funcional.
- [x] "Hola Mundo" conectado entre Frontend -> Backend -> DB.
- [x] Sistema de Logging y Monitoreo básico.
- [x] Docker Compose funcionando correctamente.
- [x] Estructura de directorios organizada y limpia.

- [x] **Módulo Académico Básico (Core):**
    - [x] Definición de esquemas para Alumnos, Profesores y Cursos.
    - [x] Endpoints base para gestión académica.
    - [x] Creación de recursos (Cursos, Profesores, Alumnos) desde la UI.
    - [x] **Módulo de Matriculación:** Inscripción de alumnos en cursos (Backend & UI).

## ✅ Fase 3: Producto Operativo (Completada - Marzo 2026)
- [x] Integración completa Frontend -> Backend con Auth.
- [x] Gestión atomática de usuarios y perfiles académicos.
- [x] Sistema de matriculación funcional.

## ✅ Fase 4: SaaS Enterprise & Root Console (Completada - Marzo 2026)
- [x] **Infraestructura SaaS:**
    - [x] Gestión de múltiples colegios y subdominios.
    - [x] Clasificación por países y regiones.
- [x] **Consola de Plataforma (Root Console):**
    - [x] Dashboard financiero integral (MRR, Forecast Anual).
    - [x] Gestión centralizada de licencias y planes.
    - [x] Vista de detalle y edición de instituciones cliente.
    - [x] Aislamiento total de datos académicos para el dueño de la plataforma.

## 🚧 Fase 5: Expansión y Polish (En Progreso)
- [x] **Importación Masiva:** Carga de usuarios vía Excel/CSV (`POST /admin/bulk-import`).
- [x] **Visibilidad Académica:** Vista de boletines para alumnos y padres (`GET /academic/my-report-card`).
- [x] **Personalización:** Gestión de logos y marca blanca por colegio (`PUT /admin/branding`).
- [x] **Internacionalización:** ES/EN con `react-i18next`, persistencia en `localStorage`.
- [x] **Infraestructura Avanzada:**
    - [x] Pipeline de CI/CD con GitHub Actions.
    - [x] Automatización de SSL con Certbot/Nginx.
    - [ ] API preparada para futura App Móvil.

## 🚀 Fase 6: Comunicación y Seguimiento (Planificado)
- [ ] **Módulo de Notificaciones:** Sistema de alertas por email y notificaciones push para faltas y calificaciones.
- [ ] **Portal para Padres:** Vista simplificada para acudientes con seguimiento de asistencia y notas en tiempo real.
- [ ] **Comunicados Escolares:** Publicación de noticias, circulares y eventos del calendario institucional.

## 🛡️ Fase 7: Seguridad y Respaldo (Planificado)
- [ ] **Auditoría de Acciones:** Registro detallado (Audit Logs) de quién modificó datos académicos o de configuración.
- [ ] **Backups Automáticos:** Programación de copias de seguridad incrementales a S3/Almacenamiento en la nube.
- [ ] **Autenticación Multi-factor (2FA):** Mayor seguridad para perfiles administrativos y de rectoría.

## 🧠 Fase 8: Analítica Avanzada y Operaciones (Planificado)
- [ ] **Generador de Reportes PDF:** Creación automática de boletines, certificados y planillas de asistencia en PDF.
- [ ] **Analítica Predictiva (IA):** Identificación temprana de alumnos con riesgo de deserción o bajo rendimiento.
- [ ] **Modo Offline (PWA):** Soporte de caché y Service Workers para uso de la plataforma en zonas rurales con baja conectividad.

## 🎯 Metas por Milestone

### Milestone 1 (Marzo 2026) ✅
- Infraestructura completa funcionando
- "Hola Mundo" end-to-end
- Documentación inicial

### Milestone 2 (Marzo 2026) ✅
- MVP funcional con autenticación y RBAC
- Gestión completa de usuarios, cursos, calificaciones y asistencia
- Consola Root con dashboard SaaS financiero
- Importación masiva, boletín estudiantil y personalización (Logo/Marca Blanca)
- Pipeline de CI/CD y automatización SSL con Certbot

### Milestone 3 (Septiembre 2026)
- Internacionalización completa y soporte Multi-idioma
- Portal para Padres y Notificaciones automáticas
- Auditoría de seguridad y Backups automáticos en la nube
- 10+ colegios piloto integrados

### Milestone 4 (Diciembre 2026)
- Optimización completa
- Aplicación móvil
- 20+ colegios clientes

## 📊 KPIs de Éxito

- **Técnicos:** 99.9% uptime, <500ms response time
- **Negocio:** 50 colegios clientes, MRR objetivo alcanzado
- **Usuario:** Satisfacción >4.5/5, retención >90%

## 🔄 Proceso de Desarrollo

- **Metodología:** Scrum con sprints de 2 semanas
- **Herramientas:** GitHub Projects, Discord para comunicación
- **Testing:** TDD con Rust, Testing Library con React
- **Code Quality:** Clippy, ESLint, Prettier

---

### Leyenda
- [x] Completado
- [ ] Pendiente
- [~] En progreso

### Notas
*   Los módulos premium se desarrollarán como servicios modulares para facilitar su activación según el plan contratado.
*   La internacionalización usa `react-i18next` con archivos de traducción JSON para soportar ES y EN desde el inicio.