#ifndef __FENNEC_HEADER__
#define __FENNEC_HEADER__

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>

typedef enum {
    NullValue = -1,
    NullFileName = 0,
    BrokenFileName = 1,
    FileError = 2,
    FennecParse = 3
} FennecConfig_ParseError;

typedef void FennecConfig_ParseResult;

typedef void FennecConfig_FennecType;

FennecConfig_ParseResult *FennecConfig_LoadFile(const char *filename);

/// @brief Drops itself and all children. Only call on the root object.
void FennecConfig_ParseResult_Drop(const FennecConfig_ParseResult *result);

bool FennecConfig_ParseResult_IsErr(const FennecConfig_ParseResult *result);

bool FennecConfig_ParseResult_IsOk(const FennecConfig_ParseResult *result);

FennecConfig_ParseError FennecConfig_ParseResult_GetErr(const FennecConfig_ParseResult *result);

/// @brief Make sure to null check!
FennecConfig_FennecType *FennecConfig_ParseResult_GetOk(const FennecConfig_ParseResult *result);

bool FennecConfig_FennecType_IsObject(const FennecConfig_FennecType *fen);

bool FennecConfig_FennecType_Object_HasKey(const FennecConfig_FennecType *fen, const char *key);

/// @brief Make sure to null check!
FennecConfig_FennecType *FennecConfig_FennecType_Object_GetKey(const FennecConfig_FennecType *fen, const char *key);

bool FennecConfig_FennecType_IsArray(const FennecConfig_FennecType *fen);

size_t FennecConfig_FennecType_Array_Len(const FennecConfig_FennecType *fen);

/// @brief Make sure to null check!
FennecConfig_FennecType *FennecConfig_FennecType_Array_GetIdx(const FennecConfig_FennecType *fen, size_t idx);

bool FennecConfig_FennecType_IsNumber(const FennecConfig_FennecType *fen);

bool FennecConfig_FennecType_IsInt(const FennecConfig_FennecType *fen);

long FennecConfig_FennecType_GetInt(const FennecConfig_FennecType *fen);

/// @brief Gets an int as a float. Used in FennecConfig_FennecType_GetFloat when it's not found as a float.
double FennecConfig_FennecType_GetInt_Float(const FennecConfig_FennecType *fen);

bool FennecConfig_FennecType_IsFloat(const FennecConfig_FennecType *fen);

double FennecConfig_FennecType_GetFloat(const FennecConfig_FennecType *fen);

bool FennecConfig_FennecType_IsString(const FennecConfig_FennecType *fen);

/// @brief Make sure to null check! 
char *FennecConfig_FennecType_GetString(const FennecConfig_FennecType *fen);

bool FennecConfig_FennecType_IsBool(const FennecConfig_FennecType *fen);

bool FennecConfig_FennecType_GetBool(const FennecConfig_FennecType *fen);

#ifdef __cplusplus
}
#endif

#endif