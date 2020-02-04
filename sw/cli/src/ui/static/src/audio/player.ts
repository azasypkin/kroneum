export enum Note {
  C0 = 0x10,
  CSharp0 = 0x20,
  D0 = 0x30,
  DSharp0 = 0x40,
  E0 = 0x50,
  F0 = 0x60,
  FSharp0 = 0x70,
  G0 = 0x80,
  GSharp0 = 0x90,
  A0 = 0xa0,
  ASharp0 = 0xb0,
  B0 = 0xc0,

  C1 = 0x11,
  CSharp1 = 0x21,
  D1 = 0x31,
  DSharp1 = 0x41,
  E1 = 0x51,
  F1 = 0x61,
  FSharp1 = 0x71,
  G1 = 0x81,
  GSharp1 = 0x91,
  A1 = 0xa1,
  ASharp1 = 0xb1,
  B1 = 0xc1,

  C2 = 0x12,
  CSharp2 = 0x22,
  D2 = 0x32,
  DSharp2 = 0x42,
  E2 = 0x52,
  F2 = 0x62,
  FSharp2 = 0x72,
  G2 = 0x82,
  GSharp2 = 0x92,
  A2 = 0xa2,
  ASharp2 = 0xb2,
  B2 = 0xc2,

  C3 = 0x13,
  CSharp3 = 0x23,
  D3 = 0x33,
  DSharp3 = 0x43,
  E3 = 0x53,
  F3 = 0x63,
  FSharp3 = 0x73,
  G3 = 0x83,
  GSharp3 = 0x93,
  A3 = 0xa3,
  ASharp3 = 0xb3,
  B3 = 0xc3,

  C4 = 0x14,
  CSharp4 = 0x24,
  D4 = 0x34,
  DSharp4 = 0x44,
  E4 = 0x54,
  F4 = 0x64,
  FSharp4 = 0x74,
  G4 = 0x84,
  GSharp4 = 0x94,
  A4 = 0xa4,
  ASharp4 = 0xb4,
  B4 = 0xc4,

  C5 = 0x15,
  CSharp5 = 0x25,
  D5 = 0x35,
  DSharp5 = 0x45,
  E5 = 0x55,
  F5 = 0x65,
  FSharp5 = 0x75,
  G5 = 0x85,
  GSharp5 = 0x95,
  A5 = 0xa5,
  ASharp5 = 0xb5,
  B5 = 0xc5,

  C6 = 0x16,
  CSharp6 = 0x26,
  D6 = 0x36,
  DSharp6 = 0x46,
  E6 = 0x56,
  F6 = 0x66,
  FSharp6 = 0x76,
  G6 = 0x86,
  GSharp6 = 0x96,
  A6 = 0xa6,
  ASharp6 = 0xb6,
  B6 = 0xc6,

  C7 = 0x17,
  CSharp7 = 0x27,
  D7 = 0x37,
  DSharp7 = 0x47,
  E7 = 0x57,
  F7 = 0x67,
  FSharp7 = 0x77,
  G7 = 0x87,
  GSharp7 = 0x97,
  A7 = 0xa7,
  ASharp7 = 0xb7,
  B7 = 0xc7,

  C8 = 0x18,
  CSharp8 = 0x28,
  D8 = 0x38,
  DSharp8 = 0x48,
  E8 = 0x58,
  F8 = 0x68,
  FSharp8 = 0x78,
  G8 = 0x88,
  GSharp8 = 0x98,
  A8 = 0xa8,
  ASharp8 = 0xb8,
  B8 = 0xc8,

  Silence = 0x00,
}

const NOTES = new Map<Note, number>([
  [Note.C0, 16.35],
  [Note.CSharp0, 17.32],
  [Note.D0, 18.35],
  [Note.DSharp0, 19.45],
  [Note.E0, 20.6],
  [Note.F0, 21.83],
  [Note.FSharp0, 23.12],
  [Note.G0, 24.5],
  [Note.GSharp0, 25.96],
  [Note.A0, 27.5],
  [Note.ASharp0, 29.14],
  [Note.B0, 30.87],
  [Note.C1, 32.7],
  [Note.CSharp1, 34.65],
  [Note.D1, 36.71],
  [Note.DSharp1, 38.89],
  [Note.E1, 41.2],
  [Note.F1, 43.65],
  [Note.FSharp1, 46.25],
  [Note.G1, 49.0],
  [Note.GSharp1, 51.91],
  [Note.A1, 55.0],
  [Note.ASharp1, 58.27],
  [Note.B1, 61.74],
  [Note.C2, 65.41],
  [Note.CSharp2, 69.3],
  [Note.D2, 73.42],
  [Note.DSharp2, 77.78],
  [Note.E2, 82.41],
  [Note.F2, 87.31],
  [Note.FSharp2, 92.5],
  [Note.G2, 98.0],
  [Note.GSharp2, 103.83],
  [Note.A2, 110.0],
  [Note.ASharp2, 116.54],
  [Note.B2, 123.47],
  [Note.C3, 130.81],
  [Note.CSharp3, 138.59],
  [Note.D3, 146.83],
  [Note.DSharp3, 155.56],
  [Note.E3, 164.81],
  [Note.F3, 174.61],
  [Note.FSharp3, 185.0],
  [Note.G3, 196.0],
  [Note.GSharp3, 207.65],
  [Note.A3, 220.0],
  [Note.ASharp3, 233.08],
  [Note.B3, 246.94],
  [Note.C4, 261.63],
  [Note.CSharp4, 277.18],
  [Note.D4, 293.66],
  [Note.DSharp4, 311.13],
  [Note.E4, 329.63],
  [Note.F4, 349.23],
  [Note.FSharp4, 369.99],
  [Note.G4, 392.0],
  [Note.GSharp4, 415.3],
  [Note.A4, 440.0],
  [Note.ASharp4, 466.16],
  [Note.B4, 493.88],
  [Note.C5, 523.25],
  [Note.CSharp5, 554.37],
  [Note.D5, 587.33],
  [Note.DSharp5, 622.25],
  [Note.E5, 659.26],
  [Note.F5, 698.46],
  [Note.FSharp5, 739.99],
  [Note.G5, 783.99],
  [Note.GSharp5, 830.61],
  [Note.A5, 880.0],
  [Note.ASharp5, 932.33],
  [Note.B5, 987.77],
  [Note.C6, 1046.5],
  [Note.CSharp6, 1108.73],
  [Note.D6, 1174.66],
  [Note.DSharp6, 1244.51],
  [Note.E6, 1318.51],
  [Note.F6, 1396.91],
  [Note.FSharp6, 1479.98],
  [Note.G6, 1567.98],
  [Note.GSharp6, 1661.22],
  [Note.A6, 1760.0],
  [Note.ASharp6, 1864.66],
  [Note.B6, 1975.53],
  [Note.C7, 2093.0],
  [Note.CSharp7, 2217.46],
  [Note.D7, 2349.32],
  [Note.DSharp7, 2489.02],
  [Note.E7, 2637.02],
  [Note.F7, 2793.83],
  [Note.FSharp7, 2959.96],
  [Note.G7, 3135.96],
  [Note.GSharp7, 3322.44],
  [Note.A7, 3520.0],
  [Note.ASharp7, 3729.31],
  [Note.B7, 3951.07],
  [Note.C8, 4186.01],
]);

const BeatsPerMinute = 150;
const SecondsInMinute = 60;

export class Player {
  public static play(melody: Array<[Note, number]>) {
    // 1 second divided by number of beats per second times number of beats (length of a note)
    const durationCoefficient = 1 / (BeatsPerMinute / SecondsInMinute);

    const context = new AudioContext();
    let time = context.currentTime;
    for (const [note, duration] of melody) {
      const oscillator = context.createOscillator();
      oscillator.type = 'square';
      oscillator.frequency.value = NOTES.get(note) || 0;
      oscillator.start(time);

      const playDuration = durationCoefficient * duration;
      oscillator.stop(time + playDuration);

      time += playDuration;

      oscillator.connect(context.destination);
    }
  }
}