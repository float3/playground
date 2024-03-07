export class Tone {
  index: number;
  freq: number;
  cents: number;
  name: string;
  Oscillator: OscillatorNode;
  constructor(
    index: number,
    freq: number,
    cents: number,
    name: string,
    node: OscillatorNode,
  ) {
    console.log("Tone");
    this.index = index;
    this.freq = freq;
    this.cents = cents;
    this.name = name;
    this.Oscillator = node;
  }
}

declare global {
  interface Window {
    createTone: (
      index: number,
      freq: number,
      cents: number,
      name: string,
      oscillator: OscillatorNode,
    ) => Tone;
  }
}

export function createTone(
  index: number,
  freq: number,
  cents: number,
  name: string,
  oscillator: OscillatorNode,
): Tone {
  console.log("createTone");
  return new Tone(index, freq, cents, name, oscillator);
}
