#ifndef __FENNEC_HEADER__
#define __FENNEC_HEADER__

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <string.h>

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

/// It isn't recommended to operate on this by iteself. Rather, you should copy the data to your own representation and then free it with FennecConfig_FennecType_Free.
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
void FennecConfig_FennecValue_Free(FennecConfig_FennecValue *fen);

bool FennecConfig_FennecValue_IsObject(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Object;
}

FennecConfig_FennecValue_Object *FennecConfig_FennecValue_GetObject(FennecConfig_FennecValue *fen) {
    if (!FennecConfig_FennecValue_IsObject(fen)) {
        return (FennecConfig_FennecValue_Object *)0;
    }

    return &fen->value.object;
}

bool FennecConfig_FennecValue_Object_HasKey(FennecConfig_FennecValue_Object *obj, char *key) {
    for (int i = 0; i < obj->len; i++) {
        if (strcmp(key, obj->keys[i])) {
            return true;
        }
    }
    return false;
}

FennecConfig_FennecValue *FennecConfig_FennecValue_Object_GetKey(FennecConfig_FennecValue_Object *obj, char *key) {
    for (int i = 0; i < obj->len; i++) {
        if (strcmp(key, obj->keys[i])) {
            return &obj->values[i];
        }
    }

    return (FennecConfig_FennecValue *)0;
}

bool FennecConfig_FennecValue_IsArray(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Array;
}

FennecConfig_FennecValue_Array *FennecConfig_FennecValue_GetArray(FennecConfig_FennecValue *fen) {
    if (!FennecConfig_FennecValue_IsArray(fen)) {
        return (FennecConfig_FennecValue_Array *)0;
    }

    return &fen->value.array;
}

size_t FennecConfig_FennecValue_Array_Len(FennecConfig_FennecValue_Array *arr) {
    return arr->len;
}

FennecConfig_FennecValue *FennecConfig_FennecValue_Array_GetIdx(FennecConfig_FennecValue_Array *arr, size_t idx) {
    if (idx >= arr->len) {
        return (FennecConfig_FennecValue *)0;
    }
    
    return &arr->arr[idx];
}

bool FennecConfig_FennecValue_IsString(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_String;
}

char *FennecConfig_FennecValue_GetString(FennecConfig_FennecValue *fen) {
    if (!FennecConfig_FennecValue_IsString(fen)) {
        return (char *)0;
    }

    return fen->value.string;
}

bool FennecConfig_FennecValue_IsFloat(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Float;
}

double FennecConfig_FennecValue_GetFloat(FennecConfig_FennecValue *fen) {
    if (!FennecConfig_FennecValue_IsFloat(fen)) {
        return 0;
    }

    return fen->value.f;
}

bool FennecConfig_FennecValue_IsInt(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Int;
}

long FennecConfig_FennecValue_GetInt(FennecConfig_FennecValue *fen) {
    if (!FennecConfig_FennecValue_IsInt(fen)) {
        return 0;
    }

    return fen->value.i;
}

bool FennecConfig_FennecValue_IsNumber(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Float || fen->type == FennecConfig_FennecValue_Type_Int;
}

double FennecConfig_FennecValue_GetNumber(FennecConfig_FennecValue *fen) {
    if (FennecConfig_FennecValue_IsInt(fen)) {
        return (double)fen->value.i;
    }

    if (FennecConfig_FennecValue_IsFloat(fen)) {
        return fen->value.f;
    }
    
    return 0;
}

bool FennecConfig_FennecValue_IsBool(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Bool;
}

bool FennecConfig_FennecValue_GetBool(FennecConfig_FennecValue *fen) {
    if (!FennecConfig_FennecValue_IsBool(fen)) {
        return false;
    }
    
    return fen->value.b;
}

bool FennecConfig_FennecValue_IsNull(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Null;
}

bool FennecConfig_FennecValue_IsError(FennecConfig_FennecValue *fen) {
    return fen->type == FennecConfig_FennecValue_Type_Error;
}

#ifdef __cplusplus
}
#endif

#endif