const plugin = require('tailwindcss/plugin')

module.exports = plugin(function ({ addComponents }) {
    addComponents({
        '.glightbox-clean ': {
            '.gslide-description': {
                '@apply bg-white dark:bg-zink-600': {},
            },

            '.gslide-title': {
                '@apply text-slate-800 dark:text-zink-50 mb-4': {},
            },
        },

        'button.svelte-1a1bwoj': {
            '@apply w-full': {},
        }
    })
})