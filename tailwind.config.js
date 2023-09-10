/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
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
				onPrimaryContainer: {
					light: 'var(--m3-primary-10)',
					dark: 'var(--m3-neutral-97)'
				},
				containers: {
					0: {
						light: 'var(--m3-neutral-97)',
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
						light: 'var(--m3-neutral-91)',
						dark: 'var(--m3-neutral-16)'
					},
					5: {
						light: 'var(--m3-neutral-89)',
						dark: 'var(--m3-neutral-21)'
					},
					6: {
						light: 'var(--m3-neutral-86)',
						dark: 'var(--m3-neutral-27)'
					}
				}
			}
		}
	},
	plugins: []
};
