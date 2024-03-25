/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			keyframes: {
				shimmer: {
					'100%': {
						transform: 'translateX(100%)'
					}
				}
			},
			colors: {
				overlay: {
					dark: '#000a',
					light: ''
				},
				brandColor: '#028090',
				ringColor: 'var(--m3-neutral-89)',
				primary: {
					light: 'var(--m3-primary-40)',
					dark: 'var(--m3-primary-80)'
				},
				secondary: {
					light: 'var(--m3-secondary-40)',
					dark: 'var(--m3-secondary-80)'
				},
				outline: {
					light: 'var(--m3-neutral-50)',
					dark: 'var(--m3-neutral-60)'
				},
				primaryContainer: {
					light: 'var(--m3-primary-90)',
					dark: 'var(--m3-primary-30)',
					hover: {
						light: 'var(--m3-primary-80)',
						dark: 'var(--m3-primary-40)'
					}
				},
				secondaryContainer: {
					light: 'var(--m3-secondary-90)',
					dark: 'var(--m3-secondary-30)',
					hover: {
						light: 'var(--m3-secondary-80)',
						dark: 'var(--m3-secondary-40)'
					}
				},
				outline: {
					light: 'var(--m3-neutralVariant-50)',
					dark: 'var(--m3-neutralVariant-60)'
				},
				onPrimaryContainer: {
					light: 'var(--m3-primary-10)',
					dark: 'var(--m3-primary-90)'
				},
				onSecondaryContainer: {
					light: 'var(--m3-secondary-10)',
					dark: 'var(--m3-secondary-90)'
				},
				errorContainer: {
					light: 'var(--m3-error-90)',
					dark: 'var(--m3-error-30)',
					hover: {
						light: 'var(--m3-error-80)',
						dark: 'var(--m3-error-40)'
					}
				},
				onErrorContainer: {
					light: 'var(--m3-error-10)',
					dark: 'var(--m3-error-90)'
				},
				containers: {
					0: {
						light: 'var(--m3-neutral-98)',
						dark: 'var(--m3-neutral-6)'
					},
					1: {
						light: 'var(--m3-neutral-100)',
						dark: 'var(--m3-neutral-4)'
					},
					2: {
						light: 'var(--m3-neutral-96)',
						dark: 'var(--m3-neutral-10)'
					},
					3: {
						light: 'var(--m3-neutral-94)',
						dark: 'var(--m3-neutral-12)'
					},
					4: {
						light: 'var(--m3-neutral-92',
						dark: 'var(--m3-neutral-16)'
					},
					5: {
						light: 'var(--m3-neutral-90)',
						dark: 'var(--m3-neutral-20)'
					},
					6: {
						light: 'var(--m3-neutral-86)',
						dark: 'var(--m3-neutral-28)'
					}
				}
			}
		}
	},
	plugins: []
};
