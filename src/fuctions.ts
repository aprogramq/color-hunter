import { dlopen, ptr } from 'bun:ffi'
import { BaseHexColor, HexColor, HexColorCold, HexColorPastele, HexColorWarm } from './hex'
import type { OptionValue, paletteT } from './types'
import fs from 'fs'
import type { UseState } from './types'

export function randomColor(
	palette:any,
	setPalette: UseState<paletteT>,
	option: OptionValue
): NodeJS.Timeout {
	const colorInterval = setInterval(() => {
		const newColors: BaseHexColor[] = []
		for (let i = 0; i < palette.size; i++) {
			let color = new HexColor()

			if (option == 'cold') color = new HexColorCold()
			else if (option == 'pastel') color = new HexColorPastele()
			else if (option == 'warm') color = new HexColorWarm()

			newColors.push(color)
			if (newColors.length > 1) {
				color.correctChanels(newColors[i - 1]!)
			}
		}
		setPalette((p) => ({ ...p, colors: [...p.colors, newColors] }))
		setPalette((p) => ({...p, position: p.position + 1 }))
	}, 300)
	return colorInterval
}

export function savePaletteWrapedC(palette: BaseHexColor[], countColorsPalette: number): number {
	const { canvasSo } = require('./c/canvas.ts')
	const libData = Buffer.from(canvasSo, 'base64')
	if (!fs.existsSync('/tmp/canvas_lib.so'))
		fs.writeFileSync('/tmp/canvas_lib.so', libData, { encoding: 'binary', mode: 0o775 })
	const cLib = dlopen('/tmp/canvas_lib.so', {
		save_palette: {
			returns: 'int',
			args: ['int', 'pointer', 'pointer'],
		},
	})
	const paletteHex: ArrayLike<string>[] = []
	palette.forEach((hex) => paletteHex.push(hex.get().replace('#', '')))
	const buffersPalette = paletteHex.map((str) => {
		const buf = new Uint8Array(Buffer.from(str + '\0'))
		return buf
	})
	const pointerPalette = buffersPalette.map((buf) => ptr(buf))
	const palettePtr = ptr(new BigUint64Array(pointerPalette.map((p) => BigInt(p))))

	const textColors: ArrayLike<string>[] = []
	palette.forEach((color) => textColors.push(color.textColor.replace('#', '')))
	const buffersTextColors = textColors.map((str) => {
		const buf = new Uint8Array(Buffer.from(str + '\0'))
		return buf
	})
	const pointerTextColors = buffersTextColors.map((buf) => ptr(buf))
	const textColorPtr = ptr(new BigUint64Array(pointerTextColors.map((p) => BigInt(p))))

	cLib.symbols.save_palette(countColorsPalette, palettePtr, textColorPtr)
	return 1
}
