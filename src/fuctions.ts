import { hexToRgb, RGBA, rgbToHex } from "@opentui/core";
import { type Dispatch, type SetStateAction } from "react";
import { cc, ptr } from "bun:ffi"

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

function getRandomInt(max: number) {
	return Math.floor(Math.random() * max)
}

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
export function randomColor(count: number, setColorsPalette: UseState<RGBA[][]>, setPosition: UseState<number>): NodeJS.Timeout {
	const colorInterval = setInterval(() => {
		const newColors: RGBA[] = [];
		for (let i = 0; i < count; i++) {
			let randomDecimal = getRandomInt(16777215);
			const isColor = /^([0-9a-f]{3}){1,2}$/i;
			const colorHex = randomDecimal.toString(16)
			let colorRgb;

			if (!isColor.test(colorHex)) {
				console.log("change")
				randomDecimal = getRandomInt(16777215)
				colorRgb = RGBA.fromHex(randomDecimal.toString(16))
			} else {
				colorRgb = hexToRgb(colorHex)
			}

			newColors.push(hexToRgb(randomDecimal.toString(16)));
		}

		setColorsPalette((prevColorsPalette) => [...prevColorsPalette, newColors])
		setPosition((pos: number) => pos + 1)
	}, 300);
	return colorInterval;
}

export function savePaletteWrapedC(palette:RGBA[], countColorsPalette:number) {

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
