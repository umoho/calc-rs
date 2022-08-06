#include <stdio.h>
#include <stdlib.h>

#include "calc.h"

int main() {
    char * re = calculate("114514 / 50");
    printf("%s\n", re);

    free_result(re); // note that re is managed by rust, do not call free() on re!
    re = NULL;
}