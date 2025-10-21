import { hexToRgb, KeyHandler, RGBA, TextAttributes } from '@opentui/core'
import { useKeyboard } from '@opentui/react'
import { useEffect, useState } from 'react'
import { Buffer } from 'node:buffer'
import fs from 'fs'

export function Options({ exitOption }) {
	const [pathInput, setPathInput] = useState<string>('')
	const [positionFocusedInput, setPositionFocusedInput] = useState<number>(1)
	const [selectionMode, setSelectionMode] = useState<boolean>(true)
	const [displaySaveMessage, setDisplaySaveMessage] = useState<boolean>(false)

	const pathToSettingFile = '/home/aprogramb/.config/color-hunter/settings.json'
	const file = fs.readFileSync(pathToSettingFile)
	const jsonFile = JSON.parse(file.toString())

	useKeyboard((key) => {
		if (key.name === 'j' && selectionMode) setPositionFocusedInput((pos) => pos + 1)
		else if (key.name === 'k' && selectionMode) setPositionFocusedInput((pos) => (pos > 1 ? pos - 1 : pos))
		else if (key.name === 'escape') setSelectionMode(true)
		else if (key.name === 'return' && selectionMode) setSelectionMode(false)
		else if (key.name === 'return' && !selectionMode) {
			jsonFile['savePath'] = pathInput
			console.log(pathInput)
			fs.writeFileSync(pathToSettingFile, JSON.stringify(jsonFile, null, 4))
			setDisplaySaveMessage(true)
		} else if (key.name === 'e' && selectionMode) exitOption()
	})
	return (
		<>
			<box>
				<box justifyContent="center" alignItems="center" marginTop={3} height={'50%'}>
					<ascii-font font="tiny" text="Options" fg={hexToRgb('#7c86ff')} />
					<box title="Path to Save" width={50} marginTop={1} borderStyle="single">
						<input
							value={jsonFile['savePath']}
							onInput={(val) => setPathInput(val)}
							backgroundColor={positionFocusedInput === 1 ? '#333333' : '#000000'}
							padding={1}
							focused={positionFocusedInput === 1 && !selectionMode}
							textColor="#4f46e5"
						/>
					</box>
				</box>
				{displaySaveMessage && (
					<box backgroundColor={'#05df72'} position="absolute" right={0} width={20} height={2}>
						<text fg={'#0d542b'}>Successfully</text>
					</box>
				)}
			</box>
		</>
	)
}
