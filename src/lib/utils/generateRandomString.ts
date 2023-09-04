export function generateRandomString(length = 20) {
	const chars = 'abcdefghijklmnopqrstuvwxyz';
	return Array(length)
		.fill('')
		.map(() => chars[Math.floor(Math.random() * chars.length)])
		.join('');
}
