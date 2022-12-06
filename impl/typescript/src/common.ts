export const ID_START = /[a-z$_]/gi
export const ID_CONT = /[a-z0-9$_\-]/gi
export const ACCEPTABLE_ID = /^[a-z$_][a-z0-9$_\-]*$/gi
export const WHITESPASE = /\s/g
export const BOOLEAN = /^(([Tt]rue|1b)|([Ff]alse|0b))/
export const NOT_BOOLEAN = /^(([Tt]rue|1b)|([Ff]alse|0b))[a-z0-9$_\-]/
export const BINARY_LITERAL = /^([01])b([01]+)/i
export const OCTAL_LITERAL = /^([)01])o([0-7]+)/i
export const HEX_LITERAL = /^([01])x([)0-9a-f]+)/i
export const NUMBER = /^(\-?[0-9]+(\.[0-9]+)?)/
export const NULL = /^(null|nil|void)/
export const NOT_NULL = /^(null|nil|void)[a-z0-9$_\-]/
export const COLOR_CODES = {
    reset: "\u001b[0m",
    red:   "\u001b[31m",
    blue:  "\u001b[34m"
}

export class ParserError extends Error {
    constructor(message: string, text: string, start: number) {
        let row = 0
        let col = 0
        for (let i = 0; i < start; i++) {
            if (text[i] == "\n") {
                col = 0
                row++
            } else col++
        }
        super(message + "\n    Line: "+(row+1)+", Column: "+col+"\n        "+COLOR_CODES.blue+text.split("\n")[row]+"\n        "+" ".repeat(col)+COLOR_CODES.red+"^ Here"+COLOR_CODES.reset)
    }
}