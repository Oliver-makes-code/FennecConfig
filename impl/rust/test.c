#include "fennec.h"
#include <stdio.h>

int main(int argv, char* argc) {
    FennecConfig_ParseResult *result = FennecConfig_LoadFile("../../specification.fennec");

    FennecConfig_FennecType *fen = FennecConfig_ParseResult_GetOk(result);

    FennecConfig_FennecType *key_1 = FennecConfig_FennecType_Object_GetKey(fen, "key1");

    bool is_str = FennecConfig_FennecType_IsString(key_1);

    if (is_str) {
        printf("key1 is \"%s\"", FennecConfig_FennecType_GetString(key_1));
    }

    FennecConfig_ParseResult_Drop(result);
}
