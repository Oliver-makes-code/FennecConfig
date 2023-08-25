#include "fennec.h"
#include <stdio.h>

int main(int argv, char* argc) {
    FennecConfig_FennecValue *fen = FennecConfig_ParseFile("../../specification.fennec");

    printf("%u %s", fen->type, fen->value.object.keys[0]);

    FennecConfig_FennecType_Free(fen);
    
    return 0;
}
