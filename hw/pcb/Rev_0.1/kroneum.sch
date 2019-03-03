EESchema Schematic File Version 4
LIBS:kroneum-cache
EELAYER 26 0
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
F 2 "Package_SO:TSSOP-20_4.4x6.5mm_P0.65mm" H 2100 2750 50  0001 R CNN
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
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 6000 3600 50  0001 C CNN
F 3 "" H 6000 3600 50  0001 C CNN
	1    6000 3400
	1    0    0    -1  
$EndComp
$Comp
L Connector:USB_B_Micro J1
U 1 1 5C1E9F15
P 5350 2350
F 0 "J1" H 5405 2817 50  0000 C CNN
F 1 "USB_B_Micro" H 5405 2726 50  0000 C CNN
F 2 "Connector_USB:USB_Micro-B_Molex-105017-0001" H 5500 2300 50  0001 C CNN
F 3 "~" H 5500 2300 50  0001 C CNN
	1    5350 2350
	1    0    0    -1  
$EndComp
$Comp
L Device:C C4
U 1 1 5C1EC1E3
P 7300 2300
F 0 "C4" H 7415 2346 50  0000 L CNN
F 1 "1u" H 7415 2255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7338 2150 50  0001 C CNN
F 3 "~" H 7300 2300 50  0001 C CNN
	1    7300 2300
	1    0    0    -1  
$EndComp
$Comp
L Device:C C2
U 1 1 5C1EC257
P 7750 2300
F 0 "C2" H 7865 2346 50  0000 L CNN
F 1 "10n" H 7865 2255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7788 2150 50  0001 C CNN
F 3 "~" H 7750 2300 50  0001 C CNN
	1    7750 2300
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR06
U 1 1 5C1EC59B
P 7300 2500
F 0 "#PWR06" H 7300 2250 50  0001 C CNN
F 1 "GND" H 7305 2327 50  0000 C CNN
F 2 "" H 7300 2500 50  0001 C CNN
F 3 "" H 7300 2500 50  0001 C CNN
	1    7300 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	3300 3250 3350 3250
NoConn ~ 5650 2550
NoConn ~ 5250 2750
$Comp
L power:GND #PWR01
U 1 1 5C1EDFBA
P 5350 2750
F 0 "#PWR01" H 5350 2500 50  0001 C CNN
F 1 "GND" H 5355 2577 50  0000 C CNN
F 2 "" H 5350 2750 50  0001 C CNN
F 3 "" H 5350 2750 50  0001 C CNN
	1    5350 2750
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
L Device:Crystal Y1
U 1 1 5C1EE5F1
P 7100 4700
F 0 "Y1" H 7100 4968 50  0000 C CNN
F 1 "16 Mhz" H 7100 4877 50  0000 C CNN
F 2 "Crystal:Crystal_HC49-4H_Vertical" H 7100 4700 50  0001 C CNN
F 3 "~" H 7100 4700 50  0001 C CNN
	1    7100 4700
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
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 7250 3600 50  0001 C CNN
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
L power:VDDA #PWR015
U 1 1 5C2010A1
P 7300 2150
F 0 "#PWR015" H 7300 2000 50  0001 C CNN
F 1 "VDDA" H 7317 2323 50  0000 C CNN
F 2 "" H 7300 2150 50  0001 C CNN
F 3 "" H 7300 2150 50  0001 C CNN
	1    7300 2150
	1    0    0    -1  
$EndComp
Wire Wire Line
	7300 2500 7300 2450
Connection ~ 7300 2150
Connection ~ 7300 2450
$Comp
L power:VDDA #PWR04
U 1 1 5C202694
P 3350 3350
F 0 "#PWR04" H 3350 3200 50  0001 C CNN
F 1 "VDDA" V 3367 3478 50  0000 L CNN
F 2 "" H 3350 3350 50  0001 C CNN
F 3 "" H 3350 3350 50  0001 C CNN
	1    3350 3350
	0    1    1    0   
$EndComp
$Comp
L Device:C C1
U 1 1 5C202927
P 6350 2300
F 0 "C1" H 6465 2346 50  0000 L CNN
F 1 "0.1u" H 6465 2255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6388 2150 50  0001 C CNN
F 3 "~" H 6350 2300 50  0001 C CNN
	1    6350 2300
	1    0    0    -1  
$EndComp
$Comp
L Device:C C3
U 1 1 5C20292E
P 6850 2300
F 0 "C3" H 6965 2346 50  0000 L CNN
F 1 "4.7u" H 6965 2255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6888 2150 50  0001 C CNN
F 3 "~" H 6850 2300 50  0001 C CNN
	1    6850 2300
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR08
U 1 1 5C202935
P 6350 2500
F 0 "#PWR08" H 6350 2250 50  0001 C CNN
F 1 "GND" H 6355 2327 50  0000 C CNN
F 2 "" H 6350 2500 50  0001 C CNN
F 3 "" H 6350 2500 50  0001 C CNN
	1    6350 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	6350 2500 6350 2450
Connection ~ 6350 2450
Wire Wire Line
	7300 2150 7750 2150
Wire Wire Line
	7300 2450 7750 2450
Wire Wire Line
	6350 2150 6850 2150
Wire Wire Line
	6350 2450 6850 2450
$Comp
L power:VDD #PWR07
U 1 1 5C203D31
P 6350 2150
F 0 "#PWR07" H 6350 2000 50  0001 C CNN
F 1 "VDD" H 6367 2323 50  0000 C CNN
F 2 "" H 6350 2150 50  0001 C CNN
F 3 "" H 6350 2150 50  0001 C CNN
	1    6350 2150
	1    0    0    -1  
$EndComp
Connection ~ 6350 2150
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
P 8350 4350
F 0 "R1" V 8143 4350 50  0000 C CNN
F 1 "470" V 8234 4350 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 8280 4350 50  0001 C CNN
F 3 "~" H 8350 4350 50  0001 C CNN
	1    8350 4350
	0    1    1    0   
$EndComp
$Comp
L Device:Buzzer BZ1
U 1 1 5C206E6D
P 8600 4450
F 0 "BZ1" H 8753 4479 50  0000 L CNN
F 1 "Buzzer" H 8753 4388 50  0000 L CNN
F 2 "Buzzer_Murata_PKMCS0909E4000-R1:Buzzer_Murata_PKMCS0909E4000-R1" V 8575 4550 50  0001 C CNN
F 3 "~" V 8575 4550 50  0001 C CNN
	1    8600 4450
	1    0    0    -1  
$EndComp
Text GLabel 2400 4050 3    50   Input ~ 0
Buzzer
Text GLabel 2300 4050 3    50   Input ~ 0
USB_D-
Text GLabel 2200 4050 3    50   Input ~ 0
USB_D+
Text GLabel 5650 2350 2    50   Input ~ 0
USB_D+
Text GLabel 5650 2450 2    50   Input ~ 0
USB_D-
$Comp
L power:GND #PWR011
U 1 1 5C207A2D
P 8500 4550
F 0 "#PWR011" H 8500 4300 50  0001 C CNN
F 1 "GND" H 8505 4377 50  0000 C CNN
F 2 "" H 8500 4550 50  0001 C CNN
F 3 "" H 8500 4550 50  0001 C CNN
	1    8500 4550
	1    0    0    -1  
$EndComp
Text GLabel 8200 4350 0    50   Input ~ 0
Buzzer
Text GLabel 7050 3400 0    50   Input ~ 0
NRST
Text GLabel 3100 2850 1    50   Input ~ 0
NRST
Wire Wire Line
	7250 4700 7300 4700
Wire Wire Line
	6900 4700 6950 4700
$Comp
L Device:C C5
U 1 1 5C1EE906
P 6750 4700
F 0 "C5" V 6498 4700 50  0000 C CNN
F 1 "30p" V 6589 4700 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6788 4550 50  0001 C CNN
F 3 "~" H 6750 4700 50  0001 C CNN
	1    6750 4700
	0    1    1    0   
$EndComp
$Comp
L Device:C C6
U 1 1 5C1EE83A
P 7450 4700
F 0 "C6" V 7702 4700 50  0000 C CNN
F 1 "30p" V 7611 4700 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7488 4550 50  0001 C CNN
F 3 "~" H 7450 4700 50  0001 C CNN
	1    7450 4700
	0    -1   -1   0   
$EndComp
Wire Wire Line
	6600 4700 6600 4950
Wire Wire Line
	6600 4950 7100 4950
Wire Wire Line
	7600 4950 7600 4700
$Comp
L power:GND #PWR05
U 1 1 5C20A7AC
P 7100 4950
F 0 "#PWR05" H 7100 4700 50  0001 C CNN
F 1 "GND" H 7105 4777 50  0000 C CNN
F 2 "" H 7100 4950 50  0001 C CNN
F 3 "" H 7100 4950 50  0001 C CNN
	1    7100 4950
	1    0    0    -1  
$EndComp
Connection ~ 7100 4950
Wire Wire Line
	7100 4950 7600 4950
Wire Wire Line
	6900 4700 6900 4250
Connection ~ 6900 4700
Wire Wire Line
	7300 4700 7300 4250
Connection ~ 7300 4700
Text GLabel 6900 4250 1    50   Input ~ 0
OSC_IN
Text GLabel 7300 4250 1    50   Input ~ 0
OSC_OUT
Text GLabel 2400 2850 1    50   Input ~ 0
OSC_IN
Text GLabel 2300 2850 1    50   Input ~ 0
OSC_OUT
Text GLabel 2000 2850 1    50   Input ~ 0
BOOT0
$Comp
L Device:R R2
U 1 1 5C20BB63
P 4250 3450
F 0 "R2" V 4043 3450 50  0000 C CNN
F 1 "10k" V 4134 3450 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 4180 3450 50  0001 C CNN
F 3 "~" H 4250 3450 50  0001 C CNN
	1    4250 3450
	0    1    1    0   
$EndComp
$Comp
L Switch:SW_DPDT_x2 SW1
U 1 1 5C20BDA0
P 4600 3450
F 0 "SW1" H 4600 3125 50  0000 C CNN
F 1 "Boot_Mode_Btn" H 4600 3216 50  0000 C CNN
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 4600 3450 50  0001 C CNN
F 3 "" H 4600 3450 50  0001 C CNN
	1    4600 3450
	1    0    0    1   
$EndComp
Wire Wire Line
	4800 3550 5050 3550
$Comp
L power:GND #PWR020
U 1 1 5C20CB7E
P 5050 3550
F 0 "#PWR020" H 5050 3300 50  0001 C CNN
F 1 "GND" H 5055 3377 50  0000 C CNN
F 2 "" H 5050 3550 50  0001 C CNN
F 3 "" H 5050 3550 50  0001 C CNN
	1    5050 3550
	1    0    0    -1  
$EndComp
Wire Wire Line
	4800 3350 5050 3350
Text GLabel 4100 3450 0    50   Input ~ 0
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
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 8500 3600 50  0001 C CNN
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
P 5050 3350
F 0 "#PWR0102" H 5050 3200 50  0001 C CNN
F 1 "VDD" H 5067 3523 50  0000 C CNN
F 2 "" H 5050 3350 50  0001 C CNN
F 3 "" H 5050 3350 50  0001 C CNN
	1    5050 3350
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
$Comp
L ht7333-a:HT7333-A L1
U 1 1 5C228345
P 4650 4800
F 0 "L1" H 4700 5197 60  0000 C CNN
F 1 "HT7333-A" H 4700 5091 60  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-89-3" H 4650 4800 60  0001 C CNN
F 3 "" H 4650 4800 60  0000 C CNN
	1    4650 4800
	1    0    0    -1  
$EndComp
Text GLabel 5650 2150 2    50   Input ~ 0
USB_VBUS
$Comp
L power:GND #PWR0101
U 1 1 5C229126
P 4800 5100
F 0 "#PWR0101" H 4800 4850 50  0001 C CNN
F 1 "GND" H 4805 4927 50  0000 C CNN
F 2 "" H 4800 5100 50  0001 C CNN
F 3 "" H 4800 5100 50  0001 C CNN
	1    4800 5100
	1    0    0    -1  
$EndComp
Text GLabel 4250 4800 0    50   Input ~ 0
USB_VBUS
$Comp
L Device:Battery_Cell BT1
U 1 1 5C22C403
P 3950 4200
F 0 "BT1" H 4068 4296 50  0000 L CNN
F 1 "Battery_Cell" H 4068 4205 50  0000 L CNN
F 2 "Battery:BatteryHolder_Keystone_103_1x20mm" V 3950 4260 50  0001 C CNN
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
	5150 4650 5400 4650
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
$Comp
L Connector_Generic:Conn_02x03_Counter_Clockwise J2
U 1 1 5C3E3D6A
P 4300 2200
F 0 "J2" H 4350 2517 50  0000 C CNN
F 1 "SWD" H 4350 2426 50  0000 C CNN
F 2 "" H 4300 2200 50  0001 C CNN
F 3 "~" H 4300 2200 50  0001 C CNN
	1    4300 2200
	1    0    0    -1  
$EndComp
$Comp
L power:VDD #PWR012
U 1 1 5C3E417F
P 4100 2100
F 0 "#PWR012" H 4100 1950 50  0001 C CNN
F 1 "VDD" H 4117 2273 50  0000 C CNN
F 2 "" H 4100 2100 50  0001 C CNN
F 3 "" H 4100 2100 50  0001 C CNN
	1    4100 2100
	1    0    0    -1  
$EndComp
Text GLabel 4100 2200 0    50   Input ~ 0
SWCLK
$Comp
L power:GND #PWR014
U 1 1 5C3E423F
P 4100 2300
F 0 "#PWR014" H 4100 2050 50  0001 C CNN
F 1 "GND" H 4105 2127 50  0000 C CNN
F 2 "" H 4100 2300 50  0001 C CNN
F 3 "" H 4100 2300 50  0001 C CNN
	1    4100 2300
	1    0    0    -1  
$EndComp
Text GLabel 4600 2300 2    50   Input ~ 0
SWDIO
Text GLabel 4600 2200 2    50   Input ~ 0
NRST
Text GLabel 2000 4050 3    50   Input ~ 0
SWCLK
Text GLabel 2100 4050 3    50   Input ~ 0
SWDIO
NoConn ~ 4600 2100
$EndSCHEMATC