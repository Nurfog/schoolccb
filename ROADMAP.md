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

## 🚧 Fase 4: Funcionalidades Académicas Avanzadas (En Progreso)
- [x] **Sistema de Calificaciones:**
    - [x] Registro de notas por parte de profesores.
    - [ ] Vista de boletines para alumnos y padres.
- [x] **Gestión Escolar:**
    - [x] Horarios y periodos académicos (Semestres/Bimestres).
    - [x] Asistencia diaria.
- [ ] **Perfil y Configuración:**
    - [ ] Panel de ajustes de cuenta para el usuario.
    - [ ] Gestión de datos del colegio (logo, dirección).
- [ ] **Infraestructura Avanzada:**
    - [ ] Pipeline de CI/CD con GitHub Actions.
    - [ ] Automatización de SSL con Certbot/Nginx.

## 🎯 Metas por Milestone

### Milestone 1 (Marzo 2026) ✅
- Infraestructura completa funcionando
- "Hola Mundo" end-to-end
- Documentación inicial

### Milestone 2 (Junio 2026)
- MVP funcional con autenticación
- Gestión básica de usuarios
- Primer colegio piloto

### Milestone 3 (Septiembre 2026)
- Módulos premium implementados
- Sistema de pagos integrado
- 5+ colegios en producción

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

## 🎯 Metas por Milestone

### Milestone 1 (Marzo 2026) ✅
- Infraestructura completa funcionando
- "Hola Mundo" end-to-end
- Documentación inicial

### Milestone 2 (Junio 2026)
- MVP funcional con autenticación
- Gestión básica de usuarios
- Primer colegio piloto

### Milestone 3 (Septiembre 2026)
- Módulos premium implementados
- Sistema de pagos integrado
- 5+ colegios en producción

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
    - [ ] Pipeline de CI/CD con GitHub Actions.
- [ ] Automatización de SSL con Certbot/Nginx.
- [ ] API preparada para futura App Móvil.

---

### Leyenda
- [x] Completado
- [ ] Pendiente
- [~] En progreso

### Notas
*   La prioridad actual es estabilizar el contenedor de Rust y la conexión con la base de datos.
*   Los módulos premium se desarrollarán como servicios o librerías modulares para facilitar su activación/desactivación según el plan contratado.