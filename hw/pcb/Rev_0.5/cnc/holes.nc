(1001)
(T2  D=0.9 CR=0. TAPER=130deg - ZMIN=-2.01 - drill)
G90
G0 G53 Z0.

(Drill (usb vias))
T2 M6 (drill D=0.9 0.9 mm Drill bit - Fiberglass)
S12000 M3
G61
G54
G43 H2
G0 X-34.45 Y7.575
Z15.
Z5.
G98 G81 X-34.45 Y7.575 Z-2.01 R5. F80
X-32.55 Y7.25
X-22.775 Y19.175
X-20.825
G80
Z15.
M5
G53 Z0.
M30
