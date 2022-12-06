// deno-lint-ignore-file
import * as common from "./common.ts"

function stiringifyNode(value: any, showType: boolean, indent: number, level: number): string {
    if (Array.isArray(value)) {
        if (value.length == 0) return "[]"
        let output = "[\n"
        for (const i of value) {
            output += " ".repeat(indent*(level+1))+stiringifyNode(i, showType, indent, level+1)+"\n"
        }
        output += " ".repeat(indent*level)+"]"
        return output
    } else if (typeof value == "object" && value) {
        if (Object.keys(value).length == 0) return "{}"
        let output = "{\n"
        output += stringify(value, showType, indent, level+1)
        output += " ".repeat(indent*level)+"}"
        return output
    } else if (typeof value == "string" && value.includes("\n")) {
        return `"""${value}"""`
    }
    return JSON.stringify(value)
}

export default function stringify(values: any, showType = true, indent = 4, level = 0): string {
    if (typeof values != "object" || Array.isArray(values) || !values) return stiringifyNode(values, showType, indent, level)
    let output = ""
    for (const key in values) {
        output += " ".repeat(indent*level)
        if (key.includes("\n"))
            output += `"""${key}"""`
        else if (!common.ACCEPTABLE_ID.test(key))
            output += `"${key}"`
        else
            output += key
        const value = values[key]
        if (typeof value == "object" && value) {
            if (showType && Array.isArray(value) && value.length > 0) {
                const firstType = typeof value[0]
                let isHomogeneous = true
                for (const i of value) {
                    if (typeof i != firstType) {
                        isHomogeneous = false
                        break
                    }
                }
                if (isHomogeneous)
                    output += `: ${firstType}`
            }
            output += ` ${stiringifyNode(value, showType, indent, level)}\n`
        } else {
            if (showType && value)
                output += `: ${typeof value}`
            output += ` = ${stiringifyNode(value, showType, indent, level)}\n`
        }
    }
    return output
}