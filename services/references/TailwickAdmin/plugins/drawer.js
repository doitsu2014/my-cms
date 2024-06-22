const plugin = require('tailwindcss/plugin')

module.exports = plugin(function ({ addComponents }) {
    addComponents({
        '[drawer-end]': {
            '&.show': {
                '@apply ltr:translate-x-full rtl:-translate-x-full': {},
            }
        },
        '[drawer-start]': {
            '&.show': {
                '@apply rtl:translate-x-full ltr:-translate-x-full': {},
            }
        },
        '[drawer-bottom]': {
            '&.show': {
                '@apply translate-y-full': {},
            }
        },
        '[drawer-top]': {
            '&.show': {
                '@apply -translate-y-full': {},
            }
        },
        '[modal-center]': {
            '@apply opacity-100 top-1/2': {},

            '&.show': {
                '@apply opacity-0': {},
                top: '45%',
            }
        },
        '[modal-top]': {
            '@apply opacity-100': {},
            top: '4%',
            '&.show': {
                '@apply opacity-0 top-0': {},
            }
        },
        '[modal-bottom]': {
            bottom: '4%',
            '@apply opacity-100': {},
            '&.show': {
                '@apply opacity-0 bottom-0': {},
            }
        }
    })
})