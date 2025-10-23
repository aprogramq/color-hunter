import { useContext, useEffect, useEffectEvent, useState } from 'react'
import { useKeyboardState } from '../keyboard'
import { TextAttributes } from '@opentui/core'
import { useRenderer } from '@opentui/react'
import { BaseHexColor } from '../hex'
import { options, type sizeT } from '../types'
import { Palette } from './Colors'
import { ModalSuccess } from './modals/ModalSuccess'
import { SizeContext } from '..'

export function StartScreen({ enterOptions }: { enterOptions: () => void }) {
	const [colorsPalette, setColorsPalette] = useState<BaseHexColor[][]>([[]])
	const [displayColors, setDisplayColors] = useState<boolean>(false)
	const [position, setPosition] = useState<number>(0)
	const [countColorsPalette, setCountColorsPalette] = useState<number>(3)
	const [selectedIndex, setSelectedIndex] = useState(0)
	const [displayOptions, setDisplayOptions] = useState<boolean>(false)
	const [displaySaveMessage, setDisplaySaveMessage] = useState<boolean>(false)

	const [pause, setPause] = useState<boolean>(true)
	const [timeout, setTimeout] = useState<NodeJS.Timeout>()
	const size = useContext<sizeT>(SizeContext)

	const states = {
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
		enterOptions,
		setDisplaySaveMessage,
	}
	const displayLogo = size.height > 13 || !displayColors
	console.log(size.height)

	useKeyboardState(states)

	return (
		<>
			<box alignItems="center" justifyContent="center" flexGrow={1} marginTop={1}>
				{displayLogo && (
					<box justifyContent="center" alignItems="flex-end">
						<ascii-font font="tiny" text="Color Hunter :D" />
						<text attributes={TextAttributes.DIM} fg="#7c86ff">
							You’ll find what you need
						</text>
					</box>
				)}

				{!displayColors && !displayOptions && (
					<>
						<box width={50} justifyContent="center" alignItems="center">
							<box
								style={
									size.height > 13
										? { backgroundColor: 'white' }
										: { position: 'absolute', left: 50, top: 7, backgroundColor: 'black' }
								}
								backgroundColor={size.height > 13 ? 'white' : 'black'}
								padding={1}
								paddingLeft={4}
								paddingRight={4}
								marginTop={1}
							>
								<text fg={ size.height>13?"#000000":"#ffffff" }>[S]tart</text>
							</box>
							<select
								marginTop={1}
								paddingTop={1}
								height={size.height > 13 ? 8 : 2}
								style={{
									width: 20,
									selectedTextColor: 'white',
									focusedTextColor: '#333333',
									selectedBackgroundColor: '#352F99',
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
					<Palette colorsPalette={colorsPalette} position={position} count={countColorsPalette} />
				)}

				{pause && displayColors && (
					<>
						<text
							fg="#ffffff"
							position="absolute"
							right={size.width / 1.4}
							top={size.height / 2.1}
						>{`${position === 1 ? '' : '<-- [B]ack'}`}</text>
						<text
							fg="#ffffff"
							position="absolute"
							left={size.width / 1.4}
							top={size.height / 2.1}
						>{`${position === colorsPalette.length - 1 ? '' : '[N]ext -->'}`}</text>

						<box flexDirection="row">
							<box backgroundColor="#000000" padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
								<text fg="#ffffff">[C]ontinue</text>
							</box>
							<box backgroundColor="#000000" padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
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
			<box padding={1}>
				<box top={size.height - 2} left={5} position="absolute" width={20}>
					<text fg="#ffffff" style={{ marginBottom: 1 }}>
						{!displayColors ? '[O]ptions' : '[E]xit'}
					</text>
				</box>
				{displaySaveMessage && <ModalSuccess />}
			</box>
		</>
	)
}
