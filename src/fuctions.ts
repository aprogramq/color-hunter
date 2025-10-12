import { hexToRgb, RGBA, rgbToHex } from "@opentui/core";
import { type Dispatch, type SetStateAction } from "react";
import { cc, ptr } from "bun:ffi"
import { HexColor, HexColorCold, HexColorPastele, HexColorWarm } from "./hex";
import type { OptionValue } from "./types";

export const {
	symbols: { save_palette },
} = cc({
	source: "/home/aprogramb/projects/color-hunter/src/c/canvas.c",
	library: "cairo",
	include: ["/usr/include/cairo", "/usr/include/libpng16", "/usr/include/freetype2", "/usr/include/pixman-1"],
	symbols: {
		save_palette: {
			returns: "void", args: ["int", "pointer"],
		},
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
export function randomColor(count: number, setColorsPalette: UseState<RGBA[][]>, setPosition: UseState<number>, option:OptionValue): NodeJS.Timeout {
	const colorInterval = setInterval(() => {
		const newColors: HexColor[] = [];
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

		const rgbColors: RGBA[] = newColors.map((color) => hexToRgb(color.get()))
		setColorsPalette((prevColorsPalette) => [...prevColorsPalette, rgbColors])
		setPosition((pos: number) => pos + 1)
	}, 300);
	return colorInterval;
}

export function savePaletteWrapedC(palette: RGBA[], countColorsPalette: number) {

	const paletteHex: ArrayLike<string>[] = []
	palette.forEach((rgb) => paletteHex.push(rgbToHex(rgb).replace('#', '')))
	const buffers = paletteHex.map(str => {
		const buf = new Uint8Array(Buffer.from(str + "\0"))
		return buf;
	})

	const pointers = buffers.map(buf => ptr(buf))
	const arrayPtr = ptr(new BigUint64Array(pointers.map(p => (BigInt(p)))))

	save_palette(countColorsPalette, arrayPtr)
}
