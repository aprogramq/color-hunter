import type { BaseHexColor } from '../hex'

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

	for (let i = 0; i < count; i++) {
		colorsDisplay.push(
			<box
				justifyContent="center"
				alignItems="center"
				backgroundColor={colorsPalette[position]![i]?.get()}
				width="10%"
				height="60%"
			>
				<text justifyContent="center" fg={colorsPalette[position]![i]?.textColor}>
					{colorsPalette[position]![i] && colorsPalette[position]![i]!.get()}
				</text>
			</box>
		)
	}
	return (
		<>
			<box flexDirection="row" height={10} width={160} justifyContent="center" alignItems="center">
				{colorsDisplay}
			</box>
		</>
	)
}
