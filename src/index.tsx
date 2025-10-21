import { render, useKeyboard, useRenderer } from '@opentui/react'
import { useEffect, useState } from 'react'
import { Palette } from './components/Colors'
import { options } from './types'
import type { BaseHexColor } from './hex'
import { useKeyboardState } from './keyboard'
import { StartScreen } from './components/Start'
import { Options } from './components/Options'

function App() {
	const [diplsayStartScreen, setDisplayStartScreen] = useState<boolean>(true)
	const [displayOptions, setDisplayOptions] = useState<boolean>(false)
	const render = useRenderer()
	// useEffect(() => {
	// 	render.console.show()
	// })
	useKeyboard((key) => {
		if (key.name === 'o') {
			setDisplayOptions(true)
			setDisplayStartScreen(false)
		}
	})
	const exit = () => {
		setDisplayOptions(false)
		setDisplayStartScreen(true)
	}
	return (
		<>
			{diplsayStartScreen && <StartScreen />}
			{displayOptions && <Options exitOption={exit} />}
			{/* <Options/> */}
		</>
	)
}

render(<App />)
