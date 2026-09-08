// Copia los WASM de sql.js desde node_modules a public/assets para que Vite los
// sirva en dev y los incluya en el build (dist/assets/).
// La app es autocontenida: NUNCA cargar sql.js desde CDN.
//
// OJO (bug 2.10.1): el import 'sql.js' que resuelve Vite usa el build "browser"
// (dist/sql-wasm-browser.js), que pide `sql-wasm-browser.wasm`. Copiar SOLO
// sql-wasm.wasm provocaba fetch 404 -> initSqlJs() lanzaba -> pantalla de
// recuperación con la BD intacta. Copiamos ambos para cubrir cualquier build.
import { copyFileSync, mkdirSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = resolve(__dirname, '../node_modules/sql.js/dist');
const destDir = resolve(__dirname, '../public/assets');
mkdirSync(destDir, { recursive: true });
for (const file of ['sql-wasm.wasm', 'sql-wasm-browser.wasm']) {
    copyFileSync(resolve(srcDir, file), resolve(destDir, file));
    console.log(`✓ ${file} copiado a public/assets/`);
}
