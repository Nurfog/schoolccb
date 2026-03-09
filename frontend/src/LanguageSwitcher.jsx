import React from 'react';
import { useTranslation } from 'react-i18next';

const LanguageSwitcher = () => {
    const { i18n } = useTranslation();
    const current = i18n.language;

    const toggle = (lang) => {
        i18n.changeLanguage(lang);
        localStorage.setItem('lang', lang);
    };

    return (
        <div className="flex items-center space-x-1 bg-white/5 border border-white/10 rounded-xl p-1">
            <button
                onClick={() => toggle('es')}
                className={`px-3 py-1.5 rounded-lg text-xs font-black tracking-widest transition-all ${current === 'es'
                        ? 'bg-brand-primary text-indigo-950 shadow-lg shadow-brand-primary/20'
                        : 'text-blue-100/40 hover:text-white'
                    }`}
            >
                ES
            </button>
            <button
                onClick={() => toggle('en')}
                className={`px-3 py-1.5 rounded-lg text-xs font-black tracking-widest transition-all ${current === 'en'
                        ? 'bg-brand-primary text-indigo-950 shadow-lg shadow-brand-primary/20'
                        : 'text-blue-100/40 hover:text-white'
                    }`}
            >
                EN
            </button>
        </div>
    );
};

export default LanguageSwitcher;
