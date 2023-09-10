export function secondsToMinuteString(seconds: number) {
	const rounded = Math.round(seconds);
	const minutes = Math.floor(rounded / 60) || 0;
	const secondsNum = rounded % 60 || 0;
	const secondsStr = secondsNum.toString().padStart(2, '0');

	return minutes + ':' + secondsStr;
}
