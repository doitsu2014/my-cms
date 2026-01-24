/**
 * Initialize highlight.js globally before the app loads
 * This file is loaded as a preEntry in rsbuild.config.ts
 * to ensure highlight.js is available on window object before Quill initializes
 */

import hljs from 'highlight.js';
import 'highlight.js/styles/atom-one-dark.css';

// Expose highlight.js globally for Quill's Syntax module
// Must be done before any Quill imports in the application
(window as any).hljs = hljs;

console.log('highlight.js initialized globally:', !!(window as any).hljs);
