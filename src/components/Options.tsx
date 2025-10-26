import { hexToRgb } from '@opentui/core'
import { useKeyboard } from '@opentui/react'
import { useState } from 'react'
import fs from 'fs'
import { Modal } from './modals/Modal'

export function Options({ exitOptions, isActive }: { exitOptions: () => void; isActive: boolean }) {
	const [positionFocusedInput, setPositionFocusedInput] = useState<number>(1)
	const [selectionMode, setSelectionMode] = useState<boolean>(true)
	const [displayModalMessage, setDisplaySaveMessage] = useState<boolean>(false)

	const pathToSettingFile = '/home/aprogramb/.config/color-hunter/settings.json'
	const jsonFile = JSON.parse(fs.readFileSync(pathToSettingFile).toString())

	const [pathInput, setPathInput] = useState<string>(jsonFile['savePath'])

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
			setTimeout(() => setDisplaySaveMessage(false), 2000)
			setSelectionMode(true)
		} else if (key.name === 'e' && selectionMode) exitOptions()
	})
	if (isActive)
		return (
			<>
				<box>
					<box justifyContent="center" alignItems="center" marginTop={3} height={'50%'}>
						<ascii-font font="tiny" text="Options" fg={hexToRgb('#7c86ff')} />
						<box title="Path to Save" width={50} marginTop={1} borderStyle="single">
							<input
								focusedBackgroundColor="#222222"
								value={jsonFile['savePath']}
								onInput={(val) => setPathInput(val)}
								backgroundColor={positionFocusedInput === 1 ? '#111111' : '#000000'}
								padding={1}
								focused={positionFocusedInput === 1 && !selectionMode}
								textColor={selectionMode ? '#4f46e5' : '#000000'}
							/>
						</box>
					</box>
					<Modal activate={displayModalMessage} />
				</box>
				<box padding={1}>
					<box top={25} left={5} position="absolute" width={50}>
						<text fg="#ffffff" style={{ marginBottom: 1 }}>
							{selectionMode ? '[E]xit' : '[Esc] to selection mode'}
						</text>
					</box>
				</box>
			</>
		)
}
