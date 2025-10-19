export interface ControlWheelProp {
	pause: boolean,
	timeout: NodeJS.Timeout | null
	displayColors: boolean
}
export type ActionArgs =
	{ type: "pauseColorWheel" } |
	{ type: "startColorWheel" } |
	{ type: "continueColorWheel" } |
	{ type: "savePalette" } |
	{ type: "backToStartScreen" }

export type HexValue = string;

export type OptionValue = string | "default" | "cold" | "warm" | "pastel"

export const options = [
	{ name: 'Default', description: '', value: 'default' },
	{ name: 'Cold color', description: '', value: 'cold' },
	{ name: 'Warm color', description: '', value: 'warm' },
	{ name: 'Pastel color', description: '', value: 'pastel' },
]
