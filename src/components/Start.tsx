import { useContext, useState } from 'react'
import { useKeyboardStart } from '../keyboard'
import { BaseHexColor } from '../hex'
import { type sizeT } from '../types'
import { Palette } from './start/Colors'
import { Modal } from './Modal'
import { Header } from './start/Header'
import { StartButton } from './start/StartButton'
import { SelectMode } from './start/SelectMode'
import { SizeContext } from '../index'

export function StartScreen({ enterOptions, isActive }: { enterOptions: () => void; isActive: boolean }) {
  const [colorsPalette, setColorsPalette] = useState<BaseHexColor[][]>([[]])
  const [sizeColorsPalette, setSizeColorsPalette] = useState<number>(3)
  const [displayColors, setDisplayColors] = useState<boolean>(false)
  const [position, setPosition] = useState<number>(0)
  const [selectedIndex, setSelectedIndex] = useState<number>(0)
  const [displayModal, setDisplayModal] = useState<boolean>(false)

  const [pause, setPause] = useState<boolean>(true)
  const size = useContext<sizeT>(SizeContext)

  const states = {
    colorsPalette,
    setColorsPalette,
    displayColors,
    setDisplayColors,
    position,
    setPosition,
    sizeColorsPalette,
    setSizeColorsPalette,
    selectedIndex,
    setSelectedIndex,
    pause,
    setPause,
    enterOptions,
    setDisplayModal,
  }
  const displayLogo = size.height > 13 || !displayColors

  useKeyboardStart(states)

  if (isActive)
    return (
      <>
        <box alignItems="center" justifyContent="center" flexGrow={1} marginTop={1}>
          <Header activate={displayLogo} />
          {!displayColors && (
            <>
              <StartButton height={size.height} />
              <SelectMode height={size.height} setIndex={setSelectedIndex} />
            </>
          )}
          {displayColors && <Palette colorsPalette={colorsPalette} position={position} count={sizeColorsPalette} />}

          {pause && displayColors && (
            <>
              <text
                fg="#ffffff"
                position="absolute"
                right={size.width / 1.4}
                top={size.height / 1.9}
              >{`${position === 1 ? '' : '<-- [B]ack'}`}</text>
              <text
                fg="#ffffff"
                position="absolute"
                left={size.width / 1.4}
                top={size.height / 1.9}
              >{`${position === colorsPalette.length - 1 ? '' : '[N]ext -->'}`}</text>

              <box flexDirection="row" position="absolute" top={size.height / 1.6}>
                <box padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
                  <text fg="#ffffff">[C]ontinue</text>
                </box>
                <box padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
                  <text fg="#ffffff">[S]ave palette</text>
                </box>
              </box>
            </>
          )}
          {!pause && displayColors && (
            <>
              <text fg="#ffffff" position="absolute" top={size.height / 1.5}>
                Press [space] to stop
              </text>
            </>
          )}
        </box>
        <box padding={1}>
          <box top={size.height - 2} left={5} position="absolute" width={20}>
            <text fg="#ffffff" style={{ marginBottom: 1 }}>
              {!displayColors ? '[O]ptions' : '[E]xit'}
            </text>
          </box>
          <Modal activate={displayModal} />
        </box>
      </>
    )
}
