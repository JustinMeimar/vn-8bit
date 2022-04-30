8-bit computer schema

Register Use:
---------------------------------------------------------
  $0, $1, $2, ..... $D, $E, $F
                         Alias:     Real Rep:
  temporary resgisers:  $t0 - $t6   $0 - $6
  saved regisers:       $s0 - $s6   $7 - $A
  special use reigsers: $ra         $f

  non register-file registers:
    hi/lo: for mul and division
    upr: stores upper 8bits of address in lb/sb
    lwr: stores lower 8bits of address in lb/sb

Instruction Set:
---------------------------------------------------------
Op-Code:| Format | Notes:
--------------------------------------------------------
  0x0     |     
  0x1     |
  0x2     |
  0x3     |
  0x4
0x5
0x6
0x7
0x8
0x9
0xA
0xB
0xC
0xD
0xE
0xF
