import { type Dispatch, type SetStateAction } from 'react'
import type { BaseHexColor } from './hex'
export interface ControlWheelProp {
	pause: boolean
	timeout: NodeJS.Timeout | null
	displayColors: boolean
}
export type ActionArgs =
	| { type: 'pauseColorWheel' }
	| { type: 'startColorWheel' }
	| { type: 'continueColorWheel' }
	| { type: 'savePalette' }
	| { type: 'backToStartScreen' }

export type HexValue = string

export type OptionValue = string | 'default' | 'cold' | 'warm' | 'pastel'

export const options = [
	{ name: 'Random Colors', description: '', value: 'default' },
	{ name: 'Cold Colors', description: '', value: 'cold' },
	{ name: 'Warm Colors', description: '', value: 'warm' },
	{ name: 'Pastel Colors', description: '', value: 'pastel' },
]
export interface sizeT {
	height: number
	width: number
}
export type displayT = 'start' | 'options' | 'main'
export type optionsT = [{ name: string; description: string; value: string }]
export type UseState<T> = Dispatch<SetStateAction<T>>
export type paletteT = { colors: BaseHexColor[][]; size: number; display: boolean; position: number }
export type modalState = 'success' | 'error'

export type keyboardMainT = {
	palette: paletteT
	setPalette: UseState<paletteT>
	selectedIndex: number
	setSelectedIndex: UseState<number>
	pause: boolean
	setPause: UseState<boolean>
	actionOptions: (option: displayT) => void
	setDisplayModal: UseState<boolean>
	timeout: NodeJS.Timeout
	setMyTimeout: UseState<NodeJS.Timeout>
}
export type keyboardOptionsT = {
	selectionMode: boolean
	setSelectionMode: UseState<boolean>
	setPositionFocusedInput: UseState<number>
	saveNewOptions: () => void
	actionOptions: (display: displayT) => void
}

export type modalT = { display: boolean; text: string; state: modalState }
export type newOption = 'savePath' | 'sizePalette'
