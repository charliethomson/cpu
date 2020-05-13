init:
    setv 0
    str 0x40 ;; x = 0
    str 0x42 ;; z = 0
    setv 1
    str 0x41 ;; y = 1

loop:
    ;; print z
    lda 0x42
    out

    ;; z = x + y
    lda 0x40    ;; load x into acc
    load 0x41   ;; load y into usr
    add         ;; add y to acc (x) -> acc = x + y
    jo exit_good ;; exit if overflow
    sta 0x42    ;; store acc in z


    ;; x = y
    lda 0x41    ;; load y into acc
    sta 0x40    ;; store y in x

    ;; y = z
    lda 0x42    ;; load z into acc
    sta 0x41    ;; store z in y


    ;; while z < 255
    jmp loop    ;; reenter the loop otherwise

exit_good:
    exit 1