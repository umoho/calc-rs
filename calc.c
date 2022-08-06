#include <stdio.h>
#include <stdlib.h>

#include "calc.h"

int main() {
    char * const re = calculate("114514 / 50");

    printf("%s\n", re);

    free(re);
}