import { useKeyboard } from "@opentui/react"
import { randomColor, savePaletteWrapedC } from "./fuctions"
import { options } from "./types"
import { stat } from "node:fs"

export function useKeyboardState(state: any) {
	useKeyboard((key) => {
		if (key.name === 'space') {
			state.setPause(true)
			state.setTimeout((to: NodeJS.Timeout) => to?.close() ?? null)
		} else if (key.name === 's') {
			if (!state.displayColors) {
				state.setTimeout(
					randomColor(
						state.countColorsPalette,
						state.setColorsPalette,
						state.setPosition,
						options[state.selectedIndex]!.value
					)
				)
				state.setDisplayColors(true)
				state.setPause(false)
			} else if (state.displayColors) {
				if (savePaletteWrapedC(state.colorsPalette[state.position]!, state.countColorsPalette)) {
					state.setDisplaySaveMessage(true);
					setTimeout(() => state.setDisplaySaveMessage(false), 2000)
				}
				state.setDisplayColors(true)
			}
		} else if (key.name === 'c' && state.displayColors && state.pause) {
			state.setTimeout(
				randomColor(
					state.countColorsPalette,
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
			state.setTimeout((to: NodeJS.Timeout) => to?.close() ?? null)
		}
		else if (key.name == 'o' && !state.displayColors)
			state.enterOptions()
	}

	)
}
