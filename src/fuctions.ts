import { type Dispatch, type SetStateAction } from "react";
import { dlopen, ptr } from "bun:ffi"
import { BaseHexColor, HexColor, HexColorCold, HexColorPastele, HexColorWarm } from "./hex";
import type { OptionValue } from "./types";
import { canvasSo } from "./c/canvas.ts";
import fs from "fs"

// export const {
// 	symbols: { save_palette },
// } = cc({
// 	source: "/home/aprogramb/projects/javascript/color-hunter/src/c/canvas.c",
// 	library: "cairo",
// 	include: ["/usr/include/cairo", "/usr/include/libpng16", "/usr/include/freetype2", "/usr/include/pixman-1"],
// 	symbols: {
// 		save_palette: {
// 			returns: "int", args: ["int", "pointer", "pointer"],
// 		},
// 	},
// })
const libData = Buffer.from(canvasSo, "base64");
fs.writeFileSync("/tmp/canvas_lib.so", libData, { encoding: "binary" })
const cLib = dlopen("/tmp/canvas_lib.so", {
	save_palette: {
		returns: "int", args: ["int", "pointer", "pointer"],
	},

})

type UseState<T> = Dispatch<SetStateAction<T>>;

export function load(setLoaderValue: UseState<string>) {
	let currentPosition: number = 0;
	const loadStadies = ["|", "/", "-"];
	const loaderSpiner = setInterval(() => {
		setLoaderValue(`Loading ${loadStadies[currentPosition]}`);
		if (currentPosition == 2) currentPosition = 0;
		else currentPosition += 1;
	}, 300);
	return loaderSpiner;

}
export function randomColor(count: number, setColorsPalette: UseState<BaseHexColor[][]>, setPosition: UseState<number>, option: OptionValue): NodeJS.Timeout {
	const colorInterval = setInterval(() => {

		const newColors: BaseHexColor[] = [];
		for (let i = 0; i < count; i++) {
			let color = new HexColor;

			if (option == "cold")
				color = new HexColorCold
			else if (option == 'pastel')
				color = new HexColorPastele
			else if (option == 'warm')
				color = new HexColorWarm

			newColors.push(color);
			if (newColors.length > 1) {
				color.correctChanels(newColors[i - 1]!)
			}
		}
		setColorsPalette((prevColorsPalette) => [...prevColorsPalette, newColors])
		setPosition((pos: number) => pos + 1)
	}, 300);
	return colorInterval;
}

export function savePaletteWrapedC(palette: BaseHexColor[], countColorsPalette: number): number {

	const paletteHex: ArrayLike<string>[] = []
	palette.forEach((hex) => paletteHex.push(hex.get().replace('#', '')))
	const buffersPalette = paletteHex.map(str => {
		const buf = new Uint8Array(Buffer.from(str + "\0"))
		return buf;
	})
	const pointerPalette = buffersPalette.map(buf => ptr(buf))
	const palettePtr = ptr(new BigUint64Array(pointerPalette.map(p => (BigInt(p)))))

	const textColors: ArrayLike<string>[] = [];
	palette.forEach((color) => textColors.push(color.textColor.replace('#', '')))
	const buffersTextColors = textColors.map(str => {
		const buf = new Uint8Array(Buffer.from(str + "\0"))
		return buf;
	})
	const pointerTextColors = buffersTextColors.map(buf => ptr(buf))
	const textColorPtr = ptr(new BigUint64Array(pointerTextColors.map(p => (BigInt(p)))))


	return cLib.symbols.save_palette(countColorsPalette, palettePtr, textColorPtr)
}
