# Módulos Premium - Documentación Completa

## 📋 Descripción General

El sistema tiene **dos perspectivas de billing** claramente diferenciadas:

### 1. 🏫 Billing por Colegio (Cada institución)
Cada colegio ve y gestiona **su propio plan** desde su instancia.

**Endpoints:**
- `GET /api/billing/plans` - Ver planes disponibles
- `GET /api/billing/my-plan` - Ver mi plan actual
- `POST /api/billing/checkout` - Suscribirse/upgrade

**Acceso:** Admin de cada colegio

---

### 2. 👑 Root Console (Dueño de la Plataforma - TÚ)
Tú gestionas **TODOS los colegios y licencias** desde la consola root.

**Endpoints:**
- `GET /api/saas/dashboard` - Métricas globales (MRR, colegios, etc.)
- `GET /api/saas/licenses` - Todas las licencias
- `GET /api/saas/schools/stats` - Colegios con estadísticas
- `POST /api/saas/licenses` - Crear/actualizar licencia manualmente
- `GET /api/saas/licenses/expiring` - Licencias por vencer

**Acceso:** SuperAdmin (root)

---

## 💰 Planes Disponibles (Para Colegios)

### Plan Basic - $49 USD/mes
**Ideal para colegios pequeños que necesitan gestión académica esencial.**

**Incluye:**
- ✅ Gestión Académica Core (cursos, estudiantes, profesores)
- ✅ Reportes Básicos (notas y asistencia)
- ✅ Importación Masiva CSV
- ✅ Personalización de Marca (logo y colores)
- ✅ Hasta 50 usuarios
- ✅ Hasta 500 estudiantes

**No incluye:**
- ❌ Módulo financiero
- ❌ Generación de PDFs oficiales
- ❌ Notificaciones por email
- ❌ Portal para padres

---

### Plan Premium - $99 USD/mes ⭐ (Más Popular)
**Para colegios en crecimiento que buscan automatizar procesos.**

**Todo lo del plan Basic, más:**
- ✅ Módulo Financiero (pagos, pensiones, morosidad)
- ✅ Generación de PDFs (boletines, certificados)
- ✅ Notificaciones por Email
- ✅ Portal para Padres
- ✅ Hasta 200 usuarios
- ✅ Hasta 2,000 estudiantes

---

### Plan Enterprise - $249 USD/mes
**Solución completa para instituciones grandes y redes educativas.**

**Todo lo del plan Premium, más:**
- ✅ Notificaciones SMS
- ✅ Push Notifications (app móvil)
- ✅ Analítica Avanzada y BI
- ✅ API Access completo
- ✅ Audit Logs (cumplimiento)
- ✅ Autenticación 2FA
- ✅ Integraciones Personalizadas
- ✅ Soporte Prioritario 24/7
- ✅ White Label completo
- ✅ Multi-Sede
- ✅ Usuarios ilimitados
- ✅ Estudiantes ilimitados

---

## 🔧 Implementación Técnica

### Estructura de Features

```rust
pub enum FeatureType {
    // CORE (Todos los planes)
    AcademicCore,
    BasicReports,
    CsvImport,
    Branding,

    // PREMIUM
    FinancialModule,
    PdfGeneration,
    EmailNotifications,
    ParentPortal,

    // ENTERPRISE
    SmsNotifications,
    PushNotifications,
    AdvancedAnalytics,
    ApiAccess,
    AuditLogs,
    TwoFactorAuth,
    CustomIntegrations,
    PrioritySupport,
    WhiteLabel,
    MultiCampus,
}
```

### Verificación de Acceso

#### Desde Handlers (usando macro)
```rust
#[get("/financial/payments")]
pub async fn list_payments(
    repo: web::Data<Repository>,
    claims: Claims,
) -> HttpResponse {
    // Verificar si el colegio tiene acceso al módulo financiero
    require_feature!(repo, claims, FeatureType::FinancialModule)?;
    
    // Lógica del endpoint...
}
```

#### Manual
```rust
let license = repo.get_license_by_school(school_id).await?;
let plan: PlanType = license.plan_type.parse().unwrap_or(PlanType::Basic);

if !plan.has_feature(&FeatureType::PdfGeneration) {
    return HttpResponse::Forbidden().json(json!({
        "error": "Feature not available in your plan",
        "upgrade_required": true
    }));
}
```

---

## 📊 Endpoints de Billing

### `GET /api/billing/plans`
Obtiene todos los planes disponibles con sus features detalladas.

**Response:**
```json
[
  {
    "name": "Basic",
    "price_monthly_usd": 49,
    "price_yearly_usd": 490,
    "max_students": 500,
    "max_users": 50,
    "features": [
      {
        "name": "Gestión Académica",
        "description": "Cursos, estudiantes, profesores, matrículas",
        "included": true
      }
    ],
    "popular": false
  }
]
```

### `GET /api/billing/my-plan`
Obtiene el plan actual del colegio autenticado.

**Response:**
```json
{
  "plan": {
    "name": "Premium",
    "price_monthly_usd": 99,
    "price_yearly_usd": 990,
    "max_students": 2000,
    "max_users": 200,
    "features": [...],
    "popular": true
  },
  "license": {
    "status": "active",
    "expiry_date": "2026-12-31T23:59:59Z",
    "auto_renew": true,
    "plan_type": "premium"
  },
  "features": [
    {
      "feature": "academic_core",
      "enabled": true,
      "limit": 2000,
      "used": 1240
    }
  ]
}
```

---

## 🎯 Límites por Plan

| Límite | Basic | Premium | Enterprise |
|--------|-------|---------|------------|
| Usuarios | 50 | 200 | Ilimitado |
| Estudiantes | 500 | 2,000 | Ilimitado |
| Almacenamiento | 5 GB | 25 GB | 100 GB |
| API Requests/día | 1,000 | 10,000 | Ilimitado |
| Soporte | Email (48h) | Email+Chat (24h) | 24/7 Prioritario |
| SLA | - | 99.5% | 99.9% |

---

## 🔄 Upgrade/Downgrade

### Upgrade
- El upgrade es **inmediato**
- Se cobra la diferencia proporcional del mes
- Las features se desbloquean al confirmar el pago

### Downgrade
- El downgrade aplica al **siguiente ciclo de facturación**
- Los datos excedentes se mantienen pero no se pueden editar
- Se notifica con 30 días de anticipación

---

## 💳 Métodos de Pago

- **Tarjeta de Crédito/Débito** (Stripe)
- **Transferencia Bancaria** (Enterprise)
- **PayPal** (próximamente)

---

## 🧪 Período de Prueba

- **14 días gratis** para todos los planes
- Acceso completo a todas las features del plan seleccionado
- No requiere tarjeta de crédito para iniciar
- Al finalizar, debe seleccionar un plan para continuar

---

## 🏷️ Descuentos

### Descuento Anual
- **2 meses gratis** al pagar anualmente
- Ejemplo: Premium anual = $990 USD (ahorra $288)

### Descuentos por Volumen
- **10+ colegios**: 10% de descuento
- **25+ colegios**: 20% de descuento
- **50+ colegios**: 30% de descuento (contactar ventas)

### Descuentos Especiales
- **Sin fines de lucro**: 25% de descuento
- **Gubernamental**: 30% de descuento
- **Países en desarrollo**: 40% de descuento

---

*Documentación actualizada: Marzo 2026*

---

## 👑 Root Console - Guía para el Dueño de la Plataforma

### ¿Qué es la Root Console?

La **Root Console** es tu panel de control como dueño de la plataforma SaaS. Desde aquí gestionas TODOS los colegios clientes.

---

### 📊 Dashboard Root (`GET /api/saas/dashboard`)

**Respuesta:**
```json
{
  "total_schools": 15,
  "total_users": 847,
  "active_licenses": 12,
  "trial_licenses": 2,
  "expiring_licenses": 3,
  "expired_licenses": 1,
  "mrr": 1487.00,
  "annual_forecast": 17844.00,
  "revenue_by_plan": {
    "basic": 294,
    "premium": 693,
    "enterprise": 498
  }
}
```

**Métricas clave:**
- **MRR (Monthly Recurring Revenue):** Ingreso recurrente mensual total
- **Annual Forecast:** Proyección anual (MRR × 12)
- **Revenue by Plan:** Ingresos desglosados por tipo de plan

---

### 🎫 Gestión de Licencias

#### Ver todas las licencias (`GET /api/saas/licenses`)

```json
[
  {
    "id": "uuid",
    "school_id": "uuid",
    "school_name": "Colegio Central Bogotá",
    "plan_type": "premium",
    "status": "active",
    "expiry_date": "2026-12-31T23:59:59Z",
    "auto_renew": true,
    "card_last4": "4242"
  }
]
```

#### Crear licencia manualmente (`POST /api/saas/licenses`)

**Request:**
```json
{
  "school_id": "uuid-del-colegio",
  "plan_type": "premium",
  "status": "active",
  "expiry_date": "2026-12-31T23:59:59Z",
  "auto_renew": true
}
```

**Casos de uso:**
- Registrar colegio nuevo que paga por transferencia
- Actualizar plan después de contacto con ventas
- Corregir licencias vencidas

---

### 🏫 Gestión de Colegios

#### Ver todos los colegios (`GET /api/saas/schools`)

```json
[
  {
    "id": "uuid",
    "name": "Colegio Central Bogotá",
    "subdomain": "ccb",
    "is_system_admin": false,
    "country_id": 170
  }
]
```

#### Ver colegio con estadísticas (`GET /api/saas/schools/stats`)

```json
[
  {
    "id": "uuid",
    "name": "Colegio Central Bogotá",
    "subdomain": "ccb",
    "user_count": 45,
    "license_status": "active",
    "license_plan": "premium",
    "country_code": "CO",
    "logo_url": "https://...",
    "primary_color": "#6366f1"
  }
]
```

#### Actualizar colegio (`PUT /api/saas/schools/{id}`)

**Request:**
```json
{
  "name": "Nuevo Nombre del Colegio",
  "subdomain": "nuevo-subdomain",
  "country_id": 170
}
```

---

### 🔔 Notificaciones Automáticas (Root)

El sistema envía alertas automáticas al dueño de la plataforma:

| Evento | Destinatario | Canal |
|--------|--------------|-------|
| Nuevo colegio registrado | Root | Email |
| Pago recibido | Root | Email |
| Pago fallido | Root + Colegio | Email |
| Licencia por vencer (30 días) | Root + Colegio | Email |
| Licencia vencida | Root + Colegio | Email |
| Trial por terminar (3 días) | Colegio | Email |

---

### 📈 Reportes para el Dueño

#### MRR por Plan
```
Basic:      $294/mes  (6 colegios × $49)
Premium:    $693/mes  (7 colegios × $99)
Enterprise: $498/mes  (2 colegios × $249)
----------------------------------------
Total MRR:  $1,485/mes
```

#### Proyección Anual
```
MRR × 12 = $1,485 × 12 = $17,820/año
```

#### Licencias por Vencer (próximos 30 días)
```
- Colegio ABC: Premium - Vence 2026-04-15
- Colegio XYZ: Basic - Vence 2026-04-20
```

---

### 🛠️ Comandos Útiles (Root)

```bash
# Ver todos los colegios
curl -H "Authorization: Bearer {token}" \
  https://api.tudominio.com/api/saas/schools

# Ver licencia de un colegio específico
curl -H "Authorization: Bearer {token}" \
  https://api.tudominio.com/api/saas/licenses

# Crear licencia manualmente
curl -X POST \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "school_id": "uuid",
    "plan_type": "premium",
    "status": "active",
    "expiry_date": "2026-12-31T23:59:59Z",
    "auto_renew": true
  }' \
  https://api.tudominio.com/api/saas/licenses

# Ver dashboard completo
curl -H "Authorization: Bearer {token}" \
  https://api.tudominio.com/api/saas/dashboard
```

---

### 🔐 Crear Usuario SuperAdmin (Root)

```sql
-- Insertar usuario root manualmente
INSERT INTO users (school_id, role_id, name, email, password_hash)
VALUES (
  (SELECT id FROM schools WHERE subdomain = 'ccb'),
  (SELECT id FROM roles WHERE name = 'admin'),
  'Tu Nombre',
  'tu_email@tudominio.com',
  '$argon2id$v=19$m=4096,t=3,p=1$...' -- Hash generado con argon2
)
ON CONFLICT (email) DO NOTHING;
```

---

## 📊 Comparativa: Billing por Colegio vs Root Console

| Característica | Colegio | Root (Tú) |
|----------------|---------|-----------|
| **Ve su propio plan** | ✅ Sí | ✅ Sí (de todos) |
| **Puede hacer upgrade** | ✅ Sí | ✅ Sí (manual) |
| **Ve MRR total** | ❌ No | ✅ Sí |
| **Gestiona otros colegios** | ❌ No | ✅ Sí |
| **Crea licencias** | ❌ No | ✅ Sí |
| **Ve licencias por vencer** | ❌ Solo la suya | ✅ Todas |
| **Recibe pagos** | ❌ No | ✅ Sí |
| **Acceso a /api/billing/** | ✅ Sí | ❌ No (usa /api/saas/) |
| **Acceso a /api/saas/** | ❌ No | ✅ Sí |

---

## 🚀 Flujo de Trabajo Típico

### 1. Colegio se registra solo
```
1. Colegio crea cuenta → Trial de 14 días
2. Root recibe notificación de nuevo colegio
3. Colegio explora la plataforma
4. Root hace seguimiento (opcional)
```

### 2. Colegio hace upgrade
```
1. Colegio va a "Planes" en su UI
2. Selecciona Premium y paga con Stripe
3. Root recibe notificación de pago
4. Licencia se actualiza automáticamente
5. MRR se recalcula
```

### 3. Root gestiona licencia manualmente
```
1. Colegio contacta por email (pago por transferencia)
2. Root crea licencia manualmente desde API
3. Root marca plan y fecha de vencimiento
4. Colegio recibe notificación de activación
```

### 4. Licencia por vencer
```
1. Sistema detecta licencias por vencer (30 días)
2. Root recibe lista de colegios por renovar
3. Root contacta colegios para renovación
4. Colegio renueva (automático o manual)
```

---

## 💡 Consejos para el Dueño de la Plataforma

1. **Revisa el dashboard semanalmente** - Monitorea MRR y licencias por vencer
2. **Configura notificaciones** - Para no perder renovaciones
3. **Ofrece descuentos anuales** - 2 meses gratis mejora el cash flow
4. **Seguimiento a trials** - Contacta colegios en prueba antes de que venzan
5. **Enterprise requiere contacto** - No permitas pago automático, negocia directamente

---

*Documentación actualizada: Marzo 2026*
