import React, { useEffect } from 'react';

const Modal = ({ isOpen, onClose, title, children }) => {
    useEffect(() => {
        if (isOpen) {
            document.body.style.overflow = 'hidden';
        } else {
            document.body.style.overflow = 'unset';
        }
        return () => { document.body.style.overflow = 'unset'; };
    }, [isOpen]);

    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            <div
                className="absolute inset-0 bg-black/60 backdrop-blur-sm animate-in fade-in duration-300"
                onClick={onClose}
            ></div>
            <div className="relative w-full max-w-lg bg-[#111827] border border-white/10 rounded-[2.5rem] shadow-2xl p-8 animate-in zoom-in-95 duration-300 overflow-hidden">
                {/* Decorative background Elements */}
                <div className="absolute top-0 right-0 w-32 h-32 bg-cyan-500/10 blur-3xl rounded-full -translate-y-1/2 translate-x-1/2"></div>

                <header className="flex justify-between items-center mb-8">
                    <h3 className="text-2xl font-black tracking-tight">{title}</h3>
                    <button
                        onClick={onClose}
                        className="w-10 h-10 flex items-center justify-center bg-white/5 hover:bg-white/10 rounded-xl border border-white/10 transition-all active:scale-90"
                    >
                        ✕
                    </button>
                </header>

                <div className="relative z-10">
                    {children}
                </div>
            </div>
        </div>
    );
};

export default Modal;
