EESchema Schematic File Version 4
LIBS:kroneum-cache
EELAYER 29 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L MCU_ST_STM32F0:STM32F042F4Px U1
U 1 1 5C1E98D0
P 2600 3450
F 0 "U1" V 2600 3400 50  0000 R CNN
F 1 "STM32F042F4Px" V 2500 3650 50  0000 R CNN
F 2 "tssop-20x6.5mmx4:TSSOP-20_4.4x6.5mm_P0.65mm" H 2100 2750 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00105814.pdf" H 2600 3450 50  0001 C CNN
	1    2600 3450
	0    1    1    0   
$EndComp
$Comp
L Switch:SW_Push SW2
U 1 1 5C1E9C39
P 6000 3400
F 0 "SW2" H 6000 3685 50  0000 C CNN
F 1 "Ctrl_One_Btn" H 6000 3594 50  0000 C CNN
F 2 "tl3342:tl3342" H 6000 3600 50  0001 C CNN
F 3 "" H 6000 3600 50  0001 C CNN
	1    6000 3400
	1    0    0    -1  
$EndComp
$Comp
L Connector:USB_B_Micro J1
U 1 1 5C1E9F15
P 5700 2300
F 0 "J1" H 5755 2767 50  0000 C CNN
F 1 "USB_B_Micro" H 5755 2676 50  0000 C CNN
F 2 "micro_usb:USB_Micro-B_Molex-105017-0001" H 5850 2250 50  0001 C CNN
F 3 "~" H 5850 2250 50  0001 C CNN
	1    5700 2300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3300 3250 3350 3250
NoConn ~ 6000 2500
$Comp
L power:GND #PWR01
U 1 1 5C1EDFBA
P 5700 2700
F 0 "#PWR01" H 5700 2450 50  0001 C CNN
F 1 "GND" H 5705 2527 50  0000 C CNN
F 2 "" H 5700 2700 50  0001 C CNN
F 3 "" H 5700 2700 50  0001 C CNN
	1    5700 2700
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR02
U 1 1 5C1EE186
P 1800 3250
F 0 "#PWR02" H 1800 3000 50  0001 C CNN
F 1 "GND" H 1805 3077 50  0000 C CNN
F 2 "" H 1800 3250 50  0001 C CNN
F 3 "" H 1800 3250 50  0001 C CNN
	1    1800 3250
	1    0    0    -1  
$EndComp
$Comp
L Device:C C7
U 1 1 5C1EFF71
P 7250 3700
F 0 "C7" V 7502 3700 50  0000 C CNN
F 1 "0.1u" V 7411 3700 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7288 3550 50  0001 C CNN
F 3 "~" H 7250 3700 50  0001 C CNN
	1    7250 3700
	0    -1   -1   0   
$EndComp
$Comp
L Switch:SW_Push SW3
U 1 1 5C1EFD70
P 7250 3400
F 0 "SW3" H 7250 3685 50  0000 C CNN
F 1 "Reset_Btn" H 7250 3594 50  0000 C CNN
F 2 "tl3342:tl3342" H 7250 3600 50  0001 C CNN
F 3 "" H 7250 3600 50  0001 C CNN
	1    7250 3400
	1    0    0    -1  
$EndComp
Wire Wire Line
	7050 3400 7050 3700
Wire Wire Line
	7050 3700 7100 3700
Wire Wire Line
	7400 3700 7450 3700
Wire Wire Line
	7450 3700 7450 3400
Wire Wire Line
	7450 3400 7600 3400
Connection ~ 7450 3400
$Comp
L power:GND #PWR013
U 1 1 5C1FDD8F
P 7600 3400
F 0 "#PWR013" H 7600 3150 50  0001 C CNN
F 1 "GND" H 7605 3227 50  0000 C CNN
F 2 "" H 7600 3400 50  0001 C CNN
F 3 "" H 7600 3400 50  0001 C CNN
	1    7600 3400
	1    0    0    -1  
$EndComp
$Comp
L Device:C C1
U 1 1 5C202927
P 6700 2250
F 0 "C1" H 6815 2296 50  0000 L CNN
F 1 "0.1u" H 6815 2205 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6738 2100 50  0001 C CNN
F 3 "~" H 6700 2250 50  0001 C CNN
	1    6700 2250
	1    0    0    -1  
$EndComp
$Comp
L Device:C C3
U 1 1 5C20292E
P 7200 2250
F 0 "C3" H 7315 2296 50  0000 L CNN
F 1 "4.7u" H 7315 2205 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7238 2100 50  0001 C CNN
F 3 "~" H 7200 2250 50  0001 C CNN
	1    7200 2250
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR08
U 1 1 5C202935
P 6700 2450
F 0 "#PWR08" H 6700 2200 50  0001 C CNN
F 1 "GND" H 6705 2277 50  0000 C CNN
F 2 "" H 6700 2450 50  0001 C CNN
F 3 "" H 6700 2450 50  0001 C CNN
	1    6700 2450
	1    0    0    -1  
$EndComp
Wire Wire Line
	6700 2450 6700 2400
Connection ~ 6700 2400
Wire Wire Line
	6700 2100 7200 2100
Wire Wire Line
	6700 2400 7200 2400
$Comp
L power:VDD #PWR07
U 1 1 5C203D31
P 6700 2100
F 0 "#PWR07" H 6700 1950 50  0001 C CNN
F 1 "VDD" H 6717 2273 50  0000 C CNN
F 2 "" H 6700 2100 50  0001 C CNN
F 3 "" H 6700 2100 50  0001 C CNN
	1    6700 2100
	1    0    0    -1  
$EndComp
Connection ~ 6700 2100
$Comp
L power:VDD #PWR03
U 1 1 5C204AB4
P 3350 3250
F 0 "#PWR03" H 3350 3100 50  0001 C CNN
F 1 "VDD" V 3367 3378 50  0000 L CNN
F 2 "" H 3350 3250 50  0001 C CNN
F 3 "" H 3350 3250 50  0001 C CNN
	1    3350 3250
	0    1    1    0   
$EndComp
Wire Wire Line
	3300 3350 3350 3350
$Comp
L Device:R R1
U 1 1 5C206A14
P 7050 4250
F 0 "R1" V 6843 4250 50  0000 C CNN
F 1 "470" V 6934 4250 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 6980 4250 50  0001 C CNN
F 3 "~" H 7050 4250 50  0001 C CNN
	1    7050 4250
	0    1    1    0   
$EndComp
$Comp
L Device:Buzzer BZ1
U 1 1 5C206E6D
P 7300 4350
F 0 "BZ1" H 7453 4379 50  0000 L CNN
F 1 "Buzzer" H 7453 4288 50  0000 L CNN
F 2 "Buzzer_Murata_PKMCS0909E4000-R1:Buzzer_Murata_PKMCS0909E4000-R1" V 7275 4450 50  0001 C CNN
F 3 "~" V 7275 4450 50  0001 C CNN
	1    7300 4350
	1    0    0    -1  
$EndComp
Text GLabel 2400 4050 3    50   Input ~ 0
Buzzer
Text GLabel 2300 4050 3    50   Input ~ 0
USB_D-
Text GLabel 2200 4050 3    50   Input ~ 0
USB_D+
Text GLabel 6000 2300 2    50   Input ~ 0
USB_D+
Text GLabel 6000 2400 2    50   Input ~ 0
USB_D-
$Comp
L power:GND #PWR011
U 1 1 5C207A2D
P 7200 4450
F 0 "#PWR011" H 7200 4200 50  0001 C CNN
F 1 "GND" H 7205 4277 50  0000 C CNN
F 2 "" H 7200 4450 50  0001 C CNN
F 3 "" H 7200 4450 50  0001 C CNN
	1    7200 4450
	1    0    0    -1  
$EndComp
Text GLabel 6900 4250 0    50   Input ~ 0
Buzzer
Text GLabel 7050 3400 0    50   Input ~ 0
NRST
Text GLabel 3100 2850 1    50   Input ~ 0
NRST
Text GLabel 2000 2850 1    50   Input ~ 0
BOOT0
$Comp
L power:GND #PWR020
U 1 1 5C20CB7E
P 4350 3700
F 0 "#PWR020" H 4350 3450 50  0001 C CNN
F 1 "GND" H 4355 3527 50  0000 C CNN
F 2 "" H 4350 3700 50  0001 C CNN
F 3 "" H 4350 3700 50  0001 C CNN
	1    4350 3700
	1    0    0    -1  
$EndComp
Text GLabel 4250 3400 0    50   Input ~ 0
BOOT0
Text GLabel 3100 4050 3    50   Input ~ 0
Ctrl_One
Text GLabel 2900 4050 3    50   Input ~ 0
Ctrl_Ten
NoConn ~ 2500 4050
NoConn ~ 2600 4050
NoConn ~ 3000 4050
NoConn ~ 2800 4050
NoConn ~ 2700 4050
NoConn ~ 2100 2850
$Comp
L Device:C C8
U 1 1 5C20FD6A
P 6000 3650
F 0 "C8" V 6250 3650 50  0000 C CNN
F 1 "0.1u" V 6150 3650 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6038 3500 50  0001 C CNN
F 3 "~" H 6000 3650 50  0001 C CNN
	1    6000 3650
	0    1    1    0   
$EndComp
Wire Wire Line
	6200 3400 6300 3400
Wire Wire Line
	6150 3650 6300 3650
Wire Wire Line
	6300 3650 6300 3400
Wire Wire Line
	5800 3400 5700 3400
Wire Wire Line
	5700 3400 5700 3650
Wire Wire Line
	5700 3650 5850 3650
Text GLabel 5700 3400 0    50   Input ~ 0
Ctrl_One
Wire Wire Line
	6300 3400 6500 3400
Connection ~ 6300 3400
$Comp
L Switch:SW_Push SW4
U 1 1 5C211C0A
P 8500 3400
F 0 "SW4" H 8500 3685 50  0000 C CNN
F 1 "Ctrl_Ten_Btn" H 8500 3594 50  0000 C CNN
F 2 "tl3342:tl3342" H 8500 3600 50  0001 C CNN
F 3 "" H 8500 3600 50  0001 C CNN
	1    8500 3400
	1    0    0    -1  
$EndComp
$Comp
L Device:C C9
U 1 1 5C211C11
P 8500 3650
F 0 "C9" V 8750 3650 50  0000 C CNN
F 1 "0.1u" V 8650 3650 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 8538 3500 50  0001 C CNN
F 3 "~" H 8500 3650 50  0001 C CNN
	1    8500 3650
	0    1    1    0   
$EndComp
Wire Wire Line
	8700 3400 8800 3400
Wire Wire Line
	8650 3650 8800 3650
Wire Wire Line
	8800 3650 8800 3400
Wire Wire Line
	8300 3400 8200 3400
Wire Wire Line
	8200 3400 8200 3650
Wire Wire Line
	8200 3650 8350 3650
Text GLabel 8200 3400 0    50   Input ~ 0
Ctrl_Ten
Wire Wire Line
	8800 3400 9000 3400
Connection ~ 8800 3400
$Comp
L power:VDD #PWR0102
U 1 1 5C2140A0
P 4950 3400
F 0 "#PWR0102" H 4950 3250 50  0001 C CNN
F 1 "VDD" H 4967 3573 50  0000 C CNN
F 2 "" H 4950 3400 50  0001 C CNN
F 3 "" H 4950 3400 50  0001 C CNN
	1    4950 3400
	1    0    0    -1  
$EndComp
$Comp
L power:VDD #PWR0103
U 1 1 5C22284D
P 5700 4150
F 0 "#PWR0103" H 5700 4000 50  0001 C CNN
F 1 "VDD" V 5700 4350 50  0000 C CNN
F 2 "" H 5700 4150 50  0001 C CNN
F 3 "" H 5700 4150 50  0001 C CNN
	1    5700 4150
	0    1    1    0   
$EndComp
$Comp
L power:VDDA #PWR0104
U 1 1 5C2228D5
P 5700 4450
F 0 "#PWR0104" H 5700 4300 50  0001 C CNN
F 1 "VDDA" V 5717 4578 50  0000 L CNN
F 2 "" H 5700 4450 50  0001 C CNN
F 3 "" H 5700 4450 50  0001 C CNN
	1    5700 4450
	0    1    1    0   
$EndComp
Text GLabel 6000 2100 2    50   Input ~ 0
USB_VBUS
$Comp
L power:GND #PWR0101
U 1 1 5C229126
P 4800 5200
F 0 "#PWR0101" H 4800 4950 50  0001 C CNN
F 1 "GND" H 4805 5027 50  0000 C CNN
F 2 "" H 4800 5200 50  0001 C CNN
F 3 "" H 4800 5200 50  0001 C CNN
	1    4800 5200
	1    0    0    -1  
$EndComp
Text GLabel 4200 4650 0    50   Input ~ 0
USB_VBUS
$Comp
L Device:Battery_Cell BT1
U 1 1 5C22C403
P 3950 4200
F 0 "BT1" H 4068 4296 50  0000 L CNN
F 1 "Battery_Cell" H 4068 4205 50  0000 L CNN
F 2 "LINX_BAT-HLD-001:LINX_BAT-HLD-001_0" V 3950 4260 50  0001 C CNN
F 3 "~" V 3950 4260 50  0001 C CNN
	1    3950 4200
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0105
U 1 1 5C22CDA7
P 3950 4300
F 0 "#PWR0105" H 3950 4050 50  0001 C CNN
F 1 "GND" H 3955 4127 50  0000 C CNN
F 2 "" H 3950 4300 50  0001 C CNN
F 3 "" H 3950 4300 50  0001 C CNN
	1    3950 4300
	1    0    0    -1  
$EndComp
$Comp
L Diode:BAT54C D1
U 1 1 5C22D4D9
P 5400 4300
F 0 "D1" V 5446 4388 50  0000 L CNN
F 1 "BAT54C" V 5355 4388 50  0000 L CNN
F 2 "Package_TO_SOT_SMD:SOT-23" H 5475 4425 50  0001 L CNN
F 3 "http://www.diodes.com/_files/datasheets/ds11005.pdf" H 5320 4300 50  0001 C CNN
	1    5400 4300
	0    -1   -1   0   
$EndComp
Wire Wire Line
	5700 4150 5700 4300
Wire Wire Line
	5400 4650 5400 4600
Wire Wire Line
	3950 4000 5400 4000
Wire Wire Line
	5600 4300 5700 4300
Connection ~ 5700 4300
Wire Wire Line
	5700 4300 5700 4450
$Comp
L power:PWR_FLAG #FLG0101
U 1 1 5C230B98
P 5700 4300
F 0 "#FLG0101" H 5700 4375 50  0001 C CNN
F 1 "PWR_FLAG" V 5700 4428 50  0000 L CNN
F 2 "" H 5700 4300 50  0001 C CNN
F 3 "~" H 5700 4300 50  0001 C CNN
	1    5700 4300
	0    1    1    0   
$EndComp
$Comp
L power:VDD #PWR09
U 1 1 5C3B98E7
P 6500 3400
F 0 "#PWR09" H 6500 3250 50  0001 C CNN
F 1 "VDD" H 6517 3573 50  0000 C CNN
F 2 "" H 6500 3400 50  0001 C CNN
F 3 "" H 6500 3400 50  0001 C CNN
	1    6500 3400
	1    0    0    -1  
$EndComp
$Comp
L power:VDD #PWR010
U 1 1 5C3B994A
P 9000 3400
F 0 "#PWR010" H 9000 3250 50  0001 C CNN
F 1 "VDD" H 9017 3573 50  0000 C CNN
F 2 "" H 9000 3400 50  0001 C CNN
F 3 "" H 9000 3400 50  0001 C CNN
	1    9000 3400
	1    0    0    -1  
$EndComp
Text GLabel 2000 4050 3    50   Input ~ 0
SWCLK
Text GLabel 2100 4050 3    50   Input ~ 0
SWDIO
$Comp
L Switch:SW_Push SW1
U 1 1 5C424334
P 4550 3400
F 0 "SW1" H 4550 3685 50  0000 C CNN
F 1 "Boot_Mode_Btn" H 4550 3594 50  0000 C CNN
F 2 "tl3342:tl3342" H 4550 3600 50  0001 C CNN
F 3 "" H 4550 3600 50  0001 C CNN
	1    4550 3400
	1    0    0    -1  
$EndComp
$Comp
L Device:R R2
U 1 1 5C4250E3
P 4350 3550
F 0 "R2" V 4250 3550 50  0000 C CNN
F 1 "1M" V 4450 3550 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 4280 3550 50  0001 C CNN
F 3 "~" H 4350 3550 50  0001 C CNN
	1    4350 3550
	1    0    0    -1  
$EndComp
Wire Wire Line
	4250 3400 4350 3400
Connection ~ 4350 3400
Wire Wire Line
	4750 3400 4950 3400
Text Label 1700 2350 0    50   ~ 0
Internall_Pull_Up_25k(min)-40k(typ)-55k(max)
$Comp
L power:VDD #PWR0106
U 1 1 5C4799FF
P 3350 3350
F 0 "#PWR0106" H 3350 3200 50  0001 C CNN
F 1 "VDD" V 3367 3478 50  0000 L CNN
F 2 "" H 3350 3350 50  0001 C CNN
F 3 "" H 3350 3350 50  0001 C CNN
	1    3350 3350
	0    1    1    0   
$EndComp
$Comp
L Device:C C2
U 1 1 5C561B67
P 4450 5150
F 0 "C2" V 4700 5150 50  0000 C CNN
F 1 "1u" V 4600 5150 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 4488 5000 50  0001 C CNN
F 3 "~" H 4450 5150 50  0001 C CNN
	1    4450 5150
	0    1    1    0   
$EndComp
$Comp
L Device:C C4
U 1 1 5C563CA4
P 5150 5150
F 0 "C4" V 5400 5150 50  0000 C CNN
F 1 "1u" V 5300 5150 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5188 5000 50  0001 C CNN
F 3 "~" H 5150 5150 50  0001 C CNN
	1    5150 5150
	0    1    1    0   
$EndComp
Connection ~ 5300 4650
Wire Wire Line
	5300 4650 5400 4650
Wire Wire Line
	4200 5150 4300 5150
Wire Wire Line
	5300 4650 5300 5150
Wire Wire Line
	4600 5150 4800 5150
Connection ~ 4800 5150
Wire Wire Line
	4800 5150 4800 5200
Wire Wire Line
	4800 5150 5000 5150
Text GLabel 2300 2850 1    50   Input ~ 0
SCL
Text GLabel 2400 2850 1    50   Input ~ 0
SDA
$Comp
L Connector:TestPoint TP1
U 1 1 5C6F2C19
P 3750 2350
F 0 "TP1" H 3700 2550 50  0000 L CNN
F 1 "TestPoint" H 3600 2650 50  0000 L CNN
F 2 "TestPoint:TestPoint_THTPad_2.0x2.0mm_Drill1.0mm" H 3950 2350 50  0001 C CNN
F 3 "~" H 3950 2350 50  0001 C CNN
	1    3750 2350
	1    0    0    -1  
$EndComp
Text GLabel 3750 2350 3    50   Input ~ 0
SDA
$Comp
L Connector:TestPoint TP2
U 1 1 5C6F8355
P 4150 2350
F 0 "TP2" H 4100 2550 50  0000 L CNN
F 1 "TestPoint" H 4000 2650 50  0000 L CNN
F 2 "TestPoint:TestPoint_THTPad_2.0x2.0mm_Drill1.0mm" H 4350 2350 50  0001 C CNN
F 3 "~" H 4350 2350 50  0001 C CNN
	1    4150 2350
	1    0    0    -1  
$EndComp
Text GLabel 4150 2350 3    50   Input ~ 0
SCL
$Comp
L Connector:TestPoint TP3
U 1 1 5C6F8CE1
P 4550 2350
F 0 "TP3" H 4500 2550 50  0000 L CNN
F 1 "TestPoint" H 4400 2650 50  0000 L CNN
F 2 "TestPoint:TestPoint_THTPad_2.0x2.0mm_Drill1.0mm" H 4750 2350 50  0001 C CNN
F 3 "~" H 4750 2350 50  0001 C CNN
	1    4550 2350
	1    0    0    -1  
$EndComp
Text GLabel 4550 2350 3    50   Input ~ 0
SWCLK
$Comp
L Connector:TestPoint TP4
U 1 1 5C6F9685
P 4950 2350
F 0 "TP4" H 4900 2550 50  0000 L CNN
F 1 "TestPoint" H 4800 2650 50  0000 L CNN
F 2 "TestPoint:TestPoint_THTPad_2.0x2.0mm_Drill1.0mm" H 5150 2350 50  0001 C CNN
F 3 "~" H 5150 2350 50  0001 C CNN
	1    4950 2350
	1    0    0    -1  
$EndComp
Text GLabel 4950 2350 3    50   Input ~ 0
SWDIO
Wire Wire Line
	5600 2700 5700 2700
Connection ~ 5700 2700
$Comp
L Regulator_Linear:MCP1700-3302E_SOT89 U2
U 1 1 5CFEE32E
P 4800 4650
F 0 "U2" H 4800 4892 50  0000 C CNN
F 1 "MCP1700-3302E_SOT89" H 4800 4801 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-89-3" H 4800 4850 50  0001 C CNN
F 3 "http://ww1.microchip.com/downloads/en/DeviceDoc/20001826D.pdf" H 4800 4600 50  0001 C CNN
	1    4800 4650
	1    0    0    -1  
$EndComp
Wire Wire Line
	4800 4950 4800 5150
Wire Wire Line
	5100 4650 5300 4650
Wire Wire Line
	4200 4650 4200 5150
Wire Wire Line
	4200 4650 4500 4650
$EndSCHEMATC