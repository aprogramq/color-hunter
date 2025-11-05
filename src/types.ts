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
  { name: 'default', description: '', value: 'default' },
  { name: 'cold color', description: '', value: 'cold' },
  { name: 'warm color', description: '', value: 'warm' },
  { name: 'pastel color', description: '', value: 'pastel' },
]
export interface sizeT {
  height: number
  width: number
}
export type displayT = 'start' | 'options' | 'main'
export type optionsT = [{ name: string; description: string; value: string }]
export type UseState<T> = Dispatch<SetStateAction<T>>
export type paletteT = {colors: BaseHexColor[][], size:number, display:boolean, position:number}
