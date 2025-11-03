import { useContext } from 'react'
import type { BaseHexColor } from '../../hex'
import type { sizeT } from '../../types'
import { SizeContext } from '../../index.tsx'

export function Palette({
  colorsPalette,
  position,
  count,
}: {
  colorsPalette: BaseHexColor[][]
  position: number
  count: number
}) {
  const colorsDisplay = []
  const size = useContext<sizeT>(SizeContext)

  for (let i = 0; i < count; i++) {
    colorsDisplay.push(
      <box
        key={i.toString()}
        justifyContent="center"
        alignItems="center"
        backgroundColor={colorsPalette[position]![i]?.get()}
        width="10%"
        height={size.height > 9 ? '60%' : '100%'}
      >
        <text justifyContent="center" alignItems="center" fg={colorsPalette[position]![i]?.textColor}>
          {colorsPalette[position]![i] && colorsPalette[position]![i]!.get()}
        </text>
      </box>
    )
  }
  if (colorsPalette)
    return (
      <>
        <box flexDirection="row" height={10} width={160} justifyContent="center" alignItems="center">
          {colorsDisplay}
        </box>
      </>
    )
}
