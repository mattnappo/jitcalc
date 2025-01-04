section	.text
	global _start
_start:
    ;; + + * - /
	mov	eax, 0
	
	inc eax ; from +
	inc eax ; from +
	shr eax, 1 ; from *
	dec eax ; from -
	shl eax, 1 ; from /