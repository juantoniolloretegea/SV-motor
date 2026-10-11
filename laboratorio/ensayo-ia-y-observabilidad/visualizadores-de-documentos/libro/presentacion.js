/* Presentación humana: no modifica originales ni ejerce controles del SV. */
(() => {
  'use strict';
  const caja = document.querySelector('.sv-language-tools');
  const estado = document.getElementById('sv-language-status');
  if (!caja || !estado) return;
  const actual = new URL(location.href);
  const original = new URL(actual); original.searchParams.delete('itvia_lang');
  document.getElementById('sv-original').href = original.href;
  document.querySelectorAll('pre,code,.math,.sv-language-tools,.sv-library-nav').forEach(e => {
    e.classList.add('notranslate'); e.setAttribute('translate','no');
  });
  if (!caja.dataset.worker) {
    if (actual.searchParams.has('itvia_lang')) {
      caja.open = true;
      estado.textContent = 'La traducción no está disponible en esta respuesta. Se conserva el original en español.';
    }
    return;
  }
  const idioma = caja.dataset.idioma;
  if (!idioma) {
    estado.textContent = 'Original en español. Seleccione un idioma en el selector de la matriz editorial.';
    return;
  }
  caja.open = true;
  document.querySelectorAll('a[href]').forEach(a => {
    const u = new URL(a.href, actual);
    if (u.origin === actual.origin && !u.pathname.startsWith('/libros/') &&
        (u.pathname.endsWith('.html') || !u.pathname.split('/').pop().includes('.')) && a.id !== 'sv-original') {
      u.searchParams.set('itvia_lang', idioma); a.href = u.href;
    }
  });
  // Cookie de este subdominio: no cambia preferencias de los demás portales.
  document.cookie = 'googtrans=/es/' + idioma + '; path=/; SameSite=Lax' + (actual.protocol === 'https:' ? '; Secure' : '');
  let iniciado = false;
  window.googleTranslateElementInit = () => {
    try {
      new window.google.translate.TranslateElement({pageLanguage:'es',includedLanguages:idioma,autoDisplay:false}, 'google_translate_element');
      iniciado = true;
      estado.textContent = 'Servicio de traducción cargado. Contraste términos, fórmulas y sentido con el original en español.';
    } catch (_) { estado.textContent = 'No se pudo iniciar la traducción. El original sigue disponible.'; }
  };
  const script = document.createElement('script');
  script.src = 'https://translate.google.com/translate_a/element.js?cb=googleTranslateElementInit';
  script.async = true;
  script.onerror = () => { estado.textContent = 'El traductor no ha respondido. El original sigue disponible.'; };
  document.head.append(script);
  setTimeout(() => { if (!iniciado) estado.textContent = 'No se ha confirmado la carga del traductor. El original sigue disponible.'; },15000);
})();
/* © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0). */
