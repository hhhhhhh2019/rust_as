addn pc, pc, start - $

var:
	ds 1, 2, 2 + 4 * 3

start:
	addn r1, pc, var - $
	loas r2, r1, 2 * 2

	chst r0
