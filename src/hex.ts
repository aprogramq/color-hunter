import type { HexValue } from './types'

export class BaseHexColor {
    limitBrightness: number = 130
    textColor: string = this.correctTextColor()
    red: HexValue = '00'
    green: HexValue = '00'
    blue: HexValue = '00'

    get(): HexValue {
        return '#' + this.red + this.green + this.blue
    }

    correctTextColor(): HexValue {
        const r = parseInt(this.red, 16)
        const g = parseInt(this.green, 16)
        const b = parseInt(this.blue, 16)
        const brightness = (r * 299 + g * 587 + b * 114) / 1000
        return brightness > 125 ? '#333333' : '#ffffff'
    }

    setRgb(r: number, g: number, b: number) {
        this.red = toHex(r)
        this.green = toHex(g)
        this.blue = toHex(b)
    }
    correctChanels(PrevHexColor: BaseHexColor) {}
}

export class HexColor extends BaseHexColor {
    constructor() {
        super()
        const hue = Math.random()
        const saturation = Math.random()
        const lightness = Math.random()
        const [r, g, b] = hslToRgb(hue, saturation, lightness)
        this.setRgb(r, g, b)
        this.textColor = this.correctTextColor()
    }
}

export class HexColorCold extends HexColor {
    constructor() {
        super()
        const hue = Math.random() * 1
        const saturation = Math.random() * 0.3
        const lightness = Math.random() * (0.5 - 0.1) + 0.1
        const [r, g, b] = hslToRgb(hue, saturation, lightness)
        this.setRgb(r, g, b)
        this.textColor = this.correctTextColor()
    }
    override correctChanels(PrevHexColor: BaseHexColor): void {}
}
export class HexColorWarm extends HexColor {
    constructor() {
        super()
        const hue = Math.random() * (0.11 - 0.03) + 0.03
        const saturation = Math.random() * (0.8 - 0.1) + 0.1
        const lightness = Math.random() * (0.8 - 0.1) + 0.1
        const [r, g, b] = hslToRgb(hue, saturation, lightness)
        this.setRgb(r, g, b)
        this.textColor = this.correctTextColor()
    }
}
export class HexColorPastele extends HexColor {
    constructor() {
        super()
        const hue = Math.random()
        const saturation = Math.random() * (0.5 - 0.4) + 0.4
        const lightness = Math.random() * (0.85 - 0.7) + 0.7
        const [r, g, b] = hslToRgb(hue, saturation, lightness)
        this.setRgb(r, g, b)
        this.textColor = `#${(r - 100).toString(16)}${(g - 100).toString(16)}${(b - 100).toString(16)}`
    }
}

function hslToRgb(h: number, s: number, l: number): [number, number, number] {
    let r, g, b

    if (s === 0) {
        r = g = b = l
    } else {
        const hue2rgb = (p: number, q: number, t: number) => {
            if (t < 0) t += 1
            if (t > 1) t -= 1
            if (t < 1 / 6) return p + (q - p) * 6 * t
            if (t < 1 / 2) return q
            if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6
            return p
        }

        const q = l < 0.5 ? l * (1 + s) : l + s - l * s
        const p = 2 * l - q
        r = hue2rgb(p, q, h + 1 / 3)
        g = hue2rgb(p, q, h)
        b = hue2rgb(p, q, h - 1 / 3)
    }

    return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)]
}

function toHex(n: number): HexValue {
    const hex = n.toString(16)
    return hex.length === 1 ? '0' + hex : hex
}
