
import React from 'react';
import ReactDOM from 'react-dom/client';
// Fuente Inter autocontenida (solo subset latin): sin Google Fonts en runtime.
import '@fontsource/inter/latin-400.css';
import '@fontsource/inter/latin-500.css';
import '@fontsource/inter/latin-600.css';
import '@fontsource/inter/latin-700.css';
import App from './App';
import { invoke } from '@tauri-apps/api/core';

const AUTO_RETRY_KEY = 'cuaderno_ui_recovery_attempted';

// v2.10.1: si la UI arranca SIN estilos (caché WebKit corrupta del .deb anterior
// que servía el index.html con CDN de Tailwind), limpiar la caché y recargar una
// sola vez por sesión. Con el build autocontenido esto no debería ocurrir, pero
// es la red de seguridad que evita quedarse con la UI "en vertical" para siempre.
async function recoverBrokenUI() {
    if (!('__TAURI_INTERNALS__' in window)) return;
    if (sessionStorage.getItem(AUTO_RETRY_KEY)) return;
    // Esperar a que la página termine de cargar y el CSS local esté aplicado.
    // (2 frames eran insuficientes en WebKitGTK con caché fría: daba falsos
    // positivos de "UI rota" y forzaba limpieza de caché + reload innecesarios).
    await new Promise<void>((r) => {
        if (document.readyState === 'complete') return r();
        window.addEventListener('load', () => r(), { once: true });
    });
    await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
    await new Promise((r) => setTimeout(r, 300));
    const probe = document.createElement('div');
    probe.className = 'bg-neutral-100';
    probe.style.cssText = 'position:absolute;left:-9999px;top:-9999px;width:10px;height:10px';
    document.body.appendChild(probe);
    const bg = getComputedStyle(probe).backgroundColor;
    document.body.removeChild(probe);
    const cssApplied = bg === 'rgb(245, 245, 245)'; // Tailwind neutral-100 = #f5f5f5
    if (cssApplied) return;
    sessionStorage.setItem(AUTO_RETRY_KEY, '1');
    try {
        await invoke('clear_webkit_cache');
        window.location.reload();
    } catch {
        // Si falla, seguir con la UI actual (el usuario puede reinstalar/limpiar).
    }
}

void recoverBrokenUI();

const rootElement = document.getElementById('root');
if (!rootElement) {
  throw new Error("Could not find root element to mount to");
}

const root = ReactDOM.createRoot(rootElement);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
