import { hexToRgb, RGBA, TextAttributes } from '@opentui/core'
import { render, useKeyboard, useRenderer } from '@opentui/react'
import { useEffect, useReducer, useState } from 'react'
import { randomColor, savePaletteWrapedC } from './fuctions'
import { Palette } from './components/Colors'
import type { ControlWheelProp, ActionArgs } from './types'
import type { BaseHexColor } from './hex'

function App() {
	const [colorsPalette, setColorsPalette] = useState<BaseHexColor[][]>([[]])
	const [countColorsPalette, setCountColorsPalette] = useState<number>(3)
	const [position, setPosition] = useState<number>(0)
	const [selectedIndex, setSelectedIndex] = useState(0)
	const render = useRenderer()

	const options = [
		{ name: 'Default', description: '', value: 'default' },
		{ name: 'Cold color', description: '', value: 'cold' },
		{ name: 'Warm color', description: '', value: 'warm' },
		{ name: 'Pastel color', description: '', value: 'pastel' },
	]

	const height: number = render.height
	const width: number = render.width

	// useEffect(() => {
	// 	render.console.show()
	// })
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
				timeout: randomColor(
					countColorsPalette,
					setColorsPalette,
					setPosition,
					options[selectedIndex]!.value
				),
				displayColors: true,
				pause: false,
			}
		} else if (action.type === 'savePalette') {
			savePaletteWrapedC(colorsPalette[position]!, countColorsPalette)
			return { ...state, displayColors: true }
		} else if (action.type === 'backToStartScreen') {
			setPosition(0)
			setColorsPalette([[]])
			return { ...state, displayColors: false, timeout: state.timeout?.close() ?? null }
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
		} else if (key.name === 'e' && state.displayColors) {
			dispatch({ type: 'backToStartScreen' })
		}
	})

	return (
		<box alignItems="center" justifyContent="center" flexGrow={1}>
			<box justifyContent="center" alignItems="flex-end">
				<ascii-font font="tiny" text="Color Hunter :D" />
				<text attributes={TextAttributes.DIM} fg="#7c86ff">
					You’ll find what you need
				</text>
			</box>

			{!state.displayColors && (
				<>
					<box width={50} justifyContent="center" alignItems="center">
						<box
							backgroundColor={'white'}
							padding={1}
							paddingLeft={4}
							paddingRight={4}
							marginTop={1}
						>
							<text fg="#000000">[S]tart</text>
						</box>
						<select
							marginTop={1}
							paddingTop={1}
							style={{ height: 8, width: 20, selectedTextColor: "white", focusedTextColor:"#333333" }}
							options={options}
							focused={true}
							onChange={(index, option) => {
								setSelectedIndex(index)
							}}
						/>
					</box>
				</>
			)}
			{state.displayColors && colorsPalette && (
				<Palette
					colorsPalette={colorsPalette}
					position={position}
					width={width}
					count={countColorsPalette}
				/>
			)}

			{/*   {state.pause && position > 0 && state.displayColors && (             */}
			{/* )} */}
			{/*   {state.pause && position < colorsPalette.length - 1 && state.displayColors && (             */}
			{/* )} */}
			{state.pause && state.displayColors && (
				<>
					{/* TODO: Add logic to arrows  */}

					<text
						fg="#ffffff"
						position="absolute"
						right={width / 1.4}
						top={height / 2}
					>{`<-- [B]ack`}</text>
					<text
						fg="#ffffff"
						position="absolute"
						left={width / 1.4}
						top={height / 2}
					>{`[N]ext -->`}</text>

					<box flexDirection="row">
						<box
							backgroundColor="#000000"
							padding={1}
							paddingLeft={4}
							paddingRight={4}
							marginTop={1}
						>
							<text fg="#ffffff">[C]ontinue</text>
						</box>
						<box
							backgroundColor={'#000000'}
							padding={1}
							paddingLeft={4}
							paddingRight={4}
							marginTop={1}
						>
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
