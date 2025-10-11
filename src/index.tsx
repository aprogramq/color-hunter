import { RGBA, TextAttributes } from '@opentui/core'
import { render, useKeyboard, useRenderer } from '@opentui/react'
import { useEffect, useReducer, useState } from 'react'
import { randomColor, savePaletteWrapedC } from './fuctions'
import { Palette } from './components/Colors'
import type { ControlWheelProp, ActionArgs } from './types'

function App() {
	const [colorsPalette, setColorsPalette] = useState<RGBA[][]>([[]])
	const [countColorsPalette, setCountColorsPalette] = useState<number>(3)
	const [position, setPosition] = useState<number>(0)
	const render = useRenderer()
	// useEffect(() => {
	// 	render.console.show()
	// 	render.terminalHeight
	// })
	const height:number = render.height
	const [state, dispatch] = useReducer(colorReducer, {
		pause: true,
		timeout: null,
		displayColors: false,
	})

	function colorReducer(state: ControlWheelProp, action: ActionArgs): ControlWheelProp {
		if (action.type === 'pauseColorWheel') {
			return {
				...state,
				pause: true,
				timeout: state.timeout?.close() ?? null,
			}
		} else if (action.type === 'startColorWheel' || action.type === 'continueColorWheel') {
			if (action.type === 'continueColorWheel') setPosition(colorsPalette.length - 1)
			return {
				timeout: randomColor(countColorsPalette, setColorsPalette, setPosition),
				displayColors: true,
				pause: false,
			}
		} else if (action.type === 'savePalette') {
			savePaletteWrapedC(colorsPalette[position]!, countColorsPalette)
			return { ...state, displayColors: true }
		}
		return state
	}

	useKeyboard((key) => {
		if (key.name === 'space') {
			dispatch({ type: 'pauseColorWheel' })
		} else if (key.name === 's') {
			if (!state.displayColors) dispatch({ type: 'startColorWheel' })
			else if (state.pause && state.displayColors) dispatch({ type: 'savePalette' })
		} else if (key.name === 'c' && state.displayColors && state.pause) {
			dispatch({ type: 'continueColorWheel' })
		} else if (key.name === 'b' && state.pause && position > 1) {
			setPosition((pos) => pos - 1)
		} else if (key.name === 'n' && state.pause && position < colorsPalette.length - 1) {
			setPosition((pos) => pos + 1)
		}
	})

	return (
		<box alignItems="center" justifyContent="center" flexGrow={1}>
			<box justifyContent="center" alignItems="flex-end">
				<ascii-font font="tiny" text="Color Hunter :D" />
				<text attributes={TextAttributes.DIM} fg="#7c86ff">
					You’ll find what you need
				</text>
				{/* <text>Position:{position}</text> */}
			</box>

			{!state.displayColors && (
				<box width={50} justifyContent="center" alignItems="center">
					<box backgroundColor={'white'} padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
						<text fg="#000000">[S]tart</text>
					</box>
				</box>
			)}
			{state.displayColors && <Palette colorsPalette={colorsPalette} position={position} />}

			{/*   {state.pause && position > 0 && state.displayColors && (             */}
			{/* )} */}
			{/*   {state.pause && position < colorsPalette.length - 1 && state.displayColors && (             */}
			{/* )} */}
			{state.pause && state.displayColors && (
				<>
					{/* TODO: Add logic to arrows  */}
					<text fg="#ffffff" position="absolute" left={25} top={height / 2}>{`<-- [B]ack`}</text>
					<text fg="#ffffff" position="absolute" right={25} top={height / 2}>{`[N]ext -->`}</text>

					<box flexDirection="row">
						<box backgroundColor="#000000" padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
							<text fg="#ffffff">[C]ontinue</text>
						</box>
						<box backgroundColor={'#000000'} padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
							<text fg="#ffffff">[S]ave palette</text>
						</box>
					</box>
				</>
			)}
			{!state.pause && state.displayColors ? (
				<box backgroundColor="#000000" padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
					<text fg="#ffffff">Press [space] to stop</text>
				</box>
			) : null}
		</box>
	)
}

render(<App />)
