#include "fennec.h"
#include <stdio.h>

int main(int argv, char* argc) {
    FennecConfig_FennecValue *fen = FennecConfig_ParseString("test = \"owo\" owo = 15 uwu = false");

    FennecConfig_FennecType_Free(fen);
    
    return 0;
}
