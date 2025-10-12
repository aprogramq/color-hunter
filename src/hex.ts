import type { HexValue } from "./types";

export class HexColor {
	red: HexValue = this.rangeHex(0, 256);
	green: HexValue = this.rangeHex(0, 256);
	blue: HexValue = this.rangeHex(0, 256);

	randomHex(): string {
		let hex: HexValue = Math.floor(Math.random() * 255).toString(16)
		if (hex.length == 1) hex = "0" + hex
		return hex
	}
	rangeHex(min: number, max: number): string {
		let hex: HexValue = Math.floor(Math.random() * (max - min) + min).toString(16)
		if (hex.length == 1) hex = "0" + hex
		return hex
	}

	get(): string {
		return "#" + this.red + this.green + this.blue
	}

	correctChanels(PrevHexColor: HexColor) { }

}

export class HexColorCold extends HexColor {
	constructor() {
		super()
		this.red = this.rangeHex(0, 135);
		this.green = this.rangeHex(0, 135);
		this.blue = this.rangeHex(150, 256);
	}

	override correctChanels(PrevHexColor: HexColor) {
		const intRed = parseInt(this.red, 16);
		const intGreen = parseInt(this.green, 16)
		const intBlue = parseInt(this.blue, 16)

		if (intRed - parseInt(PrevHexColor.red, 16) < 75)
			this.red = (intRed + parseInt(this.rangeHex(75, 100), 16)).toString(16)

		if (intGreen - parseInt(PrevHexColor.green, 16) < 40)
			this.green = (intGreen + parseInt(this.rangeHex(40, 100), 16)).toString(16)

		if (intBlue < intRed || intBlue < intGreen) {
			this.blue = this.rangeHex(0, 256 - intBlue)
		}


		console.log(`red: ${this.red} green:${this.green} blue:${this.blue}`)
	}
}
export class HexColorWarm extends HexColor {
	constructor() {
		super()
		this.red = this.rangeHex(200, 256);
		this.green = this.rangeHex(50, 120);
		this.blue = "30";
	}

	override correctChanels(PrevHexColor: HexColor) {
		const intRed = parseInt(this.red, 16);
		const intGreen = parseInt(this.green, 16)
		const intBlue = parseInt(this.blue, 16)

		// if (intRed < parseInt(PrevHexColor.red, 16)) {
		//
		// 	this.red = this.rangeHex(intRed, 255-(parseInt(PrevHexColor.red, 16) ))
		// 	if (parseInt(this.red, 16) < 10)
		// 		this.red = "0" + this.red
		// 	console.log(parseInt(this.red,16 ))
		// }
		// if (parseInt(this.red, 16) - parseInt(PrevHexColor.red, 16) < 75)
		// 	this.red = (parseInt(this.red, 16) + parseInt(this.rangeHex(75, 100), 16)).toString(16)
		//
		// if (parseInt(this.green, 16) - parseInt(PrevHexColor.green, 16) < 40)
		// 	this.green = (parseInt(this.green, 16) + parseInt(this.rangeHex(40, 100), 16)).toString(16)
	}
}
export class HexColorPastele extends HexColor {
	constructor() {
		super()
		this.red = this.rangeHex(150, 256);
		this.green = this.rangeHex(150, 256);
		this.blue = this.rangeHex(150, 256);
	}

	override correctChanels(PrevHexColor: HexColor) {
	}
}


