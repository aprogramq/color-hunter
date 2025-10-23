import { useKeyboard } from "@opentui/react"
import { randomColor, savePaletteWrapedC } from "./fuctions"
import { options } from "./types"
import { stat } from "node:fs"
import { useState } from "react"

export function useKeyboardStart(state: any) {
	const [timeout, setMyTimeout] = useState<NodeJS.Timeout>()
	useKeyboard((key) => {
		if (key.name === 'space') {
			state.setPause(true)
			setMyTimeout((to: NodeJS.Timeout) => to?.close() ?? null)
		} else if (key.name === 's') {
			if (!state.displayColors) {
				setMyTimeout(
					randomColor(
						state.sizeColorsPalette,
						state.setColorsPalette,
						state.setPosition,
						options[state.selectedIndex]!.value
					)
				)
				state.setDisplayColors(true)
				state.setPause(false)
			} else if (state.displayColors) {
				if (savePaletteWrapedC(state.colorsPalette[state.position]!, state.sizeColorsPalette)) {
					state.setDisplayModal(true);
					setTimeout(() => state.setDisplayModal(false), 2000)
				}
				state.setDisplayColors(true)
			}
		} else if (key.name === 'c' && state.displayColors && state.pause) {
			setMyTimeout(
				randomColor(
					state.sizeColorsPalette,
					state.setColorsPalette,
					state.setPosition,
					options[state.selectedIndex]!.value
				)
			)
			state.setPosition(state.colorsPalette.length - 1)
			state.setDisplayColors(true)
			state.setPause(false)
		} else if (key.name === 'b' && state.pause && state.position > 1) {
			state.setPosition((pos: number) => pos - 1)
		} else if (
			key.name === 'n' &&
			state.pause &&
			state.position < state.colorsPalette.length - 1
		) {
			state.setPosition((pos: number) => pos + 1)
		} else if (key.name === 'e' && state.displayColors) {
			state.setPosition(0)
			state.setColorsPalette([[]])
			state.setDisplayColors(false)
			state.setSelectedIndex(0)
			setMyTimeout((to: NodeJS.Timeout) => to?.close() ?? null)
		}
		else if (key.name == 'o' && !state.displayColors)
			state.enterOptions()
	}

	)
}
