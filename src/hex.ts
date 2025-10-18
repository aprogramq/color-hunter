import type { HexValue } from "./types";

export class BaseHexColor {

	textColor: string = "#ffffff";
	red: HexValue = "00"
	green: HexValue = "00"
	blue: HexValue = "00";

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

	get(): HexValue {
		return "#" + this.red + this.green + this.blue
	}

	correctChanels(PrevHexColor: HexColor) { }
}

export class HexColor extends BaseHexColor {
	constructor() {
		super()
		this.red = this.rangeHex(0, 256);
		this.green = this.rangeHex(0, 256);
		this.blue = this.rangeHex(0, 256);
	}


	override correctChanels(PrevHexColor: HexColor) { }

}

export class HexColorCold extends HexColor {
	constructor() {
		super()
		this.red = this.rangeHex(50, 128);
		this.green = this.rangeHex(50, 128);
		this.blue = this.rangeHex(100, 256);
	}

	override correctChanels(PrevHexColor: HexColor) {
		const intRed = parseInt(this.red, 16);
		const intGreen = parseInt(this.green, 16)
		const intBlue = parseInt(this.blue, 16)

		// if (intRed - parseInt(PrevHexColor.red, 16) < 75)
		// 	this.red = (intRed + parseInt(this.rangeHex(75, 100), 16)).toString(16)

		if (intGreen - parseInt(PrevHexColor.green, 16) < 40)
			this.green = (intGreen + parseInt(this.rangeHex(40, 100), 16)).toString(16)

		if (intBlue < intRed || intBlue < intGreen) {
			this.blue = this.rangeHex(0, 256 - intBlue)
		}


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
	}
}
export class HexColorPastele extends HexColor {
	constructor() {
		super()
		this.red = this.rangeHex(150, 256);
		this.green = this.rangeHex(150, 256);
		this.blue = this.rangeHex(150, 256)
		this.textColor = `#${(parseInt(this.red, 16) - 100).toString(16)}${(parseInt(this.green, 16) - 100).toString(16)}${(parseInt(this.blue, 16) - 100).toString(16)}`
	}

	override correctChanels(PrevHexColor: HexColor) {
	}

}


