const audioContext = new AudioContext();

let audioElements: HTMLAudioElement[] = [];
let gainNodes: { id: string; node: GainNode }[] = [];

let masterGainNode: GainNode = new GainNode(audioContext);
masterGainNode.connect(audioContext.destination);

export function setGlobalVolume(volume: number) {
	masterGainNode.gain.setValueAtTime(volume, masterGainNode.context.currentTime + 0.001);
}

export function connectElement(querySelector: string): GainNode {
	let audioElement = document.querySelector(querySelector) as HTMLAudioElement;
	if (audioElements.map((e) => e.id).includes(audioElement.id)) {
		audioElements = audioElements.filter((e) => e.id !== audioElement.id);
		gainNodes = gainNodes.filter((e) => e.id !== audioElement.id);
	}

	audioElements.push(audioElement);

	let gainNode = new GainNode(audioContext);
	gainNodes.push({ id: audioElement.id, node: gainNode });

	let track = audioContext.createMediaElementSource(audioElement);
	track.connect(gainNode);

	gainNode.connect(masterGainNode);
	return gainNode;
}

export function play() {
	audioElements.forEach((e) => e.play());
}
export function pause() {
	audioElements.forEach((e) => e.pause());
}
export function seek(time: number) {
	audioElements.forEach((e) => (e.currentTime = time));
}

export function reset() {
	pause();
	audioElements = [];
	gainNodes = [];
}
