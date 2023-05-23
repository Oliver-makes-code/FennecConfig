// deno-lint-ignore-file
import * as common from "./common.ts"

const TokenType = {
    identifier: "identifier",
    flag: "flag",
    string: "string",
    number: "number",
    boolean: "boolean",
    null: "null",
    symbol: "symbol",
    type: "type"
} as const

type TokenType = typeof TokenType[keyof typeof TokenType]

interface Token {
    value: string|boolean|number|null
    type: TokenType
    start: number
    end: number
} 

function* tokenize(text: string): Generator<Token, void, unknown> {
    for (let i = 0; i < text.length; i++) {
        const idx = i
        const char = text[i]
        const substr = text.substring(i)
        const matchNull = substr.match(common.NULL)
        const matchNotNull = substr.match(common.NOT_NULL)
        const matchBoolean = substr.match(common.BOOLEAN)
        const matchNotBoolean = substr.match(common.NOT_BOOLEAN)
        const matchBinary = substr.match(common.BINARY_LITERAL)
        const matchOctal = substr.match(common.OCTAL_LITERAL)
        const matchHex = substr.match(common.HEX_LITERAL)
        const matchNumber = substr.match(common.NUMBER)
        if (char.match(common.WHITESPASE)) {
            continue
        } else if (char == ":") {
            for (; i < text.length; i++) if ("=[{".includes(text[i])) break
            i--
        } else if (char == "#") {
            for (i++; i < text.length; i++) {
                if (text[i] == "\n") break
            }
        } else if (matchNull && !matchNotNull) {
            i += matchNull[0].length - 1
            yield {
                value: null,
                type: TokenType.boolean,
                start: idx,
                end: i
            }
        } else if (matchBoolean && !matchBinary && !matchNotBoolean) {
            const bool = !!matchBoolean[2]
            i += matchBoolean[0].length - 1
            yield {
                value: bool,
                type: TokenType.boolean,
                start: idx,
                end: i
            }
        } else if (matchBinary) {
            const number = parseInt(matchBinary[2], 2) * (matchBinary[1] == "0" ? 1 : -1)
            i += matchBinary[0].length - 1
            yield {
                value: number,
                type: TokenType.number,
                start: idx,
                end: i
            }
        } else if (matchOctal) {
            const number = parseInt(matchOctal[2], 8) * (matchOctal[1] == "0" ? 1 : -1)
            i += matchOctal[0].length - 1
            yield {
                value: number,
                type: TokenType.number,
                start: idx,
                end: i
            }
        } else if (matchHex) {
            const number = parseInt(matchHex[2], 16) * (matchHex[1] == "0" ? 1 : -1)
            i += matchHex[0].length - 1
            yield {
                value: number,
                type: TokenType.number,
                start: idx,
                end: i
            }
        } else if (matchNumber) {
            const number = parseFloat(matchNumber[1])
            i += matchNumber[0].length - 1
            yield {
                value: number,
                type: TokenType.number,
                start: idx,
                end: i
            }
        } else if (char.match(common.ID_START)) {
            let id = char
            for (i++; i < text.length; i++) {
                const next = text[i]
                if (next.match(common.ID_CONT)) {
                    id += next
                } else {
                    break
                }
            }
            i--
            yield {
                value: id,
                type: TokenType.identifier,
                start: idx,
                end: i
            }
        } else if ("=[]{}".includes(char)) {
            yield {
                value: char,
                type: TokenType.symbol,
                start: idx,
                end: i
            }
        } else if (substr.startsWith('-"""')) {
            let str = ""
            let closed = false
            for (i += 4; i < text.length; i++) {
                const subsubstr = text.substring(i)
                if (subsubstr.startsWith("\\") && subsubstr.length > 1) {
                    str += "\\"
                    i++
                } else if (subsubstr.startsWith('"""')) {
                    closed = true
                    break
                }
                str += text[i]
            }
            if (!closed) throw new common.ParserError("String not closed.", text, idx)
            i += 2
            yield {
                value: str.split("\n").map(value => value.trim()).join("\n").trim(),
                type: TokenType.string,
                start: idx,
                end: i
            }
        } else if (char == "-") {
            let id = ''
            for (i++; i < text.length; i++) {
                const next = text[i]
                if (next.match(common.ID_CONT)) {
                    id += next
                } else {
                    break
                }
            }
            i--
            yield {
                value: id,
                type: TokenType.flag,
                start: idx,
                end: i
            }
        } else if (substr.startsWith('"""')) {
            let str = ""
            let closed = false
            for (i += 3; i < text.length; i++) {
                const subsubstr = text.substring(i)
                if (subsubstr.startsWith("\\") && subsubstr.length > 1) {
                    str += "\\"
                    i++
                } else if (subsubstr.startsWith('"""')) {
                    closed = true
                    break
                }
                str += text[i]
            }
            if (!closed) throw new common.ParserError("String not closed.", text, idx)
            i += 2
            yield {
                value: str,
                type: TokenType.string,
                start: idx,
                end: i
            }
        } else if (char == '"') {
            let str = ""
            let closed = false
            for (i++; i < text.length; i++) {
                const next = text[i]
                if (next == "\\") {
                    str += next
                    i++
                } else if (next == '"') {
                    closed = true
                    break
                }
                str += text[i]
            }
            if (!closed) throw new common.ParserError("String not closed.", text, idx)
            if (str.includes("\n")) throw new common.ParserError("Single quoted string cannot contain newline.", text, idx)
            yield {
                value: str,
                type: TokenType.string,
                start: idx,
                end: i
            }
        } else {
            throw new common.ParserError("Undexpected character: "+char, text, idx)
        }
    }
}

function parseTree(text: string, tokens: Generator<Token, void, unknown>): any {
    let out = {} as {[key: string]: any}
    let isFirst = true
    for (const token of tokens) {
        if (isFirst) {
            if (token.type == TokenType.boolean || token.type == TokenType.null || token.type == TokenType.number) {
                const next = tokens.next()
                if (next.value) throw new common.ParserError("Found multiple values where one was expected.", text, next.value.start)
                return token.value
            }
            if (token.type == TokenType.symbol && token.value == "[")  {
                const array = parseArray(text, token, tokens)
                const next = tokens.next()
                if (next.value) throw new common.ParserError("Found multiple values where one was expected.", text, next.value.start)
                return array
            }
            if (token.type == TokenType.symbol && token.value == "{")  {
                const object = parseObject(text, token, tokens)
                const next = tokens.next()
                if (next.value) throw new common.ParserError("Found multiple values where one was expected.", text, next.value.start)
                return object
            }
        }
        if (token.type == TokenType.flag) {
            out[token.value as string] = true
            isFirst = false
            continue
        }
        if (token.type != TokenType.identifier && token.type != TokenType.string) throw new common.ParserError("Unexpected "+token.type+" found when required identifer", text, token.start)
        let next = tokens.next().value
        if (!next && isFirst && token.type == TokenType.string) return token.value
        else if (!next) throw new common.ParserError("Unexpected EOF", text, token.end)
        if (next.type == TokenType.type) {
            const prev = next
            next = tokens.next().value
            if (!next) throw new common.ParserError("Unexpected EOF", text, prev.end)
        }
        if (next.type == TokenType.symbol && (next.value == "]" || next.value == "}")) throw new common.ParserError("Found unexpected symbol in object", text, next.start)
        
        if (next.type != TokenType.symbol) throw new common.ParserError("Expected symbol in object", text, next.start)
        if (next.value == "=") {
            const val = tokens.next().value
            if (!val) throw new common.ParserError("Unexpected EOF", text, next.end)
            out[token.value as string] = val.value
        } else if (next.value == "[") {
            out[token.value as string] = parseArray(text, next, tokens)
        } else if (next.value == "{") {
            out[token.value as string] = parseObject(text, next, tokens)
        } else {
            throw new common.ParserError("Found unexpected symbol in object", text, next.start)
        }
        isFirst = false
    }
    return out
}

function parseArray(text: string, token: Token, tokens: Generator<Token, void, unknown>): any[] {
    const out: any[] = []
    let last = tokens.next()
    let lastToken: Token = token
    while (!last.done) {
        const token = last.value!
        lastToken = token
        if (token.type == TokenType.identifier){
            throw new common.ParserError("Found identifier in array", text, token.start)
        } else if (token.type == TokenType.flag) {
            throw new common.ParserError("Found identifier in array", text, token.start)
        } else if (token.type == TokenType.type) {
            throw new common.ParserError("Found type hint in array", text, token.start)
        } else if (token.type == TokenType.symbol && token.value == "]") {
            return out
        } else if (token.type == TokenType.symbol && token.value == "[") {
            out.push(parseArray(text, token, tokens))
        } else if (token.type == TokenType.symbol && token.value == "{") {
            out.push(parseObject(text, token, tokens))
        }else {
            out.push(token.value)
        }
        last = tokens.next()
    }
    throw new common.ParserError("Array not closed", text, (lastToken ?? token).end)
}

function parseObject(text: string, token: Token, tokens: Generator<Token, void, unknown>): {[key: string]: any} {
    const out = {} as {[key: string]: any}
    let last: IteratorResult<Token, void> = tokens.next()
    let lastToken = token
    while (!last.done) {
        const token = last.value!
        lastToken = token
        if (token.type == TokenType.symbol && token.value == "}") {
            return out
        }
        if (token.type == TokenType.flag) {
            out[token.value as string] = true
            continue
        }
        if (token.type != TokenType.identifier && token.type != TokenType.string) throw new common.ParserError("Unexpected "+token.type+" found when required identifer", text, token.start)
        let next = tokens.next().value
        if (!next) throw new common.ParserError("Unexpected EOF", text, token.end)
        lastToken = next
        if (next.type == TokenType.type) {
            const prev = next
            next = tokens.next().value
            if (!next) throw new common.ParserError("Unexpected EOF", text, prev.end)
            lastToken = next
        }
        if (next.type == TokenType.symbol && (next.value == "]" || next.value == "}")) throw new common.ParserError("Found unexpected symbol in object", text, next.start)
        if (next.type != TokenType.symbol) throw new common.ParserError("Expected symbol in object", text, next.start)
        if (next.value == "=") {
            const val = tokens.next().value
            if (!val) throw new common.ParserError("Unexpected EOF", text, next.end)
            lastToken = val
            out[token.value as string] = val.value
        } else if (next.value == "[") {
            out[token.value as string] = parseArray(text, next, tokens)
        } else if (next.value == "{") {
            out[token.value as string] = parseObject(text, next, tokens)
        } else {
            throw new common.ParserError("Found unexpected symbol in object", text, next.start)
        }
        last = tokens.next()
    }
    throw new common.ParserError("Object not closed", text, lastToken.end)
}

export default function parse(text: string) {
    const tokens = tokenize(text)
    return parseTree(text, tokens)
}
