#ifndef __FENNEC_HEADER__
#define __FENNEC_HEADER__

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>

typedef enum FennecConfig_FennecValue_Type {
    FennecConfig_FennecValue_Type_Object,
    FennecConfig_FennecValue_Type_Array,
    FennecConfig_FennecValue_Type_String,
    FennecConfig_FennecValue_Type_Float,
    FennecConfig_FennecValue_Type_Int,
    FennecConfig_FennecValue_Type_Bool,
    FennecConfig_FennecValue_Type_Null,
    FennecConfig_FennecValue_Type_Error
} FennecConfig_FennecValue_Type;

struct FennecConfig_FennecValue;

/// Ignore the capacity values, these are kept track of to properly free memory without leaking.
typedef struct FennecConfig_FennecValue_Object {
    size_t len;
    size_t keys_capacity;
    char **keys;
    size_t values_capacity;
    struct FennecConfig_FennecValue *values;
} FennecConfig_FennecValue_Object;

/// Ignore the capacity values, these are kept track of to properly free memory without leaking.
typedef struct FennecConfig_FennecValue_Array {
    size_t len;
    size_t capacity;
    struct FennecConfig_FennecValue *arr;
} FennecConfig_FennecValue_Array;

typedef struct FennecConfig_FennecValue {
    FennecConfig_FennecValue_Type type;
    union {
        FennecConfig_FennecValue_Object object;
        FennecConfig_FennecValue_Array array;
        char *string;
        double f;
        long i;
        bool b;
    } value;
} FennecConfig_FennecValue;

FennecConfig_FennecValue *FennecConfig_ParseString(char *str);

FennecConfig_FennecValue *FennecConfig_ParseFile(char *filename);

/// Only call this on the FennecValues you get from the Parse* functions. Never call it on their children.
void FennecConfig_FennecType_Free(FennecConfig_FennecValue *fen);

#ifdef __cplusplus
}
#endif

#endif