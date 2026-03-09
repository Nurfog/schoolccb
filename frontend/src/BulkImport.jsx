import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import api from './api';

const BulkImport = ({ onComplete }) => {
    const { t } = useTranslation();
    const [file, setFile] = useState(null);
    const [loading, setLoading] = useState(false);
    const [result, setResult] = useState(null);
    const [error, setError] = useState(null);

    const handleFileChange = (e) => {
        setFile(e.target.files[0]);
        setError(null);
    };

    const handleUpload = async () => {
        if (!file) {
            setError('Por favor selecciona un archivo CSV.');
            return;
        }

        setLoading(true);
        setError(null);

        const formData = new FormData();
        formData.append('file', file);

        try {
            // Using a raw fetch because the existing api.js might not handle FormData well without modification
            const token = api.getToken();
            const response = await fetch('http://localhost:8080/admin/bulk-import', {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${token}`
                },
                body: formData
            });

            const data = await response.json();
            if (response.ok) {
                setResult(data);
                if (onComplete) onComplete();
            } else {
                setError(data.error || 'Error al importar usuarios.');
            }
        } catch (err) {
            setError('Error de conexión con el servidor.');
        } finally {
            setLoading(false);
        }
    };

    const downloadTemplate = () => {
        const content = "name,email,password,role\nJuan Perez,juan@example.com,pass123,alumno\nMaria Docente,maria@example.com,pass456,profesor";
        const blob = new Blob([content], { type: 'text/csv' });
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'plantilla_usuarios.csv';
        a.click();
    };

    return (
        <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md animate-in fade-in slide-in-from-bottom-5 duration-500">
            <h2 className="text-2xl font-black mb-6 flex items-center space-x-3">
                <span className="text-3xl">📤</span>
                <span>{t('bulk_import.title')}</span>
            </h2>

            <div className="space-y-6">
                <div className="bg-indigo-500/10 border border-indigo-500/20 p-6 rounded-2xl">
                    <h3 className="text-sm font-black uppercase tracking-widest text-indigo-300 mb-2">Instrucciones</h3>
                    <ul className="text-sm text-blue-100/60 space-y-2 list-disc list-inside font-medium">
                        <li>El archivo debe estar en formato <span className="text-white font-bold">CSV</span> (.csv).</li>
                        <li>Debe incluir las columnas: <span className="text-white font-bold">name, email, password, role</span>.</li>
                        <li>{t('bulk_import.file_format_instruction_part1')} <span className="text-white font-bold">CSV</span> (.csv).</li>
                        <li>{t('bulk_import.columns_instruction_part1')} <span className="text-white font-bold">name, email, password, role</span>.</li>
                        <li>{t('bulk_import.roles_instruction_part1')} <span className="text-cyan-400 font-bold">{t('bulk_import.role_student')}</span> {t('bulk_import.and')} <span className="text-cyan-400 font-bold">{t('bulk_import.role_teacher')}</span>.</li>
                    </ul>
                    <button
                        onClick={downloadTemplate}
                        className="mt-4 text-indigo-400 hover:text-indigo-300 text-xs font-black uppercase tracking-widest flex items-center space-x-2"
                    >
                        <span>📥</span> <span>{t('bulk_import.download_template')}</span>
                    </button>
                </div>

                <div className="border-2 border-dashed border-white/10 rounded-2xl p-10 text-center hover:border-cyan-500/50 transition-colors">
                    <input
                        type="file"
                        accept=".csv"
                        onChange={handleFileChange}
                        className="hidden"
                        id="csv-upload"
                    />
                    <label htmlFor="csv-upload" className="cursor-pointer">
                        <span className="text-5xl block mb-4">📄</span>
                        <span className="text-blue-100/40 font-bold">
                            {file ? `${t('bulk_import.select_file')}: ${file.name}` : t('bulk_import.select_file')}
                        </span>
                    </label>
                </div>

                {error && (
                    <div className="bg-red-500/10 border border-red-500/50 p-4 rounded-xl text-red-400 text-sm font-bold flex items-center space-x-3">
                        <span>❌</span> <span>{error}</span>
                    </div>
                )}

                {result && (
                    <div className="bg-emerald-500/10 border border-emerald-500/50 p-4 rounded-xl text-emerald-400 text-sm font-bold flex items-center space-x-3">
                        <span>✅</span> <span>{result.imported_count} {t('bulk_import.success')}</span>
                    </div>
                )}

                <button
                    onClick={handleUpload}
                    disabled={loading || !file}
                    className="w-full bg-gradient-to-r from-violet-600 to-cyan-600 hover:from-violet-500 hover:to-cyan-500 disabled:opacity-50 disabled:cursor-not-allowed py-4 rounded-2xl font-black uppercase tracking-widest transition-all shadow-lg shadow-cyan-500/20"
                >
                    {loading ? t('bulk_import.uploading') : t('bulk_import.upload')}
                </button>
            </div>
        </div>
    );
};

export default BulkImport;
