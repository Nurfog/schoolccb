import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import api from './api';

const BrandingConfig = ({ school, onUpdate }) => {
    const { t } = useTranslation();
    const [logoUrl, setLogoUrl] = useState(school?.logo_url || '');
    const [primaryColor, setPrimaryColor] = useState(school?.primary_color || '#06b6d4');
    const [secondaryColor, setSecondaryColor] = useState(school?.secondary_color || '#4f46e5');
    const [loading, setLoading] = useState(false);
    const [message, setMessage] = useState(null);

    const handleSubmit = async (e) => {
        e.preventDefault();
        setLoading(true);
        setMessage(null);
        try {
            const updated = await api.put('/admin/branding', {
                logo_url: logoUrl,
                primary_color: primaryColor,
                secondary_color: secondaryColor
            });
            setMessage({ type: 'success', text: t('branding.success') });
            if (onUpdate) onUpdate(updated);
        } catch (err) {
            console.error(err);
            setMessage({ type: 'error', text: t('branding.error') });
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="bg-white/5 border border-white/10 p-10 rounded-[40px] backdrop-blur-3xl shadow-2xl animate-in fade-in slide-in-from-bottom-10 duration-700">
            <div className="mb-10">
                <h2 className="text-4xl font-black mb-2 bg-gradient-to-r from-white to-white/40 bg-clip-text text-transparent italic">
                    {t('branding.title')}
                </h2>
                <p className="text-blue-100/40 font-medium tracking-widest text-xs uppercase">
                    {t('branding.subtitle')}
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-8">
                <div>
                    <label className="block text-[10px] font-black uppercase tracking-[0.3em] text-blue-100/30 mb-4 px-2">
                        {t('branding.logo_label')}
                    </label>
                    <input
                        type="text"
                        value={logoUrl}
                        onChange={(e) => setLogoUrl(e.target.value)}
                        className="w-full bg-white/5 border border-white/10 rounded-2xl px-6 py-4 text-white placeholder-white/10 focus:outline-none focus:ring-2 focus:ring-cyan-500/50 transition-all font-medium"
                        placeholder={t('branding.logo_placeholder')}
                    />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div>
                        <label className="block text-[10px] font-black uppercase tracking-[0.3em] text-blue-100/30 mb-4 px-2">
                            {t('branding.primary_color')}
                        </label>
                        <div className="flex items-center space-x-4 bg-white/5 border border-white/10 rounded-2xl p-2 focus-within:ring-2 focus-within:ring-cyan-500/50 transition-all">
                            <input
                                type="color"
                                value={primaryColor}
                                onChange={(e) => setPrimaryColor(e.target.value)}
                                className="w-12 h-12 rounded-xl bg-transparent border-none cursor-pointer"
                            />
                            <input
                                type="text"
                                value={primaryColor}
                                onChange={(e) => setPrimaryColor(e.target.value)}
                                className="bg-transparent border-none text-white focus:outline-none font-mono text-sm"
                            />
                        </div>
                    </div>

                    <div>
                        <label className="block text-[10px] font-black uppercase tracking-[0.3em] text-blue-100/30 mb-4 px-2">
                            {t('branding.secondary_color')}
                        </label>
                        <div className="flex items-center space-x-4 bg-white/5 border border-white/10 rounded-2xl p-2 focus-within:ring-2 focus-within:ring-cyan-500/50 transition-all">
                            <input
                                type="color"
                                value={secondaryColor}
                                onChange={(e) => setSecondaryColor(e.target.value)}
                                className="w-12 h-12 rounded-xl bg-transparent border-none cursor-pointer"
                            />
                            <input
                                type="text"
                                value={secondaryColor}
                                onChange={(e) => setSecondaryColor(e.target.value)}
                                className="bg-transparent border-none text-white focus:outline-none font-mono text-sm"
                            />
                        </div>
                    </div>
                </div>

                {message && (
                    <div className={`p-4 rounded-2xl text-sm font-bold animate-in zoom-in-95 duration-300 ${message.type === 'success' ? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20' : 'bg-red-500/10 text-red-400 border border-red-500/20'
                        }`}>
                        {message.text}
                    </div>
                )}

                <button
                    type="submit"
                    disabled={loading}
                    className="w-full bg-white text-indigo-950 font-black py-4 rounded-2xl hover:bg-cyan-400 hover:scale-[1.02] active:scale-[0.98] transition-all disabled:opacity-50 disabled:hover:scale-100"
                >
                    {loading ? t('buttons.saving') : t('buttons.save')}
                </button>
            </form>
        </div>
    );
};

export default BrandingConfig;
