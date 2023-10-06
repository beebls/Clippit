import {
	TonalPalette,
	argbFromHex,
	themeFromSourceColor,
	type Theme
} from '@material/material-color-utilities';

export function generateMaterialPalette(color: string | number[]) {
	let cssStr = ':root {';
	document.querySelectorAll('#material-you-colors').forEach((e) => e.remove());
	// Eventually want to have it parse rgb n stuff
	let theme: Theme | undefined;
	if (typeof color === 'string') {
		if (color[0] !== '#') return;
		if (color.length !== 7) return;
		localStorage.setItem('materialYouBaseColor', color);

		theme = themeFromSourceColor(argbFromHex(color));
	} else if (typeof color?.[0] === 'number') {
		let r = color[0].toString(16).padStart(2, '0');
		let g = color[1].toString(16).padStart(2, '0');
		let b = color[2].toString(16).padStart(2, '0');
		theme = themeFromSourceColor(argbFromHex(`#${r}${g}${b}`));
	}
	if (!theme) return;

	const paletteRoots = Object.keys(theme.palettes);
	paletteRoots.forEach((e) => {
		// @ts-ignore
		const initValue = theme.palettes[e];
		let palette2: { color: string; tone: number }[] = [];
		let palette: string[] = [];
		for (let i = 0; i < 101; i = i + 2) {
			const color = TonalPalette.fromHct(initValue).tone(i).toString(16).slice(2);
			palette2.push({ tone: i, color: '#' + color });
			palette[i] = '#' + color;
			cssStr = cssStr + `--m3-${e}-${i}: #${color}; `;
		}
		// palettes[e] = palette;
		// palettes2 = [...palettes2, { name: e, values: palette2 }];
	});

	cssStr = cssStr + '}';
	const styleElem = document.createElement('style');
	styleElem.id = 'material-you-colors';
	styleElem.innerHTML = cssStr;
	document.head.appendChild(styleElem);
}
