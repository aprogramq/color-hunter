import { useKeyboard } from '@opentui/react'
import { randomColor, savePaletteWrapedC } from './fuctions.ts'
import {
	options,
	type displayT,
	type optionsT,
	type paletteT,
	type keyboardMainT,
	type UseState,
	type keyboardOptionsT,
} from './types'
import { useEffect, useState } from 'react'
import type { DiagnosticMessageChain } from 'typescript'
import { ConsolePosition } from '@opentui/core'

export function useKeyboardMain(state: keyboardMainT) {
	useKeyboard((key) => {
		switch (key.name) {
			case 'space':
				state.setPause(true)
				clearTimeout(state.timeout)
				break
			case 's':
				if (savePaletteWrapedC(state.palette.colors[state.palette.position]!, state.palette.size)) {
					state.setDisplayModal(true)
					setTimeout(() => state.setDisplayModal(false), 2000)
				}
				state.setPalette((p) => ({ ...p, display: true }))

				break
			case 'c':
				if (state.palette.display && state.pause) {
					state.setMyTimeout(
						randomColor(state.palette, state.setPalette, options[state.selectedIndex]!.value)
					)

					state.setPalette((p) => ({ ...p, position: p.colors.length - 1, display: true }))
					state.setPause(false)
				}
				break
			case 'b':
				if (state.pause && state.palette.position > 1)
					state.setPalette((p) => ({ ...p, position: p.position - 1 }))
				break
			case 'n':
				if (state.pause && state.palette.position < state.palette.colors.length - 1)
					state.setPalette((p) => ({ ...p, position: p.position + 1 }))
				break
			case 'e':
				state.setPalette((p) => ({ ...p, position: 0, colors: [[]], display: false }))
				state.setSelectedIndex(0)
				clearTimeout(state.timeout)
				state.actionOptions('start')
				break
			case 'o':
				if (!state.palette.display) state.actionOptions('options')
				break
		}
	})
}

export function useKeyboardStart(actionOptions: (option: displayT) => void) {
	useKeyboard((key) => {
		if (key.name === 's') actionOptions('main')
		if (key.name === 'o') actionOptions('options')
	})
}

export function useKeyboardOptions({
	selectionMode,
	setSelectionMode,
	setPositionFocusedInput,
	saveNewOptions,
	actionOptions,
}: keyboardOptionsT) {
	useKeyboard((key) => {
		if (key.name === 'j' || (key.name === 'down' && selectionMode))
			setPositionFocusedInput((pos) => (pos > 1 ? 1 : pos + 1))
		else if (key.name === 'k' || (key.name === 'up' && selectionMode))
			setPositionFocusedInput((pos) => (pos > 1 ? pos - 1 : pos))
		else if (key.name === 'escape') setSelectionMode(true)
		else if (key.name === 'return' && selectionMode) setSelectionMode(false)
		else if (key.name === 'return' && !selectionMode) saveNewOptions()
		else if (key.name === 'e' && selectionMode) actionOptions('start')
	})
}
