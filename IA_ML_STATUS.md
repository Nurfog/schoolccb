# 🤖 IA/ML - Inteligencia Artificial Implementada

## 📊 ESTADO: **100% COMPLETADO** ✅

**Fecha:** Marzo 2026  
**Versión:** 1.0.0  
**IA Provider:** Ollama (Llama 3.2 + Llama 3.1) + Whisper

---

## 🧠 FEATURES DE IA IMPLEMENTADAS

### 1. **Chatbot de Soporte** ✅
- **Endpoint:** `POST /api/ai/chatbot`
- **Modelo:** Llama 3.2
- **Descripción:** Asistente virtual para estudiantes y padres
- **Casos de uso:**
  - Consultas sobre notas y asistencia
  - Información de trámites administrativos
  - Contacto de profesores
  - Calendarios y eventos

### 2. **Análisis de Riesgo de Deserción** ✅
- **Endpoint:** `POST /api/ai/analyze-dropout-risk`
- **Modelo:** Llama 3.1
- **Descripción:** Predice riesgo de deserción escolar
- **Inputs:**
  - Porcentaje de asistencia
  - Promedio de notas
  - Incidentes de comportamiento
  - Factores socioeconómicos
- **Output:** Nivel de riesgo (BAJO, MEDIO, ALTO, CRÍTICO) + recomendaciones

### 3. **Generación de Feedback** ✅
- **Endpoint:** `POST /api/ai/generate-feedback`
- **Modelo:** Llama 3.2
- **Descripción:** Genera feedback automático para estudiantes
- **Inputs:**
  - Nombre del estudiante
  - Calificaciones por materia
  - Asistencia
  - Comentarios del profesor
- **Output:** Feedback constructivo de 150-200 palabras

### 4. **Clasificación de Consultas** ✅
- **Endpoint:** `POST /api/ai/classify-query`
- **Modelo:** Llama 3.2
- **Descripción:** Clasifica consultas automáticamente
- **Categorías:**
  - ACADÉMICO
  - ADMINISTRATIVO
  - ASISTENCIA
  - TÉCNICO
  - OTROS

### 5. **Resumen de Texto** ✅
- **Endpoint:** `POST /api/ai/summarize`
- **Modelo:** Llama 3.2
- **Descripción:** Resume textos largos
- **Casos de uso:**
  - Comunicados largos
  - Actas de reuniones
  - Documentos administrativos

### 6. **Análisis de Sentimiento** ✅
- **Endpoint:** `POST /api/ai/analyze-sentiment`
- **Modelo:** Llama 3.2
- **Descripción:** Analiza sentimiento en texto
- **Casos de uso:**
  - Feedback de estudiantes/padres
  - Encuestas de satisfacción
  - Quejas y reclamos
- **Output:** POSITIVO, NEUTRO, NEGATIVO + confianza

### 7. **Transcripción de Audio (Whisper)** ✅
- **Endpoint:** `POST /api/ai/transcribe`
- **Modelo:** Whisper
- **Descripción:** Transcribe audio a texto
- **Casos de uso:**
  - Reuniones transcritas
  - Notas de voz a texto
  - Accesibilidad

### 8. **Estado de Servicios IA** ✅
- **Endpoint:** `GET /api/ai/status`
- **Descripción:** Verifica conectividad con Ollama y Whisper

---

## 🔧 CONFIGURACIÓN

### Variables de Entorno

```env
# IA/ML Configuration
OLLAMA_URL=http://t-800.norteamericano.cl:11434
WHISPER_URL=http://t-800.norteamericano.cl:9000
OLLAMA_MODEL=llama3.2
OLLAMA_EMBEDDING_MODEL=llama3.1
```

### Modelos Disponibles

| Modelo | Puerto | Uso | Tamaño |
|--------|--------|-----|--------|
| **Llama 3.2** | 11434 | Chat, feedback, clasificación | ~3GB |
| **Llama 3.1** | 11434 | Análisis complejos | ~8GB |
| **Whisper** | 9000 | Transcripción de audio | ~1GB |

---

## 📊 KPIs DE IA/ML

### KPIs Técnicos

| KPI | Objetivo | Actual | Estado |
|-----|----------|--------|--------|
| Chatbot accuracy | >90% | Por medir en producción | 📊 |
| Dropout prediction | >85% | Por medir en producción | 📊 |
| Sentiment analysis | >80% | Por medir en producción | 📊 |
| Audio transcription | >95% | Por medir en producción | 📊 |
| Feedback generation | >85% | Por medir en producción | 📊 |
| Response time (p95) | <5s | Por medir | 📊 |
| Uptime IA services | 99% | Por medir | 📊 |

### KPIs de Uso

| KPI | Objetivo | Actual | Estado |
|-----|----------|--------|--------|
| Consultas chatbot/día | 100+ | Por medir | 📊 |
| Análisis dropout/mes | 50+ | Por medir | 📊 |
| Feedbacks generados/mes | 200+ | Por medir | 📊 |
| Transcripciones/mes | 20+ | Por medir | 📊 |

---

## 🎯 CASOS DE USO POR ROL

### Estudiantes
- ✅ Chatbot para consultas académicas
- ✅ Feedback automático en boletines
- ✅ Transcripción de clases grabadas

### Padres
- ✅ Chatbot para información administrativa
- ✅ Análisis de sentimiento en encuestas
- ✅ Feedback de progreso de hijos

### Profesores
- ✅ Generación automática de feedback
- ✅ Análisis de riesgo de deserción
- ✅ Clasificación de consultas
- ✅ Transcripción de reuniones

### Administradores
- ✅ Análisis de sentimiento de feedback
- ✅ Resumen de documentos largos
- ✅ Chatbot para trámites
- ✅ Reportes de riesgo de deserción

### Orientación Escolar
- ✅ Análisis detallado de dropout risk
- ✅ Recomendaciones personalizadas
- ✅ Seguimiento de estudiantes en riesgo

---

## 📝 EJEMPLOS DE USO

### Chatbot de Soporte

```bash
curl -X POST http://localhost:8080/api/ai/chatbot \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "¿Cuándo es la próxima reunión de apoderados?",
    "history": []
  }'
```

### Análisis de Riesgo de Deserción

```bash
curl -X POST http://localhost:8080/api/ai/analyze-dropout-risk \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "attendance": 75.5,
    "average_grade": 5.2,
    "behavior_incidents": 3,
    "socioeconomic_factors": "Familia monoparental, madre trabajadora"
  }'
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
    "Evaluar situación socioeconómica para beca",
    "Seguimiento semanal de asistencia"
  ],
  "metadata": {
    "attendance": 75.5,
    "average_grade": 5.2,
    "behavior_incidents": 3
  }
}
```

### Generación de Feedback

```bash
curl -X POST http://localhost:8080/api/ai/generate-feedback \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "student_name": "María González",
    "grades": [
      {"subject": "Matemáticas", "grade": 6.5},
      {"subject": "Lenguaje", "grade": 5.8},
      {"subject": "Ciencias", "grade": 6.0}
    ],
    "attendance": 92.5,
    "teacher_comments": "Excelente participación en clases"
  }'
```

### Transcripción de Audio

```bash
curl -X POST http://localhost:8080/api/ai/transcribe \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "audio_url": "https://storage.schoolccb.com/audios/reunion-2026-03-20.mp3",
    "language": "es"
  }'
```

---

## 🔒 SEGURIDAD Y PRIVACIDAD

### Consideraciones de IA

1. **No se almacenan conversaciones** - Las consultas a IA son stateless
2. **Datos anonimizados** - Se envían solo datos necesarios
3. **Timeout de respuestas** - Máximo 30 segundos por consulta
4. **Rate limiting** - 10 consultas/minuto por usuario
5. **Logs de auditoría** - Todas las consultas quedan registradas en audit_logs

### Permisos Requeridos

| Endpoint | Roles Permitidos |
|----------|------------------|
| `/api/ai/chatbot` | Todos |
| `/api/ai/analyze-dropout-risk` | admin, root, orientacion |
| `/api/ai/generate-feedback` | admin, profesor |
| `/api/ai/classify-query` | Todos |
| `/api/ai/summarize` | admin, profesor |
| `/api/ai/analyze-sentiment` | admin |
| `/api/ai/transcribe` | admin, profesor |
| `/api/ai/status` | admin |

---

## 📈 ROADMAP DE IA/ML

### Fase 9 (Completado ✅)
- [x] Chatbot de soporte
- [x] Análisis de dropout risk
- [x] Generación de feedback
- [x] Clasificación de consultas
- [x] Resumen de texto
- [x] Análisis de sentimiento
- [x] Transcripción de audio
- [x] Endpoint de estado

### Fase 10 (Futuro 📋)
- [ ] Fine-tuning de modelos con datos del colegio
- [ ] Embeddings para búsqueda semántica
- [ ] RAG (Retrieval-Augmented Generation) para documentación
- [ ] Detección de plagio en tareas
- [ ] Recomendación de recursos educativos
- [ ] Análisis predictivo de rendimiento
- [ ] Chatbot con voz (TTS)

---

## 🛠️ TROUBLESHOOTING

### IA no responde

```bash
# Verificar estado
curl http://localhost:8080/api/ai/status

# Verificar Ollama
curl http://t-800.norteamericano.cl:11434/api/tags

# Verificar Whisper
curl http://t-800.norteamericano.cl:9000/health
```

### Respuestas lentas

- Usar Llama 3.2 en lugar de 3.1 para chat
- Reducir `num_predict` en opciones
- Implementar caching de respuestas frecuentes

### Errores de parsing

- Verificar formato de respuesta de IA
- Implementar retry con backoff
- Loggear respuestas crudas para debugging

---

## 📞 SOPORTE

**Documentación:** `/home/juan/dev/schoolccb/`  
**Issues:** GitHub Issues  
**Email:** soporte@schoolccb.com  

---

**🤖 IA/ML 100% IMPLEMENTADA - LISTA PARA PRODUCCIÓN! 🚀**

*Generado: Marzo 2026*  
*Versión: 1.0.0*  
*Estado: 100% completado ✅*
