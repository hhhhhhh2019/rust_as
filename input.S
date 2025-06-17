#include "include.S"

addn pc, pc, start - $

var:
	ds 1, 2, 2 + 4 * 3

start:
	addn r1, pc, var - $
	loas r2, r1, 2 * 2

	addn r1, r0, PAGING | INTERRUPTS
	chst r0
