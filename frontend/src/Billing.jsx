import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import api from './api';

const PlanCard = ({ plan, currentPlan, onUpgrade, isPopular }) => {
  const isCurrentPlan = currentPlan === plan.name.toLowerCase();
  
  return (
    <div 
      className={`relative bg-white/5 border rounded-3xl p-8 backdrop-blur-md transition-all hover:bg-white/10 ${
        isPopular ? 'border-cyan-500/50 shadow-lg shadow-cyan-500/20' : 'border-white/10'
      } ${isCurrentPlan ? 'border-green-500/50' : ''}`}
    >
      {isPopular && !isCurrentPlan && (
        <div className="absolute -top-4 left-1/2 transform -translate-x-1/2">
          <span className="bg-gradient-to-r from-cyan-500 to-indigo-500 text-white text-xs font-black px-4 py-1 rounded-full">
            ⭐ MÁS POPULAR
          </span>
        </div>
      )}
      
      {isCurrentPlan && (
        <div className="absolute -top-4 left-1/2 transform -translate-x-1/2">
          <span className="bg-green-500 text-white text-xs font-black px-4 py-1 rounded-full">
            ✓ PLAN ACTUAL
          </span>
        </div>
      )}
      
      <div className="text-center mb-6">
        <h3 className="text-2xl font-black text-white mb-2">{plan.name}</h3>
        <div className="flex items-baseline justify-center gap-1">
          <span className="text-4xl font-black text-cyan-400">
            ${plan.price_monthly_usd}
          </span>
          <span className="text-blue-100/60 text-sm">/mes</span>
        </div>
        <p className="text-blue-100/40 text-xs mt-2">
          o ${plan.price_yearly_usd}/año (2 meses gratis)
        </p>
      </div>
      
      <div className="space-y-3 mb-8">
        <div className="flex items-center justify-between text-sm">
          <span className="text-blue-100/60">Usuarios</span>
          <span className="text-white font-bold">
            {plan.max_users ? `${plan.max_users} máx` : 'Ilimitado'}
          </span>
        </div>
        <div className="flex items-center justify-between text-sm">
          <span className="text-blue-100/60">Estudiantes</span>
          <span className="text-white font-bold">
            {plan.max_students ? `${plan.max_students} máx` : 'Ilimitado'}
          </span>
        </div>
      </div>
      
      <div className="border-t border-white/10 pt-6 mb-8">
        <h4 className="text-xs font-black uppercase tracking-widest text-blue-100/40 mb-4">
          Características
        </h4>
        <ul className="space-y-2">
          {plan.features.slice(0, 6).map((feature, idx) => (
            <li key={idx} className="flex items-start gap-2 text-sm">
              <span className={feature.included ? 'text-green-400' : 'text-red-400'}>
                {feature.included ? '✓' : '✗'}
              </span>
              <span className={feature.included ? 'text-blue-100/80' : 'text-blue-100/30'}>
                {feature.name}
              </span>
            </li>
          ))}
          {plan.features.length > 6 && (
            <li className="text-xs text-cyan-400 pt-2">
              + {plan.features.length - 6} características más
            </li>
          )}
        </ul>
      </div>
      
      <button
        onClick={() => onUpgrade(plan.name)}
        disabled={isCurrentPlan}
        className={`w-full py-3 rounded-xl font-black text-sm tracking-widest transition-all ${
          isCurrentPlan
            ? 'bg-gray-500/50 text-gray-400 cursor-not-allowed'
            : isPopular
            ? 'bg-gradient-to-r from-cyan-500 to-indigo-500 text-white hover:scale-105 shadow-lg shadow-cyan-500/20'
            : 'bg-white/10 text-white hover:bg-white/20'
        }`}
      >
        {isCurrentPlan ? 'Plan Actual' : plan.name === 'Enterprise' ? 'Contactar Ventas' : 'Comenzar Prueba'}
      </button>
    </div>
  );
};

const FeatureComparison = ({ plans }) => {
  const allFeatures = plans[0]?.features || [];
  
  return (
    <div className="bg-white/5 border border-white/10 rounded-3xl overflow-hidden backdrop-blur-md">
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-white/10">
              <th className="p-4 text-left text-xs font-black uppercase tracking-widest text-blue-100/40">
                Característica
              </th>
              {plans.map(plan => (
                <th key={plan.name} className="p-4 text-center">
                  <div className="text-lg font-black text-white">{plan.name}</div>
                  <div className="text-xs text-cyan-400">${plan.price_monthly_usd}/mes</div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5">
            {allFeatures.map((feature, idx) => (
              <tr key={idx} className="hover:bg-white/5 transition-colors">
                <td className="p-4">
                  <div className="text-sm font-bold text-white">{feature.name}</div>
                  <div className="text-xs text-blue-100/40">{feature.description}</div>
                </td>
                {plans.map(plan => {
                  const planFeature = plan.features.find(f => f.name === feature.name);
                  return (
                    <td key={plan.name} className="p-4 text-center">
                      <span className={`text-xl ${planFeature?.included ? 'text-green-400' : 'text-red-400/30'}`}>
                        {planFeature?.included ? '✓' : '—'}
                      </span>
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

function Billing() {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(true);
  const [plans, setPlans] = useState([]);
  const [currentPlan, setCurrentPlan] = useState(null);
  const [view, setView] = useState('plans'); // 'plans' o 'comparison'
  const [notification, setNotification] = useState(null);

  useEffect(() => {
    fetchPlans();
    fetchCurrentPlan();
  }, []);

  const fetchPlans = async () => {
    try {
      const data = await api.get('/billing/plans');
      setPlans(data);
    } catch (err) {
      console.error('Error fetching plans:', err);
    }
  };

  const fetchCurrentPlan = async () => {
    try {
      const data = await api.get('/billing/my-plan');
      setCurrentPlan(data.plan?.name?.toLowerCase());
    } catch (err) {
      console.error('Error fetching current plan:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleUpgrade = async (planName) => {
    if (planName === 'Enterprise') {
      setNotification({
        type: 'info',
        message: 'Para el plan Enterprise, contacta a ventas@schoolccb.com'
      });
      return;
    }

    setNotification({
      type: 'success',
      message: `¡Solicitud de upgrade a ${planName} recibida! Te redirigiremos a la pasarela de pago.`
    });
    
    // Aquí iría la integración con Stripe
    // window.location.href = `/api/billing/checkout?plan=${planName.toLowerCase()}`;
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-cyan-400 text-xl">Cargando planes...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-[#0a0f1e] via-[#0f172a] to-[#1e293b] text-white p-10">
      {/* Notification */}
      {notification && (
        <div className={`fixed top-8 right-8 z-[100] p-4 rounded-2xl shadow-2xl border animate-in slide-in-from-right-10 duration-500 ${
          notification.type === 'success'
            ? 'bg-cyan-500/10 border-cyan-500/50 text-cyan-400'
            : 'bg-indigo-500/10 border-indigo-500/50 text-indigo-400'
        }`}>
          <div className="flex items-center space-x-3">
            <span className="text-xl">{notification.type === 'success' ? '✅' : 'ℹ️'}</span>
            <span className="font-bold text-sm">{notification.message}</span>
          </div>
        </div>
      )}

      {/* Header */}
      <div className="max-w-7xl mx-auto mb-12">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h1 className="text-4xl font-black tracking-tight mb-2">
              Planes y Precios
            </h1>
            <p className="text-blue-100/50 text-sm">
              Elige el plan perfecto para tu institución educativa
            </p>
          </div>
          
          <div className="flex gap-2">
            <button
              onClick={() => setView('plans')}
              className={`px-4 py-2 rounded-xl text-sm font-bold transition-all ${
                view === 'plans'
                  ? 'bg-cyan-500/20 border border-cyan-500/50 text-cyan-400'
                  : 'bg-white/5 border border-white/10 text-blue-100/60 hover:bg-white/10'
              }`}
            >
              Planes
            </button>
            <button
              onClick={() => setView('comparison')}
              className={`px-4 py-2 rounded-xl text-sm font-bold transition-all ${
                view === 'comparison'
                  ? 'bg-cyan-500/20 border border-cyan-500/50 text-cyan-400'
                  : 'bg-white/5 border border-white/10 text-blue-100/60 hover:bg-white/10'
              }`}
            >
              Comparar
            </button>
          </div>
        </div>

        {/* Trial Banner */}
        <div className="bg-gradient-to-r from-cyan-600/20 via-indigo-600/20 to-purple-600/20 border border-cyan-500/30 rounded-3xl p-6 backdrop-blur-md">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-xl font-black text-white mb-1">
                🎉 14 días de prueba gratis
              </h3>
              <p className="text-blue-100/60 text-sm">
                Prueba cualquier plan sin compromiso. Sin tarjeta de crédito requerida.
              </p>
            </div>
            <div className="text-4xl">🚀</div>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="max-w-7xl mx-auto">
        {view === 'plans' ? (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {plans.map(plan => (
              <PlanCard
                key={plan.name}
                plan={plan}
                currentPlan={currentPlan}
                onUpgrade={handleUpgrade}
                isPopular={plan.popular}
              />
            ))}
          </div>
        ) : (
          <FeatureComparison plans={plans} />
        )}

        {/* FAQ Section */}
        <div className="mt-16">
          <h2 className="text-2xl font-black text-center mb-8">
            Preguntas Frecuentes
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {[
              {
                q: '¿Puedo cambiar de plan en cualquier momento?',
                a: 'Sí, puedes hacer upgrade inmediatamente. El downgrade aplica al siguiente ciclo.'
              },
              {
                q: '¿Hay descuento por pago anual?',
                a: 'Sí, obtienes 2 meses gratis al pagar anualmente en cualquier plan.'
              },
              {
                q: '¿Qué métodos de pago aceptan?',
                a: 'Tarjeta de crédito/débito, transferencia bancaria (Enterprise) y próximamente PayPal.'
              },
              {
                q: '¿Puedo cancelar mi suscripción?',
                a: 'Sí, puedes cancelar en cualquier momento. Tu cuenta permanecerá activa hasta el final del período.'
              }
            ].map((faq, idx) => (
              <div key={idx} className="bg-white/5 border border-white/10 rounded-2xl p-6">
                <h3 className="text-lg font-bold text-white mb-2">{faq.q}</h3>
                <p className="text-blue-100/60 text-sm">{faq.a}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

export default Billing;
