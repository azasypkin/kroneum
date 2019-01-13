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
P 3800 3000
F 0 "U1" V 3800 2950 50  0000 R CNN
F 1 "STM32F042F4Px" V 3700 3200 50  0000 R CNN
F 2 "Package_SO:TSSOP-20_4.4x6.5mm_P0.65mm" H 3300 2300 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00105814.pdf" H 3800 3000 50  0001 C CNN
	1    3800 3000
	0    1    1    0   
$EndComp
$Comp
L Switch:SW_Push SW2
U 1 1 5C1E9C39
P 5950 5300
F 0 "SW2" H 5950 5585 50  0000 C CNN
F 1 "Ctrl_One_Btn" H 5950 5494 50  0000 C CNN
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 5950 5500 50  0001 C CNN
F 3 "" H 5950 5500 50  0001 C CNN
	1    5950 5300
	1    0    0    -1  
$EndComp
$Comp
L Connector:USB_B_Micro J1
U 1 1 5C1E9F15
P 6950 2600
F 0 "J1" H 7005 3067 50  0000 C CNN
F 1 "USB_B_Micro" H 7005 2976 50  0000 C CNN
F 2 "Connector_USB:USB_Micro-B_Molex-105017-0001" H 7100 2550 50  0001 C CNN
F 3 "~" H 7100 2550 50  0001 C CNN
	1    6950 2600
	1    0    0    -1  
$EndComp
$Comp
L Device:C C4
U 1 1 5C1EC1E3
P 6300 3550
F 0 "C4" H 6415 3596 50  0000 L CNN
F 1 "1u" H 6415 3505 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6338 3400 50  0001 C CNN
F 3 "~" H 6300 3550 50  0001 C CNN
	1    6300 3550
	1    0    0    -1  
$EndComp
$Comp
L Device:C C2
U 1 1 5C1EC257
P 6750 3550
F 0 "C2" H 6865 3596 50  0000 L CNN
F 1 "10n" H 6865 3505 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6788 3400 50  0001 C CNN
F 3 "~" H 6750 3550 50  0001 C CNN
	1    6750 3550
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR06
U 1 1 5C1EC59B
P 6300 3750
F 0 "#PWR06" H 6300 3500 50  0001 C CNN
F 1 "GND" H 6305 3577 50  0000 C CNN
F 2 "" H 6300 3750 50  0001 C CNN
F 3 "" H 6300 3750 50  0001 C CNN
	1    6300 3750
	1    0    0    -1  
$EndComp
Wire Wire Line
	4500 2800 4550 2800
NoConn ~ 7250 2800
NoConn ~ 6850 3000
$Comp
L power:GND #PWR01
U 1 1 5C1EDFBA
P 6950 3000
F 0 "#PWR01" H 6950 2750 50  0001 C CNN
F 1 "GND" H 6955 2827 50  0000 C CNN
F 2 "" H 6950 3000 50  0001 C CNN
F 3 "" H 6950 3000 50  0001 C CNN
	1    6950 3000
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR02
U 1 1 5C1EE186
P 3000 2800
F 0 "#PWR02" H 3000 2550 50  0001 C CNN
F 1 "GND" H 3005 2627 50  0000 C CNN
F 2 "" H 3000 2800 50  0001 C CNN
F 3 "" H 3000 2800 50  0001 C CNN
	1    3000 2800
	1    0    0    -1  
$EndComp
$Comp
L Device:Crystal Y1
U 1 1 5C1EE5F1
P 8100 3550
F 0 "Y1" H 8100 3818 50  0000 C CNN
F 1 "16 Mhz" H 8100 3727 50  0000 C CNN
F 2 "Crystal:Crystal_HC49-4H_Vertical" H 8100 3550 50  0001 C CNN
F 3 "~" H 8100 3550 50  0001 C CNN
	1    8100 3550
	1    0    0    -1  
$EndComp
$Comp
L Device:C C7
U 1 1 5C1EFF71
P 7500 4800
F 0 "C7" V 7752 4800 50  0000 C CNN
F 1 "0.1u" V 7661 4800 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7538 4650 50  0001 C CNN
F 3 "~" H 7500 4800 50  0001 C CNN
	1    7500 4800
	0    -1   -1   0   
$EndComp
$Comp
L Switch:SW_Push SW3
U 1 1 5C1EFD70
P 7500 4500
F 0 "SW3" H 7500 4785 50  0000 C CNN
F 1 "Reset_Btn" H 7500 4694 50  0000 C CNN
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 7500 4700 50  0001 C CNN
F 3 "" H 7500 4700 50  0001 C CNN
	1    7500 4500
	1    0    0    -1  
$EndComp
Wire Wire Line
	7300 4500 7300 4800
Wire Wire Line
	7300 4800 7350 4800
Wire Wire Line
	7650 4800 7700 4800
Wire Wire Line
	7700 4800 7700 4500
Wire Wire Line
	7700 4500 7850 4500
Connection ~ 7700 4500
$Comp
L power:GND #PWR013
U 1 1 5C1FDD8F
P 7850 4500
F 0 "#PWR013" H 7850 4250 50  0001 C CNN
F 1 "GND" H 7855 4327 50  0000 C CNN
F 2 "" H 7850 4500 50  0001 C CNN
F 3 "" H 7850 4500 50  0001 C CNN
	1    7850 4500
	1    0    0    -1  
$EndComp
$Comp
L power:VDDA #PWR015
U 1 1 5C2010A1
P 6300 3400
F 0 "#PWR015" H 6300 3250 50  0001 C CNN
F 1 "VDDA" H 6317 3573 50  0000 C CNN
F 2 "" H 6300 3400 50  0001 C CNN
F 3 "" H 6300 3400 50  0001 C CNN
	1    6300 3400
	1    0    0    -1  
$EndComp
Wire Wire Line
	6300 3750 6300 3700
Connection ~ 6300 3400
Connection ~ 6300 3700
$Comp
L power:VDDA #PWR04
U 1 1 5C202694
P 4550 2900
F 0 "#PWR04" H 4550 2750 50  0001 C CNN
F 1 "VDDA" V 4567 3028 50  0000 L CNN
F 2 "" H 4550 2900 50  0001 C CNN
F 3 "" H 4550 2900 50  0001 C CNN
	1    4550 2900
	0    1    1    0   
$EndComp
$Comp
L Device:C C1
U 1 1 5C202927
P 5350 3550
F 0 "C1" H 5465 3596 50  0000 L CNN
F 1 "0.1u" H 5465 3505 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5388 3400 50  0001 C CNN
F 3 "~" H 5350 3550 50  0001 C CNN
	1    5350 3550
	1    0    0    -1  
$EndComp
$Comp
L Device:C C3
U 1 1 5C20292E
P 5850 3550
F 0 "C3" H 5965 3596 50  0000 L CNN
F 1 "4.7u" H 5965 3505 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5888 3400 50  0001 C CNN
F 3 "~" H 5850 3550 50  0001 C CNN
	1    5850 3550
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR08
U 1 1 5C202935
P 5350 3750
F 0 "#PWR08" H 5350 3500 50  0001 C CNN
F 1 "GND" H 5355 3577 50  0000 C CNN
F 2 "" H 5350 3750 50  0001 C CNN
F 3 "" H 5350 3750 50  0001 C CNN
	1    5350 3750
	1    0    0    -1  
$EndComp
Wire Wire Line
	5350 3750 5350 3700
Connection ~ 5350 3700
Wire Wire Line
	6300 3400 6750 3400
Wire Wire Line
	6300 3700 6750 3700
Wire Wire Line
	5350 3400 5850 3400
Wire Wire Line
	5350 3700 5850 3700
$Comp
L power:VDD #PWR07
U 1 1 5C203D31
P 5350 3400
F 0 "#PWR07" H 5350 3250 50  0001 C CNN
F 1 "VDD" H 5367 3573 50  0000 C CNN
F 2 "" H 5350 3400 50  0001 C CNN
F 3 "" H 5350 3400 50  0001 C CNN
	1    5350 3400
	1    0    0    -1  
$EndComp
Connection ~ 5350 3400
$Comp
L power:VDD #PWR03
U 1 1 5C204AB4
P 4550 2800
F 0 "#PWR03" H 4550 2650 50  0001 C CNN
F 1 "VDD" V 4567 2928 50  0000 L CNN
F 2 "" H 4550 2800 50  0001 C CNN
F 3 "" H 4550 2800 50  0001 C CNN
	1    4550 2800
	0    1    1    0   
$EndComp
Wire Wire Line
	4500 2900 4550 2900
$Comp
L Device:R R1
U 1 1 5C206A14
P 5700 2650
F 0 "R1" V 5493 2650 50  0000 C CNN
F 1 "470" V 5584 2650 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 5630 2650 50  0001 C CNN
F 3 "~" H 5700 2650 50  0001 C CNN
	1    5700 2650
	0    1    1    0   
$EndComp
$Comp
L Device:Buzzer BZ1
U 1 1 5C206E6D
P 5950 2750
F 0 "BZ1" H 6103 2779 50  0000 L CNN
F 1 "Buzzer" H 6103 2688 50  0000 L CNN
F 2 "Buzzer_Murata_PKMCS0909E4000-R1:Buzzer_Murata_PKMCS0909E4000-R1" V 5925 2850 50  0001 C CNN
F 3 "~" V 5925 2850 50  0001 C CNN
	1    5950 2750
	1    0    0    -1  
$EndComp
Text GLabel 3600 3600 3    50   Input ~ 0
Buzzer
Text GLabel 3500 3600 3    50   Input ~ 0
USB_D-
Text GLabel 3400 3600 3    50   Input ~ 0
USB_D+
Text GLabel 7250 2600 2    50   Input ~ 0
USB_D+
Text GLabel 7250 2700 2    50   Input ~ 0
USB_D-
$Comp
L power:GND #PWR011
U 1 1 5C207A2D
P 5850 2850
F 0 "#PWR011" H 5850 2600 50  0001 C CNN
F 1 "GND" H 5855 2677 50  0000 C CNN
F 2 "" H 5850 2850 50  0001 C CNN
F 3 "" H 5850 2850 50  0001 C CNN
	1    5850 2850
	1    0    0    -1  
$EndComp
Text GLabel 5550 2650 0    50   Input ~ 0
Buzzer
Text GLabel 7300 4500 0    50   Input ~ 0
Reset
Text GLabel 4300 2400 1    50   Input ~ 0
Reset
Wire Wire Line
	8250 3550 8300 3550
Wire Wire Line
	7900 3550 7950 3550
$Comp
L Device:C C5
U 1 1 5C1EE906
P 7750 3550
F 0 "C5" V 7498 3550 50  0000 C CNN
F 1 "30p" V 7589 3550 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7788 3400 50  0001 C CNN
F 3 "~" H 7750 3550 50  0001 C CNN
	1    7750 3550
	0    1    1    0   
$EndComp
$Comp
L Device:C C6
U 1 1 5C1EE83A
P 8450 3550
F 0 "C6" V 8702 3550 50  0000 C CNN
F 1 "30p" V 8611 3550 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 8488 3400 50  0001 C CNN
F 3 "~" H 8450 3550 50  0001 C CNN
	1    8450 3550
	0    -1   -1   0   
$EndComp
Wire Wire Line
	7600 3550 7600 3800
Wire Wire Line
	7600 3800 8100 3800
Wire Wire Line
	8600 3800 8600 3550
$Comp
L power:GND #PWR05
U 1 1 5C20A7AC
P 8100 3800
F 0 "#PWR05" H 8100 3550 50  0001 C CNN
F 1 "GND" H 8105 3627 50  0000 C CNN
F 2 "" H 8100 3800 50  0001 C CNN
F 3 "" H 8100 3800 50  0001 C CNN
	1    8100 3800
	1    0    0    -1  
$EndComp
Connection ~ 8100 3800
Wire Wire Line
	8100 3800 8600 3800
Wire Wire Line
	7900 3550 7900 3100
Connection ~ 7900 3550
Wire Wire Line
	8300 3550 8300 3100
Connection ~ 8300 3550
Text GLabel 7900 3100 1    50   Input ~ 0
OSC_IN
Text GLabel 8300 3100 1    50   Input ~ 0
OSC_OUT
Text GLabel 3600 2400 1    50   Input ~ 0
OSC_IN
Text GLabel 3500 2400 1    50   Input ~ 0
OSC_OUT
Text GLabel 3200 2400 1    50   Input ~ 0
BOOT0
$Comp
L Device:R R2
U 1 1 5C20BB63
P 5750 4550
F 0 "R2" V 5543 4550 50  0000 C CNN
F 1 "10k" V 5634 4550 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 5680 4550 50  0001 C CNN
F 3 "~" H 5750 4550 50  0001 C CNN
	1    5750 4550
	0    1    1    0   
$EndComp
$Comp
L Switch:SW_DPDT_x2 SW1
U 1 1 5C20BDA0
P 6100 4550
F 0 "SW1" H 6100 4225 50  0000 C CNN
F 1 "Boot_Mode_Btn" H 6100 4316 50  0000 C CNN
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 6100 4550 50  0001 C CNN
F 3 "" H 6100 4550 50  0001 C CNN
	1    6100 4550
	1    0    0    1   
$EndComp
Wire Wire Line
	6300 4650 6550 4650
$Comp
L power:GND #PWR020
U 1 1 5C20CB7E
P 6550 4650
F 0 "#PWR020" H 6550 4400 50  0001 C CNN
F 1 "GND" H 6555 4477 50  0000 C CNN
F 2 "" H 6550 4650 50  0001 C CNN
F 3 "" H 6550 4650 50  0001 C CNN
	1    6550 4650
	1    0    0    -1  
$EndComp
Wire Wire Line
	6300 4450 6550 4450
Text GLabel 5600 4550 0    50   Input ~ 0
BOOT0
Text GLabel 4300 3600 3    50   Input ~ 0
Ctrl_One
Text GLabel 4100 3600 3    50   Input ~ 0
Ctrl_Ten
NoConn ~ 3700 3600
NoConn ~ 3800 3600
NoConn ~ 4200 3600
NoConn ~ 4000 3600
NoConn ~ 3900 3600
NoConn ~ 3300 3600
NoConn ~ 3200 3600
NoConn ~ 3300 2400
$Comp
L Device:C C8
U 1 1 5C20FD6A
P 5950 5550
F 0 "C8" V 6200 5550 50  0000 C CNN
F 1 "0.1u" V 6100 5550 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5988 5400 50  0001 C CNN
F 3 "~" H 5950 5550 50  0001 C CNN
	1    5950 5550
	0    1    1    0   
$EndComp
Wire Wire Line
	6150 5300 6250 5300
Wire Wire Line
	6100 5550 6250 5550
Wire Wire Line
	6250 5550 6250 5300
Wire Wire Line
	5750 5300 5650 5300
Wire Wire Line
	5650 5300 5650 5550
Wire Wire Line
	5650 5550 5800 5550
Text GLabel 5650 5300 0    50   Input ~ 0
Ctrl_One
Wire Wire Line
	6250 5300 6450 5300
Connection ~ 6250 5300
$Comp
L Switch:SW_Push SW4
U 1 1 5C211C0A
P 7700 5300
F 0 "SW4" H 7700 5585 50  0000 C CNN
F 1 "Ctrl_Ten_Btn" H 7700 5494 50  0000 C CNN
F 2 "Button_Switch_SMD:SW_SPST_PTS645" H 7700 5500 50  0001 C CNN
F 3 "" H 7700 5500 50  0001 C CNN
	1    7700 5300
	1    0    0    -1  
$EndComp
$Comp
L Device:C C9
U 1 1 5C211C11
P 7700 5550
F 0 "C9" V 7950 5550 50  0000 C CNN
F 1 "0.1u" V 7850 5550 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7738 5400 50  0001 C CNN
F 3 "~" H 7700 5550 50  0001 C CNN
	1    7700 5550
	0    1    1    0   
$EndComp
Wire Wire Line
	7900 5300 8000 5300
Wire Wire Line
	7850 5550 8000 5550
Wire Wire Line
	8000 5550 8000 5300
Wire Wire Line
	7500 5300 7400 5300
Wire Wire Line
	7400 5300 7400 5550
Wire Wire Line
	7400 5550 7550 5550
Text GLabel 7400 5300 0    50   Input ~ 0
Ctrl_Ten
Wire Wire Line
	8000 5300 8200 5300
Connection ~ 8000 5300
$Comp
L power:VDD #PWR0102
U 1 1 5C2140A0
P 6550 4450
F 0 "#PWR0102" H 6550 4300 50  0001 C CNN
F 1 "VDD" H 6567 4623 50  0000 C CNN
F 2 "" H 6550 4450 50  0001 C CNN
F 3 "" H 6550 4450 50  0001 C CNN
	1    6550 4450
	1    0    0    -1  
$EndComp
$Comp
L power:VDD #PWR0103
U 1 1 5C22284D
P 4500 5000
F 0 "#PWR0103" H 4500 4850 50  0001 C CNN
F 1 "VDD" V 4500 5200 50  0000 C CNN
F 2 "" H 4500 5000 50  0001 C CNN
F 3 "" H 4500 5000 50  0001 C CNN
	1    4500 5000
	0    1    1    0   
$EndComp
$Comp
L power:VDDA #PWR0104
U 1 1 5C2228D5
P 4500 5300
F 0 "#PWR0104" H 4500 5150 50  0001 C CNN
F 1 "VDDA" V 4517 5428 50  0000 L CNN
F 2 "" H 4500 5300 50  0001 C CNN
F 3 "" H 4500 5300 50  0001 C CNN
	1    4500 5300
	0    1    1    0   
$EndComp
$Comp
L ht7333-a:HT7333-A L1
U 1 1 5C228345
P 3450 5650
F 0 "L1" H 3500 6047 60  0000 C CNN
F 1 "HT7333-A" H 3500 5941 60  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-89-3" H 3450 5650 60  0001 C CNN
F 3 "" H 3450 5650 60  0000 C CNN
	1    3450 5650
	1    0    0    -1  
$EndComp
Text GLabel 7250 2400 2    50   Input ~ 0
USB_VBUS
$Comp
L power:GND #PWR0101
U 1 1 5C229126
P 3600 5950
F 0 "#PWR0101" H 3600 5700 50  0001 C CNN
F 1 "GND" H 3605 5777 50  0000 C CNN
F 2 "" H 3600 5950 50  0001 C CNN
F 3 "" H 3600 5950 50  0001 C CNN
	1    3600 5950
	1    0    0    -1  
$EndComp
Text GLabel 3050 5650 0    50   Input ~ 0
USB_VBUS
$Comp
L Device:Battery_Cell BT1
U 1 1 5C22C403
P 2750 5050
F 0 "BT1" H 2868 5146 50  0000 L CNN
F 1 "Battery_Cell" H 2868 5055 50  0000 L CNN
F 2 "Battery:BatteryHolder_Keystone_103_1x20mm" V 2750 5110 50  0001 C CNN
F 3 "~" V 2750 5110 50  0001 C CNN
	1    2750 5050
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0105
U 1 1 5C22CDA7
P 2750 5150
F 0 "#PWR0105" H 2750 4900 50  0001 C CNN
F 1 "GND" H 2755 4977 50  0000 C CNN
F 2 "" H 2750 5150 50  0001 C CNN
F 3 "" H 2750 5150 50  0001 C CNN
	1    2750 5150
	1    0    0    -1  
$EndComp
$Comp
L Diode:BAT54C D1
U 1 1 5C22D4D9
P 4200 5150
F 0 "D1" V 4246 5238 50  0000 L CNN
F 1 "BAT54C" V 4155 5238 50  0000 L CNN
F 2 "Package_TO_SOT_SMD:SOT-23" H 4275 5275 50  0001 L CNN
F 3 "http://www.diodes.com/_files/datasheets/ds11005.pdf" H 4120 5150 50  0001 C CNN
	1    4200 5150
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4500 5000 4500 5150
Wire Wire Line
	3950 5500 4200 5500
Wire Wire Line
	4200 5500 4200 5450
Wire Wire Line
	2750 4850 4200 4850
Wire Wire Line
	4400 5150 4500 5150
Connection ~ 4500 5150
Wire Wire Line
	4500 5150 4500 5300
$Comp
L power:PWR_FLAG #FLG0101
U 1 1 5C230B98
P 4500 5150
F 0 "#FLG0101" H 4500 5225 50  0001 C CNN
F 1 "PWR_FLAG" V 4500 5278 50  0000 L CNN
F 2 "" H 4500 5150 50  0001 C CNN
F 3 "~" H 4500 5150 50  0001 C CNN
	1    4500 5150
	0    1    1    0   
$EndComp
$Comp
L power:VDD #PWR09
U 1 1 5C3B98E7
P 6450 5300
F 0 "#PWR09" H 6450 5150 50  0001 C CNN
F 1 "VDD" H 6467 5473 50  0000 C CNN
F 2 "" H 6450 5300 50  0001 C CNN
F 3 "" H 6450 5300 50  0001 C CNN
	1    6450 5300
	1    0    0    -1  
$EndComp
$Comp
L power:VDD #PWR010
U 1 1 5C3B994A
P 8200 5300
F 0 "#PWR010" H 8200 5150 50  0001 C CNN
F 1 "VDD" H 8217 5473 50  0000 C CNN
F 2 "" H 8200 5300 50  0001 C CNN
F 3 "" H 8200 5300 50  0001 C CNN
	1    8200 5300
	1    0    0    -1  
$EndComp
$EndSCHEMATC
