(1001)
(T7  D=1.5 CR=0. - ZMIN=-2. - flat end mill)
G90
G0 G53 Z0.

(2D Pocket (slide))
T7 M6 (flat end mill D=1.5 1.5 mm Fishtail Upcut Endmill - Acrylic Max Pass Depth 0.8 mm)
S16000 M3
G64
G54
G43 H7
G0 X41.799 Y23.446
Z15.
Z5.
G1 Z1.8 F600
Z-0.65
X41.807 Y23.447 Z-0.696
X41.828 Y23.448 Z-0.738
X41.861 Y23.451 Z-0.771
X41.903 Y23.454 Z-0.793
X41.949 Y23.458 Z-0.8
G3 X42.45 Y24. I-0.043 J0.542
G1 Y45.
G3 X42.2 Y45.25 I-0.25 J0.
G1 X3.
G3 X2.75 Y45. I0. J-0.25
G1 Y3.
G3 X3. Y2.75 I0.25 J0.
G1 X42.2
G3 X42.45 Y3. I0. J0.25
G1 Y24.
Y45. Z-1.533
G3 X42.2 Y45.25 Z-1.547 I-0.25 J0.
G1 X40.684 Z-1.6
X3.
G3 X2.75 Y45. I0. J-0.25
G1 Y3.
G3 X3. Y2.75 I0.25 J0.
G1 X42.2
G3 X42.45 Y3. I0. J0.25
G1 Y45.
G3 X42.2 Y45.25 I-0.25 J0.
G1 X40.684
X29.229 Z-2.
X3.
G3 X2.75 Y45. I0. J-0.25
G1 Y3.
G3 X3. Y2.75 I0.25 J0.
G1 X42.2
G3 X42.45 Y3. I0. J0.25
G1 Y45.
G3 X42.2 Y45.25 I-0.25 J0.
G1 X29.229
X29.19 Y45.245
X29.154 Y45.23
X29.123 Y45.206
X28.593 Y44.676
X28.56 Y44.643 Z-1.993
X28.53 Y44.613 Z-1.971
X28.507 Y44.59 Z-1.938
X28.492 Y44.575 Z-1.896
X28.487 Y44.57 Z-1.85
G0 Z15.
M5
G53 Z0.
M30