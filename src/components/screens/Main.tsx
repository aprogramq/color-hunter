import { useContext, useEffect, useState, type Dispatch, type SetStateAction } from 'react'
import type { BaseHexColor } from '../../hex'
import { type displayT, type sizeT, type UseState, type paletteT, options } from '../../types'
import { SizeContext } from '../..'
import { useKeyboardMain } from '../../keyboard'
import { Modal } from '../Modal'
import { Palette } from '../start/Colors'
import { randomColor } from '../../fuctions'
import { Header } from '../start/Header'
import { ConsolePosition } from '@opentui/core'
import { walkUpBindingElementsAndPatterns } from 'typescript'
import { Hint } from '../Hints'
import os from 'os'
import fs from 'fs'

export default function Main({
  actionOptions,
  selectedIndex,
  setSelectedIndex,
}: {
  actionOptions: (option: displayT) => void
  selectedIndex: number
  setSelectedIndex: UseState<number>
}) {
  const settings = JSON.parse(
    fs.readFileSync(`/home/${os.userInfo().username}/.config/color-hunter/settings.json`).toString()
  )

  const [palette, setPalette] = useState<paletteT>({
    colors: [[]] as BaseHexColor[][],
    size: settings['sizePalette'] > 8 ? 8 : settings['sizePalette'],
    display: false,
    position: 0,
  })
  const [pause, setPause] = useState<boolean>(false)
  const size = useContext<sizeT>(SizeContext)
  const displayLogo = size.height > 13 || !palette.display
  const [displayModal, setDisplayModal] = useState<boolean>(false)
  const [timeout, setMyTimeout] = useState<NodeJS.Timeout>(setTimeout(() => void 1000))

  useEffect(() => {
    setMyTimeout(randomColor(palette, setPalette, options[selectedIndex]!.value))
    setPalette((p) => ({ ...p, display: true }))
  }, [])

  const states = {
    palette,
    setPalette,
    selectedIndex,
    setSelectedIndex,
    pause,
    setPause,
    actionOptions,
    setDisplayModal,
    timeout,
    setMyTimeout,
  }

  useKeyboardMain(states)

  const arrowPosition = () => {
    if (palette.size <= 3) return size.width / (1.8 - palette.size * 0.1)
    else if (palette.size <= 5) return size.width / (1.8 - palette.size * 0.09)
    else if (palette.size <= 8) return size.width / (1.8 - palette.size * 0.085)
  }

  return (
    <box alignItems="center" justifyContent="center" flexGrow={1} marginTop={1}>
      <Modal activate={displayModal} state="success" text="Successfully" />
      <Header activate={displayLogo} />
      <Palette colorsPalette={palette.colors} position={palette.position} count={palette.size} />

      <text
        fg="#ffffff"
        visible={pause}
        position="absolute"
        right={arrowPosition()}
        top={size.height / 1.9}
      >{`${palette.position === 1 ? '' : '<-- [B]ack'}`}</text>
      <text
        fg="#ffffff"
        visible={pause}
        position="absolute"
        left={arrowPosition()}
        top={size.height / 1.9}
      >{`${palette.position === palette.colors.length - 1 ? '' : '[N]ext -->'}`}</text>

      <box flexDirection="row" position="absolute" top={size.height / 1.6} visible={pause}>
        <box padding={1} paddingLeft={4} paddingRight={4} marginTop={1} visible={pause}>
          <text fg="#ffffff">[C]ontinue</text>
        </box>
        <box padding={1} paddingLeft={4} paddingRight={4} marginTop={1} visible={pause}>
          <text fg="#ffffff">[S]ave palette</text>
        </box>
      </box>
      <box visible={!pause} flexDirection="row" position="absolute" top={size.height / 1.5}>
        <text>Press [space] to stop</text>
      </box>

      <Hint text="[E]xit" />
    </box>
  )
}
