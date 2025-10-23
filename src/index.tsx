import { render, useKeyboard, useRenderer } from '@opentui/react'
import { createContext, useEffect, useState } from 'react'
import { Palette } from './components/Colors'
import { options, type sizeT } from './types'
import type { BaseHexColor } from './hex'
import { useKeyboardState } from './keyboard'
import { StartScreen } from './components/Start'
import { Options } from './components/Options'
import { initialize } from './init'

export const SizeContext = createContext<sizeT>({height:0, width:0})

function App() {
	const [diplsayStartScreen, setDisplayStartScreen] = useState<boolean>(true)
	const [displayOptions, setDisplayOptions] = useState<boolean>(false)
	const render = useRenderer()
	// useEffect(()=>{
	// 	render.console.show()
	// })

	const height: number = render.height
	const width: number = render.width
	const size = {height: render.height, width: render.width}

	initialize()

	const enterOptions = () => {
		setDisplayOptions(true)
		setDisplayStartScreen(false)
	}
	const exitOptions = () => {
		setDisplayOptions(false)
		setDisplayStartScreen(true)
	}
	return (
		<>
			<SizeContext value={size}>
				{diplsayStartScreen && <StartScreen enterOptions={enterOptions} />}
				{displayOptions && <Options exitOptions={exitOptions} />}
				{/* <Options/> */}
			</SizeContext>
		</>
	)
}

render(<App />)
