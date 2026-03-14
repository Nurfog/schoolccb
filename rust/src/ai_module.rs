// ============================================
// Módulo de IA/ML - Ollama + Whisper
// Fase 9 - Inteligencia Artificial
// ============================================

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// Configuración de IA
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIConfig {
    pub ollama_url: String,
    pub whisper_url: String,
    pub model_chat: String,
    pub model_embedding: String,
}

impl AIConfig {
    pub fn from_env() -> Self {
        Self {
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_| "http://t-800.norteamericano.cl:11434".to_string()),
            whisper_url: env::var("WHISPER_URL").unwrap_or_else(|_| "http://t-800.norteamericano.cl:9000".to_string()),
            model_chat: env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".to_string()),
            model_embedding: env::var("OLLAMA_EMBEDDING_MODEL").unwrap_or_else(|_| "llama3.1".to_string()),
        }
    }
}

/// Cliente de IA
pub struct AIClient {
    pub config: AIConfig,
    client: Client,
}

impl AIClient {
    pub fn new(config: AIConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Chat con Llama (Ollama)
    pub async fn chat(
        &self,
        system_prompt: &str,
        user_message: &str,
        temperature: f32,
    ) -> Result<String, String> {
        let request = OllamaChatRequest {
            model: self.config.model_chat.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_message.to_string(),
                },
            ],
            stream: false,
            options: Some(ChatOptions {
                temperature,
                top_p: 0.9,
                num_predict: 512,
            }),
        };

        let url = format!("{}/api/chat", self.config.ollama_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Error calling Ollama: {}", e))?;

        let ollama_response: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Error parsing Ollama response: {}", e))?;

        Ok(ollama_response.message.content)
    }

    /// Transcripción de audio con Whisper
    pub async fn transcribe_audio(&self, audio_url: &str, language: Option<&str>) -> Result<WhisperTranscribeResponse, String> {
        let request = WhisperTranscribeRequest {
            audio_url: audio_url.to_string(),
            language: language.map(String::from),
        };

        let url = format!("{}/transcribe", self.config.whisper_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Error calling Whisper: {}", e))?;

        let transcription: WhisperTranscribeResponse = response
            .json()
            .await
            .map_err(|e| format!("Error parsing Whisper response: {}", e))?;

        Ok(transcription)
    }

    /// Análisis de riesgo de deserción escolar
    pub async fn analyze_dropout_risk(
        &self,
        attendance_percentage: f64,
        average_grade: f64,
        behavior_incidents: i32,
        socioeconomic_factors: &str,
    ) -> Result<AIAnalysisResult, String> {
        let prompt = format!(
            r#"Analiza el riesgo de deserción escolar de un estudiante con los siguientes datos:
- Asistencia: {:.1}%
- Promedio de notas: {:.2}
- Incidentes de comportamiento: {}
- Factores socioeconómicos: {}

Proporciona:
1. Nivel de riesgo (BAJO, MEDIO, ALTO, CRÍTICO)
2. Porcentaje de confianza (0-100)
3. 3-5 recomendaciones específicas para prevenir la deserción
4. Factores de riesgo identificados

Formato de respuesta:
RIESGO: [nivel]
CONFIANZA: [porcentaje]%
RECOMENDACIONES:
- [recomendación 1]
- [recomendación 2]
- [recomendación 3]
FACTORES: [lista de factores]"#,
            attendance_percentage,
            average_grade,
            behavior_incidents,
            socioeconomic_factors
        );

        let response = self.chat(
            "Eres un experto en educación y psicología escolar especializado en prevención de deserción escolar. Analiza datos académicos y proporciona recomendaciones accionables.",
            &prompt,
            0.3,
        ).await?;

        let risk_level = extract_risk_level(&response);
        let confidence = extract_confidence(&response);
        let recommendations = extract_recommendations(&response);

        Ok(AIAnalysisResult {
            analysis_type: "dropout_risk".to_string(),
            result: risk_level,
            confidence,
            recommendations,
            metadata: serde_json::json!({
                "attendance": attendance_percentage,
                "average_grade": average_grade,
                "behavior_incidents": behavior_incidents,
                "full_analysis": response
            }),
        })
    }

    /// Generar feedback automático para estudiante
    pub async fn generate_feedback(
        &self,
        student_name: &str,
        grades: Vec<(&str, f64)>,
        attendance: f64,
        teacher_comments: &str,
    ) -> Result<String, String> {
        let grades_str = grades
            .iter()
            .map(|(subject, grade)| format!("{}: {:.2}", subject, grade))
            .collect::<Vec<_>>()
            .join(", ");

        let prompt = format!(
            r#"Genera un feedback constructivo y motivador para el estudiante {} con los siguientes datos:
- Calificaciones: {}
- Asistencia: {:.1}%
- Comentarios del profesor: {}

El feedback debe:
1. Ser positivo y alentador
2. Reconocer logros
3. Señalar áreas de mejora de forma constructiva
4. Incluir recomendaciones específicas
5. Tener un tono cercano pero profesional

Extensión: 150-200 palabras."#,
            student_name,
            grades_str,
            attendance,
            teacher_comments
        );

        self.chat(
            "Eres un profesor experto en comunicación educativa. Genera feedback constructivo para estudiantes.",
            &prompt,
            0.5,
        ).await
    }

    /// Clasificar consulta de estudiante/padre
    pub async fn classify_query(&self, query: &str) -> Result<String, String> {
        let prompt = format!(
            r#"Clasifica la siguiente consulta en una de estas categorías:
- ACADÉMICO (notas, cursos, tareas)
- ADMINISTRATIVO (matrícula, pagos, documentos)
- ASISTENCIA (inasistencias, justificaciones)
- TÉCNICO (problemas con la plataforma)
- OTROS

Consulta: "{}"

Responde solo con el nombre de la categoría en mayúsculas."#,
            query
        );

        self.chat(
            "Eres un asistente virtual que clasifica consultas escolares.",
            &prompt,
            0.1,
        ).await
    }

    /// Resumir texto
    pub async fn summarize_text(&self, text: &str, max_length: usize) -> Result<String, String> {
        let prompt = format!(
            r#"Resume el siguiente texto en máximo {} palabras, manteniendo la información clave:

{}

Proporciona solo el resumen, sin introducciones ni conclusiones."#,
            max_length,
            text
        );

        self.chat(
            "Eres un asistente que resume textos de forma concisa y clara.",
            &prompt,
            0.3,
        ).await
    }

    /// Análisis de sentimiento
    pub async fn analyze_sentiment(&self, text: &str) -> Result<AIAnalysisResult, String> {
        let prompt = format!(
            r#"Analiza el sentimiento del siguiente texto:

"{}"

Clasifica como:
- POSITIVO (satisfacción, alegría, gratitud)
- NEUTRO (información objetiva, sin emoción clara)
- NEGATIVO (queja, frustración, enojo)

Proporciona:
1. Clasificación
2. Porcentaje de confianza
3. Palabras clave que indican el sentimiento
4. Recomendación de acción (si es negativo)"#,
            text
        );

        let response = self.chat(
            "Eres un experto en análisis de sentimiento de texto en español.",
            &prompt,
            0.2,
        ).await?;

        let sentiment = extract_sentiment(&response);
        let confidence = extract_confidence(&response);

        Ok(AIAnalysisResult {
            analysis_type: "sentiment_analysis".to_string(),
            result: sentiment,
            confidence,
            recommendations: vec![],
            metadata: serde_json::json!({
                "text": text,
                "full_analysis": response
            }),
        })
    }

    /// Chatbot de soporte
    pub async fn chatbot_support(
        &self,
        conversation_history: Vec<(String, String)>,
        user_message: &str,
        school_context: &str,
    ) -> Result<String, String> {
        let history_str = conversation_history
            .iter()
            .map(|(role, msg)| format!("{}: {}", role, msg))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"Contexto del colegio: {}

Historial de conversación:
{}

Usuario: {}

Asistente: "#,
            school_context,
            history_str,
            user_message
        );

        self.chat(
            r#"Eres un asistente virtual de soporte para un colegio. Tus funciones son:
1. Responder preguntas sobre notas, asistencia, calendarios
2. Guiar en trámites administrativos
3. Proporcionar información de contacto de profesores/administración
4. Ser amable, profesional y útil
5. Si no sabes algo, derivar a la persona correspondiente
6. Mantener respuestas concisas (max 100 palabras)

Importante: No inventes información. Si no estás seguro, indica que consultarás."#,
            &prompt,
            0.5,
        ).await
    }
}

// ============================================
// Structs para Ollama
// ============================================

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    pub options: Option<ChatOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaChatResponse {
    pub model: String,
    pub message: ChatMessage,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatOptions {
    pub temperature: f32,
    pub top_p: f32,
    pub num_predict: i32,
}

// ============================================
// Structs para Whisper
// ============================================

#[derive(Debug, Serialize)]
pub struct WhisperTranscribeRequest {
    pub audio_url: String,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WhisperTranscribeResponse {
    pub text: String,
    pub language: String,
    pub duration: f64,
}

// ============================================
// Structs para Análisis de IA
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysisResult {
    pub analysis_type: String,
    pub result: String,
    pub confidence: f32,
    pub recommendations: Vec<String>,
    pub metadata: serde_json::Value,
}

// ============================================
// Funciones Helper para Parsing
// ============================================

fn extract_risk_level(text: &str) -> String {
    if text.contains("CRÍTICO") {
        "CRÍTICO".to_string()
    } else if text.contains("ALTO") {
        "ALTO".to_string()
    } else if text.contains("MEDIO") {
        "MEDIO".to_string()
    } else {
        "BAJO".to_string()
    }
}

fn extract_confidence(text: &str) -> f32 {
    for line in text.lines() {
        if line.contains("CONFIANZA") || line.contains("%") {
            if let Some(start) = line.find(|c: char| c.is_numeric()) {
                if let Some(end) = line[start..].find(|c: char| !c.is_numeric() && c != '.') {
                    if let Ok(conf) = line[start..start+end].parse::<f32>() {
                        return conf / 100.0;
                    }
                }
            }
        }
    }
    0.7
}

fn extract_recommendations(text: &str) -> Vec<String> {
    let mut recommendations = Vec::new();
    let mut in_recommendations = false;

    for line in text.lines() {
        if line.contains("RECOMENDACIONES") {
            in_recommendations = true;
            continue;
        }
        if in_recommendations {
            if line.starts_with('-') || line.starts_with('*') {
                let rec = line.trim_start_matches(|c: char| !c.is_alphanumeric()).trim().to_string();
                if !rec.is_empty() {
                    recommendations.push(rec);
                }
            } else if !line.trim().is_empty() && !recommendations.is_empty() {
                break;
            }
        }
    }

    if recommendations.is_empty() {
        recommendations.push("Consultar con orientación escolar".to_string());
    }

    recommendations
}

fn extract_sentiment(text: &str) -> String {
    if text.contains("POSITIVO") {
        "POSITIVO".to_string()
    } else if text.contains("NEGATIVO") {
        "NEGATIVO".to_string()
    } else {
        "NEUTRO".to_string()
    }
}

// ============================================
// Funciones de Utilidad
// ============================================

pub fn get_ai_config() -> AIConfig {
    AIConfig::from_env()
}

pub fn create_ai_client() -> AIClient {
    AIClient::new(get_ai_config())
}
