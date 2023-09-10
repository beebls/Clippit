import {
	TonalPalette,
	argbFromHex,
	themeFromSourceColor
} from '@material/material-color-utilities';

export function generateMaterialPalette(color: string) {
	// palettes2 = [];
	// palettes = [];
	if (color[0] !== '#') return;
	if (color.length !== 7) return;
	localStorage.setItem('materialYouBaseColor', color);
	let cssStr = ':root {';
	document.querySelectorAll('#material-you-colors').forEach((e) => e.remove());

	const theme = themeFromSourceColor(argbFromHex(color));
	const paletteRoots = Object.keys(theme.palettes);
	paletteRoots.forEach((e) => {
		// @ts-ignore
		const initValue = theme.palettes[e];
		let palette2: { color: string; tone: number }[] = [];
		let palette: string[] = [];
		for (let i = 0; i < 101; i = i + 1) {
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
