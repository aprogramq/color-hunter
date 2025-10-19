import { TextAttributes } from '@opentui/core'
import { render, useRenderer } from '@opentui/react'
import { useEffect, useState } from 'react'
import { Palette } from './components/Colors'
import { options } from './types'
import type { BaseHexColor } from './hex'
import { useKeyboardState } from './keyboard'

function App() {
	const [colorsPalette, setColorsPalette] = useState<BaseHexColor[][]>([[]])
	const [displayColors, setDisplayColors] = useState<boolean>(false)
	const [position, setPosition] = useState<number>(0)
	const [countColorsPalette, setCountColorsPalette] = useState<number>(3)
	const [selectedIndex, setSelectedIndex] = useState(0)

	const [pause, setPause] = useState<boolean>(true)
	const [timeout, setTimeout] = useState<NodeJS.Timeout>()

	const render = useRenderer()
	const height: number = render.height
	const width: number = render.width
	// useEffect(() => {
	// 	render.console.show()
	// 	console.log(position)
	// })

	useKeyboardState({
		colorsPalette,
		setColorsPalette,
		displayColors,
		setDisplayColors,
		position,
		setPosition,
		countColorsPalette,
		setCountColorsPalette,
		selectedIndex,
		setSelectedIndex,
		pause,
		setPause,
		timeout,
		setTimeout,
	})

	return (
		<box alignItems="center" justifyContent="center" flexGrow={1}>
			<box justifyContent="center" alignItems="flex-end">
				<ascii-font font="tiny" text="Color Hunter :D" />
				<text attributes={TextAttributes.DIM} fg="#7c86ff">
					You’ll find what you need
				</text>
			</box>

			{!displayColors && (
				<>
					<box width={50} justifyContent="center" alignItems="center">
						<box backgroundColor={'white'} padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
							<text fg="#000000">[S]tart</text>
						</box>
						<select
							marginTop={1}
							paddingTop={1}
							style={{
								height: 8,
								width: 20,
								selectedTextColor: 'white',
								focusedTextColor: '#333333',
							}}
							options={options}
							focused={true}
							onChange={(index, option) => {
								setSelectedIndex(index)
							}}
						/>
					</box>
				</>
			)}
			{displayColors && colorsPalette && (
				<Palette colorsPalette={colorsPalette} position={position} width={width} count={countColorsPalette} />
			)}

			{pause && displayColors && (
				<>
					{/* TODO: Add logic to arrows  */}

					<text
						fg="#ffffff"
						position="absolute"
						right={width / 1.4}
						top={height / 2}
					>{`${position === 1 ? '' : '<-- [B]ack'}`}</text>
					<text
						fg="#ffffff"
						position="absolute"
						left={width / 1.4}
						top={height / 2}
					>{`${position === colorsPalette.length - 1 ? '' : '[N]ext -->'}`}</text>

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
			{!pause && displayColors ? (
				<box backgroundColor="#000000" padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
					<text fg="#ffffff">Press [space] to stop</text>
				</box>
			) : null}
		</box>
	)
}

render(<App />)
